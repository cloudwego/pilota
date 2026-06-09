use std::{
    hint::black_box,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use criterion::{Criterion, criterion_group, criterion_main};
use pilota::FastStr;
use pilota_thrift_fieldmask::{FieldMask, FieldMaskBuilder};
use pilota_thrift_reflect::{
    service::Register,
    thrift_reflection::{FieldDescriptor, FileDescriptor, StructDescriptor, TypeDescriptor},
};

const WIDE_FIELD_COUNT: i32 = 512;

struct RealIdlFixture {
    field_mask: FieldMask,
    request_type: TypeDescriptor,
    a_type: TypeDescriptor,
    list_a_value_type: TypeDescriptor,
    request_struct: StructDescriptor,
    request_f16: FieldDescriptor,
}

struct WideFixture {
    field_mask: FieldMask,
    wide_type: TypeDescriptor,
    leaf_type: TypeDescriptor,
    wide_struct: StructDescriptor,
    wide_last_field: FieldDescriptor,
}

fn real_idl_fixture() -> &'static RealIdlFixture {
    static FIXTURE: OnceLock<RealIdlFixture> = OnceLock::new();

    FIXTURE.get_or_init(|| {
        let fieldmask_desc = parse_and_register("../examples/idl/fieldmask.thrift");
        parse_and_register("../examples/idl/base.thrift");

        let request_struct = fieldmask_desc
            .find_struct_by_name("Request")
            .expect("Request descriptor")
            .clone();
        let request_type = request_struct.type_descriptor();
        let a_type = request_struct
            .find_field_by_name("f11")
            .expect("f11 descriptor")
            .r#type
            .clone();
        let list_a_value_type = request_struct
            .find_field_by_name("f13")
            .expect("f13 descriptor")
            .r#type
            .value_type
            .as_deref()
            .expect("f13 value type")
            .clone();
        let request_f16 = request_struct
            .find_field_by_name("f16")
            .expect("f16 descriptor")
            .clone();

        let paths = [
            "$.f1",
            "$.f9[1]",
            "$.f11.a",
            "$.f13[0].a",
            "$.f16{\"key1\"}[1].a",
            "$.f17[*]{\"key1\"}",
        ];
        let field_mask = FieldMaskBuilder::new(&request_type, &paths)
            .build()
            .expect("field mask");

        RealIdlFixture {
            field_mask,
            request_type,
            a_type,
            list_a_value_type,
            request_struct,
            request_f16,
        }
    })
}

fn wide_fixture() -> &'static WideFixture {
    static FIXTURE: OnceLock<WideFixture> = OnceLock::new();

    FIXTURE.get_or_init(|| {
        let filepath = FastStr::new("bench/wide.thrift");
        let leaf_type = path_type(&filepath, "Leaf");
        let wide_type = path_type(&filepath, "Wide");

        let leaf_struct = StructDescriptor {
            filepath: filepath.clone(),
            name: "Leaf".into(),
            fields: vec![FieldDescriptor {
                filepath: filepath.clone(),
                name: "leaf".into(),
                r#type: primitive_type(&filepath, "i32"),
                requiredness: "optional".into(),
                id: 1,
                ..Default::default()
            }],
            ..Default::default()
        };

        let mut fields = Vec::with_capacity(WIDE_FIELD_COUNT as usize);
        for id in 1..WIDE_FIELD_COUNT {
            fields.push(FieldDescriptor {
                filepath: filepath.clone(),
                name: FastStr::new(format!("f{id}")),
                r#type: primitive_type(&filepath, "i32"),
                requiredness: "optional".into(),
                id,
                ..Default::default()
            });
        }
        fields.push(FieldDescriptor {
            filepath: filepath.clone(),
            name: FastStr::new(format!("f{WIDE_FIELD_COUNT}")),
            r#type: leaf_type.clone(),
            requiredness: "optional".into(),
            id: WIDE_FIELD_COUNT,
            ..Default::default()
        });

        let wide_struct = StructDescriptor {
            filepath: filepath.clone(),
            name: "Wide".into(),
            fields,
            ..Default::default()
        };
        let wide_last_field = wide_struct
            .find_field_by_name(format!("f{WIDE_FIELD_COUNT}").as_str())
            .expect("wide last field")
            .clone();

        Register::register(
            filepath.clone(),
            FileDescriptor {
                filepath,
                structs: vec![leaf_struct, wide_struct.clone()],
                ..Default::default()
            },
        );

        let paths = [
            format!("$.f{}", WIDE_FIELD_COUNT - 1),
            format!("$.f{WIDE_FIELD_COUNT}.leaf"),
        ];
        let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
        let field_mask = FieldMaskBuilder::new(&wide_type, &path_refs)
            .build()
            .expect("wide field mask");

        WideFixture {
            field_mask,
            wide_type,
            leaf_type,
            wide_struct,
            wide_last_field,
        }
    })
}

fn parse_and_register(relative_path: &str) -> FileDescriptor {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(relative_path);
    let content = std::fs::read_to_string(&path).expect("read thrift IDL");
    let mut ast = pilota_thrift_parser::FileParser::new(
        pilota_thrift_parser::FileSource::new_with_path(PathBuf::from(&path), &content)
            .expect("file source"),
    )
    .parse()
    .expect("parse thrift IDL");
    ast.path = Arc::from(path.canonicalize().expect("canonical IDL path"));

    let desc: FileDescriptor = (&ast).into();
    Register::register(FastStr::new(ast.path.to_string_lossy()), desc.clone());
    desc
}

