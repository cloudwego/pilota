use std::path::PathBuf;

use bytes::BytesMut;
use pilota::thrift::Message as _;

fn main() {
    let idl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/idl/apache.thrift");
    println!("{}", idl_path.display());

    let content = std::fs::read_to_string(&idl_path).unwrap();
    let ret = pilota_thrift_parser::parser::thrift::FileParser::new(
        pilota_thrift_parser::FileSource::new_with_path(idl_path.clone(), &content).unwrap(),
    )
    .parse();
    let mut parsed_file = match ret {
        Ok(parsed_file) => parsed_file,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    // let mut parsed_file =
    // pilota_thrift_parser::parser::thrift::FileParser::new(idl_path.clone())
    //     .parse()
    //     .unwrap();

    parsed_file.path = idl_path.into();

    let descriptor: pilota_thrift_reflect::thrift_reflection::FileDescriptor =
        (&parsed_file).into();
    println!("{:?}", descriptor);
    let mut data = BytesMut::new();
    let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut data, true);
    let _ = descriptor.encode(&mut protocol);
    println!("{:?}", data);

    let mut data = data.freeze();
    let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut data, true);
    let decoded = pilota_thrift_reflect::thrift_reflection::FileDescriptor::decode(&mut protocol);
    println!("{:?}", decoded);
}
