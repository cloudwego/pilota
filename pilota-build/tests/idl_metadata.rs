//! IDL metadata exposed to plugins, checked directly against the source
//! declarations.
use std::sync::{Arc, Mutex};

use pilota_build::{
    Builder, Context, DefId, Output, Plugin,
    db::RirDatabase,
    middle::context::SourceType,
    plugin::walk_item,
    rir::{Item, MethodSource},
    tags::{
        protobuf::{ClientStreaming, ServerStreaming},
        thrift::ThriftStreamingMode,
    },
    ty::Ty,
};
use serde_json::{Value, json};

#[derive(Clone, Default)]
struct Inspect(Arc<Mutex<Vec<Value>>>);

fn item_path(cx: &Context, id: DefId) -> String {
    format!(
        "crate::{}",
        cx.item_path(id)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("::")
    )
}

fn type_path(cx: &Context, ty: &Ty) -> String {
    cx.codegen_item_ty(ty.kind.clone())
        .qualified_path(|id| item_path(cx, id).into())
        .to_string()
}

impl Plugin for Inspect {
    fn on_item(&mut self, cx: &Context, id: DefId, item: Arc<Item>) {
        if let Item::Service(service) = &*item {
            let thrift = matches!(cx.source_data().source_type, SourceType::Thrift);
            let raw = service.name.raw_str().to_string();
            let queried = cx.service_methods(id);
            assert_eq!(queried, cx.service_methods(id));
            let mut methods = queried.iter().map(|method| {
                let mode = if thrift {
                    match cx.thrift_streaming_mode(method.def_id) {
                        ThriftStreamingMode::Client => "ClientStreaming", ThriftStreamingMode::Server => "ServerStreaming", ThriftStreamingMode::Bidirectional => "Bidirectional", _ => "Unary",
                    }
                } else {
                    match (cx.node_contains_tag::<ClientStreaming>(method.def_id),cx.node_contains_tag::<ServerStreaming>(method.def_id)) {
                        (false,false)=>"Unary", (true,false)=>"ClientStreaming", (false,true)=>"ServerStreaming", (true,true)=>"Bidirectional",
                    }
                };
                let exceptions = method.exceptions.as_ref().map(|path| {
                    let item = cx.item(path.did).unwrap();
                    let Item::Enum(value) = &*item else {
                        panic!("declared exceptions must preserve their method-level enum")
                    };
                    json!({
                        "name": value.name.raw_str().as_str(),
                        "rust_type": item_path(cx, path.did),
                        "variants": value.variants.iter().map(|variant| json!({
                            "name": variant.name.raw_str().as_str(),
                            "rust_name": cx.rust_name(variant.did).to_string(),
                            "id": variant.id,
                            "types": variant.fields.iter().map(|ty| type_path(cx, ty)).collect::<Vec<_>>(),
                        })).collect::<Vec<_>>()
                    })
                });
                let inherited_from = match method.source {
                    MethodSource::Own => None,
                    MethodSource::Extend(parent) => Some(item_path(cx, parent)),
                };
                json!({
                    "name": method.name.raw_str().as_str(),
                    "rust_name": cx.rust_name(method.def_id).to_string(),
                    "mode": mode,
                    "thrift_annotation": thrift.then(|| format!("{:?}", cx.thrift_streaming_mode(method.def_id))),
                    "oneway": method.oneway,
                    "inherited_from": inherited_from,
                    "args": method.args.iter().map(|arg| json!({
                        "name": arg.name.raw_str().as_str(),
                        "rust_name": cx.rust_name(arg.def_id).to_string(),
                        "id": arg.id,
                        "kind": format!("{:?}", arg.kind),
                        "rust_type": type_path(cx, &arg.ty),
                    })).collect::<Vec<_>>(),
                    "return_type": type_path(cx, &method.ret),
                    "exceptions": exceptions,
                })
            }).collect::<Vec<_>>();
            methods.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
            self.0.lock().unwrap().push(json!({
                "service": raw,
                "rust_name": cx.rust_name(id).to_string(),
                "rust_path": item_path(cx, id),
                "payload": if thrift { "Thrift" } else { "Protobuf" },
                "methods": methods,
            }));
        }
        walk_item(self, cx, id, item);
    }
}

fn inspect(files: &[(&str, &str)], inputs: &[(&str, bool)]) -> Vec<Value> {
    let root = tempfile::tempdir().unwrap();
    for (name, source) in files {
        std::fs::write(root.path().join(name), source).unwrap();
    }
    let capture = Inspect::default();
    for (index, (name, protobuf)) in inputs.iter().enumerate() {
        let input = root.path().join(name);
        let output = Output::File(root.path().join(format!("output_{index}.rs")));
        if *protobuf {
            Builder::pb()
                .include_dirs(vec![root.path().to_path_buf()])
                .plugin(capture.clone())
                .try_compile([input], output)
                .unwrap();
        } else {
            Builder::thrift()
                .plugin(capture.clone())
                .try_compile([input], output)
                .unwrap();
        }
    }
    let mut rows = capture.0.lock().unwrap().clone();
    rows.sort_by(|a, b| a["rust_path"].as_str().cmp(&b["rust_path"].as_str()));
    rows
}

