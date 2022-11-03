#![cfg(test)]

use std::path::Path;

use tempfile::tempdir;

fn diff_file(old: impl AsRef<Path>, new: impl AsRef<Path>) {
    let old_content = unsafe { String::from_utf8_unchecked(std::fs::read(old).unwrap()) };

    let new_content = unsafe { String::from_utf8_unchecked(std::fs::read(new).unwrap()) };

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
            .compile(&[source], target)
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
        let path = dir.path().join(&format!(
            "{}.rs",
            target
                .as_ref()
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap()
        ));

        f(source.as_ref(), &path);
        diff_file(target, path);
    }
}

fn test_thrift(source: impl AsRef<Path>, target: impl AsRef<Path>) {
    test_with_builder(source, target, |source, target| {
        crate::Builder::thrift()
            .ignore_unused(false)
            .compile(&[source], target)
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
fn test_touch() {
    let file_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("must_gen_items.thrift");

    let mut out_path = file_path.clone();
    out_path.set_extension("rs");

    test_with_builder(file_path, out_path, |source, target| {
        crate::Builder::thrift()
            .touch([(source.into(), vec!["A"])])
            .compile(&[source], target)
    });
}
