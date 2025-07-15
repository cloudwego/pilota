// mod fieldmask;
pub mod zero_value {
    include!(concat!(env!("OUT_DIR"), "/zero_value.rs"));
}

pub mod fieldmask {
    include!(concat!(env!("OUT_DIR"), "/fieldmask.rs"));
}

#[test]
fn test_pb_encode_zero_value() {
    use pilota::pb::Message as _;
    let mut a = zero_value::zero_value::A::default();

    a.str_map.insert("key1".into(), "value".into());
    a.str_map.insert("key2".into(), "".into());
    a.s1 = "s1".into();
    a.s2 = Some("s2".into());
    a.b = zero_value::zero_value::B {
        s3: "s3".into(),
        ..Default::default()
    };
    a.c = zero_value::zero_value::C {
        s4: Some("s4".into()),
        ..Default::default()
    };
    a.c.b.push(zero_value::zero_value::B {
        s3: "s5".into(),
        ..Default::default()
    });
    a.c.b.push(zero_value::zero_value::B {
        s3: "s6".into(),
        ..Default::default()
    });
    println!("a: {:?}", a);

    // encode a
    let mut encode_a = pilota::pb::LinkedBytes::new();
    a.encode(&mut encode_a).unwrap();
    let encoded_a = encode_a.bytes().clone().freeze();
    println!("encode a: {:?}", encoded_a);

    // decode a
    let decode_a = encoded_a.clone();
    let decode_a_to_unknown_a = encoded_a.clone();
    let decoded_a = zero_value::zero_value::A::decode(decode_a).unwrap();
    println!("decode a: {:?}", decoded_a);
    assert_eq!(decoded_a, a);

    // decode a to unknown_a
    let decoded_a_to_unknown_a =
        zero_value::zero_value::UnknownA::decode(decode_a_to_unknown_a).unwrap();
    println!("decode a to unknown_a: {:?}", decoded_a_to_unknown_a);

    // encode unknown_a
    let mut encode_unknown_a = pilota::pb::LinkedBytes::new();
    decoded_a_to_unknown_a
        .encode(&mut encode_unknown_a)
        .unwrap();
    let encoded_unknown_a = encode_unknown_a.bytes().clone().freeze();
    println!("encode unknown_a: {:?}", encoded_unknown_a);
    assert_eq!(encoded_unknown_a.as_ref(), encoded_a.as_ref());

    // decode unknown_a
    let decode_unknown_a = encoded_unknown_a.clone();
    let decode_unknown_a_to_a = encoded_unknown_a.clone();
    let decoded_unknown_a = zero_value::zero_value::UnknownA::decode(decode_unknown_a).unwrap();
    println!("decode unknown_a: {:?}", decoded_unknown_a);
    assert_eq!(decoded_unknown_a, decoded_a_to_unknown_a);

    // decode unknown_a to a
    let decoded_unknown_a_to_a = zero_value::zero_value::A::decode(decode_unknown_a_to_a).unwrap();
    println!("decode unknown_a to a: {:?}", decoded_unknown_a_to_a);
    assert_eq!(decoded_unknown_a_to_a, a);

    // encode c
    let mut encode_c = pilota::pb::LinkedBytes::new();
    decoded_a.c.encode(&mut encode_c).unwrap();
    let encoded_c = encode_c.bytes().clone().freeze();
    println!("encode c: {:?}", encoded_c);

    // decode c to unknown_c
    let decode_c_to_unknown_c = encoded_c.clone();
    let decoded_c_to_unknown_c = zero_value::zero_value::Cc::decode(decode_c_to_unknown_c).unwrap();
    println!("decode c to unknown_c: {:?}", decoded_c_to_unknown_c);

    // encode unknown_c
    let mut encode_unknown_c = pilota::pb::LinkedBytes::new();
    decoded_c_to_unknown_c
        .encode(&mut encode_unknown_c)
        .unwrap();
    let encoded_unknown_c = encode_unknown_c.bytes().clone().freeze();
    println!("encode unknown_c: {:?}", encoded_unknown_c);
    assert_eq!(encoded_unknown_c.as_ref(), encoded_c.as_ref());

    // decode unknown_c
    let decoded_unknown_c = zero_value::zero_value::Cc::decode(encoded_unknown_c.clone()).unwrap();
    println!("decode unknown_c: {:?}", decoded_unknown_c);
    assert_eq!(decoded_unknown_c, decoded_c_to_unknown_c);

    // decode unknown_c to c
    let decoded_c = zero_value::zero_value::C::decode(encoded_unknown_c).unwrap();
    println!("decode unknown_c to c: {:?}", decoded_c);
    assert_eq!(decoded_c, a.c);
}

