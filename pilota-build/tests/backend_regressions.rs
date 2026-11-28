use std::sync::{Arc, Mutex};

use pilota_build::{
    Builder, CodegenBackend, Context, DefId, IdlService, MakeBackend, Output,
    codegen::thrift::ThriftBackend,
    db::RirDatabase,
    rir::Item,
    tags::protobuf::{ClientStreaming, ServerStreaming},
};

struct ContextHooks(Arc<Mutex<Vec<String>>>);

impl ContextHooks {
    fn paths(&self, cx: &Context) {
        for (id, node) in cx.nodes().iter() {
            if matches!(&node.kind, pilota_build::rir::NodeKind::Item(item) if matches!(&**item, Item::Service(_)))
            {
                for method in cx.service_methods(*id).iter() {
                    let ty = cx.codegen_item_ty(method.ret.kind.clone());
                    self.0
                        .lock()
                        .unwrap()
                        .push(ty.global_path("crate").to_string());
                    pilota_build::middle::context::tls::CUR_ITEM
                        .set(id, || self.0.lock().unwrap().push(ty.to_string()));
                    let mut referenced = Vec::new();
                    let nested = pilota_build::middle::ty::CodegenTy::Vec(Arc::new(ty));
                    let qualified = nested.qualified_path(|id| {
                        referenced.push(id);
                        "crate::Value".into()
                    });
                    assert_eq!(qualified, "::std::vec::Vec<crate::Value>");
                    assert_eq!(referenced.len(), 1);
                }
            }
        }
    }
}

impl MakeBackend for ContextHooks {
    type Target = ThriftBackend;
    fn configure_context(&self, cx: &Context) -> anyhow::Result<()> {
        self.paths(cx);
        Ok(())
    }
    fn validate(&self, cx: &Context) -> anyhow::Result<()> {
        self.paths(cx);
        Ok(())
    }
    fn make_backend(self, cx: Context) -> Self::Target {
        ThriftBackend::new(cx)
    }
}

#[test]
fn backend_hooks_can_resolve_types_during_compile_and_init_service() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("service.thrift");
    std::fs::write(
        &input,
        "struct Value {} service Echo { Value call(1:Value value) }",
    )
    .unwrap();
    for init in [false, true] {
        let paths = Arc::new(Mutex::new(Vec::new()));
        let builder = Builder::thrift().with_backend(ContextHooks(paths.clone()));
        if init {
            builder
                .init_service(IdlService {
                    path: input.clone(),
                    config: Default::default(),
                })
                .unwrap();
        } else {
            builder
                .try_compile([&input], Output::File(temp.path().join("out.rs")))
                .unwrap();
        }
        let paths = paths.lock().unwrap();
        assert_eq!(paths.len(), 4);
        assert!(paths.iter().all(|path| path.contains("Value")), "{paths:?}");
    }
}

#[derive(Clone, Default)]
struct Emitted(Arc<Mutex<Vec<(bool, bool)>>>);

#[derive(Clone)]
struct Backend {
    cx: Context,
    emitted: Emitted,
}

impl MakeBackend for Emitted {
    type Target = Backend;
    fn make_backend(self, cx: Context) -> Backend {
        // Exercise the public item emitter with a shared deduplication scope,
        // as used by generators that combine items into a common module.
        let probe = pilota_build::Codegen::new(Backend {
            cx: cx.clone(),
            emitted: self,
        });
        let mut duplicates = Default::default();
        let mut output = String::new();
        for (id, node) in cx.nodes().iter() {
            if matches!(&node.kind, pilota_build::rir::NodeKind::Item(item) if matches!(&**item, Item::Service(_)))
            {
                probe.write_item(&mut output, (*id).into(), &mut duplicates);
            }
        }
        Backend {
            cx,
            emitted: Emitted::default(),
        }
    }
}

impl CodegenBackend for Backend {
    const PROTOCOL: &'static str = "protobuf";
    fn cx(&self) -> &Context {
        &self.cx
    }
    fn codegen_service_impl(&self, id: DefId, _: &mut String, _: &pilota_build::rir::Service) {
        for method in self.cx.service_methods(id).iter() {
            let tags = self.cx.node_tags(method.def_id).unwrap();
            self.emitted.0.lock().unwrap().push((
                tags.contains::<ClientStreaming>(),
                tags.contains::<ServerStreaming>(),
            ));
        }
    }
    fn codegen_workspace_dependencies(&self) -> &'static str {
        "pilota = { version = \"*\", features = [\"pb-encode-default-value\"] }"
    }
}

#[test]
fn protobuf_dedup_preserves_all_four_streaming_contracts() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(
        temp.path().join("common.proto"),
        "syntax = \"proto3\"; package common; message Value {}",
    )
    .unwrap();
    let mut inputs = Vec::new();
    for (name, request, response) in [
        ("a", "", ""),
        ("b", "stream ", ""),
        ("c", "", "stream "),
        ("d", "stream ", "stream "),
        ("e", "", ""),
    ] {
        let path = temp.path().join(format!("{name}.proto"));
        std::fs::write(&path, format!("syntax = \"proto3\"; package {name}; import \"common.proto\"; service Test {{ rpc Call({request}common.Value) returns ({response}common.Value); }}")).unwrap();
        inputs.push(path);
    }
    let emitted = Emitted::default();
    Builder::pb()
        .include_dirs(vec![temp.path().to_owned()])
        .dedup(["Test".into()])
        .with_backend(emitted.clone())
        .try_compile(inputs, Output::File(temp.path().join("out.rs")))
        .unwrap();
    let mut modes = emitted.0.lock().unwrap().clone();
    modes.sort();
    assert_eq!(
        modes,
        [(false, false), (false, true), (true, false), (true, true)]
    );
}

#[test]
fn workspace_backend_features_merge_with_base_and_path_dependencies() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("input.proto");
    std::fs::write(&input, "syntax = \"proto3\"; package test; message Value {} service Echo { rpc Call(Value) returns (Value); }").unwrap();
    std::fs::write(
        temp.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n[workspace.dependencies]\npilota = { path = \"../pilota\" }\n",
    )
    .unwrap();
    Builder::pb()
        .include_dirs(vec![temp.path().to_owned()])
        .with_backend(Emitted::default())
        .try_compile([input], Output::Workspace(temp.path().to_owned()))
        .unwrap();
    let manifest: toml::Value =
        toml::from_str(&std::fs::read_to_string(temp.path().join("Cargo.toml")).unwrap()).unwrap();
    let dep = &manifest["workspace"]["dependencies"]["pilota"];
    assert_eq!(dep["path"].as_str(), Some("../pilota"));
    assert!(dep.get("version").is_none());
    assert_eq!(dep["features"][0].as_str(), Some("pb-encode-default-value"));
}
