//! A downstream plugin uses only neutral Pilota APIs in every output layout.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
};

use pilota_build::{
    Builder, Context, DefId, IdlService, MakeBackend, Output, Plugin, db::RirDatabase,
    parser::Parser, plugin::walk_item, rir::Item,
};

#[path = "support/contract_plugin.rs"]
mod contract_plugin;

#[derive(Clone, Default)]
struct Attach(Arc<Mutex<Vec<String>>>);

impl Plugin for Attach {
    fn on_item(&mut self, cx: &Context, id: DefId, item: Arc<Item>) {
        if let Item::Service(service) = &*item {
            self.0
                .lock()
                .unwrap()
                .push(service.name.raw_str().to_string());
            let module = format!("{}_contract", cx.rust_name(id).to_ascii_lowercase());
            let methods = cx.service_methods(id);
            let first = &methods[0];
            let qualify = |target| {
                let path = cx.related_item_path(id, target);
                if path.starts_with("::") || path.starts_with("crate::") {
                    path
                } else {
                    format!("super::{}", path.strip_prefix("self::").unwrap_or(&path)).into()
                }
            };
            let request = cx
                .codegen_item_ty(first.args[0].ty.kind.clone())
                .qualified_path(qualify);
            let response = cx
                .codegen_item_ty(first.ret.kind.clone())
                .qualified_path(qualify);
            cx.with_adjust_mut(id, |adjust| {
                adjust.add_nested_item(
                    format!(
                        "pub mod {module} {{
                        pub const NEUTRAL_PLUGIN_MARKER: &str = {:?};
                        pub type Request = {request};
                        pub type Response = {response};
                    }}",
                        service.name.raw_str().as_str()
                    )
                    .into(),
                );
            });
        }
        walk_item(self, cx, id, item);
    }
}