#[test]
fn test_thrift_fieldmask() {
    use pilota::thrift::Message as _;
    let desc = fieldmask::fieldmask::fieldmask::Request::get_descriptor().type_descriptor();
    println!("{:?}", desc);

    let request_fieldmask = pilota_thrift_fieldmask::FieldMaskBuilder::new(
        &desc,
        &[
            "$.f1",
            "$.f9[1, 3]",
            "$.f11.b",
            "$.f12[0][*]",
            "$.f14{*}",
            "$.f15{ \"key1\",\"key3\"}",
            "$.f16{\"key1\"}[1].a",
            "$.f17[*]{\"key1\"}",
            "$.base.Addr",
        ],
    )
    .with_options(pilota_thrift_fieldmask::Options::new().with_black_list_mode(true))
    .build()
    .unwrap();
    println!("{:?}", request_fieldmask);

    let mut request = fieldmask::fieldmask::fieldmask::Request {
        f1: Some(true),
        f2: Some(1),
        f3: Some(1),
        f4: Some(1),
        f5: Some(1),
        f6: Some(1.0),
        f7: Some("1".into()),
        f8: Some(pilota::Bytes::from_static(b"1")),
        f9: vec![1, 2, 3],
        f10: Some(pilota::AHashSet::from_iter(vec!["1".into(), "2".into()])),
        f11: Some(fieldmask::fieldmask::fieldmask::A {
            a: Some(1),
            b: Some("2".into()),
            ..Default::default()
        }),
        f12: Some(vec![vec![1, 2, 3], vec![1, 2]]),
        f13: Some(vec![
            fieldmask::fieldmask::fieldmask::A {
                a: Some(1),
                b: Some("2".into()),
                ..Default::default()
            },
            fieldmask::fieldmask::fieldmask::A {
                a: Some(1),
                b: Some("2".into()),
                ..Default::default()
            },
        ]),
        f14: Some(pilota::AHashMap::from_iter(vec![
            (1, "1".into()),
            (2, "2".into()),
        ])),
        f15: Some(pilota::AHashMap::from_iter(vec![
            (
                "key1".into(),
                fieldmask::fieldmask::fieldmask::A {
                    a: Some(1),
                    b: Some("2".into()),
                    ..Default::default()
                },
            ),
            (
                "key2".into(),
                fieldmask::fieldmask::fieldmask::A {
                    a: Some(1),
                    b: Some("2".into()),
                    ..Default::default()
                },
            ),
            (
                "key3".into(),
                fieldmask::fieldmask::fieldmask::A {
                    a: Some(1),
                    b: Some("2".into()),
                    ..Default::default()
                },
            ),
        ])),
        f16: Some(pilota::AHashMap::from_iter(vec![(
            "key1".into(),
            vec![
                fieldmask::fieldmask::fieldmask::A {
                    a: Some(1),
                    b: Some("2".into()),
                    ..Default::default()
                },
                fieldmask::fieldmask::fieldmask::A {
                    a: Some(1),
                    b: Some("2".into()),
                    ..Default::default()
                },
            ],
        )])),
        f17: Some(vec![pilota::AHashMap::from_iter(vec![
            ("key1".into(), 1),
            ("key2".into(), 2),
        ])]),
        base: Some(fieldmask::fieldmask::base::r#loop::Base {
            addr: "127.0.0.1:8080".into(),
            log_id: Some("logid".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let request_clone = request.clone();
    let mut buf = pilota::BytesMut::new();
    let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut buf, true);
    request_clone.encode(&mut protocol).unwrap();
    println!("before mask:{:?}", buf);

    request.set_field_mask(request_fieldmask);

    println!("{:?}", request);

    let mut buf = pilota::BytesMut::new();
    let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut buf, true);
    request.encode(&mut protocol).unwrap();
    println!("{:?}", buf);

    let mut encoded_buf = buf.freeze();
    let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut encoded_buf, true);
    let parsed_request = fieldmask::fieldmask::fieldmask::Request::decode(&mut protocol).unwrap();
    println!("{:?}", parsed_request);
}
