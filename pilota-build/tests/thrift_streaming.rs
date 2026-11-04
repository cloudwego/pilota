use std::{
    path::Path,
    process::Command,
    sync::{Arc, Mutex},
};

use pilota_build::{
    Builder, Context, MakeBackend, Output, codegen::thrift::ThriftBackend, db::RirDatabase,
    rir::Item, tags::thrift::ThriftStreamingMode as Mode,
};

type Inspection = Vec<(String, Vec<(String, Mode)>, bool)>;

#[derive(Clone)]
struct Inspect(Arc<Mutex<Inspection>>);

impl MakeBackend for Inspect {
    type Target = ThriftBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        for (id, node) in context.nodes().iter() {
            if let pilota_build::rir::NodeKind::Item(item) = &node.kind {
                if let Item::Service(service) = &**item {
                    let methods = context
                        .service_methods(*id)
                        .iter()
                        .map(|method| {
                            let mode = context.thrift_streaming_mode(method.def_id);
                            if mode.is_streaming() {
                                assert_eq!(method.args.len(), 1);
                                assert!(method.exceptions.is_none());
                            }
                            (method.name.to_string(), mode)
                        })
                        .collect();
                    self.0.lock().unwrap().push((
                        service.name.to_string(),
                        methods,
                        context.has_thrift_streaming_methods(*id),
                    ));
                }
            }
        }
        ThriftBackend::new(context)
    }
}

fn compile(path: &Path, out: &Path) {
    Builder::thrift().compile([path], Output::File(out.into()));
}

// Runs in a subprocess because the existing Builder/Parser API reports fatal
// IDL diagnostics by exiting, rather than returning a Result.
#[test]
fn compile_child() {
    if let (Some(input), Some(output)) = (
        std::env::var_os("PILOTA_STREAMING_TEST_INPUT"),
        std::env::var_os("PILOTA_STREAMING_TEST_OUTPUT"),
    ) {
        compile(Path::new(&input), Path::new(&output));
    }
}

fn rejects(files: &[(&str, &str)], expected: &[&str]) {
    let temp = tempfile::tempdir().unwrap();
    for (name, content) in files {
        std::fs::write(temp.path().join(name), content).unwrap();
    }
    let output = temp.path().join("out.rs");
    let result = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "compile_child", "--nocapture"])
        .env("PILOTA_STREAMING_TEST_INPUT", temp.path().join(files[0].0))
        .env("PILOTA_STREAMING_TEST_OUTPUT", &output)
        .output()
        .unwrap();
    assert!(!result.status.success(), "accepted invalid IDL");
    let stderr = String::from_utf8_lossy(&result.stderr);
    for fragment in expected {
        assert!(stderr.contains(fragment), "missing {fragment:?}: {stderr}");
    }
    assert!(!output.exists(), "must not emit a partial service");
    assert!(
        !stderr.contains("panicked"),
        "diagnostic must not panic: {stderr}"
    );
}

#[test]
fn unused_service_throws_rejects_the_entire_file() {
    rejects(
        &[(
            "bad.thrift",
            "struct Req {} exception Ex {} service Good { Req ok(1:Req req) }\nservice Unused {\n Req broken(1:Req req) throws (1:Ex ex) (streaming.mode=\"server\")\n}",
        )],
        &[
            "bad.thrift:3:",
            "service `Unused`",
            "method `broken`",
            "streaming `throws`",
        ],
    );
}

#[test]
fn repaired_streaming_throws_generates_both_services() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("repaired.thrift");
    std::fs::write(&input, "namespace rs repaired struct Req {} exception Ex {} service Good { Req ok(1:Req req) throws(1:Ex ex) } service PreviouslyInvalid { Req watch(1:Req req) (streaming.mode=\"server\") }").unwrap();
    let inspection = Inspect(Arc::default());
    Builder::thrift()
        .with_backend(inspection.clone())
        .try_compile([input], Output::File(temp.path().join("output.rs")))
        .unwrap();
    let services = inspection.0.lock().unwrap();
    assert!(
        services
            .iter()
            .any(|(name, _, streaming)| name == "Good" && !streaming)
    );
    assert!(
        services
            .iter()
            .any(|(name, _, streaming)| name == "PreviouslyInvalid" && *streaming)
    );
    let generated = std::fs::read_to_string(temp.path().join("output.rs")).unwrap();
    assert!(generated.contains("GoodOkArgsSend"));
    assert!(generated.contains("GoodOkResultRecv"));
    assert!(!generated.contains("PreviouslyInvalidWatchArgsSend"));
}

