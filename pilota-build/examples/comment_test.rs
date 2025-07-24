use std::path::PathBuf;

fn main() {
    let idl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data/thrift/comment_test.thrift");
    
    let out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data/thrift/comment_test.rs");

    pilota_build::Builder::thrift()
        .ignore_unused(false)
        .compile_with_config(
            vec![pilota_build::IdlService::from_path(idl_path)],
            pilota_build::Output::File(out_path.into()),
        );
    
    println!("Generated code with comments preserved!");
}