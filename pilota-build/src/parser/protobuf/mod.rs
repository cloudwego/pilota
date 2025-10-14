use std::{collections::HashMap, path::PathBuf, sync::Arc};

use ahash::AHashMap;
use faststr::FastStr;
use itertools::Itertools;
use normpath::PathExt;
use pilota::Bytes;
use protobuf::{
    Message as _,
    descriptor::{
        DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
        MethodDescriptorProto, ServiceDescriptorProto,
        field_descriptor_proto::{Label, Type},
    },
};
use rustc_hash::{FxHashMap, FxHashSet};

use super::Parser;
use crate::{
    IdentName,
    index::Idx,
    ir::{
        self, FieldKind, Item, Path, TyKind,
        ext::{
            self,
            pb::{ExtendeeIndex, ExtendeeKind, FieldType},
        },
    },
    symbol::{EnumRepr, FileId, Ident},
    tags::{
        PilotaName, RustType, RustWrapperArc, SerdeAttribute, Tags,
        protobuf::{
            ClientStreaming, Deprecated, OneOf, OptionalRepeated, ProstType, Repeated,
            ServerStreaming,
        },
    },
};

#[derive(Default)]
pub struct ProtobufParser {
    inner: protobuf_parse::Parser,
    include_dirs: Vec<PathBuf>,
    input_files: FxHashSet<PathBuf>,
}

#[derive(PartialEq, Eq)]
pub enum Syntax {
    Proto2,
    Proto3,
}

struct Lower {
    next_file_id: FileId,
    files: FxHashMap<String, FileId>,
    cur_package: Option<String>,
    cur_syntax: Syntax,
}

