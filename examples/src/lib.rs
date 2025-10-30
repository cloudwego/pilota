pub mod zero_value {
    include!(concat!(env!("OUT_DIR"), "/zero_value.rs"));
}

pub mod fieldmask {
    include!(concat!(env!("OUT_DIR"), "/fieldmask.rs"));
}

pub mod custom_options {
    include!(concat!(env!("OUT_DIR"), "/custom_options.rs"));
}

#[test]
fn test_pb_encode_zero_value() {
    use std::sync::Arc;

    use pilota::pb::Message as _;
    let mut a = zero_value::zero_value::A::default();

    a.str_map.insert("key1".into(), "value".into());
    a.str_map.insert("key2".into(), "".into());
    a.s1 = "s1".into();
    a.s2 = Some("s2".into());
    a.b = Some(Arc::new(zero_value::zero_value::B {
        s3: "s3".into(),
        ..Default::default()
    }));
    a.c = Some(zero_value::zero_value::C {
        s4: Some("s4".into()),
        ..Default::default()
    });
    let c = a.c.as_mut().unwrap();
    if c.bb.is_none() {
        c.bb = Some(Vec::new());
    }
    c.bb.as_mut()
        .unwrap()
        .push(Arc::new(zero_value::zero_value::B {
            s3: "s5".into(),
            ..Default::default()
        }));

    c.bb.as_mut()
        .unwrap()
        .push(Arc::new(zero_value::zero_value::B {
            s3: "s6".into(),
            ..Default::default()
        }));

    println!("a: {:?}", a);

    // encode a
    let mut encode_a = pilota::LinkedBytes::new();
    a.encode(&mut encode_a).unwrap();
    let encoded_a = encode_a.concat().freeze();
    println!("encode a: {:?}", encoded_a);

    // decode a
    let decode_a = encoded_a.clone();
    let decode_a_to_unknown_a = encoded_a.clone();
    let decoded_a = zero_value::zero_value::A::decode(decode_a.clone()).unwrap();
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
    let encoded_unknown_a = encode_unknown_a.concat().freeze();
    println!("encode unknown_a: {:?}", encoded_unknown_a);
    println!("encoded_a: {:?}", encoded_a); // binary sequence is different

    // decode unknown_a
    let decode_unknown_a = encoded_unknown_a.clone();
    let _decode_unknown_a_to_a = encoded_unknown_a.clone();
    let decoded_unknown_a = zero_value::zero_value::UnknownA::decode(decode_unknown_a).unwrap();
    println!("decode unknown_a: {:?}", decoded_unknown_a);
    assert_eq!(decoded_unknown_a, decoded_a_to_unknown_a);

    // decode a
    let decoded_a = zero_value::zero_value::A::decode(decode_a.clone()).unwrap();
    println!("decode unknown_a to a: {:?}", decoded_a);
    assert_eq!(decoded_a, a);

    println!("--------------------------------");

    // encode c
    let mut encode_c = pilota::pb::LinkedBytes::new();
    let _ = decoded_a.c.map(|c| c.encode(&mut encode_c)).unwrap();
    let encoded_c = encode_c.concat().freeze();
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
    let encoded_unknown_c = encode_unknown_c.concat().freeze();
    println!("encode unknown_c: {:?}", encoded_unknown_c);
    assert_eq!(encoded_unknown_c.as_ref(), encoded_c.as_ref());

    // decode unknown_c
    let decoded_unknown_c = zero_value::zero_value::Cc::decode(encoded_unknown_c.clone()).unwrap();
    println!("decode unknown_c: {:?}", decoded_unknown_c);
    assert_eq!(decoded_unknown_c, decoded_c_to_unknown_c);

    // decode unknown_c to c
    let decoded_c = zero_value::zero_value::C::decode(encoded_unknown_c).unwrap();
    println!("decode unknown_c to c: {:?}", decoded_c);
    assert_eq!(decoded_c, a.c.unwrap());

    // test deprecated
    use zero_value::zero_value::TestService;

    // test f32 and f64
    let _ = zero_value::zero_value::file_descriptor_zero_value()
        .messages()
        .for_each(|m| {
            // the name is same with the idl definition
            if m.name() == "BB" {
                let opt = m.proto().options.as_ref().unwrap();
                if let Ok(f32_opt) = zero_value::zero_value::exts_zero_value::f32.get(opt) {
                    println!("f32_opt: {:?}", f32_opt);
                    assert_eq!(f32_opt, 1.23);
                }
                if let Ok(f64_opt) = zero_value::zero_value::exts_zero_value::f64.get(opt) {
                    println!("f64_opt: {:?}", f64_opt);
                    assert_eq!(f64_opt, 3.21);
                }
            }
        });
}

