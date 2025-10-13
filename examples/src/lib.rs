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

    // test deprecated
    use zero_value::zero_value::TestService;

    // test f32 and f64
    let bb = zero_value::zero_value::file_descriptor_zero_value()
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

// #[test]
// fn test_thrift_fieldmask() {
//     use pilota::thrift::Message as _;
//     let desc =
// fieldmask::fieldmask::fieldmask::Request::get_descriptor().type_descriptor();
//     println!("{:?}", desc);

//     let request_fieldmask = pilota_thrift_fieldmask::FieldMaskBuilder::new(
//         &desc,
//         &[
//             "$.f1",
//             "$.f9[1, 3]",
//             "$.f11.b",
//             "$.f12[0][*]",
//             "$.f14{*}",
//             "$.f15{ \"key1\",\"key3\"}",
//             "$.f16{\"key1\"}[1].a",
//             "$.f17[*]{\"key1\"}",
//             "$.base.Addr",
//         ],
//     )
//     .with_options(pilota_thrift_fieldmask::Options::new().
// with_black_list_mode(true))     .build()
//     .unwrap();
//     println!("{:?}", request_fieldmask);

//     let mut request = fieldmask::fieldmask::fieldmask::Request {
//         f1: Some(true),
//         f2: Some(1),
//         f3: Some(1),
//         f4: Some(1),
//         f5: Some(1),
//         f6: Some(1.0),
//         f7: Some("1".into()),
//         f8: Some(pilota::Bytes::from_static(b"1")),
//         f9: vec![1, 2, 3],
//         f10: Some(pilota::AHashSet::from_iter(vec!["1".into(), "2".into()])),
//         f11: Some(fieldmask::fieldmask::fieldmask::A {
//             a: Some(1),
//             b: Some("2".into()),
//             ..Default::default()
//         }),
//         f12: Some(vec![vec![1, 2, 3], vec![1, 2]]),
//         f13: Some(vec![
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
//         ]),
//         f14: Some(pilota::AHashMap::from_iter(vec![
//             (1, "1".into()),
//             (2, "2".into()),
//         ])),
//         f15: Some(pilota::AHashMap::from_iter(vec![
//             (
//                 "key1".into(),
//                 fieldmask::fieldmask::fieldmask::A {
//                     a: Some(1),
//                     b: Some("2".into()),
//                     ..Default::default()
//                 },
//             ),
//             (
//                 "key2".into(),
//                 fieldmask::fieldmask::fieldmask::A {
//                     a: Some(1),
//                     b: Some("2".into()),
//                     ..Default::default()
//                 },
//             ),
//             (
//                 "key3".into(),
//                 fieldmask::fieldmask::fieldmask::A {
//                     a: Some(1),
//                     b: Some("2".into()),
//                     ..Default::default()
//                 },
//             ),
//         ])),
//         f16: Some(pilota::AHashMap::from_iter(vec![(
//             "key1".into(),
//             vec![
//                 fieldmask::fieldmask::fieldmask::A {
//                     a: Some(1),
//                     b: Some("2".into()),
//                     ..Default::default()
//                 },
//                 fieldmask::fieldmask::fieldmask::A {
//                     a: Some(1),
//                     b: Some("2".into()),
//                     ..Default::default()
//                 },
//             ],
//         )])),
//         f17: Some(vec![pilota::AHashMap::from_iter(vec![
//             ("key1".into(), 1),
//             ("key2".into(), 2),
//         ])]),
//         base: Some(fieldmask::fieldmask::base::r#loop::Base {
//             addr: "127.0.0.1:8080".into(),
//             log_id: Some("logid".into()),
//             ..Default::default()
//         }),
//         ..Default::default()
//     };

//     let request_clone = request.clone();
//     let mut buf = pilota::BytesMut::new();
//     let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut buf,
// true);     request_clone.encode(&mut protocol).unwrap();
//     println!("before mask:{:?}", buf);

//     request.set_field_mask(request_fieldmask);

//     println!("{:?}", request);

//     let mut buf = pilota::BytesMut::new();
//     let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut buf,
// true);     request.encode(&mut protocol).unwrap();
//     println!("{:?}", buf);

//     let mut encoded_buf = buf.freeze();
//     let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut
// encoded_buf, true);     let parsed_request =
// fieldmask::fieldmask::fieldmask::Request::decode(&mut protocol).unwrap();
//     println!("{:?}", parsed_request);
// }