impl Default for Lower {
    fn default() -> Self {
        Self {
            next_file_id: FileId::from_u32(0),
            files: Default::default(),
            cur_package: None,
            cur_syntax: Syntax::Proto3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum WellKnownFileName {
    Descriptor,
    Any,
    Api,
    Duration,
    Empty,
    FieldMask,
    SourceContext,
    Struct,
    Timestamp,
    Type,
    Wrappers,
    NotWellKnown,
}

pub const DESCRIPTOR_PROTO_NAME: &str = "google/protobuf/descriptor.proto";
pub const ANY_PROTO_NAME: &str = "google/protobuf/any.proto";
pub const API_PROTO_NAME: &str = "google/protobuf/api.proto";
pub const DURATION_PROTO_NAME: &str = "google/protobuf/duration.proto";
pub const EMPTY_PROTO_NAME: &str = "google/protobuf/empty.proto";
pub const FIELD_MASK_PROTO_NAME: &str = "google/protobuf/field_mask.proto";
pub const SOURCE_CONTEXT_PROTO_NAME: &str = "google/protobuf/source_context.proto";
pub const STRUCT_PROTO_NAME: &str = "google/protobuf/struct.proto";
pub const TIMESTAMP_PROTO_NAME: &str = "google/protobuf/timestamp.proto";
pub const TYPE_PROTO_NAME: &str = "google/protobuf/type.proto";
pub const WRAPPERS_PROTO_NAME: &str = "google/protobuf/wrappers.proto";

impl WellKnownFileName {
    pub fn name(&self) -> &str {
        match self {
            WellKnownFileName::Descriptor => DESCRIPTOR_PROTO_NAME,
            WellKnownFileName::Any => ANY_PROTO_NAME,
            WellKnownFileName::Api => API_PROTO_NAME,
            WellKnownFileName::Duration => DURATION_PROTO_NAME,
            WellKnownFileName::Empty => EMPTY_PROTO_NAME,
            WellKnownFileName::FieldMask => FIELD_MASK_PROTO_NAME,
            WellKnownFileName::SourceContext => SOURCE_CONTEXT_PROTO_NAME,
            WellKnownFileName::Struct => STRUCT_PROTO_NAME,
            WellKnownFileName::Timestamp => TIMESTAMP_PROTO_NAME,
            WellKnownFileName::Type => TYPE_PROTO_NAME,
            WellKnownFileName::Wrappers => WRAPPERS_PROTO_NAME,
            WellKnownFileName::NotWellKnown => "",
        }
    }

    pub fn mod_name(&self) -> &'static str {
        match self {
            WellKnownFileName::Descriptor => "::pilota::pb::descriptor",
            WellKnownFileName::Any => "::pilota::pb::well_known_types::any",
            WellKnownFileName::Api => "::pilota::pb::well_known_types::api",
            WellKnownFileName::Duration => "::pilota::pb::well_known_types::duration",
            WellKnownFileName::Empty => "::pilota::pb::well_known_types::empty",
            WellKnownFileName::FieldMask => "::pilota::pb::well_known_types::field_mask",
            WellKnownFileName::SourceContext => "::pilota::pb::well_known_types::source_context",
            WellKnownFileName::Struct => "::pilota::pb::well_known_types::struct_",
            WellKnownFileName::Timestamp => "::pilota::pb::well_known_types::timestamp",
            WellKnownFileName::Type => "::pilota::pb::well_known_types::type_",
            WellKnownFileName::Wrappers => "::pilota::pb::well_known_types::wrappers",
            WellKnownFileName::NotWellKnown => "",
        }
    }
}

impl From<&str> for WellKnownFileName {
    fn from(s: &str) -> Self {
        match s {
            DESCRIPTOR_PROTO_NAME => WellKnownFileName::Descriptor,
            ANY_PROTO_NAME => WellKnownFileName::Any,
            API_PROTO_NAME => WellKnownFileName::Api,
            DURATION_PROTO_NAME => WellKnownFileName::Duration,
            EMPTY_PROTO_NAME => WellKnownFileName::Empty,
            FIELD_MASK_PROTO_NAME => WellKnownFileName::FieldMask,
            SOURCE_CONTEXT_PROTO_NAME => WellKnownFileName::SourceContext,
            STRUCT_PROTO_NAME => WellKnownFileName::Struct,
            TIMESTAMP_PROTO_NAME => WellKnownFileName::Timestamp,
            TYPE_PROTO_NAME => WellKnownFileName::Type,
            WRAPPERS_PROTO_NAME => WellKnownFileName::Wrappers,
            _ => WellKnownFileName::NotWellKnown,
        }
    }
}

impl Lower {
    fn lower_extendee(&self, s: &str) -> Option<ExtendeeKind> {
        match s {
            ".google.protobuf.FileOptions" => Some(ExtendeeKind::File),
            ".google.protobuf.MessageOptions" => Some(ExtendeeKind::Message),
            ".google.protobuf.FieldOptions" => Some(ExtendeeKind::Field),
            ".google.protobuf.EnumOptions" => Some(ExtendeeKind::Enum),
            ".google.protobuf.EnumValueOptions" => Some(ExtendeeKind::EnumValue),
            ".google.protobuf.ServiceOptions" => Some(ExtendeeKind::Service),
            ".google.protobuf.MethodOptions" => Some(ExtendeeKind::Method),
            ".google.protobuf.OneofOptions" => Some(ExtendeeKind::Oneof),
            _ => None,
        }
    }

    fn lower_pb_field_type(
        &self,
        ty: Option<protobuf::EnumOrUnknown<protobuf::descriptor::field_descriptor_proto::Type>>,
    ) -> Option<ext::pb::FieldType> {
        use protobuf::descriptor::field_descriptor_proto::Type as T;
        let ty = ty?;
        Some(match ty.enum_value().unwrap() {
            T::TYPE_BOOL => FieldType::Bool,
            T::TYPE_INT32 => FieldType::Int32,
            T::TYPE_INT64 => FieldType::Int64,
            T::TYPE_UINT32 => FieldType::UInt32,
            T::TYPE_UINT64 => FieldType::UInt64,
            T::TYPE_FLOAT => FieldType::Float,
            T::TYPE_DOUBLE => FieldType::Double,
            T::TYPE_STRING => FieldType::String,
            T::TYPE_BYTES => FieldType::Bytes,
            T::TYPE_MESSAGE => FieldType::Message,
            T::TYPE_ENUM => FieldType::Enum,
            _ => return None,
        })
    }

    fn lower_extension(
        &mut self,
        f: &protobuf::descriptor::FieldDescriptorProto,
        nested_messages: &AHashMap<FastStr, &DescriptorProto>,
    ) -> Option<Arc<ext::pb::Extendee>> {
        let extendee_str = f.extendee();
        if extendee_str.is_empty() {
            return None;
        }
        let extendee = self.lower_extendee(extendee_str)?;
        let field_ty = self.lower_pb_field_type(f.type_)?;
        let item_ty = self.lower_ty(f.type_, f.type_name.as_deref(), nested_messages, false);
        let extendee = Arc::new(ext::pb::Extendee {
            name: FastStr::new(f.name()).into(),
            index: ExtendeeIndex {
                extendee_kind: extendee,
                tag_id: f.number() as u32,
            },
            extendee_ty: ext::pb::ExtendeeType { field_ty, item_ty },
        });
        Some(extendee)
    }

    fn str2path(&self, s: &str) -> ir::Path {
        if s.is_empty() {
            return ir::Path::default();
        }
        ir::Path {
            segments: Arc::from_iter(s.split('.').map(FastStr::new).map(Ident::from)),
        }
    }

    fn lower_ty(
        &self,
        type_: Option<protobuf::EnumOrUnknown<protobuf::descriptor::field_descriptor_proto::Type>>,
        type_name: Option<&str>,
        nested_messages: &AHashMap<FastStr, &DescriptorProto>,
        is_wrapper_arc: bool,
    ) -> ir::Ty {
        let mut tags = Tags::default();
        if is_wrapper_arc {
            tags.insert(RustWrapperArc(true));
        }

        if let Some(name) = type_name {
            if let Some(msg) = nested_messages.get(name) {
                if msg.options.has_map_entry() {
                    let key = &msg.field[0];
                    let value = &msg.field[1];
                    assert_eq!("key", key.name());
                    assert_eq!("value", value.name());
                    return ir::Ty {
                        kind: ir::TyKind::Map(
                            Arc::from(self.lower_ty(
                                key.type_,
                                key.type_name.as_deref(),
                                nested_messages,
                                false,
                            )),
                            Arc::from(self.lower_ty(
                                value.type_,
                                value.type_name.as_deref(),
                                nested_messages,
                                false,
                            )),
                        ),
                        tags: Arc::new(tags),
                    };
                }
            }

            assert_eq!(".", &name[..1]);

            return ir::Ty {
                kind: ir::TyKind::Path(self.str2path(&name[1..])),
                tags: Arc::new(tags),
            };
        }
        let Some(ty) = type_ else { panic!() };

        let kind = match ty.enum_value().unwrap() {
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_DOUBLE => ir::TyKind::F64,
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FLOAT => ir::TyKind::F32,
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT64 => ir::TyKind::I64,
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT64 => ir::TyKind::UInt64,
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT32 => ir::TyKind::I32,
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED64 => {
                tags.insert(ProstType::Fixed64);
                ir::TyKind::UInt64
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED32 => {
                tags.insert(ProstType::Fixed32);
                ir::TyKind::UInt32
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL => ir::TyKind::Bool,
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING => ir::TyKind::String,
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_GROUP => todo!(),
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_BYTES => ir::TyKind::Bytes,
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT32 => ir::TyKind::UInt32,
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED32 => {
                tags.insert(ProstType::SFixed32);
                ir::TyKind::I32
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED64 => {
                tags.insert(ProstType::SFixed64);
                ir::TyKind::I64
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT32 => {
                tags.insert(ProstType::SInt32);
                ir::TyKind::I32
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT64 => {
                tags.insert(ProstType::SInt64);
                ir::TyKind::I64
            }

            protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE
            | protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM => unreachable!(),
        };

        ir::Ty {
            kind,
            tags: Arc::new(tags),
        }
    }

    fn lower_enum(&self, e: &EnumDescriptorProto) -> ir::Item {
        ir::Item {
            related_items: Default::default(),
            tags: Arc::new(self.extract_enum_tags(e)),
            kind: ir::ItemKind::Enum(ir::Enum {
                name: FastStr::new(e.name()).into(),
                variants: e
                    .value
                    .iter()
                    .map(|v| ir::EnumVariant {
                        id: v.number,
                        name: FastStr::new(v.name()).into(),
                        discr: v.number.map(|v| v as i64),
                        tags: Arc::new(self.extract_enum_value_tags(v)),
                        fields: Default::default(),
                        item_exts: ext::ItemExts::Pb(ext::pb::ItemExts {
                            used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                                ExtendeeKind::EnumValue,
                                v.options.special_fields.unknown_fields(),
                            ),
                        }),
                    })
                    .collect_vec(),
                repr: Some(EnumRepr::I32),
                item_exts: ext::ItemExts::Pb(ext::pb::ItemExts {
                    used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                        ExtendeeKind::Enum,
                        e.options.special_fields.unknown_fields(),
                    ),
                }),
            }),
        }
    }

    fn lower_message(
        &mut self,
        message: &DescriptorProto,
        parent_messages: &mut Vec<String>,
    ) -> Vec<ir::Item> {
        let fq_message_name = format!(
            "{}{}.{}{}",
            if self.cur_package.is_none() { "" } else { "." },
            self.cur_package.as_deref().unwrap_or(""),
            {
                let mut s = String::new();
                parent_messages.iter().for_each(|m| {
                    s.push_str(m);
                    s.push('.')
                });
                s
            },
            message.name()
        );

        let nested_messages = message
            .nested_type
            .iter()
            .map(|m| (format!("{}.{}", fq_message_name, m.name()).into(), m))
            .collect::<AHashMap<FastStr, _>>();

        let mut fields = Vec::default();
        let mut oneof_fields = FxHashMap::default();

        message.field.iter().enumerate().for_each(|(idx, field)| {
            if field.proto3_optional.unwrap_or(false) {
                fields.push((idx, field))
            } else if let Some(oneof_index) = field.oneof_index {
                oneof_fields
                    .entry(oneof_index)
                    .or_insert_with(Vec::default)
                    .push((idx, field))
            } else {
                fields.push((idx, field))
            }
        });

        let mut nested_items: Vec<_> = Default::default();

        let mut extra_fields = Vec::default();

        message.oneof_decl.iter().enumerate().for_each(|(idx, d)| {
            if let Some(fields) = oneof_fields.remove(&(idx as i32)) {
                nested_items.push(Arc::new(ir::Item {
                    related_items: Default::default(),
                    tags: Arc::new(crate::tags!(OneOf)),
                    kind: ir::ItemKind::Enum(ir::Enum {
                        name: FastStr::new(d.name()).into(),
                        repr: None,
                        variants: fields
                            .iter()
                            .map(|(_, f)| ir::EnumVariant {
                                discr: None,
                                id: f.number,
                                name: FastStr::new(f.name()).into(),
                                fields: vec![self.lower_ty(
                                    f.type_,
                                    f.type_name.as_deref(),
                                    &nested_messages,
                                    false,
                                )],
                                tags: Default::default(),
                                item_exts: ext::ItemExts::Pb(ext::pb::ItemExts {
                                    used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                                        ExtendeeKind::Field,
                                        f.options.special_fields.unknown_fields(),
                                    ),
                                }),
                            })
                            .collect_vec(),
                        item_exts: ext::ItemExts::Pb(ext::pb::ItemExts {
                            used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                                ExtendeeKind::Oneof,
                                d.options.special_fields.unknown_fields(),
                            ),
                        }),
                    }),
                }));

                extra_fields.push((
                    fields[0].0,
                    ir::Field {
                        name: FastStr::new(d.name()).into(),
                        id: -1,
                        ty: ir::Ty {
                            kind: ir::TyKind::Path(Path {
                                segments: Arc::from([
                                    FastStr::new(message.name()).into(),
                                    FastStr::new(d.name()).into(),
                                ]),
                            }),
                            tags: Default::default(),
                        },
                        tags: Arc::new(crate::tags!(OneOf)),
                        kind: ir::FieldKind::Optional,
                        default: None,
                        item_exts: ext::ItemExts::Pb(ext::pb::ItemExts {
                            used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                                ExtendeeKind::Field,
                                d.options.special_fields.unknown_fields(),
                            ),
                        }),
                    },
                ));
            }
        });

        parent_messages.push(message.name().into());

        nested_messages
            .iter()
            .filter(|(_, m)| !m.options.has_map_entry())
            .for_each(|(_, m)| {
                self.lower_message(m, parent_messages)
                    .into_iter()
                    .for_each(|item| nested_items.push(Arc::new(item)))
            });

        parent_messages.pop();

        message
            .enum_type
            .iter()
            .for_each(|e| nested_items.push(Arc::new(self.lower_enum(e))));

        let item = ir::Item {
            related_items: Default::default(),
            tags: Arc::new(self.extract_message_tags(message)),
            kind: ir::ItemKind::Message(ir::Message {
                fields: fields
                    .iter()
                    .map(|(idx, f)| {
                        let mut ty =
                            self.lower_ty(f.type_, f.type_name.as_deref(), &nested_messages, false);

                        let is_map = matches!(ty.kind, TyKind::Map(_, _));
                        let repeated = !is_map && matches!(f.label(), Label::LABEL_REPEATED);

                        if repeated {
                            ty = ir::Ty {
                                kind: ir::TyKind::Vec(Arc::from(ty)),
                                tags: Default::default(),
                            }
                        }

                        let optional = (|| {
                            if is_map {
                                return false;
                            }

                            match self.cur_syntax {
                                Syntax::Proto3 => {
                                    f.proto3_optional()
                                        || (!repeated && matches!(f.type_(), Type::TYPE_MESSAGE))
                                }
                                Syntax::Proto2 => f.label() == Label::LABEL_OPTIONAL,
                            }
                        })();

                        let mut tags = self.extract_field_tags(f);
                        if repeated {
                            tags.insert(Repeated);
                        }

                        (
                            *idx,
                            ir::Field {
                                default: None,
                                id: f.number(),
                                name: FastStr::new(f.name()).into(),
                                ty,
                                tags: Arc::new(tags),
                                kind: if optional {
                                    FieldKind::Optional
                                } else {
                                    FieldKind::Required
                                },
                                item_exts: ext::ItemExts::Pb(ext::pb::ItemExts {
                                    used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                                        ExtendeeKind::Field,
                                        f.options.special_fields.unknown_fields(),
                                    ),
                                }),
                            },
                        )
                    })
                    .chain(extra_fields)
                    .sorted_unstable_by_key(|(idx, _)| *idx)
                    .map(|(_, f)| f)
                    .collect(),
                name: FastStr::new(message.name()).into(),
                is_wrapper: false,
                item_exts: ext::ItemExts::Pb(ext::pb::ItemExts {
                    used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                        ExtendeeKind::Message,
                        message.options.special_fields.unknown_fields(),
                    ),
                }),
            }),
        };

        // nested extendees
        let extendees = message
            .extension
            .iter()
            .filter_map(|e| self.lower_extension(e, &nested_messages))
            .collect::<Vec<_>>();

        if nested_items.is_empty() && extendees.is_empty() {
            vec![item]
        } else {
            let name = item.name();
            let mut tags = Tags::default();
            tags.insert(PilotaName(name.0.mod_ident()));
            if !extendees.is_empty() {
                nested_items.push(Arc::new(ir::Item {
                    related_items: Default::default(),
                    tags: Arc::new(Tags::default()),
                    kind: ir::ItemKind::Const(ir::Const {
                        name: FastStr::new(format!("__PILOTA_PB_EXT_{}", name)).into(),
                        ty: ir::Ty {
                            kind: ir::TyKind::String,
                            tags: Default::default(),
                        },
                        lit: ir::Literal::String(Arc::from("extensions")),
                    }),
                }));
            }
            vec![
                item,
                Item {
                    related_items: Default::default(),
                    tags: Arc::from(tags),
                    kind: ir::ItemKind::Mod(ir::Mod {
                        name: Ident { sym: name },
                        items: nested_items,
                        extensions: ext::ModExts::Pb(ext::pb::ModExts {
                            extendees: ext::pb::Extendees(extendees),
                        }),
                    }),
                },
            ]
        }
    }

