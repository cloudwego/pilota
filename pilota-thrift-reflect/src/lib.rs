use std::collections::BTreeMap;

use ahash::AHashMap;
use descriptor::thrift_reflection::ConstValueType;
use pilota::{FastStr, OrderedFloat};

include!("descriptor.rs");
pub use descriptor::*;

pub mod error;
pub mod service;

pub enum ThriftType {
    String,
    Byte,
    Bool,
    Binary,
    I8,
    I16,
    I32,
    I64,
    Double,
    Uuid,
    List,
    Set,
    Map,
    Void,
    Path(FastStr),
}

impl std::fmt::Display for ThriftType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThriftType::String => write!(f, "string"),
            ThriftType::Byte => write!(f, "byte"),
            ThriftType::Bool => write!(f, "bool"),
            ThriftType::Binary => write!(f, "binary"),
            ThriftType::I8 => write!(f, "i8"),
            ThriftType::I16 => write!(f, "i16"),
            ThriftType::I32 => write!(f, "i32"),
            ThriftType::I64 => write!(f, "i64"),
            ThriftType::Double => write!(f, "double"),
            ThriftType::Uuid => write!(f, "uuid"),
            ThriftType::List => write!(f, "list"),
            ThriftType::Set => write!(f, "set"),
            ThriftType::Map => write!(f, "map"),
            ThriftType::Void => write!(f, "void"),
            ThriftType::Path(path) => write!(f, "{}", path),
        }
    }
}

impl From<&str> for ThriftType {
    fn from(s: &str) -> Self {
        match s {
            "string" => ThriftType::String,
            "byte" => ThriftType::Byte,
            "bool" => ThriftType::Bool,
            "binary" => ThriftType::Binary,
            "i8" => ThriftType::I8,
            "i16" => ThriftType::I16,
            "i32" => ThriftType::I32,
            "i64" => ThriftType::I64,
            "double" => ThriftType::Double,
            "uuid" => ThriftType::Uuid,
            "list" => ThriftType::List,
            "set" => ThriftType::Set,
            "map" => ThriftType::Map,
            "void" => ThriftType::Void,
            _ => ThriftType::Path(FastStr::new(s)),
        }
    }
}

