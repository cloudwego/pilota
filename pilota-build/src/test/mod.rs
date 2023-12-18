#![cfg(test)]

use std::path::Path;

use tempfile::tempdir;

use crate::{plugin::SerdePlugin, IdlService};

fn diff_file(old: impl AsRef<Path>, new: impl AsRef<Path>) {
    let old_content =
        unsafe { String::from_utf8_unchecked(std::fs::read(old).unwrap()) }.replace("\r", "");

    let new_content =
        unsafe { String::from_utf8_unchecked(std::fs::read(new).unwrap()) }.replace("\r", "");

    let patch = diffy::create_patch(&old_content, &new_content);
    if !patch.hunks().is_empty() {
        panic!("\n{}\n", patch)
    }
}

fn test_protobuf(source: impl AsRef<Path>, target: impl AsRef<Path>) {
    test_with_builder(source, target, |source, target| {
        crate::Builder::protobuf()
            .ignore_unused(false)
            .include_dirs(vec![source.parent().unwrap().to_path_buf()])
            .compile_with_config(
                vec![IdlService::from_path(source.to_path_buf())],
                crate::Output::File(target.into()),
            )
    });
}

fn test_with_builder<F: FnOnce(&Path, &Path)>(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
    f: F,
) {
    if std::env::var("UPDATE_TEST_DATA").as_deref() == Ok("1") {
        f(source.as_ref(), target.as_ref());
    } else {
        let dir = tempdir().unwrap();
        let path = dir.path().join(format!(
            "{}.rs",
            target
                .as_ref()
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap()
        ));
        println!("{path:?}");

        f(source.as_ref(), &path);
        diff_file(target, path);
    }
}

fn test_thrift(source: impl AsRef<Path>, target: impl AsRef<Path>) {
    test_with_builder(source, target, |source, target| {
        crate::Builder::thrift()
            .ignore_unused(false)
            .compile_with_config(
                vec![IdlService::from_path(source.to_owned())],
                crate::Output::File(target.into()),
            )
    });
}

fn test_plugin_thrift(source: impl AsRef<Path>, target: impl AsRef<Path>) {
    test_with_builder(source, target, |source, target| {
        crate::Builder::thrift()
            .ignore_unused(false)
            .plugin(SerdePlugin)
            .compile_with_config(
                vec![IdlService::from_path(source.to_path_buf())],
                crate::Output::File(target.into()),
            )
    });
}

fn test_plugin_proto(source: impl AsRef<Path>, target: impl AsRef<Path>) {
    test_with_builder(source, target, |source, target| {
        crate::Builder::protobuf()
            .ignore_unused(false)
            .plugin(SerdePlugin)
            .compile_with_config(
                vec![IdlService::from_path(source.to_path_buf())],
                crate::Output::File(target.into()),
            )
    });
}

#[test]
fn test_thrift_gen() {
    let test_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("thrift");

    test_data_dir.read_dir().unwrap().for_each(|f| {
        let f = f.unwrap();

        let path = f.path();

        if let Some(ext) = path.extension() {
            if ext == "thrift" {
                let mut rs_path = path.clone();
                rs_path.set_extension("rs");
                test_thrift(path, rs_path);
            }
        }
    });
}

#[test]
fn test_protobuf_gen() {
    let test_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("protobuf");

    test_data_dir.read_dir().unwrap().for_each(|f| {
        let f = f.unwrap();

        let path = f.path();

        if let Some(ext) = path.extension() {
            if ext == "proto" {
                let mut rs_path = path.clone();
                rs_path.set_extension("rs");
                test_protobuf(path, rs_path);
            }
        }
    });
}

#[test]
fn test_plugin_gen() {
    let test_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("plugin");

    test_data_dir.read_dir().unwrap().for_each(|f| {
        let f = f.unwrap();

        let path = f.path();

        if let Some(ext) = path.extension() {
            if ext == "thrift" {
                let mut rs_path = path.clone();
                rs_path.set_extension("rs");
                test_plugin_thrift(path, rs_path);
            } else if ext == "proto" {
                let mut rs_path = path.clone();
                rs_path.set_extension("rs");
                test_plugin_proto(path, rs_path);
            }
        }
    });
}

#[test]
fn test_touch() {
    let file_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("must_gen_items.thrift");

    let mut out_path = file_path.clone();
    out_path.set_extension("rs");

    test_with_builder(file_path, out_path, |source, target| {
        crate::Builder::thrift()
            .touch([(source.into(), vec!["A"])])
            .compile_with_config(
                vec![IdlService::from_path(source.to_path_buf())],
                crate::Output::File(target.into()),
            )
    });
}

#[test]
fn test_unknown_fields() {
    let file_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("unknown_fields.thrift");

    let mut out_path = file_path.clone();
    out_path.set_extension("rs");

    test_with_builder(file_path, out_path, |source, target| {
        crate::Builder::thrift()
            .ignore_unused(false)
            .keep_unknown_fields([source.into()])
            .plugin(SerdePlugin)
            .compile_with_config(
                vec![IdlService::from_path(source.to_path_buf())],
                crate::Output::File(target.into()),
            )
    });
}

mod tests {
    use pilota::{
        prost::bytes::BytesMut,
        thrift::{binary::TBinaryProtocol, Message},
    };

    use self::decode_error::decode_error::A;

    include!("../../test_data/thrift/decode_error.rs");

    #[test]
    fn test_decode_error() {
        let mut data = BytesMut::from(&[
            12_u8, 0, 1, 12, 0, 1, 11, 0, 1, 0, 0, 0, 10, 104, 101, 108, 108, 111, 32, 119, 111,
            114, 108, 100, 0, 0, 0,
        ] as &[u8])
        .freeze();

        let err = A::decode(&mut TBinaryProtocol::new(&mut data, false)).unwrap_err();

        assert_eq!(format!("{}", err), "decode struct `A` field(#1) failed, caused by decode struct `B` field(#1) failed, caused by invalid ttype 100")
    }
}