    pub fn lower_service(&self, service: &ServiceDescriptorProto) -> ir::Item {
        let service_tags = self.extract_service_tags(service);
        let rust_wrapper_arc_all = service_tags.get::<RustWrapperArc>().is_some_and(|v| v.0);

        ir::Item {
            tags: Arc::new(service_tags),
            related_items: Default::default(),
            kind: ir::ItemKind::Service(ir::Service {
                name: FastStr::new(service.name()).into(),
                methods: service
                    .method
                    .iter()
                    .map(|m| {
                        let mut tags = self.extract_method_tags(m);
                        if m.client_streaming() {
                            tags.insert(ClientStreaming);
                        }
                        if m.server_streaming() {
                            tags.insert(ServerStreaming);
                        }

                        let mut arg_tags = Tags::default();
                        if rust_wrapper_arc_all {
                            arg_tags.insert(RustWrapperArc(true));
                        }

                        ir::Method {
                            name: FastStr::new(m.name()).into(),
                            tags: Arc::new(tags),
                            args: vec![ir::Arg {
                                name: "req".into(),
                                id: -1,
                                ty: self.lower_ty(
                                    None,
                                    m.input_type.as_deref(),
                                    &Default::default(),
                                    rust_wrapper_arc_all,
                                ),
                                tags: Arc::new(arg_tags),
                                attribute: FieldKind::Required,
                            }],
                            oneway: false,
                            ret: self.lower_ty(
                                None,
                                m.output_type.as_deref(),
                                &Default::default(),
                                rust_wrapper_arc_all,
                            ),
                            exceptions: None,
                            item_exts: ext::ItemExts::Pb(ext::pb::ItemExts {
                                used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                                    ExtendeeKind::Method,
                                    m.options.special_fields.unknown_fields(),
                                ),
                            }),
                        }
                    })
                    .collect_vec(),
                extend: vec![],
                item_exts: ext::ItemExts::Pb(ext::pb::ItemExts {
                    used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                        ExtendeeKind::Service,
                        service.options.special_fields.unknown_fields(),
                    ),
                }),
            }),
        }
    }

    pub fn lower(
        &mut self,
        files: &[protobuf::descriptor::FileDescriptorProto],
    ) -> Vec<Arc<ir::File>> {
        let mut file_map = HashMap::with_capacity(files.len());
        files.iter().for_each(|f| {
            self.files
                .insert(f.name().to_string(), self.next_file_id.inc_one());
            file_map.insert(f.name(), f);
        });

        files
            .iter()
            .map(|f| {
                self.cur_package.clone_from(&f.package);
                self.cur_syntax = match f.syntax() {
                    "proto3" => Syntax::Proto3,
                    _ => Syntax::Proto2,
                };

                let file_id = *self.files.get(f.name()).unwrap();

                let package = self.str2path(f.package());

                let messages = f
                    .message_type
                    .iter()
                    .flat_map(|m| self.lower_message(m, &mut Vec::new()))
                    .collect_vec()
                    .into_iter();
                let enums = f
                    .enum_type
                    .iter()
                    .map(|e| self.lower_enum(e))
                    .collect_vec()
                    .into_iter();
                let services = f
                    .service
                    .iter()
                    .map(|s| self.lower_service(s))
                    .collect_vec()
                    .into_iter();

                let descriptor_bytes = {
                    let bytes_vec = f
                        .write_to_bytes()
                        .expect("serialize FileDescriptorProto failed");
                    Bytes::from(bytes_vec)
                };

                let mut f = ir::File {
                    package,
                    uses: f
                        .dependency
                        .iter()
                        .map(|d| {
                            (
                                self.str2path(file_map.get(&**d).unwrap().package()),
                                *self.files.get(d).unwrap(),
                            )
                        })
                        .collect(),
                    id: file_id,
                    items: messages
                        .chain(enums)
                        .chain(services)
                        .map(Arc::from)
                        .collect::<Vec<_>>(),
                    descriptor: descriptor_bytes,
                    extensions: ext::FileExts::Pb(ext::pb::FileExts {
                        well_known_file_name: WellKnownFileName::from(f.name()),
                        extendees: ext::pb::Extendees(
                            f.extension
                                .iter()
                                .filter_map(|e| self.lower_extension(e, &Default::default()))
                                .collect::<Vec<_>>(),
                        ),
                        used_options: ext::pb::UsedOptions::from_pb_unknown_fields(
                            ExtendeeKind::File,
                            f.options.special_fields.unknown_fields(),
                        ),
                    }),
                };

                if f.items.is_empty() && f.extensions.has_extendees() {
                    f.items.push(Arc::new(ir::Item {
                        related_items: Default::default(),
                        tags: Arc::new(Tags::default()),
                        kind: ir::ItemKind::Const(ir::Const {
                            name: FastStr::new(format!("__PILOTA_PB_EXT_{}", file_id.as_u32()))
                                .into(),
                            ty: ir::Ty {
                                kind: ir::TyKind::String,
                                tags: Default::default(),
                            },
                            lit: ir::Literal::String(Arc::from("extensions")),
                        }),
                    }));
                }

                let f = Arc::from(f);

                self.cur_package = None;

                f
            })
            .collect::<Vec<_>>()
    }

    fn extract_service_tags(&self, service: &ServiceDescriptorProto) -> Tags {
        let mut tags = Tags::default();
        if service.options.is_some() {
            let options = &service.options;

            // defined in google.protobuf.ServiceOptions
            if options.deprecated() {
                tags.insert(Deprecated(true));
            }

            // defined in pilota.proto
            if options.rust_wrapper_arc() {
                tags.insert(RustWrapperArc(true));
            }
        }
        tags
    }

    fn extract_message_tags(&self, message: &DescriptorProto) -> Tags {
        let mut tags = Tags::default();
        if message.options.is_some() {
            let options = &message.options;

            // defined in google.protobuf.MessageOptions
            if options.deprecated() {
                tags.insert(Deprecated(true));
            }

            // defined in pilota.proto
            if let Some(serde_attr) = options.serde_attribute() {
                tags.insert(SerdeAttribute(serde_attr));
            }
            if let Some(name) = options.name() {
                tags.insert(PilotaName(name));
            }
        }
        tags
    }

    fn extract_enum_tags(&self, field: &EnumDescriptorProto) -> Tags {
        let mut tags = Tags::default();
        if field.options.is_some() {
            let options = &field.options;

            // defined in google.protobuf.EnumOptions
            if options.deprecated() {
                tags.insert(Deprecated(true));
            }

            // defined in pilota.proto
            if let Some(serde_attr) = options.serde_attribute() {
                tags.insert(SerdeAttribute(serde_attr));
            }
            if let Some(name) = options.name() {
                tags.insert(PilotaName(name));
            }
        }
        tags
    }

    fn extract_enum_value_tags(&self, field: &EnumValueDescriptorProto) -> Tags {
        let mut tags = Tags::default();
        if field.options.is_some() {
            let options = &field.options;

            // defined in google.protobuf.EnumValueOptions
            if options.deprecated() {
                tags.insert(Deprecated(true));
            }

            // defined in pilota.proto
            if let Some(serde_attr) = options.serde_attribute() {
                tags.insert(SerdeAttribute(serde_attr));
            }
        }
        tags
    }

    fn extract_method_tags(&self, field: &MethodDescriptorProto) -> Tags {
        let mut tags = Tags::default();
        if field.options.is_some() {
            let options = &field.options;

            // defined in google.protobuf.MethodOptions
            if options.deprecated() {
                tags.insert(Deprecated(true));
            }
        }
        tags
    }

    fn extract_field_tags(&self, field: &FieldDescriptorProto) -> Tags {
        let mut tags = Tags::default();
        if field.options.is_some() {
            let options = &field.options;

            // defined in google.protobuf.FieldOptions
            if options.deprecated() {
                tags.insert(Deprecated(true));
            }

            // defined in pilota.proto
            if options.rust_wrapper_arc() {
                tags.insert(RustWrapperArc(true));
            }
            if let Some(serde_attr) = options.serde_attribute() {
                tags.insert(SerdeAttribute(serde_attr));
            }
            if let Some(name) = options.name() {
                tags.insert(PilotaName(name));
            }
            if let Some(rust_type) = options.rust_type() {
                tags.insert(RustType(rust_type));
            }
            if options.optional_repeated() {
                tags.insert(OptionalRepeated(true));
            }
        }
        tags
    }
}