#[test]
fn test_thrift_fieldmask() {
    use pilota::thrift::Message as _;
    let desc = fieldmask::fieldmask::fieldmask::Request::get_descriptor()
        .unwrap()
        .type_descriptor();

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

#[test]
fn test_pb_options() {
    use pilota::pb::{
        descriptor_getter::{FieldDescriptorGetter, ItemDescriptorGetter},
        Message as _,
    };

    // file options
    let fd_proto =
        custom_options::custom_options::custom_options::file_descriptor_proto_custom_options();
    if let Some(file_opts) = fd_proto.options.as_ref() {
        use custom_options::custom_options::custom_options::exts_custom_options;
        if let Ok(v) = exts_custom_options::file_version.get(file_opts) {
            println!("file_option file_version: {}", v);
            assert_eq!(v, 20250818);
        }
        if let Ok(v) = exts_custom_options::file_author.get(file_opts) {
            println!("file_option file_author: {}", v);
            assert_eq!(v, "giggle");
        }
        if let Ok(v) = exts_custom_options::file_department.get(file_opts) {
            println!("file_option file_department: {}", v);
            assert_eq!(v, "arch");
        }
        if let Ok(v) = exts_custom_options::internal_api.get(file_opts) {
            println!("file_option internal_api: {}", v);
            assert_eq!(v, false);
        }
        if let Ok(v) = exts_custom_options::file_kv.get(file_opts) {
            println!("file_option file_kv: {:?}", v);
            assert_eq!(v.key.unwrap_or_default(), "file_key");
            assert_eq!(v.value.unwrap_or_default(), "file_val");
        }
    }

    // service options
    for svc in &fd_proto.service {
        if let Some(svc_opts) = svc.options.as_ref() {
            use custom_options::custom_options::custom_options::exts_custom_options;
            if let Ok(v) = exts_custom_options::service_version.get(svc_opts) {
                println!("service_option service_version: {}", v);
            }
            if let Ok(v) = exts_custom_options::require_auth.get(svc_opts) {
                println!("service_option require_auth: {}", v);
            }
            if let Ok(v) = exts_custom_options::rate_limit.get(svc_opts) {
                println!("service_option rate_limit: {}", v);
            }
        }

        for m in &svc.method {
            if let Some(m_opts) = m.options.as_ref() {
                use custom_options::custom_options::custom_options::exts_custom_options;
                if let Ok(v) = exts_custom_options::method_require_auth.get(m_opts) {
                    println!("method_option method_require_auth: {}", v);
                }
                if let Ok(v) = exts_custom_options::method_rate_limit.get(m_opts) {
                    println!("method_option method_rate_limit: {}", v);
                }
                if let Ok(v) = exts_custom_options::endpoint.get(m_opts) {
                    println!("method_option endpoint: {}", v);
                }
                if let Ok(v) = exts_custom_options::http_method.get(m_opts) {
                    println!("method_option http_method: {}", v);
                }
                if let Ok(v) = exts_custom_options::permission.get(m_opts) {
                    println!("method_option permission: {}", v);
                }
            }
        }
    }

    // message options
    let user_proto =
        custom_options::custom_options::custom_options::User::get_descriptor_proto().unwrap();
    if let Some(user_opts) = user_proto.options.as_ref() {
        use custom_options::custom_options::custom_options::api_metadata::exts_api_metadata;
        if let Ok(v) = exts_api_metadata::test.get(user_opts) {
            assert_eq!(v, "ApiMetadatatest");
            println!("user_option api_metadata test: {}", v);
        }
        use custom_options::custom_options::custom_options::exts_custom_options;
        if let Ok(v) = exts_custom_options::db_table.get(user_opts) {
            assert_eq!(v, "users");
            println!("user_option db_table: {}", v);
        }
        if let Ok(v) = exts_custom_options::db_entity.get(user_opts) {
            assert!(v);
            println!("user_option db_entity: {}", v);
        }
        if let Ok(v) = exts_custom_options::cache_ttl_seconds.get(user_opts) {
            assert_eq!(v, 3600);
            println!("user_option cache_ttl_seconds: {}", v);
        }
        if let Ok(v) = exts_custom_options::validate.get(user_opts) {
            let all = v.all_fields_required.unwrap_or(false);
            let depth = v.max_nesting_depth.unwrap_or_default();
            let msg = v.validation_message.clone().unwrap_or_default();
            assert_eq!(all, true);
            assert_eq!(depth, 3);
            assert_eq!(msg, "User validation failed");
            println!(
                "user_option validate: all_fields_required={}, max_nesting_depth={}, validation_message={}",
                all, depth, msg
            );
        }
    }

    use custom_options::custom_options::custom_options::exts_custom_options;
    // field options
    if let Some(id_opts) = user_proto
        .get_field_descriptor_proto("id")
        .and_then(|f| f.options.as_ref())
    {
        if let Ok(v) = exts_custom_options::db_column.get(id_opts) {
            assert_eq!(v, "user_id");
            println!("user_option db_column: {}", v);
        }
    }

    if let Some(username_opts) = user_proto
        .get_field_descriptor_proto("username")
        .and_then(|f| f.options.as_ref())
    {
        if let Ok(v) = exts_custom_options::validation.get(username_opts) {
            assert_eq!(v, "required,min=3,max=50");
            println!("field_option username.validation: {}", v);
        }
        if let Ok(v) = exts_custom_options::db_index.get(username_opts) {
            assert!(v);
            println!("field_option username.db_index: {}", v);
        }
    }

    if let Some(password_opts) = user_proto
        .get_field_descriptor_proto("password")
        .and_then(|f| f.options.as_ref())
    {
        if let Ok(v) = exts_custom_options::sensitive.get(password_opts) {
            assert!(v);
            println!("field_option password.sensitive: {}", v);
        }
        if let Ok(v) = exts_custom_options::validation.get(password_opts) {
            assert_eq!(v, "required,min=8");
            println!("field_option password.validation: {}", v);
        }
        if let Ok(v) = exts_custom_options::api_doc.get(password_opts) {
            assert_eq!(v, "User password (never returned in API responses)");
            println!("field_option password.api_doc: {}", v);
        }
    }

    if let Some(email_opts) = user_proto
        .get_field_descriptor_proto("email")
        .and_then(|f| f.options.as_ref())
    {
        if let Ok(v) = exts_custom_options::validation.get(email_opts) {
            assert_eq!(v, "required,email");
            println!("field_option email.validation: {}", v);
        }
        if let Ok(v) = exts_custom_options::db_index.get(email_opts) {
            assert!(v);
            println!("field_option email.db_index: {}", v);
        }
    }

    // oneof options
    use custom_options::custom_options::custom_options::payment_info::PaymentMethod;
    let payment_method_proto = PaymentMethod::get_descriptor_proto().unwrap();
    if let Some(oneof_opts) = payment_method_proto.options.as_ref() {
        if let Ok(v) = exts_custom_options::oneof_description.get(oneof_opts) {
            assert_eq!(v, "Payment method details, only one can be selected");
            println!("oneof_option description: {}", v);
        }
        if let Ok(v) = exts_custom_options::exclusive.get(oneof_opts) {
            assert!(v);
            println!("oneof_option exclusive: {}", v);
        }
    }

    // enum options and enum value options
    if let Some(enum_proto) = fd_proto.enum_type.iter().find(|e| e.name() == "UserStatus") {
        if let Some(enum_opts) = enum_proto.options.as_ref() {
            if let Ok(v) = exts_custom_options::enum_description.get(enum_opts) {
                assert_eq!(v, "Represents the current status of a user account");
                println!("enum_option enum_description: {}", v);
            }
            if let Ok(v) = exts_custom_options::is_internal.get(enum_opts) {
                assert_eq!(v, false);
                println!("enum_option is_internal: {}", v);
            }
            if let Some(aa) = enum_opts.allow_alias {
                assert!(aa);
                println!("enum_option allow_alias: {}", aa);
            }
        }

        for ev in &enum_proto.value {
            if ev.name() == "ACTIVE" {
                if let Some(ev_opts) = ev.options.as_ref() {
                    if let Ok(v) = exts_custom_options::display_name.get(ev_opts) {
                        assert_eq!(v, "Active");
                        println!("enum_value_option (ACTIVE) display_name: {}", v);
                    }
                    if let Ok(v) = exts_custom_options::access_level.get(ev_opts) {
                        assert_eq!(v, 1);
                        println!("enum_value_option (ACTIVE) access_level: {}", v);
                    }
                    if let Ok(v) = exts_custom_options::color.get(ev_opts) {
                        assert_eq!(v, "#00FF00");
                        println!("enum_value_option (ACTIVE) color: {}", v);
                    }
                }
            }
        }
    }

    // options on nested messages
    use custom_options::custom_options::custom_options::api_metadata::{
        exts_api_metadata, Example,
    };
    let example_proto = Example::get_descriptor_proto().unwrap();
    if let Some(example_opts) = example_proto.options.as_ref() {
        if let Ok(v) = exts_api_metadata::test.get(example_opts) {
            assert_eq!(v, "test");
            println!("example_option test: {}", v);
        }
    }
}
