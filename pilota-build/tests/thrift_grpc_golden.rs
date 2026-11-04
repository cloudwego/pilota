//! Offline Kitex gRPC payload golden; updating it is an explicit Go-runner
//! step.
use std::fmt::Debug;

use pilota::{
    Bytes, LinkedBytes,
    thrift::{Message, binary::TBinaryProtocol},
};
use serde_json::{Value, json};

include!("../test_data/thrift/grpc_payload.rs");
include!("../test_data/thrift/grpc_types.rs");
use payload::{grpc_parent, grpc_thrift::*};

// An ordinary Thrift message may own non-Sync state. No Default, Clone,
// Debug, or Protobuf implementation is needed to use the payload contract.
struct SendOnly {
    value: Empty,
    _marker: std::cell::Cell<()>,
}

impl Message for SendOnly {
    fn encode<T: pilota::thrift::TOutputProtocol>(
        &self,
        protocol: &mut T,
    ) -> Result<(), pilota::thrift::ThriftException> {
        self.value.encode(protocol)
    }
    fn decode<T: pilota::thrift::TInputProtocol>(
        protocol: &mut T,
    ) -> Result<Self, pilota::thrift::ThriftException> {
        Ok(Self {
            value: Empty::decode(protocol)?,
            _marker: std::cell::Cell::new(()),
        })
    }
    async fn decode_async<T: pilota::thrift::TAsyncInputProtocol>(
        protocol: &mut T,
    ) -> Result<Self, pilota::thrift::ThriftException> {
        Ok(Self {
            value: Empty::decode_async(protocol).await?,
            _marker: std::cell::Cell::new(()),
        })
    }
    fn size<T: pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
        self.value.size(protocol)
    }
}

#[test]
fn ordinary_thrift_message_has_no_default_clone_sync_or_protobuf_requirement() {
    let value = SendOnly {
        value: Empty {},
        _marker: std::cell::Cell::new(()),
    };
    let mut bytes = LinkedBytes::new();
    value
        .encode(&mut TBinaryProtocol::new(&mut bytes, true))
        .unwrap();
    assert_eq!(bytes.concat().as_ref(), [0]);
    let _ = decode::<SendOnly>(&[0]);
}

#[test]
fn arc_service_scalar_void_union_alias_and_recursive_box_data_compile_and_round_trip() {
    use types::rust_types::*;
    let scalar = TypesScalarArgsSend { value: 42 };
    let mut output = LinkedBytes::new();
    scalar
        .encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    assert_eq!(decode::<TypesScalarArgsRecv>(&output.concat()).value, 42);
    let mut output = LinkedBytes::new();
    TypesZeroResultSend::Ok(())
        .encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    assert!(matches!(
        decode::<TypesZeroResultRecv>(&output.concat()),
        TypesZeroResultRecv::Ok(())
    ));
    let message = std::sync::Arc::new(Alias(Event::Value(7)));
    let mut output = LinkedBytes::new();
    message
        .encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    assert_eq!(*decode::<std::sync::Arc<Alias>>(&output.concat()), *message);
    let node = Node {
        next: Some(Box::new(Node { next: None })),
    };
    let mut output = LinkedBytes::new();
    node.encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    assert_eq!(decode::<Node>(&output.concat()), node);
}

fn decode<M: Message>(bytes: &[u8]) -> M {
    try_decode::<M>(bytes).unwrap()
}

fn try_decode<M: Message>(bytes: &[u8]) -> Result<M, pilota::thrift::ThriftException> {
    let mut bytes = Bytes::copy_from_slice(bytes);
    M::decode(&mut TBinaryProtocol::new(&mut bytes, true))
}

fn reencode<M: Message + PartialEq + Debug>(bytes: &[u8]) -> Vec<u8> {
    let value = decode::<M>(bytes);
    let size = value.size(&mut TBinaryProtocol::new((), false));
    let mut output = LinkedBytes::new();
    value
        .encode(&mut TBinaryProtocol::new(&mut output, true))
        .unwrap();
    let output = output.concat();
    assert_eq!(output.len(), size);
    assert_eq!(decode::<M>(&output), value);
    output.to_vec()
}