impl Parser for ProtobufParser {
    fn input<P: AsRef<std::path::Path>>(&mut self, path: P) {
        let p = path.as_ref();
        self.input_files.insert(
            p.normalize()
                .unwrap_or_else(|_| panic!("normalize path failed: {}", p.display()))
                .into_path_buf(),
        );
        self.inner.input(path);
    }

    fn include_dirs(&mut self, dirs: Vec<std::path::PathBuf>) {
        self.include_dirs.extend(dirs.clone());
        self.inner.includes(dirs);
    }

    fn parse(self) -> super::ParseResult {
        let descriptors = self.inner.parse_and_typecheck().unwrap().file_descriptors;

        let mut input_file_ids = vec![];

        let mut lower = Lower::default();

        let files = lower.lower(&descriptors);

        let mut file_ids = FxHashMap::default();
        let mut file_paths = FxHashMap::default();
        let mut file_names = FxHashMap::default();
        descriptors.iter().for_each(|f| {
            self.include_dirs.iter().for_each(|p| {
                let path = p.join(f.name());
                if path.exists() {
                    println!("cargo:rerun-if-changed={}", path.display());
                    let file_id = *lower.files.get(f.name()).unwrap();
                    let file_path: Arc<PathBuf> =
                        Arc::from(path.normalize().unwrap().into_path_buf());
                    file_ids.insert(file_path.clone(), file_id);
                    file_names.insert(
                        file_id,
                        FastStr::new(file_path.file_stem().unwrap().to_string_lossy()),
                    );
                    file_paths.insert(file_id, file_path);

                    if self
                        .input_files
                        .contains(path.normalize().unwrap().as_path())
                    {
                        input_file_ids.push(*lower.files.get(f.name()).unwrap());
                    }
                }
            });
        });

        super::ParseResult {
            files,
            input_files: input_file_ids,
            file_ids_map: file_ids,
            file_paths,
            file_names,
        }
    }
}