// #[test]
// fn test_pb_options() {
//     use pilota::pb::Message as _;
//     let fd =
// custom_options::custom_options::custom_options::file_descriptor_custom_options();
//

//     // file options
//     let fd_proto =
//         custom_options::custom_options::custom_options::file_descriptor_proto_custom_options();
//     if let Some(file_opts) = fd_proto.options.as_ref() {
//         use
// custom_options::custom_options::custom_options::exts_custom_options;
//         if let Ok(v) = exts_custom_options::file_version.get(file_opts) {
//             println!("file_option file_version: {}", v);
//             assert_eq!(v, 20250818);
//         }
//         if let Ok(v) = exts_custom_options::file_author.get(file_opts) {
//             println!("file_option file_author: {}", v);
//             assert_eq!(v, "giggle");
//         }
//         if let Ok(v) = exts_custom_options::file_department.get(file_opts) {
//             println!("file_option file_department: {}", v);
//             assert_eq!(v, "arch");
//         }
//         if let Ok(v) = exts_custom_options::internal_api.get(file_opts) {
//             println!("file_option internal_api: {}", v);
//             assert_eq!(v, false);
//         }
//         if let Ok(v) = exts_custom_options::file_kv.get(file_opts) {
//             println!("file_option file_kv: {:?}", v);
//             assert_eq!(v.key.unwrap_or_default(), "file_key");
//             assert_eq!(v.value.unwrap_or_default(), "file_val");
//         }
//     }

//     // service options
//     for svc in fd.services() {
//         let svc_proto = svc.proto();
//         if let Some(svc_opts) = svc_proto.options.as_ref() {
//             use
// custom_options::custom_options::custom_options::exts_custom_options;
//             if let Ok(v) = exts_custom_options::service_version.get(svc_opts)
// {                 println!("service_option service_version: {}", v);
//             }
//             if let Ok(v) = exts_custom_options::require_auth.get(svc_opts) {
//                 println!("service_option require_auth: {}", v);
//             }
//             if let Ok(v) = exts_custom_options::rate_limit.get(svc_opts) {
//                 println!("service_option rate_limit: {}", v);
//             }
//         }
//         for m in svc.methods() {
//             let m_proto = m.proto();
//             if let Some(m_opts) = m_proto.options.as_ref() {
//                 use
// custom_options::custom_options::custom_options::exts_custom_options;
//                 if let Ok(v) =
// exts_custom_options::method_require_auth.get(m_opts) {
// println!("method_option method_require_auth: {}", v);                 }
//                 if let Ok(v) =
// exts_custom_options::method_rate_limit.get(m_opts) {
// println!("method_option method_rate_limit: {}", v);                 }
//                 if let Ok(v) = exts_custom_options::endpoint.get(m_opts) {
//                     println!("method_option endpoint: {}", v);
//                 }
//                 if let Ok(v) = exts_custom_options::http_method.get(m_opts) {
//                     println!("method_option http_method: {}", v);
//                 }
//                 if let Ok(v) = exts_custom_options::permission.get(m_opts) {
//                     println!("method_option permission: {}", v);
//                 }
//             }
//         }
//     }

