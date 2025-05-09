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