#[test]
fn go_grpc_raw_args_result_and_all_declared_exceptions_decode_offline() {
    let mut report: Value = serde_json::from_str(include_str!(
        "../../tests/interop/kitex-codec/grpc-payload-golden.json"
    ))
    .unwrap();
    assert_eq!(report["producer"], "kitex-grpc-codec");
    assert_eq!(report["modules"]["github.com/cloudwego/kitex"], "v0.16.4");
    assert_eq!(report["modules"]["github.com/cloudwego/thriftgo"], "v0.4.5");
    for row in report["cases"].as_array_mut().unwrap() {
        let bytes = hex::decode(row["hex"].as_str().unwrap()).unwrap();
        let frame = hex::decode(row["frame_hex"].as_str().unwrap()).unwrap();
        assert_eq!(frame[0], 0);
        assert_eq!(
            u32::from_be_bytes(frame[1..5].try_into().unwrap()) as usize,
            bytes.len()
        );
        assert_eq!(frame[5..], bytes);
        let rust = match row["type"].as_str().unwrap() {
            "Message" => reencode::<grpc_parent::Message>(&bytes),
            "Empty" => reencode::<Empty>(&bytes),
            "InteropUnaryArgs" => reencode::<InteropUnaryArgsSend>(&bytes),
            "InteropUnaryResult" => reencode::<InteropUnaryResultRecv>(&bytes),
            "InteropExplicitUnaryArgs" => reencode::<InteropExplicitUnaryArgsSend>(&bytes),
            "InteropExplicitUnaryResult" => reencode::<InteropExplicitUnaryResultRecv>(&bytes),
            "InteropZeroArgs" => reencode::<InteropZeroArgsSend>(&bytes),
            "InteropZeroResult" => reencode::<InteropZeroResultRecv>(&bytes),
            "InteropMultiArgs" => reencode::<InteropMultiArgsSend>(&bytes),
            "InteropMultiResult" => reencode::<InteropMultiResultRecv>(&bytes),
            "InteropContainerArgs" => reencode::<InteropContainerArgsSend>(&bytes),
            "InteropContainerResult" => reencode::<InteropContainerResultRecv>(&bytes),
            "InteropThrowArgs" => reencode::<InteropThrowArgsSend>(&bytes),
            "InteropThrowResult" => reencode::<InteropThrowResultRecv>(&bytes),
            "InteropVoidThrowArgs" => reencode::<InteropVoidThrowArgsSend>(&bytes),
            "InteropVoidThrowResult" => reencode::<InteropVoidThrowResultRecv>(&bytes),
            other => panic!("uncovered Go payload type {other}"),
        };
        if row["name"] == "empty-stop"
            || row["name"] == "void-result"
            || row["name"] == "void-throws-success"
        {
            assert_eq!(rust, [0]);
        }
        // Field ordering is legal in both encoders. The dynamic Go verifier
        // decodes these bytes and compares Go's canonical re-encoding, so an
        // order difference is never mistaken for a semantic difference.
        row["rust_hex"] = json!(hex::encode(rust));
    }
    report["producer"] = json!("rust");
    if let Some(output) = std::env::var_os("PILOTA_GRPC_VERIFY_OUTPUT") {
        std::fs::write(output, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
}

#[test]
fn args_wrap_raw_struct_and_results_keep_the_success_and_exception_ids() {
    let report: Value = serde_json::from_str(include_str!(
        "../../tests/interop/kitex-codec/grpc-payload-golden.json"
    ))
    .unwrap();
    let bytes = |name: &str| {
        hex::decode(
            report["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|row| row["name"] == name)
                .unwrap()["hex"]
                .as_str()
                .unwrap(),
        )
        .unwrap()
    };
    let raw = bytes("streaming-raw");
    let args = bytes("unary-args");
    assert_ne!(raw, args);
    let message = decode::<grpc_parent::Message>(&raw);
    assert_eq!(message.payload.as_deref(), Some("hello\0中文".as_bytes()));
    assert_eq!(message.sequence, Some(i64::MIN));
    assert_eq!(decode::<InteropUnaryArgsSend>(&args).request, message);
    let multi = decode::<InteropMultiArgsSend>(&bytes("multi-args-non-default-ids"));
    assert_eq!(multi.left, i32::MIN);
    assert_eq!(multi.right, i64::MAX);
    assert!(matches!(
        decode::<InteropThrowResultRecv>(&bytes("throws-failure-id-2")),
        InteropThrowResultRecv::Failure(_)
    ));
    assert!(matches!(
        decode::<InteropThrowResultRecv>(&bytes("throws-other-id-7")),
        InteropThrowResultRecv::Other(_)
    ));
    assert!(matches!(
        decode::<InteropVoidThrowResultRecv>(&bytes("void-throws-success")),
        InteropVoidThrowResultRecv::Ok(())
    ));

    // Both single and multiple throws use an enum from the same included file
    // definitions. No transport-specific copy of the exception types is needed.
    let one = InteropVoidThrowException::Failure(grpc_parent::Failure::default());
    let many = InteropThrowException::Other(grpc_parent::Other::default());
    assert!(matches!(one, InteropVoidThrowException::Failure(_)));
    assert!(matches!(many, InteropThrowException::Other(_)));
}

#[test]
fn malformed_result_is_not_silently_converted_to_a_default_success() {
    let report: Value = serde_json::from_str(include_str!(
        "../../tests/interop/kitex-codec/grpc-payload-golden.json"
    ))
    .unwrap();
    for row in report["boundary"].as_array().unwrap() {
        let bytes = hex::decode(row["hex"].as_str().unwrap()).unwrap();
        match row["name"].as_str().unwrap() {
            "zero-bytes" => {
                assert_eq!(row["accepted"], false);
                assert!(try_decode::<Empty>(&bytes).is_err());
            }
            "empty-result-without-success" => {
                assert_eq!(row["accepted"], true); // Go data codec accepts; its caller checks success.
                assert!(try_decode::<InteropUnaryResultRecv>(&bytes).is_err());
            }
            "multiple-result-fields" => {
                assert_eq!(row["accepted"], true); // Intentional stricter Rust union validation.
                assert!(try_decode::<InteropThrowResultRecv>(&bytes).is_err());
            }
            "valid-stop-with-suffix" => {
                assert_eq!(row["accepted"], true);
                let mut src = Bytes::from(bytes);
                Empty::decode(&mut TBinaryProtocol::new(&mut src, true)).unwrap();
                assert_eq!(src.as_ref(), [0xa1, 0xb2]);
            }
            other => panic!("uncovered boundary {other}"),
        }
    }
    // Unknown exception alone cannot be mistaken for a successful result.
    let unknown = Bytes::from_static(b"\x08\x00\x63\x00\x00\x00\x01\x00");
    assert!(try_decode::<InteropThrowResultRecv>(&unknown).is_err());
}