#[test]
fn included_service_throws_is_not_skipped() {
    rejects(
        &[
            (
                "main.thrift",
                "include \"base.thrift\" service Child extends base.Parent {}",
            ),
            (
                "base.thrift",
                "struct Req {} exception Ex {} service Parent { Req broken(1:Req req) throws (1:Ex ex) (streaming.mode=\"client\") }",
            ),
        ],
        &[
            "base.thrift:",
            "service `Parent`",
            "method `broken`",
            "streaming `throws`",
        ],
    );
}

#[test]
fn bad_annotations_and_message_shapes_fail_before_emission() {
    for (method, diagnostic) in [
        (
            "Req f(1:Req req) (streaming.mode=\"bidi\")",
            "invalid `streaming.mode`",
        ),
        (
            "Req f(1:Req req) (streaming.mode=\"client\", streaming.mode=\"server\")",
            "duplicate `streaming.mode`",
        ),
        (
            "Req f() (streaming.mode=\"server\")",
            "exactly one argument",
        ),
        (
            "Req f(1:Req req,2:Req other) (streaming.mode=\"server\")",
            "exactly one argument",
        ),
        (
            "void f(1:Req req) (streaming.mode=\"server\")",
            "not `void`",
        ),
        (
            "oneway void f(1:Req req) (streaming.mode=\"client\")",
            "cannot be `oneway`",
        ),
        (
            "Req f(1:i32 req) (streaming.mode=\"client\")",
            "not scalar or container",
        ),
        (
            "Alias f(1:Req req) (streaming.mode=\"client\")",
            "not scalar or container",
        ),
        (
            "Req f(1:list<Req> req) (streaming.mode=\"client\")",
            "not scalar or container",
        ),
    ] {
        let idl = format!("struct Req {{}} typedef i64 Alias service S {{ {method} }}");
        rejects(
            &[("bad.thrift", &idl)],
            &["service `S`", "method `f`", diagnostic],
        );
    }
}

#[test]
fn modes_raw_types_inheritance_and_mixed_services_reach_backends() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("base.thrift"), "namespace rs base struct Req { 1:optional Req next } typedef Req Alias service Parent { Req inherited(1:Alias req) (streaming.mode=\"server\") }").unwrap();
    std::fs::write(
        temp.path().join("mixed.thrift"),
        r#"include "base.thrift"
namespace rs mixed
exception Ex {}
service Mixed extends base.Parent {
    base.Req plain(1:base.Req req) throws (1:Ex ex)
    base.Req unary(1:base.Req req) (streaming.mode="unary")
    base.Req client(1:base.Req req) (streaming.mode="client")
    base.Req server(1:base.Req req) (streaming.mode="server")
    base.Alias bidi(1:base.Alias req) (streaming.mode="bidirectional")
} (pilota.rust_wrapper_arc="true")
service Ordinary { void ping() }
service ExplicitUnary { base.Req invoke(1:base.Req req) (streaming.mode="unary") }
"#,
    )
    .unwrap();
    let inspection = Inspect(Arc::default());
    let output = temp.path().join("out.rs");
    Builder::thrift().with_backend(inspection.clone()).compile(
        [temp.path().join("mixed.thrift")],
        Output::File(output.clone()),
    );
    let services = inspection.0.lock().unwrap();
    let (_, methods, streaming) = services
        .iter()
        .find(|(name, _, _)| name == "Mixed")
        .unwrap();
    assert!(*streaming);
    assert_eq!(
        methods,
        &vec![
            ("inherited".into(), Mode::Server),
            ("plain".into(), Mode::None),
            ("unary".into(), Mode::Unary),
            ("client".into(), Mode::Client),
            ("server".into(), Mode::Server),
            ("bidi".into(), Mode::Bidirectional)
        ]
    );
    for name in ["Ordinary", "ExplicitUnary"] {
        assert!(
            !services
                .iter()
                .find(|(service, _, _)| service == name)
                .unwrap()
                .2
        );
    }
    let generated = std::fs::read_to_string(output).unwrap();
    assert!(generated.contains("MixedPlainArgsSend"));
    assert!(generated.contains("MixedUnaryArgsSend"));
    assert!(!generated.contains("MixedClientArgsSend"));
    assert!(!generated.contains("MixedBidiResultRecv"));
}

