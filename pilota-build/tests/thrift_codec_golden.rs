//! Offline, committed golden bytes produced by the fixed Kitex Go generator.
use pilota::{
    Bytes, LinkedBytes,
    thrift::{ApplicationException, Message, binary::TBinaryProtocol},
};
use serde_json::{Value, json};

include!("../test_data/thrift/ttstream_codec.rs");
use ttstream_codec::codec::{CodecMessage, Nested};

fn nested(value: &Nested) -> Value {
    let mut result = json!({"value":value.value});
    if let Some(label) = &value.label {
        result["label"] = json!(label.as_str());
    }
    result
}

// Go's generated defaults and JSON omitempty rules form the semantic oracle;
// Rust retains field presence separately in Option, without changing its API.
fn semantics(value: &CodecMessage) -> Value {
    let mut ids = value
        .ids
        .as_ref()
        .map(|ids| ids.iter().copied().collect::<Vec<_>>());
    if let Some(ids) = &mut ids {
        ids.sort_unstable();
    }
    let map = value.nested.as_ref().map(|map| {
        map.iter()
            .map(|(key, value)| (key.to_string(), json!(value)))
            .collect::<serde_json::Map<_, _>>()
    });
    let mut result = json!({
        "integer":value.integer.unwrap_or_default(),
        "flag":value.flag.unwrap_or_default(),
        "real_bits":format!("{:016x}",value.real.unwrap_or_default().to_bits()),
        "text":value.text.as_deref().unwrap_or_default(),
        "blob":hex::encode(value.blob.as_deref().unwrap_or_default()),
        "numbers":value.numbers,
        "nested":map,
        "ids":ids,
        "default_text":value.default_text.as_deref().unwrap_or("default"),
    });
    if let Some(note) = &value.note {
        result["note"] = json!(note.as_str());
    }
    if let Some(child) = &value.child {
        result["child"] = nested(child);
    }
    if let Some(children) = &value.children {
        if !children.is_empty() {
            result["children"] = Value::Array(children.iter().map(nested).collect());
        }
    }
    result
}

fn encoded<M: Message>(message: &M) -> Vec<u8> {
    let expected_size = message.size(&mut TBinaryProtocol::new((), false));
    let mut output = LinkedBytes::with_capacity(expected_size);
    message
        .encode(&mut TBinaryProtocol::new(&mut output, false))
        .unwrap();
    let output = output.concat();
    assert_eq!(
        expected_size,
        output.len(),
        "size must match the raw struct encoding"
    );
    output.to_vec()
}

#[test]
fn go_golden_decode_size_consumption_and_rust_encoding() {
    let mut report: Value =
        serde_json::from_str(include_str!("../../tests/interop/kitex-codec/golden.json")).unwrap();
    assert_eq!(report["schemaVersion"], 1);
    for case in report["cases"].as_array_mut().unwrap() {
        let name = case["name"].as_str().unwrap().to_owned();
        let bytes = hex::decode(case["hex"].as_str().unwrap()).unwrap();
        assert_eq!(
            bytes.len(),
            case["expected_consumed"].as_u64().unwrap() as usize
        );
        // Preserve a suffix to verify the codec consumes exactly one struct.
        let mut framed = bytes.clone();
        framed.extend_from_slice(b"suffix");
        let mut input = Bytes::from(framed);
        let rust_bytes = match case["type"].as_str().unwrap() {
            "CodecMessage" => {
                let message =
                    CodecMessage::decode(&mut TBinaryProtocol::new(&mut input, false)).unwrap();
                assert_eq!(semantics(&message), case["input"], "{name}");
                encoded(&message)
            }
            "ApplicationException" => {
                let message =
                    ApplicationException::decode(&mut TBinaryProtocol::new(&mut input, false))
                        .unwrap();
                assert_eq!(
                    message.kind().as_i32() as i64,
                    case["input"]["code"].as_i64().unwrap(),
                    "{name}"
                );
                assert_eq!(
                    message.message().as_str(),
                    case["input"]["message"].as_str().unwrap(),
                    "{name}"
                );
                encoded(&message)
            }
            other => panic!("unsupported golden type {other}"),
        };
        assert_eq!(
            &input[..],
            b"suffix",
            "{name}: decoder consumed beyond its message"
        );
        if case["canonical_bytes"].as_bool().unwrap() {
            assert_eq!(rust_bytes, bytes, "{name}");
        }
        case["hex"] = json!(hex::encode(&rust_bytes));
        case["expected_consumed"] = json!(rust_bytes.len());
    }
    // Explicit fixture maintenance/integration only. Ordinary cargo test stays
    // offline, never invokes Go, and never updates the checked-in golden.
    if let Some(output) = std::env::var_os("PILOTA_CODEC_VERIFY_OUTPUT") {
        std::fs::write(output, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
}

#[test]
fn ordinary_message_and_typedef_have_no_transport_dependency() {
    let original = CodecMessage {
        integer: Some(42),
        ..Default::default()
    };
    let alias = ttstream_codec::codec::MessageAlias(original);
    assert_eq!(encoded(&alias), encoded(&alias.0));
}

#[test]
fn missing_required_field_differs_from_an_explicit_zero_value() {
    let mut missing = Bytes::from_static(b"\0");
    let error = Nested::decode(&mut TBinaryProtocol::new(&mut missing, true)).unwrap_err();
    assert!(error.to_string().contains("required"));
    let mut explicit_zero = Bytes::from_static(b"\x08\0\x01\0\0\0\0\0");
    let value = Nested::decode(&mut TBinaryProtocol::new(&mut explicit_zero, true)).unwrap();
    assert_eq!(value.value, 0);
    assert_eq!(value.label, None);
}

include!("../test_data/thrift/ttstream_typing.rs");

#[test]
fn prelude_names_recursive_typedefs_and_arc_service_generation_compile() {
    use ttstream_typing::typing::{Option as Response, RequestAlias, Some as Request};
    let request = RequestAlias(Request::default());
    let response = Response::default();
    for payload in [encoded(&request), encoded(&response)] {
        assert!(!payload.is_empty());
        assert_eq!(payload.last(), Some(&0));
    }
}