// define option value extractor
pub trait PbOptionsValueExtractor<T> {
    fn extract(&self, value: protobuf::UnknownValueRef) -> T;
}
pub struct PbOptionsValueExtractorImpl {
    id: u32,
}

impl PbOptionsValueExtractor<bool> for PbOptionsValueExtractorImpl {
    fn extract(&self, value: protobuf::UnknownValueRef) -> bool {
        match value {
            protobuf::UnknownValueRef::Varint(v) => v != 0,
            _ => panic!("invalid value for option: {}", self.id),
        }
    }
}

impl PbOptionsValueExtractor<FastStr> for PbOptionsValueExtractorImpl {
    fn extract(&self, value: protobuf::UnknownValueRef) -> FastStr {
        match value {
            protobuf::UnknownValueRef::LengthDelimited(v) => match std::str::from_utf8(v) {
                Ok(s) => FastStr::new(s),
                Err(_) => panic!("invalid value for option: {}", self.id),
            },
            _ => panic!("invalid value for option: {}", self.id),
        }
    }
}

// define option constants
macro_rules! define_pb_option {
    // with default value
    ($name:ident, $id:expr, $default:expr) => {
        paste::paste! {
            pub const [<$name:upper _ID>]: u32 = $id;
            pub const [<$name:upper _DEFAULT>]: bool = $default;
        }
    };
    // without default value
    ($name:ident, $id:expr) => {
        paste::paste! {
            pub const [<$name:upper _ID>]: u32 = $id;
        }
    };
}

