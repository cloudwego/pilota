use std::{fs, process::Command};

use pilota_build::{Builder, Output};

#[test]
fn unused_workspace_types_do_not_affect_escaped_names() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    fs::write(
        root.join("unused.thrift"),
        "struct Unused {} (pilota.name=\"Send__\")",
    )
    .unwrap();
    let input = root.join("service.thrift");
    fs::write(
        &input,
        "include \"unused.thrift\"
         struct Send {}
         struct Send_ { 1: optional Send value }
         service Echo { Send_ echo(1: Send value) }",
    )
    .unwrap();
    let output = root.join("output");
    fs::create_dir(&output).unwrap();
    fs::write(output.join("Cargo.toml"), "[workspace]\nmembers = []\n").unwrap();
    Builder::thrift()
        .include_dirs(vec![root.to_path_buf()])
        .try_compile([input], Output::Workspace(output))
        .unwrap();
    let generated = fs::read_to_string(root.join("output/service/src/gen.rs")).unwrap();
    assert!(generated.contains("pub struct Send__ {"));
}

#[test]
fn reserved_type_names_compile() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let mut includes = String::new();

    for change_case in [true, false] {
        for (kind, source) in [
            (
                "structs",
                "struct Send { 1: required i32 value }
                 struct T { 1: required Send value }
                 struct Holder { 1: required T value }
                 const T DEFAULT = {\"value\": {\"value\": 42}}
                 union Choice { 1: Send Send, 2: T T }
                 service Echo { T echo(1: Send value) }",
            ),
            (
                "enums",
                "enum Send { ZERO = 0 }
                 enum T { ZERO = 0 }
                 struct Holder { 1: required Send send, 2: required T t }",
            ),
            (
                "unions",
                "union Send { 1: i32 value }
                 union T { 1: Send value }
                 struct Holder { 1: required T value }",
            ),
            (
                "typedefs",
                "typedef i32 Send
                 typedef Send T
                 struct Holder { 1: required T value }",
            ),
            (
                "exceptions",
                "exception Send { 1: required i32 value }
                 exception T { 1: required Send value }
                 service Echo { void echo() throws (1: T error) }",
            ),
            (
                "renamed",
                "struct Request { 1: required i32 value } (pilota.name=\"Send\")
                 struct Response { 1: required Request value } (pilota.name=\"T\")
                 struct Holder { 1: required Response value }",
            ),
            (
                "normalized",
                "struct send { 1: required i32 value }
                 struct t { 1: required send value }",
            ),
            (
                "duplicates",
                "struct Send { 1: required i32 value }
                 struct SEND { 1: required Send value }
                 struct T { 1: required SEND value }
                 struct t { 1: required T value }",
            ),
            (
                "suffixed",
                "struct Send { 1: required i32 value }
                 struct Send_ { 1: required Send value }
                 struct Send__ { 1: required Send_ value }
                 struct T { 1: required Send__ value }
                 struct T_ { 1: required T value }",
            ),
        ] {
            let module = format!("thrift_{kind}_{change_case}");
            let input = root.join(format!("{module}.thrift"));
            fs::write(&input, source).unwrap();
            Builder::thrift()
                .ignore_unused(false)
                .change_case(change_case)
                .compile([input], Output::File(root.join(format!("{module}.rs"))));
            let generated = fs::read_to_string(root.join(format!("{module}.rs"))).unwrap();
            if kind == "structs" {
                assert!(generated.contains("pub struct Send_ {"));
                assert!(generated.contains("pub struct T_ {"));
                assert!(generated.contains("Send(Send_)"));
                assert!(generated.contains("T(T_)"));
            }
            if kind == "suffixed" {
                assert!(generated.contains("pub struct Send___ {"));
                assert!(generated.contains("pub struct T__ {"));
            }
            includes.push_str(&format!("include!(\"{module}.rs\");\n"));
        }

        for (kind, source) in [
            (
                "messages",
                "message Send { int32 value = 1; }
                 message T { Send value = 1; }
                 message Holder {
                     T value = 1;
                     oneof choice { Send send = 2; T t = 3; }
                 }",
            ),
            (
                "enums",
                "enum Send { SEND_ZERO = 0; }
                 enum T { T_ZERO = 0; }
                 message Holder { Send send = 1; T t = 2; }",
            ),
            (
                "nested",
                "message Outer {
                     message Send { int32 value = 1; }
                     message T { Send value = 1; }
                     T value = 1;
                 }",
            ),
        ] {
            let module = format!("pb_{kind}_{change_case}");
            let input = root.join(format!("{module}.proto"));
            fs::write(&input, format!("syntax = \"proto3\";\n{source}")).unwrap();
            Builder::pb()
                .ignore_unused(false)
                .change_case(change_case)
                .include_dirs(vec![root.to_path_buf()])
                .compile([input], Output::File(root.join(format!("{module}.rs"))));
            includes.push_str(&format!("include!(\"{module}.rs\");\n"));
        }
    }

    fs::write(root.join("lib.rs"), includes).unwrap();
    let pilota = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../pilota");
    fs::write(
        root.join("Cargo.toml"),
        format!(
            "[package]\nname = \"type-name-collision\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
             [workspace]\n\
             [lib]\npath = \"lib.rs\"\n\
             [dependencies]\npilota = {{ path = {pilota:?} }}\n"
        ),
    )
    .unwrap();
    // Keep the child build separate from Cargo's lock on the parent test build.
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .current_dir(root)
        .args(["check", "--offline", "--target-dir"])
        .arg(root.join("target"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated code failed to compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