fn primitive_type(filepath: &FastStr, name: &str) -> TypeDescriptor {
    TypeDescriptor {
        filepath: filepath.clone(),
        name: FastStr::new(name),
        ..Default::default()
    }
}

fn path_type(filepath: &FastStr, name: &str) -> TypeDescriptor {
    TypeDescriptor {
        filepath: filepath.clone(),
        name: FastStr::new(name),
        ..Default::default()
    }
}

fn bench_real_idl(c: &mut Criterion) {
    let fixture = real_idl_fixture();
    let mut group = c.benchmark_group("fieldmask_get_path_real_idl");

    group.bench_function("descriptor/get_struct_desc_request", |b| {
        b.iter(|| black_box(fixture.request_type.get_struct_desc()))
    });
    group.bench_function("descriptor/get_struct_desc_a", |b| {
        b.iter(|| black_box(fixture.a_type.get_struct_desc()))
    });
    group.bench_function("baseline/type_descriptor_clone_request", |b| {
        b.iter(|| black_box(fixture.request_type.clone()))
    });
    group.bench_function("baseline/type_descriptor_clone_list_a_value", |b| {
        b.iter(|| black_box(fixture.list_a_value_type.clone()))
    });
    group.bench_function("baseline/field_descriptor_clone_f16", |b| {
        b.iter(|| black_box(fixture.request_f16.clone()))
    });
    group.bench_function("baseline/find_field_by_name_f16", |b| {
        b.iter(|| black_box(fixture.request_struct.find_field_by_name(black_box("f16"))))
    });

    bench_get_path(&mut group, "get_path/scalar_root_f1", fixture, "$.f1");
    bench_get_path(&mut group, "get_path/list_scalar_f9_1", fixture, "$.f9[1]");
    bench_get_path(
        &mut group,
        "get_path/nested_struct_f11_a",
        fixture,
        "$.f11.a",
    );
    bench_get_path(
        &mut group,
        "get_path/list_struct_f13_0_a",
        fixture,
        "$.f13[0].a",
    );
    bench_get_path(
        &mut group,
        "get_path/map_list_struct_f16_key_1_a",
        fixture,
        "$.f16{\"key1\"}[1].a",
    );
    bench_get_path(
        &mut group,
        "get_path/list_map_scalar_f17_any_key1",
        fixture,
        "$.f17[*]{\"key1\"}",
    );

    group.finish();
}

fn bench_get_path(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    name: &'static str,
    fixture: &'static RealIdlFixture,
    path: &'static str,
) {
    group.bench_function(name, |b| {
        b.iter(|| {
            black_box(
                fixture
                    .field_mask
                    .get_path(black_box(&fixture.request_type), black_box(path))
                    .expect("get_path"),
            )
        })
    });
}

fn bench_wide_descriptor(c: &mut Criterion) {
    let fixture = wide_fixture();
    let last_scalar_path = format!("$.f{}", WIDE_FIELD_COUNT - 1);
    let last_nested_path = format!("$.f{WIDE_FIELD_COUNT}.leaf");
    let last_field_name = format!("f{WIDE_FIELD_COUNT}");
    let mut group = c.benchmark_group("fieldmask_get_path_wide_descriptor");

    group.bench_function("descriptor/get_struct_desc_wide_512_fields", |b| {
        b.iter(|| black_box(fixture.wide_type.get_struct_desc()))
    });
    group.bench_function("descriptor/get_struct_desc_leaf_1_field", |b| {
        b.iter(|| black_box(fixture.leaf_type.get_struct_desc()))
    });
    group.bench_function("baseline/type_descriptor_clone_wide", |b| {
        b.iter(|| black_box(fixture.wide_type.clone()))
    });
    group.bench_function("baseline/field_descriptor_clone_last", |b| {
        b.iter(|| black_box(fixture.wide_last_field.clone()))
    });
    group.bench_function("baseline/find_field_by_name_last", |b| {
        b.iter(|| {
            black_box(
                fixture
                    .wide_struct
                    .find_field_by_name(black_box(last_field_name.as_str())),
            )
        })
    });
    group.bench_function("get_path/last_scalar_field", |b| {
        b.iter(|| {
            black_box(
                fixture
                    .field_mask
                    .get_path(
                        black_box(&fixture.wide_type),
                        black_box(last_scalar_path.as_str()),
                    )
                    .expect("get_path wide scalar"),
            )
        })
    });
    group.bench_function("get_path/last_nested_field", |b| {
        b.iter(|| {
            black_box(
                fixture
                    .field_mask
                    .get_path(
                        black_box(&fixture.wide_type),
                        black_box(last_nested_path.as_str()),
                    )
                    .expect("get_path wide nested"),
            )
        })
    });

    group.finish();
}

fn configure() -> Criterion {
    Criterion::default()
}

criterion_group! {
    name = benches;
    config = configure();
    targets = bench_real_idl, bench_wide_descriptor
}
criterion_main!(benches);