fn rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(rust_files(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    files
}

fn layouts<M, P>(builder: fn() -> Builder<M, P>, input: &str, expected: &[&str])
where
    M: MakeBackend + Send,
    M::Target: Send,
    P: Parser,
{
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data/plugin_contract");
    for workspace in [false, true] {
        for split in [false, true] {
            for enabled in [false, true] {
                let temp = tempfile::tempdir().unwrap();
                let capture = Attach::default();
                let output = if workspace {
                    std::fs::write(
                        temp.path().join("Cargo.toml"),
                        "[workspace]\nmembers = []\n",
                    )
                    .unwrap();
                    Output::Workspace(temp.path().to_path_buf())
                } else {
                    Output::File(temp.path().join("generated.rs"))
                };
                let mut builder = builder()
                    .include_dirs(vec![root.clone()])
                    .split_generated_files(split);
                if enabled {
                    builder = builder.plugin(capture.clone());
                }
                builder.try_compile([root.join(input)], output).unwrap();
                let mut visits = capture.0.lock().unwrap().clone();
                visits.sort();
                let mut expected = if enabled { expected.to_vec() } else { vec![] };
                expected.sort();
                // Duplicate visits fail even when they would emit identical text.
                assert_eq!(visits, expected, "workspace={workspace}, split={split}");
                let files = rust_files(temp.path());
                assert!(!files.is_empty());
                let mut markers = 0;
                for path in files {
                    let source = std::fs::read_to_string(&path).unwrap();
                    source
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
                    markers += source.matches("pub const NEUTRAL_PLUGIN_MARKER").count();
                    assert!(!source.contains("volo_streamx"));
                    assert!(!source.contains("pilota_build"));
                }
                assert_eq!(markers, expected.len());
            }
        }
    }
}

#[test]
fn thrift_services_are_visited_once_and_opt_in_items_follow_all_layouts() {
    layouts(Builder::thrift, "api.thrift", &["Child", "Parent"]);
}

#[test]
fn protobuf_services_are_visited_once_and_opt_in_items_follow_all_layouts() {
    layouts(Builder::pb, "api.proto", &["Child"]);
}

fn compile_contract<M, P>(
    builder: Builder<M, P>,
    protocol: &str,
    output: &Path,
    workspace: bool,
    split: bool,
    target: &Path,
) where
    M: MakeBackend + Send,
    M::Target: Send,
    P: Parser,
{
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let idl = root.join("test_data/plugin_contract");
    let protobuf = protocol == "protobuf";
    let extension = if protobuf { "proto" } else { "thrift" };
    fs::create_dir_all(output.join("consumer/src")).unwrap();
    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[workspace]\nmembers = []\nresolver = \"3\"\n\
             [workspace.dependencies]\npilota = {{ path = {:?} }}\n",
            root.join("../pilota").canonicalize().unwrap()
        ),
    )
    .unwrap();
    // Reuse this checkout's resolved runtime dependencies in the nested build.
    fs::copy(root.join("../Cargo.lock"), output.join("Cargo.lock")).unwrap();

    let mut builder = builder
        .include_dirs(vec![idl.clone()])
        .dedup(["Value".into()])
        .split_generated_files(split);
    builder = if split {
        builder
            .plugin(contract_plugin::NumberPlugin)
            .plugin(contract_plugin::ContractPlugin)
    } else {
        builder
            .plugin(contract_plugin::ContractPlugin)
            .plugin(contract_plugin::NumberPlugin)
    };

    let (imports, dependencies) = if workspace {
        let inputs: Vec<_> = ["api", "common", "left", "right"]
            .map(|name| IdlService {
                path: idl.join(format!("{name}.{extension}")),
                config: serde_yaml::from_str(&format!("crate_name: {name}_types")).unwrap(),
            })
            .into();
        builder
            .ignore_unused(false)
            .touch_files(inputs.iter().map(|service| service.path.clone()))
            .try_compile_with_config(inputs, Output::Workspace(output.to_path_buf()))
            .unwrap();
        let dependencies = "api_types = { path = \"../api_types\" }\n\
                            common_types = { path = \"../common_types\" }\n";
        if protobuf {
            (
                "",
                format!(
                    "{dependencies}left_types = {{ path = \"../left_types\" }}\n\
                     right_types = {{ path = \"../right_types\" }}\n"
                ),
            )
        } else {
            (
                "use common::{contract_common::Failure, contract_left as left_types, \
                 contract_right as right_types};\n",
                format!("{dependencies}common = {{ path = \"../common\" }}\n"),
            )
        }
    } else {
        builder
            .try_compile(
                [idl.join(format!("api.{extension}"))],
                Output::File(output.join("consumer/src/generated.rs")),
            )
            .unwrap();
        (
            if protobuf {
                "include!(\"generated.rs\");\n\
                 use generated::{contract_api as api_types, contract_common as common_types, \
                 contract_left as left_types, contract_right as right_types};\n"
            } else {
                "include!(\"generated.rs\");\n\
                 use generated::{contract_api as api_types, contract_common as common_types, \
                 contract_common::Failure, contract_left as left_types, contract_right as right_types};\n"
            },
            "pilota.workspace = true\n".to_owned(),
        )
    };
    fs::write(
        output.join("consumer/Cargo.toml"),
        format!(
            "[package]\nname = \"contract-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
             [dependencies]\n{dependencies}"
        ),
    )
    .unwrap();
    let consumer = fs::read_to_string(idl.join(format!("{protocol}_consumer.rs"))).unwrap();
    fs::write(
        output.join("consumer/src/lib.rs"),
        format!("{imports}{consumer}"),
    )
    .unwrap();
    let manifest_path = output.join("Cargo.toml");
    let mut manifest: toml::Value =
        toml::from_str(&fs::read_to_string(&manifest_path).unwrap()).unwrap();
    manifest["workspace"]["members"]
        .as_array_mut()
        .unwrap()
        .push("consumer".into());
    fs::write(manifest_path, toml::to_string(&manifest).unwrap()).unwrap();

    // A separate target directory avoids Cargo's lock on the parent test build;
    // all layouts share it so their dependencies only need to compile once.
    let result = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .current_dir(output)
        .args(["test", "--workspace", "--lib"])
        .arg("--target-dir")
        .arg(target)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{protocol}, workspace={workspace}, split={split}:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn attached_aliases_and_field_plugins_preserve_identity_in_all_layouts() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    for (layout, workspace, split) in [
        ("single", false, false),
        ("split", false, true),
        ("workspace", true, true),
    ] {
        compile_contract(
            Builder::thrift(),
            "thrift",
            &temp.path().join(format!("thrift-{layout}")),
            workspace,
            split,
            &target,
        );
        compile_contract(
            Builder::pb(),
            "protobuf",
            &temp.path().join(format!("protobuf-{layout}")),
            workspace,
            split,
            &target,
        );
    }
}