// message options
fn walk_message(md: &::pilota::pb::reflect::MessageDescriptor, depth: usize) {
    use custom_options::custom_options::custom_options::exts_custom_options;
    let indent = "  ".repeat(depth);
    let dp = md.proto();
    if let Some(opts) = dp.options.as_ref() {
        if let Ok(v) = exts_custom_options::db_table.get(opts) {
            println!("{}message_option db_table: {}", indent, v);
            assert_eq!(v, "users");
        }
        if let Ok(v) = exts_custom_options::db_entity.get(opts) {
            println!("{}message_option db_entity: {}", indent, v);
            assert_eq!(v, true);
        }
        if let Ok(v) = exts_custom_options::cache_ttl_seconds.get(opts) {
            println!("{}message_option cache_ttl_seconds: {}", indent, v);
            assert_eq!(v, 3600);
        }
        // message type extension validate: decode by unknown_fields
        if let Ok(mv) = exts_custom_options::validate.get(opts) {
            println!(
                    "{}message_option validate: all_fields_required={:?} max_nesting_depth={:?} validation_message={:?}",
                    indent,
                    mv.all_fields_required,
                    mv.max_nesting_depth,
                    mv.validation_message
                );
            assert_eq!(mv.all_fields_required, Some(true));
            assert_eq!(mv.max_nesting_depth, Some(3));
            assert_eq!(mv.validation_message, Some("User validation failed".into()));
        }
    }

    // nested extension: ApiMetadata.message Example use ApiMetadata::exts::test
    if let Some(opts) = dp.options.as_ref() {
        use custom_options::custom_options::custom_options::api_metadata::example::exts_example as ex_exts;
        use custom_options::custom_options::custom_options::api_metadata::exts_api_metadata as am_exts;
        if let Ok(v) = am_exts::test.get(opts) {
            println!("{}api_metadata.test: {}", indent, v);
        }
        if let Ok(v) = ex_exts::level.get(opts) {
            println!("{}api_metadata.example.level: {}", indent, v);
        }
    }

    //         // field options
    //         for f in &dp.field {
    //             if let Some(f_opts) = f.options.as_ref() {
    //                 if let Ok(v) = exts_custom_options::sensitive.get(f_opts) {
    //                     println!("{}field_option sensitive: {}", indent, v);
    //                 }
    //                 if let Ok(v) = exts_custom_options::validation.get(f_opts) {
    //                     println!("{}field_option validation: {}", indent, v);
    //                 }
    //                 if let Ok(v) = exts_custom_options::db_column.get(f_opts) {
    //                     println!("{}field_option db_column: {}", indent, v);
    //                 }
    //                 if let Ok(v) = exts_custom_options::db_index.get(f_opts) {
    //                     assert_eq!(v, true);
    //                 }
    //                 if let Ok(v) = exts_custom_options::api_doc.get(f_opts) {
    //                     println!("{}field_option api_doc: {}", indent, v);
    //                 }
    //             }
    //         }

    // oneof options
    for o in &dp.oneof_decl {
        if let Some(o_opts) = o.options.as_ref() {
            if let Ok(v) = exts_custom_options::oneof_description.get(o_opts) {
                println!("{}oneof_option oneof_description: {}", indent, v);
            }
            if let Ok(v) = exts_custom_options::exclusive.get(o_opts) {
                println!("{}oneof_option exclusive: {}", indent, v);
            }
        }
    }

    // enum options
    for e in &dp.enum_type {
        if let Some(e_opts) = e.options.as_ref() {
            if let Ok(v) = exts_custom_options::enum_description.get(e_opts) {
                println!("{}enum_option enum_description: {}", indent, v);
            }
            if let Ok(v) = exts_custom_options::is_internal.get(e_opts) {
                println!("{}enum_option is_internal: {}", indent, v);
            }
        }
        for ev in &e.value {
            if let Some(ev_opts) = ev.options.as_ref() {
                if let Ok(v) = exts_custom_options::display_name.get(ev_opts) {
                    println!("{}enum_value_option display_name: {}", indent, v);
                }
                if let Ok(v) = exts_custom_options::access_level.get(ev_opts) {
                    println!("{}enum_value_option access_level: {}", indent, v);
                }
                if let Ok(v) = exts_custom_options::color.get(ev_opts) {
                    println!("{}enum_value_option color: {}", indent, v);
                }
            }
        }
    }

    //         // recursive traverse nested message
    //         for nested in md.nested_messages() {
    //             walk_message(&nested, depth + 1);
    //         }
    //     }

    //     // top level message
    //     for m in fd.messages() {
    //         walk_message(&m, 0);
    //     }

//     // top level enum and enum value options
//     for e in fd.enums() {
//         use
// custom_options::custom_options::custom_options::exts_custom_options;
//         let ep = e.proto();
//         if let Some(e_opts) = ep.options.as_ref() {
//             if let Ok(v) = exts_custom_options::enum_description.get(e_opts)
// {                 println!("top_enum_option enum_description: {}", v);
//             }
//             if let Ok(v) = exts_custom_options::is_internal.get(e_opts) {
//                 println!("top_enum_option is_internal: {}", v);
//             }
//         }
//         for ev in &ep.value {
//             if let Some(ev_opts) = ev.options.as_ref() {
//                 if let Ok(v) = exts_custom_options::display_name.get(ev_opts)
// {                     println!("top_enum_value_option display_name: {}", v);
//                 }
//                 if let Ok(v) = exts_custom_options::access_level.get(ev_opts)
// {                     println!("top_enum_value_option access_level: {}", v);
//                 }
//                 if let Ok(v) = exts_custom_options::color.get(ev_opts) {
//                     println!("top_enum_value_option color: {}", v);
//                 }
//             }
//         }
//     }
// }