// define all options traits and implementations
macro_rules! define_all_options_traits {
    (
        $(
            $trait_name:ident for $options_type:ty {
                $($method_defs:tt)*
            }
        )*
    ) => {
        $(
            define_all_options_traits!(@process_trait $trait_name, $options_type, $($method_defs)*);
        )*
    };

    // Process trait and impl generation
    (@process_trait $trait_name:ident, $options_type:ty, $($method_defs:tt)*) => {
        // define trait
        pub trait $trait_name {
            define_all_options_traits!(@collect_trait_methods $($method_defs)*);
        }

        // define implementation
        impl $trait_name for $options_type {
            define_all_options_traits!(@collect_impl_methods $($method_defs)*);
        }
    };

    // Collect trait methods
    (@collect_trait_methods) => {};
    (@collect_trait_methods ($method:ident, $field_id:expr, $default:expr) -> $ret_type:ty; $($rest:tt)*) => {
        fn $method(&self) -> $ret_type;
        define_all_options_traits!(@collect_trait_methods $($rest)*);
    };
    (@collect_trait_methods opt ($method_opt:ident, $field_id_opt:expr) -> $ret_type_opt:ty; $($rest:tt)*) => {
        fn $method_opt(&self) -> Option<$ret_type_opt>;
        define_all_options_traits!(@collect_trait_methods $($rest)*);
    };

    // Collect implementation methods
    (@collect_impl_methods) => {};
    (@collect_impl_methods ($method:ident, $field_id:expr, $default:expr) -> $ret_type:ty; $($rest:tt)*) => {
        fn $method(&self) -> $ret_type {
            let Some(v) = self.special_fields.unknown_fields().get($field_id) else {
                return $default;
            };
            let extractor = PbOptionsValueExtractorImpl { id: $field_id };
            <PbOptionsValueExtractorImpl as PbOptionsValueExtractor<$ret_type>>::extract(&extractor, v)
        }
        define_all_options_traits!(@collect_impl_methods $($rest)*);
    };
    (@collect_impl_methods opt ($method_opt:ident, $field_id_opt:expr) -> $ret_type_opt:ty; $($rest:tt)*) => {
        fn $method_opt(&self) -> Option<$ret_type_opt> {
            let v = self.special_fields.unknown_fields().get($field_id_opt)?;
            let extractor = PbOptionsValueExtractorImpl { id: $field_id_opt };
            Some(<PbOptionsValueExtractorImpl as PbOptionsValueExtractor<$ret_type_opt>>::extract(&extractor, v))
        }
        define_all_options_traits!(@collect_impl_methods $($rest)*);
    };
}

