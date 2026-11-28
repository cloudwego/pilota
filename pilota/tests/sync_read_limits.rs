use pilota::{
    Bytes,
    thrift::{TInputProtocol, TType, binary::TBinaryProtocol, compact::TCompactInputProtocol},
};

#[test]
fn strings_validate_utf8_and_report_truncated_lengths_as_size_limit() {
    use pilota::thrift::{ProtocolExceptionKind, ThriftException};
    for (compact, bytes, kind) in [
        (
            false,
            &[0, 0, 0, 1, 0xff][..],
            ProtocolExceptionKind::InvalidData,
        ),
        (true, &[1, 0xff][..], ProtocolExceptionKind::InvalidData),
        (
            false,
            &[0, 0, 0, 2, 1][..],
            ProtocolExceptionKind::SizeLimit,
        ),
        (true, &[2, 1][..], ProtocolExceptionKind::SizeLimit),
    ] {
        let mut input = Bytes::copy_from_slice(bytes);
        let result = if compact {
            TCompactInputProtocol::new(&mut input).read_string()
        } else {
            TBinaryProtocol::new(&mut input, true).read_string()
        };
        assert!(
            matches!(result, Err(ThriftException::Protocol(ref error)) if error.kind() == kind),
            "{result:?}"
        );
    }
}

#[test]
fn compact_skip_handles_inline_bool_varints_binary_and_nested_fields() {
    let bytes = Bytes::from_static(&[
        0x11, 0x15, 2, 0x18, 2, 0xff, 0xfe, 0x1c, 0x45, 2, 0, 0x15, 4, 0, 42,
    ]);
    let mut input = bytes.clone();
    let mut p = TCompactInputProtocol::new(&mut input);
    assert_eq!(p.skip(TType::Struct).unwrap(), bytes.len() - 1);
    assert_eq!(p.read_byte().unwrap(), 42);
}

#[test]
fn compact_nested_struct_restores_parent_field_id() {
    let mut input = Bytes::from_static(&[0x1c, 0x45, 2, 0, 0x15, 4, 0]);
    let mut p = TCompactInputProtocol::new(&mut input);
    p.read_struct_begin().unwrap();
    assert_eq!(p.read_field_begin().unwrap().id, Some(1));
    p.skip(TType::Struct).unwrap();
    assert_eq!(p.read_field_begin().unwrap().id, Some(2));
    assert_eq!(p.read_i32().unwrap(), 2);
    assert_eq!(p.read_field_begin().unwrap().field_type, TType::Stop);
    p.read_struct_end().unwrap();
}

#[test]
fn byte_lengths_and_utf8_are_checked_without_panicking() {
    for value in [&[0xff, 0xff, 0xff, 0xff][..], &[0x7f, 0xff, 0xff, 0xff][..]] {
        let mut input = Bytes::copy_from_slice(value);
        assert!(TBinaryProtocol::new(&mut input, true).read_bytes().is_err());
    }
    let mut input = Bytes::from_static(&[0, 0, 0, 1, 0xff]);
    assert!(
        TBinaryProtocol::new(&mut input, true)
            .read_faststr()
            .is_err()
    );
    let mut input = Bytes::from_static(&[0xff, 0xff, 0xff, 0xff, 0x0f]);
    assert!(
        TCompactInputProtocol::new(&mut input)
            .read_bytes_vec()
            .is_err()
    );
    let mut input = Bytes::from_static(&[1, 0xff]);
    assert!(
        TCompactInputProtocol::new(&mut input)
            .read_faststr()
            .is_err()
    );
}

#[test]
fn compact_skip_respects_depth_and_truncation() {
    let mut input = Bytes::from_static(&[0x1c, 0]);
    assert!(
        TCompactInputProtocol::new(&mut input)
            .skip_till_depth(TType::Struct, 1)
            .is_err()
    );
    let mut input = Bytes::from_static(&[0x18, 2, 1]);
    assert!(
        TCompactInputProtocol::new(&mut input)
            .skip(TType::Struct)
            .is_err()
    );
}