#[test]
fn fixed_go_idl_matrix_has_explicit_rust_outcomes() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/interop/kitex-codec");
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(fixtures.join("idl-matrix.json")).unwrap())
            .unwrap();
    assert_eq!(report["schemaVersion"], 1);
    for case in report["cases"].as_array().unwrap() {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("out.rs");
        let result = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "compile_child", "--nocapture"])
            .env(
                "PILOTA_STREAMING_TEST_INPUT",
                fixtures.join(case["input"].as_str().unwrap()),
            )
            .env("PILOTA_STREAMING_TEST_OUTPUT", &output)
            .output()
            .unwrap();
        let accepted = case["expected_rust"] == "accepted";
        assert_eq!(
            result.status.success(),
            accepted,
            "case {}: {}",
            case["name"],
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            output.exists(),
            accepted,
            "case {} must not produce partial output",
            case["name"]
        );
    }
}

#[test]
fn fallible_builder_returns_diagnostics_without_terminating_the_host() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("bad.thrift");
    let output = temp.path().join("out.rs");
    for idl in [
        "struct Req {} exception Ex {} service Good { Req ok(1:Req req) } service Bad { Req fail(1:Req req) throws (1:Ex ex) (streaming.mode=\"client\") }",
        "struct Req {} service Bad { i32 fail(1:Req req) (streaming.mode=\"server\") }",
        "service Bad { syntax error",
        "include \"missing.thrift\" service Bad {}",
    ] {
        std::fs::write(&input, idl).unwrap();
        let result = Builder::thrift().try_compile([&input], Output::File(output.clone()));
        assert!(result.is_err(), "{idl}");
        assert!(!output.exists());
    }
    // A failed build has no global failure state and cannot poison later builds.
    std::fs::write(
        &input,
        "struct Req {} service Good { Req ok(1:Req req) (streaming.mode=\"client\") }",
    )
    .unwrap();
    Builder::thrift()
        .try_compile([input], Output::File(output.clone()))
        .unwrap();
    assert!(output.exists());
}

#[derive(Clone)]
struct Emissions(Arc<Mutex<Vec<bool>>>);

#[derive(Clone)]
struct EmissionBackend {
    context: Context,
    emissions: Emissions,
}

impl MakeBackend for Emissions {
    type Target = EmissionBackend;
    fn make_backend(self, context: Context) -> Self::Target {
        EmissionBackend {
            context,
            emissions: self,
        }
    }
}

impl pilota_build::CodegenBackend for EmissionBackend {
    const PROTOCOL: &'static str = "thrift";
    fn cx(&self) -> &Context {
        &self.context
    }
    fn codegen_service_impl(
        &self,
        id: pilota_build::DefId,
        _stream: &mut String,
        _service: &pilota_build::rir::Service,
    ) {
        self.emissions
            .0
            .lock()
            .unwrap()
            .push(self.context.has_thrift_streaming_methods(id));
    }
}

#[test]
fn service_dedup_cannot_erase_a_different_streaming_contract() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(
        temp.path().join("common.thrift"),
        "namespace rs common struct Req {}",
    )
    .unwrap();
    std::fs::write(temp.path().join("a.thrift"), "include \"common.thrift\" namespace rs a service Test { common.Req call(1:common.Req req) }").unwrap();
    std::fs::write(temp.path().join("b.thrift"), "include \"common.thrift\" namespace rs b service Test { common.Req call(1:common.Req req) (streaming.mode=\"server\") }").unwrap();
    let emissions = Emissions(Arc::default());
    Builder::thrift()
        .dedup(["Test".into()])
        .with_backend(emissions.clone())
        .try_compile(
            [temp.path().join("a.thrift"), temp.path().join("b.thrift")],
            Output::File(temp.path().join("out.rs")),
        )
        .unwrap();
    let mut modes = emissions.0.lock().unwrap().clone();
    modes.sort_unstable();
    assert_eq!(modes, [false, true]);
}

#[test]
fn workspace_and_split_generation_preserve_streaming_descriptions() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/interop/kitex-codec/idl/typing.thrift");
    for split in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        let inspection = Inspect(Arc::default());
        std::fs::create_dir(temp.path().join("workspace")).unwrap();
        std::fs::write(
            temp.path().join("workspace/Cargo.toml"),
            "[workspace]\nmembers = []\n",
        )
        .unwrap();
        Builder::thrift()
            .with_backend(inspection.clone())
            .split_generated_files(split)
            .try_compile([&fixture], Output::Workspace(temp.path().join("workspace")))
            .unwrap();
        let descriptions = inspection.0.lock().unwrap();
        let (_, methods, streaming) = descriptions
            .iter()
            .find(|(name, _, _)| name == "Mixed")
            .unwrap();
        assert!(*streaming);
        assert_eq!(
            methods.iter().map(|(_, mode)| *mode).collect::<Vec<_>>(),
            [Mode::None, Mode::Server, Mode::Bidirectional]
        );
    }
}
