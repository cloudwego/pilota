//! Generated codecs checked against the Apache Thrift binary protocol.
use pilota::{
    Bytes, LinkedBytes,
    thrift::{Message, binary::TBinaryProtocol},
};

include!("../test_data/thrift/ttstream_codec.rs");
include!("../test_data/thrift/ttstream_typing.rs");
use ttstream_codec::codec::{CodecMessage, Nested};

fn encoded<M: Message>(message: &M) -> Vec<u8> {
    let size = message.size(&mut TBinaryProtocol::new((), false));
    let mut output = LinkedBytes::new();
    message
        .encode(&mut TBinaryProtocol::new(&mut output, false))
        .unwrap();
    let bytes = output.concat();
    assert_eq!(size, bytes.len());
    bytes.to_vec()
}

#[test]
fn required_i32_uses_a_type_byte_big_endian_field_id_value_and_stop() {
    // https://github.com/apache/thrift/blob/master/doc/specs/thrift-binary-protocol.md#struct-encoding
    // I32 = 8, field id = 1, followed by the i32 value and STOP = 0.
    for value in [i32::MIN, -1, 0, 42, i32::MAX] {
        let message = Nested { value, label: None };
        let mut expected = vec![8, 0, 1];
        expected.extend_from_slice(&value.to_be_bytes());
        expected.push(0);
        assert_eq!(encoded(&message), expected);

        expected.extend_from_slice(b"suffix");
        let mut input = Bytes::from(expected);
        assert_eq!(
            Nested::decode(&mut TBinaryProtocol::new(&mut input, true)).unwrap(),
            message
        );
        assert_eq!(input.as_ref(), b"suffix");
    }
}

#[test]
fn ordinary_message_and_typedef_share_the_same_encoding() {
    let message = CodecMessage {
        integer: Some(42),
        ..Default::default()
    };
    let alias = ttstream_codec::codec::MessageAlias(message);
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

#[test]
fn prelude_names_recursive_typedefs_and_arc_service_generation_compile() {
    use ttstream_typing::typing::{Option as Response, RequestAlias, Some as Request};
    for bytes in [
        encoded(&RequestAlias(Request::default())),
        encoded(&Response::default()),
    ] {
        assert_eq!(bytes.last(), Some(&0));
    }
}
