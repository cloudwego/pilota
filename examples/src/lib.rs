use pilota::prost::Message as _;

mod zero_value;

#[test]
fn test_pb_encode_zero_value() {
    let test_data = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("idl")
        .join("zero_value.proto");

    let out_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("zero_value.rs");

    pilota_build::Builder::protobuf()
        .ignore_unused(false)
        .include_dirs(vec![test_data.parent().unwrap().to_path_buf()])
        .compile_with_config(
            vec![pilota_build::IdlService::from_path(test_data.to_path_buf())],
            pilota_build::Output::File(out_path.into()),
        );

    let mut a = zero_value::zero_value::A::default();

    a.str_map.insert("key1".into(), "value".into());
    a.str_map.insert("key2".into(), "".into());

    let mut buf = pilota::BytesMut::new();
    a.encode(&mut buf).unwrap();

    println!("{:?}", buf);
    println!("{:?}", buf.freeze().as_ref());
}