// pb options maintenance
pub struct PbOptions;
impl PbOptions {
    // defined in pilota.proto
    // define_pb_option!(rs_package, 1215201); for now, this is impossible to implement, because the parser will directly use the package field: https://github.com/stepancheg/rust-protobuf/blob/master/protobuf-parse/src/pure/convert/mod.rs#L659
    define_pb_option!(serde_attribute, 1215201);
    define_pb_option!(name, 1215202);
    define_pb_option!(rust_wrapper_arc, 1215203, false);
    define_pb_option!(rust_type, 1215204);
    define_pb_option!(optional_repeated, 1215205, false);
}

// define all options traits and implementations
define_all_options_traits! {

    // PilotaFileOptions for protobuf::descriptor::FileOptions {
    //     opt (rs_package, PbOptions::RS_PACKAGE_ID) -> FastStr;
    // }

    PilotaMessageOptions for protobuf::descriptor::MessageOptions {
        opt (serde_attribute, PbOptions::SERDE_ATTRIBUTE_ID) -> FastStr;
        opt (name, PbOptions::NAME_ID) -> FastStr;
    }

    PilotaFieldOptions for protobuf::descriptor::FieldOptions {
        (rust_wrapper_arc, PbOptions::RUST_WRAPPER_ARC_ID, PbOptions::RUST_WRAPPER_ARC_DEFAULT) -> bool;
        opt (serde_attribute, PbOptions::SERDE_ATTRIBUTE_ID) -> FastStr;
        opt (name, PbOptions::NAME_ID) -> FastStr;
        opt (rust_type, PbOptions::RUST_TYPE_ID) -> FastStr;
        (optional_repeated, PbOptions::OPTIONAL_REPEATED_ID, PbOptions::OPTIONAL_REPEATED_DEFAULT) -> bool;
    }

    PilotaEnumOptions for protobuf::descriptor::EnumOptions {
        opt (serde_attribute, PbOptions::SERDE_ATTRIBUTE_ID) -> FastStr;
        opt (name, PbOptions::NAME_ID) -> FastStr;
    }

    PilotaEnumValueOptions for protobuf::descriptor::EnumValueOptions {
        opt (serde_attribute, PbOptions::SERDE_ATTRIBUTE_ID) -> FastStr;
    }

    PilotaServiceOptions for protobuf::descriptor::ServiceOptions {
        (rust_wrapper_arc, PbOptions::RUST_WRAPPER_ARC_ID, PbOptions::RUST_WRAPPER_ARC_DEFAULT) -> bool;
    }
}

