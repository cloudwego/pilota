#![cfg(test)]

use std::{fs, fs::File, path::Path, process::Command};

use tempfile::tempdir;

use crate::{IdlService, plugin::SerdePlugin};

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

fn diff_dir(old: impl AsRef<Path>, new: impl AsRef<Path>) {
    let old_files: Vec<_> = fs::read_dir(old.as_ref())
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect();
    let new_files: Vec<_> = fs::read_dir(new.as_ref())
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect();

    if old_files.len() != new_files.len() {
        panic!(
            "Number of files are different between {} and {}: {} vs {}",
            old.as_ref().to_str().unwrap(),
            new.as_ref().to_str().unwrap(),
            old_files.len(),
            new_files.len()
        );
    }

    for old_file in old_files {
        let file_name = old_file.file_name().unwrap();
        let corresponding_new_file = new.as_ref().join(file_name);
        if !corresponding_new_file.exists() {
            panic!("File {:?} does not exist in the new directory", file_name);
        }

        if old_file.is_file() && corresponding_new_file.is_file() {
            diff_file(old_file, corresponding_new_file);
        } else if !old_file.is_file() && !corresponding_new_file.is_file() {
            diff_dir(old_file, corresponding_new_file)
        } else {
            panic!(
                "{} and {} are not both files or directories",
                old_file.to_str().unwrap(),
                corresponding_new_file.to_str().unwrap()
            );
        }
    }
}