#[test]
fn thrift_metadata_preserves_original_names_types_inheritance_and_annotations() {
    let files = [
        (
            "base.thrift",
            r#"
namespace rs base
struct Payload { 1: required string text }
exception Failure { 1: string reason }
exception Other { 1: i32 code }
service Parent {
    Payload Inherited(3: Payload request) (streaming.mode="server")
    Payload Fallible(5: Payload request) throws (2: Failure first, 7: Other second)
} (pilota.name="Base")
"#,
        ),
        (
            "service.thrift",
            r#"
include "base.thrift"
namespace rs api
typedef base.Payload Alias
service Original extends base.Parent {
    Alias Copy(2: Alias request)
    void match()
    base.Payload OriginalCase(7: base.Payload request)
        (streaming.mode="client", pilota.name="handle_original")
    base.Payload Chat(1: base.Payload request) (streaming.mode="bidirectional")
    base.Payload ExplicitUnary(1: base.Payload request) (streaming.mode="unary")
} (pilota.name="Rewritten")
"#,
        ),
    ];
    let rows = inspect(&files, &[("service.thrift", false)]);
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| assert_eq!(inspect(&files, &[("service.thrift", false)]), rows));
        }
    });
    let service = rows
        .iter()
        .find(|row| row["service"] == "Original")
        .unwrap();
    assert_eq!(service["rust_name"], "Rewritten");
    assert_eq!(service["rust_path"], "crate::api::Rewritten");
    let method = |name: &str| {
        service["methods"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["name"] == name)
            .unwrap()
    };
    assert_eq!(method("Copy")["return_type"], "crate::api::Alias");
    assert_eq!(method("Copy")["args"][0]["rust_type"], "crate::api::Alias");
    assert_eq!(method("Copy")["args"][0]["id"], 2);
    assert_eq!(method("Copy")["thrift_annotation"], "None");
    assert_eq!(method("match")["rust_name"], "r#match");
    assert_eq!(method("OriginalCase")["rust_name"], "handle_original");
    assert_eq!(method("OriginalCase")["mode"], "ClientStreaming");
    assert_eq!(method("Chat")["mode"], "Bidirectional");
    assert_eq!(method("ExplicitUnary")["mode"], "Unary");
    assert_eq!(method("ExplicitUnary")["thrift_annotation"], "Unary");
    assert_eq!(method("Inherited")["mode"], "ServerStreaming");
    assert_eq!(method("Inherited")["inherited_from"], "crate::base::Base");
    assert_eq!(method("Inherited")["args"][0]["id"], 3);
    assert_eq!(
        method("Inherited")["args"][0]["rust_type"],
        "crate::base::Payload"
    );
    assert_eq!(
        method("Fallible")["exceptions"]["rust_type"],
        "crate::base::BaseFallibleException"
    );
    let variants = method("Fallible")["exceptions"]["variants"]
        .as_array()
        .unwrap();
    assert_eq!(
        variants
            .iter()
            .map(|v| v["id"].as_i64().unwrap())
            .collect::<Vec<_>>(),
        [2, 7]
    );
    assert_eq!(variants[0]["types"], json!(["crate::base::Failure"]));
    assert_eq!(variants[1]["types"], json!(["crate::base::Other"]));
}

#[test]
fn protobuf_metadata_preserves_all_streaming_directions_without_a_package() {
    let rows = inspect(
        &[(
            "service.proto",
            r#"
syntax = "proto3";
message Payload { string text = 1; }
service Global {
    rpc match(Payload) returns (Payload);
    rpc Upload(stream Payload) returns (Payload);
    rpc Watch(Payload) returns (stream Payload);
    rpc Chat(stream Payload) returns (stream Payload);
}
"#,
        )],
        &[("service.proto", true)],
    );
    assert_eq!(rows.len(), 1);
    let service = &rows[0];
    assert_eq!(service["service"], "Global");
    assert_eq!(service["rust_path"], "crate::Global");
    for (name, mode) in [
        ("match", "Unary"),
        ("Upload", "ClientStreaming"),
        ("Watch", "ServerStreaming"),
        ("Chat", "Bidirectional"),
    ] {
        let method = service["methods"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["name"] == name)
            .unwrap();
        assert_eq!(method["mode"], mode);
        assert_eq!(method["thrift_annotation"], Value::Null);
        assert_eq!(method["return_type"], "crate::Payload");
    }
}

#[test]
fn ordinary_oneway_retains_zero_one_and_multiple_parameters() {
    let rows = inspect(
        &[(
            "oneway.thrift",
            r#"
service Shapes {
    oneway void Zero()
    oneway void One(3: i64 value)
    oneway void Many(2: i32 first, 9: i64 second)
}
"#,
        )],
        &[("oneway.thrift", false)],
    );
    assert_eq!(rows.len(), 1);
    let methods = rows[0]["methods"].as_array().unwrap();
    for (name, ids, types) in [
        ("Zero", vec![], vec![]),
        ("One", vec![3], vec!["i64"]),
        ("Many", vec![2, 9], vec!["i32", "i64"]),
    ] {
        let method = methods.iter().find(|m| m["name"] == name).unwrap();
        assert_eq!(method["oneway"], true);
        assert_eq!(method["mode"], "Unary");
        assert_eq!(method["thrift_annotation"], "None");
        assert_eq!(method["return_type"], "()");
        assert_eq!(method["exceptions"], Value::Null);
        let args = method["args"].as_array().unwrap();
        assert_eq!(
            args.iter()
                .map(|a| a["id"].as_i64().unwrap())
                .collect::<Vec<_>>(),
            ids
        );
        assert_eq!(
            args.iter()
                .map(|a| a["rust_type"].as_str().unwrap())
                .collect::<Vec<_>>(),
            types
        );
    }
}