impl From<&pilota_thrift_parser::File> for thrift_reflection::FileDescriptor {
    fn from(file: &pilota_thrift_parser::File) -> Self {
        let filepath = FastStr::new(file.path.display().to_string());
        let mut services = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut typedefs = Vec::new();
        let mut consts = Vec::new();
        let mut includes = AHashMap::new();
        let mut namespaces = AHashMap::new();
        let mut exceptions = Vec::new();
        let mut unions = Vec::new();

        for item in file.items.iter() {
            match item {
                pilota_thrift_parser::Item::Include(include) => {
                    let include_path = file
                        .path
                        .parent()
                        .unwrap_or_else(|| std::path::Path::new(""))
                        .join(include.path.0.as_str()); // relative path -> absolute path

                    includes.insert(
                        FastStr::new(
                            include
                                .path
                                .0
                                .as_str()
                                .split('/')
                                .next_back()
                                .unwrap()
                                .trim_end_matches(".thrift"),
                        ),
                        FastStr::new(include_path.to_string_lossy()),
                    );
                }
                pilota_thrift_parser::Item::Namespace(namespace) => {
                    namespaces.insert(
                        FastStr::new(namespace.scope.0.as_str()),
                        FastStr::new(
                            namespace
                                .name
                                .segments
                                .iter()
                                .map(|segment| segment.0.as_ref())
                                .collect::<Vec<_>>()
                                .join("."),
                        ),
                    );
                }
                pilota_thrift_parser::Item::Typedef(typedef) => {
                    typedefs.push((filepath.clone(), typedef).into());
                }
                pilota_thrift_parser::Item::Constant(constant) => {
                    consts.push((filepath.clone(), constant).into());
                }
                pilota_thrift_parser::Item::Enum(enum_) => {
                    enums.push((filepath.clone(), enum_).into());
                }
                pilota_thrift_parser::Item::Struct(struct_) => {
                    structs.push((filepath.clone(), struct_).into());
                }
                pilota_thrift_parser::Item::Union(union) => {
                    unions.push((filepath.clone(), union).into());
                }
                pilota_thrift_parser::Item::Exception(exception) => {
                    exceptions.push((filepath.clone(), exception).into());
                }
                pilota_thrift_parser::Item::Service(service) => {
                    services.push((filepath.clone(), service).into());
                }
                _ => {}
            }
        }
        thrift_reflection::FileDescriptor {
            filepath,
            includes,
            namespaces,
            services,
            structs,
            enums,
            typedefs,
            consts,
            exceptions,
            unions,
            ..Default::default()
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::Service)> for thrift_reflection::ServiceDescriptor {
    fn from((filepath, service): (FastStr, &pilota_thrift_parser::Service)) -> Self {
        thrift_reflection::ServiceDescriptor {
            name: FastStr::new(service.name.0.clone()),
            filepath: filepath.clone(),
            methods: service
                .functions
                .iter()
                .map(|function| (filepath.clone(), function).into())
                .collect(),
            annotations: Annotations::from(&service.annotations).0,
            ..Default::default()
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::Function)> for thrift_reflection::MethodDescriptor {
    fn from((filepath, function): (FastStr, &pilota_thrift_parser::Function)) -> Self {
        thrift_reflection::MethodDescriptor {
            name: FastStr::new(function.name.0.clone()),
            filepath: filepath.clone(),
            response: Some((filepath.clone(), &function.result_type).into()),
            args: function
                .arguments
                .iter()
                .map(|arg| (filepath.clone(), arg).into())
                .collect(),
            annotations: Annotations::from(&function.annotations).0,
            throw_exceptions: function
                .throws
                .iter()
                .map(|exception| (filepath.clone(), exception).into())
                .collect(),
            is_oneway: function.oneway,
            ..Default::default()
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::Type)> for thrift_reflection::TypeDescriptor {
    fn from((filepath, r#type): (FastStr, &pilota_thrift_parser::Type)) -> Self {
        match &r#type.0 {
            pilota_thrift_parser::Ty::String => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::String.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::Void => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::Void.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::Byte => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::Byte.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::Bool => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::Bool.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::Binary => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::Binary.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::I8 => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::I8.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::I16 => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::I16.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::I32 => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::I32.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::I64 => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::I64.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::Double => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::Double.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::Uuid => thrift_reflection::TypeDescriptor {
                filepath,
                name: FastStr::new(ThriftType::Uuid.to_string()),
                key_type: None,
                value_type: None,
                extra: None,
            },
            pilota_thrift_parser::Ty::List { value, .. } => thrift_reflection::TypeDescriptor {
                filepath: filepath.clone(),
                name: FastStr::new(ThriftType::List.to_string()),
                key_type: None,
                value_type: Some(Box::new((filepath, value.as_ref()).into())),
                extra: None,
            },
            pilota_thrift_parser::Ty::Set { value, .. } => thrift_reflection::TypeDescriptor {
                filepath: filepath.clone(),
                name: FastStr::new(ThriftType::Set.to_string()),
                key_type: None,
                value_type: Some(Box::new((filepath, value.as_ref()).into())),
                extra: None,
            },
            pilota_thrift_parser::Ty::Map { key, value, .. } => thrift_reflection::TypeDescriptor {
                filepath: filepath.clone(),
                name: FastStr::new(ThriftType::Map.to_string()),
                key_type: Some(Box::new((filepath.clone(), key.as_ref()).into())),
                value_type: Some(Box::new((filepath, value.as_ref()).into())),
                extra: None,
            },
            pilota_thrift_parser::Ty::Path(path) => thrift_reflection::TypeDescriptor {
                filepath: filepath.clone(),
                name: FastStr::new(
                    path.segments
                        .iter()
                        .map(|segment| segment.0.clone())
                        .collect::<Vec<_>>()
                        .join("."),
                ),
                key_type: None,
                value_type: None,
                extra: None,
            },
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::Field)> for thrift_reflection::FieldDescriptor {
    fn from((filepath, field): (FastStr, &pilota_thrift_parser::Field)) -> Self {
        thrift_reflection::FieldDescriptor {
            name: FastStr::new(field.name.0.clone()),
            filepath: filepath.clone(),
            r#type: (filepath.clone(), &field.ty).into(),
            requiredness: match field.attribute {
                pilota_thrift_parser::Attribute::Optional => "optional",
                pilota_thrift_parser::Attribute::Required => "required",
                pilota_thrift_parser::Attribute::Default => "default",
            }
            .into(),
            id: field.id,
            default_value: field.default.as_ref().map(|default| default.into()),
            annotations: Annotations::from(&field.annotations).0,
            ..Default::default()
        }
    }
}

impl From<&pilota_thrift_parser::ConstValue> for thrift_reflection::ConstValueDescriptor {
    fn from(const_value: &pilota_thrift_parser::ConstValue) -> Self {
        match const_value {
            pilota_thrift_parser::ConstValue::Bool(bool) => {
                thrift_reflection::ConstValueDescriptor {
                    r#type: ConstValueType::BOOL,
                    value_bool: *bool,
                    ..Default::default()
                }
            }
            pilota_thrift_parser::ConstValue::Path(path) => {
                thrift_reflection::ConstValueDescriptor {
                    r#type: ConstValueType::IDENTIFIER,
                    value_identifier: FastStr::new(
                        path.segments
                            .iter()
                            .map(|segment| segment.0.as_ref())
                            .collect::<Vec<_>>()
                            .join("."),
                    ),
                    ..Default::default()
                }
            }
            pilota_thrift_parser::ConstValue::String(string) => {
                thrift_reflection::ConstValueDescriptor {
                    r#type: ConstValueType::STRING,
                    value_string: FastStr::new(string.0.clone()),
                    ..Default::default()
                }
            }
            pilota_thrift_parser::ConstValue::Int(int) => thrift_reflection::ConstValueDescriptor {
                r#type: ConstValueType::INT,
                value_int: int.0,
                ..Default::default()
            },
            pilota_thrift_parser::ConstValue::Double(double) => {
                thrift_reflection::ConstValueDescriptor {
                    r#type: ConstValueType::DOUBLE,
                    value_double: OrderedFloat::from(double.0.clone().parse::<f64>().unwrap()),
                    ..Default::default()
                }
            }
            pilota_thrift_parser::ConstValue::List(list) => {
                thrift_reflection::ConstValueDescriptor {
                    r#type: ConstValueType::LIST,
                    value_list: Some(list.iter().map(|item| item.into()).collect()),
                    ..Default::default()
                }
            }
            pilota_thrift_parser::ConstValue::Map(map) => {
                let mut bmap = BTreeMap::new();
                for (key, value) in map {
                    bmap.insert(key.into(), value.into());
                }
                thrift_reflection::ConstValueDescriptor {
                    r#type: ConstValueType::MAP,
                    value_map: Some(bmap),
                    ..Default::default()
                }
            }
        }
    }
}

pub struct Annotations(pub AHashMap<FastStr, Vec<FastStr>>);

impl From<&pilota_thrift_parser::Annotations> for Annotations {
    fn from(annos: &pilota_thrift_parser::Annotations) -> Self {
        let mut map = AHashMap::new();
        for anno in annos.iter() {
            map.insert(
                FastStr::new(anno.key.clone()),
                vec![FastStr::new(anno.value.to_string())],
            );
        }
        Annotations(map)
    }
}

impl From<(FastStr, &pilota_thrift_parser::Struct)> for thrift_reflection::StructDescriptor {
    fn from((filepath, struct_): (FastStr, &pilota_thrift_parser::Struct)) -> Self {
        thrift_reflection::StructDescriptor {
            name: FastStr::new(struct_.name.0.clone()),
            filepath: filepath.clone(),
            fields: struct_
                .fields
                .iter()
                .map(|field| (filepath.clone(), field).into())
                .collect(),
            annotations: Annotations::from(&struct_.annotations).0,
            ..Default::default()
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::Typedef)> for thrift_reflection::TypedefDescriptor {
    fn from((filepath, typedef): (FastStr, &pilota_thrift_parser::Typedef)) -> Self {
        thrift_reflection::TypedefDescriptor {
            filepath: filepath.clone(),
            r#type: (filepath, &typedef.r#type).into(),
            alias: FastStr::new(typedef.alias.0.as_ref()),
            annotations: Annotations::from(&typedef.annotations).0,
            ..Default::default()
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::Constant)> for thrift_reflection::ConstDescriptor {
    fn from((filepath, constant): (FastStr, &pilota_thrift_parser::Constant)) -> Self {
        thrift_reflection::ConstDescriptor {
            filepath: filepath.clone(),
            name: FastStr::new(constant.name.0.clone()),
            r#type: (filepath, &constant.r#type).into(),
            value: (&constant.value).into(),
            annotations: Annotations::from(&constant.annotations).0,
            ..Default::default()
        }
    }
}

impl From<&pilota_thrift_parser::Type> for thrift_reflection::ConstValueType {
    fn from(ty: &pilota_thrift_parser::Type) -> Self {
        match ty.0 {
            pilota_thrift_parser::Ty::String => Self::STRING,
            pilota_thrift_parser::Ty::Byte => Self::INT,
            pilota_thrift_parser::Ty::Binary => Self::STRING,
            pilota_thrift_parser::Ty::Bool => Self::BOOL,
            pilota_thrift_parser::Ty::I8 => Self::INT,
            pilota_thrift_parser::Ty::I16 => Self::INT,
            pilota_thrift_parser::Ty::I32 => Self::INT,
            pilota_thrift_parser::Ty::I64 => Self::INT,
            pilota_thrift_parser::Ty::Double => Self::DOUBLE,
            pilota_thrift_parser::Ty::Uuid => Self::STRING,
            pilota_thrift_parser::Ty::List { .. } => Self::LIST,
            pilota_thrift_parser::Ty::Set { .. } => Self::LIST,
            pilota_thrift_parser::Ty::Map { .. } => Self::MAP,
            pilota_thrift_parser::Ty::Path(_) => Self::IDENTIFIER,
            pilota_thrift_parser::Ty::Void => unreachable!(),
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::Enum)> for thrift_reflection::EnumDescriptor {
    fn from((filepath, enum_): (FastStr, &pilota_thrift_parser::Enum)) -> Self {
        thrift_reflection::EnumDescriptor {
            name: FastStr::new(enum_.name.0.clone()),
            filepath: filepath.clone(),
            values: enum_
                .values
                .iter()
                .map(|value| (filepath.clone(), value).into())
                .collect(),
            annotations: Annotations::from(&enum_.annotations).0,
            ..Default::default()
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::EnumValue)> for thrift_reflection::EnumValueDescriptor {
    fn from((filepath, value): (FastStr, &pilota_thrift_parser::EnumValue)) -> Self {
        thrift_reflection::EnumValueDescriptor {
            filepath: filepath.clone(),
            name: FastStr::new(value.name.0.clone()),
            value: value.value.as_ref().map(|value| value.0).unwrap_or(-1),
            annotations: Annotations::from(&value.annotations).0,
            ..Default::default()
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::Union)> for thrift_reflection::StructDescriptor {
    fn from((filepath, union): (FastStr, &pilota_thrift_parser::Union)) -> Self {
        thrift_reflection::StructDescriptor {
            name: FastStr::new(union.name.0.clone()),
            filepath: filepath.clone(),
            fields: union
                .fields
                .iter()
                .map(|field| (filepath.clone(), field).into())
                .collect(),
            annotations: Annotations::from(&union.annotations).0,
            ..Default::default()
        }
    }
}

impl From<(FastStr, &pilota_thrift_parser::Exception)> for thrift_reflection::StructDescriptor {
    fn from((filepath, exception): (FastStr, &pilota_thrift_parser::Exception)) -> Self {
        thrift_reflection::StructDescriptor {
            filepath: filepath.clone(),
            name: FastStr::new(exception.name.0.clone()),
            fields: exception
                .fields
                .iter()
                .map(|field| (filepath.clone(), field).into())
                .collect(),
            annotations: Annotations::from(&exception.annotations).0,
            ..Default::default()
        }
    }
}