fn check_cargo_build(target: impl AsRef<Path>) {
    let tmp_dir = tempdir().unwrap();
    let tmp_target = tmp_dir.path().join(target.as_ref().file_name().unwrap());

    fn copy_dir_recursively(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let dst_path = dst.as_ref().join(entry.file_name());
            if path.is_dir() {
                copy_dir_recursively(path, dst_path)?;
            } else {
                fs::copy(path, dst_path)?;
            }
        }
        Ok(())
    }

    // `cargo build` produces the `target` directory and the `Cargo.lock` file
    // To not pollute the test data, copy the directory to a temporary one
    copy_dir_recursively(&target, &tmp_target).unwrap();

    println!("Running cargo build in {}", tmp_target.display());

    let result = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned()))
        .current_dir(&tmp_target)
        .arg("build")
        .output();

    match result {
        Ok(status) => {
            if !status.status.success() {
                eprintln!("{}", String::from_utf8_lossy(&status.stderr));
                panic!(
                    "cargo build returned non-zero exit code for {}. See above for more details",
                    tmp_target.display()
                );
            }
        }
        Err(_) => {
            panic!("cargo build failed for {}", tmp_target.display());
        }
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

fn test_pb(source: impl AsRef<Path>, target: impl AsRef<Path>) {
    test_with_builder(source, target, |source, target| {
        crate::Builder::pb()
            .ignore_unused(false)
            .include_dirs(vec![source.parent().unwrap().to_path_buf()])
            .compile_with_config(
                vec![IdlService::from_path(source.to_path_buf())],
                crate::Output::File(target.into()),
            )
    });
}

fn test_protobuf_with_split(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
    gen_dir: impl AsRef<Path>,
) {
    test_with_split_builder(source, target, gen_dir, |source, target| {
        crate::Builder::protobuf()
            .ignore_unused(false)
            .split_generated_files(true)
            .include_dirs(vec![source.parent().unwrap().to_path_buf()])
            .compile_with_config(
                vec![IdlService::from_path(source.to_path_buf())],
                crate::Output::File(target.into()),
            )
    });
}

fn test_pb_with_split(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
    gen_dir: impl AsRef<Path>,
) {
    test_with_split_builder(source, target, gen_dir, |source, target| {
        crate::Builder::pb()
            .ignore_unused(false)
            .split_generated_files(true)
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

fn test_with_builder_workspace<F: FnOnce(&Path, &Path)>(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
    f: F,
) {
    if std::env::var("UPDATE_TEST_DATA").as_deref() == Ok("1") {
        fs::remove_dir_all(&target).unwrap();
        fs::create_dir_all(&target).unwrap();
        let cargo_toml_path = target.as_ref().join("Cargo.toml");
        File::create(cargo_toml_path).unwrap();

        f(source.as_ref(), target.as_ref());

        check_cargo_build(target)
    } else {
        let dir = tempdir().unwrap();
        let path = dir.path().join(
            target
                .as_ref()
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap(),
        );
        let mut base_dir_tmp = path.clone();
        base_dir_tmp.pop();
        base_dir_tmp.push(path.file_stem().unwrap());
        println!("{path:?}");

        fs::create_dir_all(&path).unwrap();
        let cargo_toml_path = path.join("Cargo.toml");
        File::create(cargo_toml_path).unwrap();

        f(source.as_ref(), &path);
        diff_dir(&target, &base_dir_tmp);

        check_cargo_build(target)
    }
}

fn test_with_split_builder<F: FnOnce(&Path, &Path)>(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
    gen_dir: impl AsRef<Path>,
    f: F,
) {
    if std::env::var("UPDATE_TEST_DATA").as_deref() == Ok("1") {
        f(source.as_ref(), target.as_ref());
    } else {
        let dir = tempdir().unwrap();
        let path = dir.path().join(
            target
                .as_ref()
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap(),
        );
        let mut base_dir_tmp = path.clone();
        base_dir_tmp.pop();
        base_dir_tmp.push(path.file_stem().unwrap());
        println!("{path:?}");

        f(source.as_ref(), &path);
        diff_file(target, path);

        diff_dir(gen_dir, base_dir_tmp);
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

fn test_thrift_workspace(
    input_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    service_names: Vec<&str>,
) {
    let services: Vec<IdlService> = service_names
        .iter()
        .map(|name| IdlService::from_path(input_dir.as_ref().join(format!("{}.thrift", name))))
        .collect();
    test_with_builder_workspace(input_dir, output_dir, |_, target| {
        crate::Builder::thrift()
            .ignore_unused(false)
            .compile_with_config(services, crate::Output::Workspace(target.into()));
    });
}

fn test_thrift_workspace_with_split(
    input_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    service_names: Vec<&str>,
) {
    let services: Vec<IdlService> = service_names
        .iter()
        .map(|name| IdlService::from_path(input_dir.as_ref().join(format!("{}.thrift", name))))
        .collect();
    test_with_builder_workspace(input_dir, output_dir, |_, target| {
        crate::Builder::thrift()
            .ignore_unused(false)
            .split_generated_files(true)
            .compile_with_config(services, crate::Output::Workspace(target.into()))
    });
}

fn test_thrift_with_split(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
    gen_dir: impl AsRef<Path>,
) {
    test_with_split_builder(source, target, gen_dir, |source, target| {
        crate::Builder::thrift()
            .ignore_unused(false)
            .split_generated_files(true)
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

fn test_plugin_pb(source: impl AsRef<Path>, target: impl AsRef<Path>) {
    test_with_builder(source, target, |source, target| {
        crate::Builder::pb()
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
fn test_thrift_workspace_gen() {
    let test_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("thrift_workspace");

    let input_dir = test_data_dir.join("input");
    let output_dir = test_data_dir.join("output");

    test_thrift_workspace(input_dir, output_dir, vec!["article", "author", "image"]);
}

#[test]
fn test_thrift_workspace_with_split_gen() {
    let test_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("thrift_workspace_with_split");

    let input_dir = test_data_dir.join("input");
    let output_dir = test_data_dir.join("output");

    test_thrift_workspace_with_split(input_dir, output_dir, vec!["article", "author", "image"]);
}

#[test]
fn test_thrift_gen_with_split() {
    let test_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("thrift_with_split");

    test_data_dir.read_dir().unwrap().for_each(|f| {
        let f = f.unwrap();

        let path = f.path();

        if let Some(ext) = path.extension() {
            if ext == "thrift" {
                let mut rs_path = path.clone();
                rs_path.set_extension("rs");

                let mut gen_dir = path.clone();
                gen_dir.pop();
                gen_dir.push(rs_path.file_stem().unwrap());

                test_thrift_with_split(path, rs_path, gen_dir.as_path());
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
                let mut rs_path_new = path.clone();
                let path_clone = path.clone();
                rs_path.set_extension("rs");
                test_protobuf(path, rs_path);
                rs_path_new.set_extension("new_pb.rs");
                test_pb(path_clone, rs_path_new);
            }
        }
    });
}

#[test]
fn test_protobuf_gen_with_split() {
    let test_data_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("protobuf_with_split");

    test_data_dir.read_dir().unwrap().for_each(|f| {
        let f = f.unwrap();

        let path = f.path();

        if let Some(ext) = path.extension() {
            if ext == "proto" {
                let file_name = path.file_name().unwrap();
                let mut path_clone = path.clone();
                let mut path_clone_new = path.clone();

                path_clone.pop();
                path_clone.push("pb");
                path_clone.push(file_name);
                path_clone.set_extension("rs");

                path_clone_new.pop();
                path_clone_new.push("new_pb");
                path_clone_new.push(file_name);
                path_clone_new.set_extension("rs");

                let mut gen_dir = path_clone.clone();
                gen_dir.pop();
                gen_dir.push(path_clone.file_stem().unwrap());

                let mut gen_dir_new = path_clone_new.clone();
                gen_dir_new.pop();
                gen_dir_new.push(path_clone_new.file_stem().unwrap());

                test_protobuf_with_split(&path, path_clone, gen_dir.as_path());
                test_pb_with_split(&path, path_clone_new, gen_dir_new.as_path());
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
                let mut rs_path_new = path.clone();
                let path_clone = path.clone();
                rs_path.set_extension("rs");
                test_plugin_proto(path, rs_path);
                rs_path_new.set_extension("new_pb.rs");
                test_plugin_pb(path_clone, rs_path_new);
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

    // pb
    let file_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join("unknown_fields_pb.proto");

    let mut out_path = file_path.clone();
    out_path.set_extension("rs");

    test_with_builder(file_path, out_path, |source, target| {
        crate::Builder::protobuf()
            .ignore_unused(false)
            .keep_unknown_fields([source.into()])
            .include_dirs(vec![source.parent().unwrap().to_path_buf()])
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
        thrift::{Message, binary::TBinaryProtocol},
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

        assert_eq!(
            err.to_string(),
            "Protocol(ProtocolException { kind: InvalidData, message: \"decode struct `A` field(#1) failed, caused by: decode struct `B` field(#1) failed, caused by: invalid ttype 100\" })"
        )
    }
}
