use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use pilota_build::{
    BoxClonePlugin, Builder, Context, DefId, MakeBackend, Output, Plugin,
    db::RirDatabase,
    parser::Parser,
    plugin::{walk_codegen_uint, walk_item},
    rir::Item,
};

#[derive(Clone)]
struct NamedPlugin {
    target: &'static str,
    errors: Vec<String>,
    units: Arc<AtomicUsize>,
}

impl NamedPlugin {
    fn new(target: &'static str, units: Arc<AtomicUsize>) -> Self {
        Self {
            target,
            errors: Vec::new(),
            units,
        }
    }
}

impl Plugin for NamedPlugin {
    fn on_codegen_uint(&mut self, cx: &Context, items: &[DefId]) {
        self.errors.clear();
        self.units.fetch_add(1, Ordering::Relaxed);
        walk_codegen_uint(self, cx, items);
    }

    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        if let Item::Service(service) = &*item {
            let name = format!("{}_extra", cx.rust_name(def_id).to_ascii_lowercase());
            let source = format!(
                "pub mod {name} {{ pub const TARGET: &str = {:?}; }}",
                self.target
            );
            if let Err(error) = cx.with_adjust_mut(def_id, |adjust| {
                adjust.add_named_nested_item(name.into(), source.into())
            }) {
                let node = cx.node(def_id).unwrap();
                let file = &cx.file_paths()[&node.file_id];
                self.errors.push(format!(
                    "{}: service `{}`: {error}",
                    file.display(),
                    service.name.raw_str()
                ));
            }
        }
        walk_item(self, cx, def_id, item);
    }

    fn validate(&self, _: &Context) -> anyhow::Result<()> {
        if let Some(error) = self.errors.first() {
            anyhow::bail!(error.clone());
        }
        Ok(())
    }
}

fn check_plugin_diagnostics<M, P>(builder: fn() -> Builder<M, P>, file: &str, source: &str)
where
    M: MakeBackend + Send,
    M::Target: Send,
    P: Parser,
{
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join(file);
    std::fs::write(&input, source).unwrap();
    let output = temp.path().join("generated.rs");
    let units = Arc::new(AtomicUsize::new(0));

    // Both duplicate installs and conflicting controlled configurations fail before
    // writing.
    for next in ["first", "second"] {
        let error = builder()
            .include_dirs(vec![temp.path().to_path_buf()])
            .plugin(BoxClonePlugin::new(NamedPlugin::new(
                "first",
                units.clone(),
            )))
            .plugin(BoxClonePlugin::new(NamedPlugin::new(next, units.clone())))
            .try_compile([input.clone()], Output::File(output.clone()))
            .unwrap_err();
        let error = error.to_string();
        assert!(error.contains(file), "{error}");
        assert!(error.contains("service `Echo`"), "{error}");
        assert!(
            error.contains("duplicate generated item `echo_extra`"),
            "{error}"
        );
        assert!(!output.exists());
    }
    assert_eq!(units.load(Ordering::Relaxed), 4);

    // A fresh invocation after the diagnostic emits one complete module.
    builder()
        .include_dirs(vec![temp.path().to_path_buf()])
        .plugin(BoxClonePlugin::new(NamedPlugin::new(
            "repaired",
            units.clone(),
        )))
        .try_compile([input], Output::File(output.clone()))
        .unwrap();
    let source = std::fs::read_to_string(output).unwrap();
    assert_eq!(source.matches("pub mod echo_extra").count(), 1);
    assert!(source.contains("repaired"));
    assert_eq!(units.load(Ordering::Relaxed), 5);
}

#[test]
fn thrift_clone_box_and_borrow_forward_plugin_validation_before_output() {
    check_plugin_diagnostics(
        Builder::thrift,
        "probe.thrift",
        "namespace rs probe struct Value {} service Echo { Value call(1: Value req) }",
    );
}

#[test]
fn protobuf_clone_box_and_borrow_forward_plugin_validation_before_output() {
    check_plugin_diagnostics(
        Builder::pb,
        "probe.proto",
        "syntax = \"proto3\"; package probe; message Value {} service Echo { rpc Call(Value) returns (Value); }",
    );
}

#[test]
fn old_plugins_keep_the_default_validation_behavior() {
    #[derive(Clone)]
    struct OldPlugin;
    impl Plugin for OldPlugin {}
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("old.thrift");
    std::fs::write(&input, "struct Value { 1: i32 value }").unwrap();
    let output = temp.path().join("generated.rs");
    Builder::thrift()
        .ignore_unused(false)
        .plugin(OldPlugin)
        .try_compile([input], Output::File(output.clone()))
        .unwrap();
    let source = std::fs::read_to_string(output).unwrap();
    assert!(source.contains("Value"), "{source}");
}

#[test]
fn boxed_plugins_forward_variant_callbacks_when_composed() {
    struct Variants(Arc<AtomicUsize>);
    impl Plugin for Variants {
        fn on_variant(&mut self, _: &Context, _: DefId, _: Arc<pilota_build::rir::EnumVariant>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    struct Composed(Box<dyn Plugin>);
    impl Plugin for Composed {
        fn on_item(&mut self, cx: &Context, id: DefId, item: Arc<Item>) {
            walk_item(&mut self.0, cx, id, item);
        }
    }
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("values.thrift");
    std::fs::write(&input, "enum Value { First = 1, Second = 2 }").unwrap();
    let count = Arc::new(AtomicUsize::new(0));
    Builder::thrift()
        .ignore_unused(false)
        .plugin(Composed(Box::new(Variants(count.clone()))))
        .try_compile([input], Output::File(temp.path().join("out.rs")))
        .unwrap();
    assert_eq!(count.load(Ordering::Relaxed), 2);
}