// TODO: cannot implement this trait now, because the parser will directly use the package field: https://github.com/stepancheg/rust-protobuf/blob/master/protobuf-parse/src/pure/convert/mod.rs#L659
// pub trait FileDescriptorProtoExt {
//     fn rs_package(&self) -> Option<FastStr>;
// }

// impl FileDescriptorProtoExt for FileDescriptorProto {
//     fn rs_package(&self) -> Option<FastStr> {
//         self.options
//             .rs_package()
//             .or_else(|| self.package.as_ref().map(FastStr::new))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use protobuf::descriptor::{MessageOptions, field_descriptor_proto};

    #[test]
    fn lower_message_converts_map_entry_to_ir_map() {
        let mut lower = Lower::default();
        lower.cur_package = Some("pkg".into());
        lower.cur_syntax = Syntax::Proto3;

        let mut map_entry = DescriptorProto::new();
        map_entry.set_name("EntriesEntry".into());

        let mut entry_options = MessageOptions::new();
        entry_options.set_map_entry(true);
        map_entry.options = protobuf::MessageField::some(entry_options);

        let mut key_field = FieldDescriptorProto::new();
        key_field.set_name("key".into());
        key_field.set_number(1);
        key_field.set_label(field_descriptor_proto::Label::LABEL_OPTIONAL);
        key_field.set_type(field_descriptor_proto::Type::TYPE_STRING);
        map_entry.field.push(key_field);

        let mut value_field = FieldDescriptorProto::new();
        value_field.set_name("value".into());
        value_field.set_number(2);
        value_field.set_label(field_descriptor_proto::Label::LABEL_OPTIONAL);
        value_field.set_type(field_descriptor_proto::Type::TYPE_INT32);
        map_entry.field.push(value_field);

        let mut message = DescriptorProto::new();
        message.set_name("Outer".into());
        message.nested_type.push(map_entry);

        let mut field = FieldDescriptorProto::new();
        field.set_name("entries".into());
        field.set_number(1);
        field.set_label(field_descriptor_proto::Label::LABEL_REPEATED);
        field.set_type(field_descriptor_proto::Type::TYPE_MESSAGE);
        field.set_type_name(".pkg.Outer.EntriesEntry".into());
        message.field.push(field);

        let items = lower.lower_message(&message, &mut Vec::new());

        assert_eq!(items.len(), 1);

        let ir::ItemKind::Message(ir::Message { fields, .. }) = &items[0].kind else {
            panic!("expected message item");
        };

        assert_eq!(fields.len(), 1);
        let map_field = &fields[0];
        assert!(matches!(map_field.kind, FieldKind::Required));

        let ir::TyKind::Map(key_ty, value_ty) = &map_field.ty.kind else {
            panic!("expected map type");
        };

        match (&key_ty.kind, &value_ty.kind) {
            (ir::TyKind::String, ir::TyKind::I32) => {}
            other => panic!("unexpected key/value types: {:?}", other),
        }
    }

    #[test]
    fn lower_message_marks_proto3_optional_scalar_as_optional() {
        let mut lower = Lower::default();
        lower.cur_package = Some("pkg".into());
        lower.cur_syntax = Syntax::Proto3;

        let mut message = DescriptorProto::new();
        message.set_name("Foo".into());

        let mut field = FieldDescriptorProto::new();
        field.set_name("value".into());
        field.set_number(1);
        field.set_label(field_descriptor_proto::Label::LABEL_OPTIONAL);
        field.set_type(field_descriptor_proto::Type::TYPE_INT32);
        field.set_proto3_optional(true);
        message.field.push(field);

        let items = lower.lower_message(&message, &mut Vec::new());

        assert_eq!(items.len(), 1);

        let ir::ItemKind::Message(ir::Message { fields, .. }) = &items[0].kind else {
            panic!("expected message item");
        };

        assert_eq!(fields.len(), 1);
        let optional_field = &fields[0];
        assert!(matches!(optional_field.kind, FieldKind::Optional));
        assert!(matches!(optional_field.ty.kind, ir::TyKind::I32));
    }
}
