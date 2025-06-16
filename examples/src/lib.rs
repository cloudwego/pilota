<<<<<<< HEAD
=======
use pilota::{prost::Message as _, thrift::Message as _};

// mod fieldmask;
>>>>>>> 0314c00 (feat(pilota-build): codegen fieldmask)
mod zero_value;
#[test]
fn test_pb_encode_zero_value() {
    use pilota::pb::Message as _;
    let test_data = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("idl")
        .join("zero_value.proto");

    let out_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("zero_value.rs");

    pilota_build::Builder::pb()
        .ignore_unused(false)
        .include_dirs(vec![test_data.parent().unwrap().to_path_buf()])
        .keep_unknown_fields([test_data.clone().into()])
        .compile_with_config(
            vec![pilota_build::IdlService::from_path(test_data.to_path_buf())],
            pilota_build::Output::File(out_path.into()),
        );

    let mut a = zero_value::zero_value::A::default();

    a.str_map.insert("key1".into(), "value".into());
    a.str_map.insert("key2".into(), "".into());
    a.s1 = "s1".into();
    a.s2 = Some("s2".into());

    // encode a
    let mut encode_a = pilota::pb::LinkedBytes::new();
    a.encode(&mut encode_a).unwrap();
    let encoded_a = encode_a.bytes().clone().freeze();
    println!("encode a: {:?}", encoded_a);

    // decode a
    let decode_a = encoded_a.clone();
    let decode_unknown_a = encoded_a.clone();
    let decoded_a = zero_value::zero_value::A::decode(decode_a).unwrap();
    println!("decode a: {:?}", decoded_a);

    // decode unknown_a
    let decoded_unknown_a = zero_value::zero_value::UnknownA::decode(decode_unknown_a).unwrap();
    println!("decode a to unknown_a: {:?}", decoded_unknown_a);

    // encode unknown_a
    let mut encode_unknown_a = pilota::pb::LinkedBytes::new();
    decoded_unknown_a.encode(&mut encode_unknown_a).unwrap();
    let encoded_unknown_a = encode_unknown_a.bytes().clone().freeze();

    // decode unknown_a
    let decode_unknown_a = encoded_unknown_a.clone();
    let decode_a = encoded_unknown_a.clone();
    println!("encode unknown_a: {:?}", encoded_unknown_a);
    let decoded_unknown_a = zero_value::zero_value::UnknownA::decode(decode_unknown_a).unwrap();
    println!("decode unknown_a: {:?}", decoded_unknown_a);

    // decode a
    let decoded_a = zero_value::zero_value::A::decode(decode_a).unwrap();
    println!("decode unknown_a to a: {:?}", decoded_a);
}

#[test]
fn test_thrift_fieldmask() {
    let test_data = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("idl")
        .join("fieldmask.thrift");

    let out_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("fieldmask.rs");

    pilota_build::Builder::thrift()
        .ignore_unused(false)
        .include_dirs(vec![test_data.parent().unwrap().to_path_buf()])
        .with_descriptor(true)
        .with_field_mask(true)
        .compile_with_config(
            vec![pilota_build::IdlService::from_path(test_data.to_path_buf())],
            pilota_build::Output::File(out_path.into()),
        );

    // let desc = fieldmask::fieldmask::fieldmask::Request::get_descriptor().type_descriptor();
    // println!("{:?}", desc);

    // let request_fieldmask = pilota_thrift_fieldmask::FieldMaskBuilder::new(
    //     &desc,
    //     &[
    //         "$.f1",
    //         "$.f9[1, 3]",
    //         "$.f11.b",
    //         "$.f12[0][*]",
    //         "$.f14{*}",
    //         "$.f15{ \"key1\",\"key3\"}",
    //         "$.f16{\"key1\"}[1].a",
    //         "$.f17[*]{\"key1\"}",
    //     ],
    // )
    // .build()
    // .unwrap();
    // println!("{:?}", request_fieldmask);

    // let mut request = fieldmask::fieldmask::fieldmask::Request {
    //     f1: Some(true),
    //     f2: Some(1),
    //     f3: Some(1),
    //     f4: Some(1),
    //     f5: Some(1),
    //     f6: Some(1.0),
    //     f7: Some("1".into()),
    //     f8: Some(pilota::Bytes::from_static(b"1")),
    //     f9: Some(vec![1, 2, 3]),
    //     f10: Some(pilota::AHashSet::from_iter(vec!["1".into(), "2".into()])),
    //     f11: Some(fieldmask::fieldmask::fieldmask::A {
    //         a: Some(1),
    //         b: Some("2".into()),
    //         ..Default::default()
    //     }),
    //     f12: Some(vec![vec![1, 2, 3], vec![1, 2]]),
    //     f13: Some(vec![
    //         fieldmask::fieldmask::fieldmask::A {
    //             a: Some(1),
    //             b: Some("2".into()),
    //             ..Default::default()
    //         },
    //         fieldmask::fieldmask::fieldmask::A {
    //             a: Some(1),
    //             b: Some("2".into()),
    //             ..Default::default()
    //         },
    //     ]),
    //     f14: Some(pilota::AHashMap::from_iter(vec![
    //         (1, "1".into()),
    //         (2, "2".into()),
    //     ])),
    //     f15: Some(pilota::AHashMap::from_iter(vec![
    //         (
    //             "key1".into(),
    //             fieldmask::fieldmask::fieldmask::A {
    //                 a: Some(1),
    //                 b: Some("2".into()),
    //                 ..Default::default()
    //             },
    //         ),
    //         (
    //             "key2".into(),
    //             fieldmask::fieldmask::fieldmask::A {
    //                 a: Some(1),
    //                 b: Some("2".into()),
    //                 ..Default::default()
    //             },
    //         ),
    //         (
    //             "key3".into(),
    //             fieldmask::fieldmask::fieldmask::A {
    //                 a: Some(1),
    //                 b: Some("2".into()),
    //                 ..Default::default()
    //             },
    //         ),
    //     ])),
    //     f16: Some(pilota::AHashMap::from_iter(vec![(
    //         "key1".into(),
    //         vec![
    //             fieldmask::fieldmask::fieldmask::A {
    //                 a: Some(1),
    //                 b: Some("2".into()),
    //                 ..Default::default()
    //             },
    //             fieldmask::fieldmask::fieldmask::A {
    //                 a: Some(1),
    //                 b: Some("2".into()),
    //                 ..Default::default()
    //             },
    //         ],
    //     )])),
    //     f17: Some(vec![pilota::AHashMap::from_iter(vec![
    //         ("key1".into(), 1),
    //         ("key2".into(), 2),
    //     ])]),
    //     ..Default::default()
    // };

    // let request_clone = request.clone();
    // let mut buf = pilota::BytesMut::new();
    // let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut buf, true);
    // request_clone.encode(&mut protocol).unwrap();
    // println!("before mask:{:?}", buf);

    // request.set_field_mask(request_fieldmask);

    // println!("{:?}", request);

    // let mut buf = pilota::BytesMut::new();
    // let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut buf, true);
    // request.encode(&mut protocol).unwrap();
    // println!("{:?}", buf);

    // let mut encoded_buf = buf.freeze();
    // let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut encoded_buf, true);
    // let parsed_request = fieldmask::fieldmask::fieldmask::Request::decode(&mut protocol).unwrap();
    // println!("{:?}", parsed_request);
}
