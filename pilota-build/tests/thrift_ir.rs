//! Thrift metadata and backend extension contracts.
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use pilota_build::{
    Builder, Context, MakeBackend, Output,
    codegen::thrift::ThriftBackend,
    db::RirDatabase,
    rir::{Item, MethodSource, NodeKind},
};
use serde_json::{Value, json};

#[derive(Clone, Default)]
struct Inspect(Arc<Mutex<Vec<Value>>>);

struct Reject;

impl MakeBackend for Reject {
    type Target = ThriftBackend;

    fn validate(&self, _: &Context) -> anyhow::Result<()> {
        anyhow::bail!("unsupported fixture before emission")
    }

    fn make_backend(self, _: Context) -> Self::Target {
        panic!("a rejected backend must never emit code")
    }
}

#[test]
fn backend_validation_reports_an_error_without_writing_output() {
    let temp = tempfile::tempdir().unwrap();
    let idl = temp.path().join("rejected.thrift");
    std::fs::write(&idl, "struct Value { 1: i32 value }").unwrap();
    let output = temp.path().join("generated.rs");
    let error = Builder::thrift()
        .with_backend(Reject)
        .try_compile([idl], Output::File(output.clone()))
        .unwrap_err();
    assert!(error.to_string().contains("unsupported fixture"));
    assert!(!output.exists());
}

impl MakeBackend for Inspect {
    type Target = ThriftBackend;

    fn make_backend(self, cx: Context) -> Self::Target {
        for (id, node) in cx.nodes().iter() {
            let NodeKind::Item(item) = &node.kind else {
                continue;
            };
            let Item::Service(service) = &**item else {
                continue;
            };
            for method in cx.service_methods(*id).iter() {
                let exceptions = method.exceptions.as_ref().map(|path| {
                    let item = cx.item(path.did).unwrap();
                    let Item::Enum(enumeration) = &*item else {
                        panic!("exceptions must be a method-level enum")
                    };
                    json!({
                        "name": enumeration.name.raw_str().as_str(),
                        "variants": enumeration.variants.iter().map(|variant| json!({
                            "name": variant.name.raw_str().as_str(), "id": variant.id,
                            "payload_types": variant.fields.len(),
                        })).collect::<Vec<_>>(),
                    })
                });
                self.0.lock().unwrap().push(json!({
                    "service": service.name.raw_str().as_str(),
                    "method": method.name.raw_str().as_str(),
                    "mode": format!("{:?}", cx.thrift_streaming_mode(method.def_id)),
                    "args": method.args.iter().map(|arg| json!({"name": arg.name.raw_str().as_str(), "id": arg.id})).collect::<Vec<_>>(),
                    "inherited": matches!(method.source, MethodSource::Extend(_)),
                    "exceptions": exceptions,
                }));
            }
        }
        ThriftBackend::new(cx)
    }
}

#[test]
fn backend_can_reuse_structured_ir_and_exception_identity() {
    let idl = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data/thrift/payload.thrift");
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("payload.rs");
    let inspection = Inspect::default();
    Builder::thrift()
        .with_backend(inspection.clone())
        .try_compile([idl], Output::File(output.clone()))
        .unwrap();
    let rows = inspection.0.lock().unwrap();
    let method = |name: &str| {
        rows.iter()
            .find(|row| row["service"] == "Interop" && row["method"] == name)
            .unwrap()
    };
    let inherited = method("Inherited");
    assert_eq!(inherited["inherited"], true);
    assert_eq!(inherited["mode"], "Server");
    assert_eq!(method("Unary")["mode"], "None");
    assert_eq!(method("ExplicitUnary")["mode"], "Unary");
    assert_eq!(method("Client")["mode"], "Client");
    assert_eq!(method("Bidi")["mode"], "Bidirectional");
    assert_eq!(
        method("Multi")["args"],
        json!([{"name":"left","id":2},{"name":"right","id":9}])
    );
    assert_eq!(method("Zero")["args"], json!([]));
    assert_eq!(method("Unary")["exceptions"], Value::Null);
    assert_eq!(
        method("Throw")["exceptions"],
        json!({"name":"InteropThrowException","variants":[
            {"name":"failure","id":2,"payload_types":1}, {"name":"other","id":7,"payload_types":1}
        ]})
    );
    assert_eq!(
        method("VoidThrow")["exceptions"]["variants"],
        json!([
            {"name":"failure","id":4,"payload_types":1}
        ])
    );
    let generated = std::fs::read_to_string(&output).unwrap();
    for name in [
        "InteropUnaryArgsSend",
        "InteropUnaryResultRecv",
        "InteropMultiArgsSend",
        "InteropThrowException",
        "InteropVoidThrowException",
    ] {
        assert!(generated.contains(name), "missing {name}");
    }
    assert!(!generated.contains("InteropBidiArgsSend"));
}

#[test]
fn explicit_unary_parameter_matrix_preserves_the_existing_idl_rules() {
    for annotated in [false, true] {
        for count in 0..=2 {
            let temp = tempfile::tempdir().unwrap();
            let input = temp.path().join("matrix.thrift");
            let output = temp.path().join("matrix.rs");
            let args = (1..=count)
                .map(|id| format!("{id}: Message arg{id}"))
                .collect::<Vec<_>>()
                .join(",");
            let annotation = if annotated {
                "(streaming.mode=\"unary\")"
            } else {
                ""
            };
            std::fs::write(
                &input,
                format!(
                    "struct Message {{}} service Matrix {{ Message Call({args}) {annotation} }}"
                ),
            )
            .unwrap();
            let result = Builder::thrift().try_compile([input], Output::File(output.clone()));
            assert_eq!(
                result.is_ok(),
                !annotated || count == 1,
                "annotated={annotated}, args={count}"
            );
            assert_eq!(output.exists(), result.is_ok());
            if let Err(error) = result {
                assert!(error.to_string().contains("exactly one argument"));
            }
        }
    }
}

#[test]
fn ordinary_thrift_still_accepts_oneway_and_streaming_union_aliases() {
    let temp = tempfile::tempdir().unwrap();
    let input = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data/thrift/backend_types.thrift");
    let output = temp.path().join("types.rs");

    Builder::thrift()
        .ignore_unused(false)
        .try_compile([input], Output::File(output.clone()))
        .unwrap();
    let generated = std::fs::read_to_string(&output).unwrap();
    for expected in [
        "pub enum Event",
        "pub struct Alias",
        "::std::boxed::Box<Node>",
        "pub struct RecursivePayload",
        "pub struct EmptyPayload",
        "::std::boxed::Box<RecursivePayload>",
        "pub trait Types",
    ] {
        assert!(generated.contains(expected), "missing {expected}");
    }
}
