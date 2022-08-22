use std::sync::Arc;

use fxhash::FxHashMap;
use heck::ToSnakeCase;
use itertools::Itertools;
use protobuf::descriptor::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, EnumDescriptorProto, ServiceDescriptorProto,
};

use super::Parser;
use crate::{
    index::Idx,
    ir::{self, FieldKind, Item, Path, TyKind},
    symbol::{EnumRepr, FileId, Ident},
    tags::{
        protobuf::{
            ClientStreaming, Fixed32, Fixed64, OneOf, Repeated, SFixed32, SFixed64, SInt32, SInt64,
            ServerStreaming,
        },
        Tags,
    },
};

#[derive(Default)]
pub struct ProtobufParser {
    inner: protobuf_parse::Parser,
}

struct Lower {
    next_file_id: FileId,
    files: FxHashMap<String, FileId>,
    cur_package: Option<String>,
}

impl Default for Lower {
    fn default() -> Self {
        Self {
            next_file_id: FileId::from_u32(0),
            files: Default::default(),
            cur_package: None,
        }
    }
}

impl Lower {
    fn str2path(&self, s: &str) -> ir::Path {
        let segs = s.split('.').collect::<Vec<_>>();
        ir::Path {
            segments: Arc::from_iter(
                segs[0..segs.len() - 1]
                    .iter()
                    .map(|s| s.to_snake_case())
                    .chain(std::iter::once(segs[segs.len() - 1].to_string()))
                    .map(Ident::from),
            ),
        }
    }

    fn lower_ty(
        &self,
        type_: Option<protobuf::EnumOrUnknown<protobuf::descriptor::field_descriptor_proto::Type>>,
        type_name: Option<&str>,
        nested_messages: &FxHashMap<String, &DescriptorProto>,
    ) -> ir::Ty {
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
                            )),
                            Arc::from(self.lower_ty(
                                value.type_,
                                value.type_name.as_deref(),
                                nested_messages,
                            )),
                        ),
                        tags: Default::default(),
                    };
                }
            }

            assert_eq!(".", &name[..1]);
            let cur_pkg = self.cur_package.as_deref().unwrap_or("");
            if name[1..].starts_with(cur_pkg) {
                return ir::Ty {
                    kind: ir::TyKind::Path(ir::Path {
                        segments: Arc::from_iter([Ident::from(name.split('.').last().unwrap())]),
                    }),
                    tags: Default::default(),
                };
            } else {
                return ir::Ty {
                    kind: ir::TyKind::Path(self.str2path(&name[1..])),
                    tags: Default::default(),
                };
            }
        }

        if let Some(ty) = type_ {
            let mut tags = Tags::default();
            let kind = match ty.enum_value().unwrap() {
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_DOUBLE => ir::TyKind::F64,
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_FLOAT => ir::TyKind::F32,
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT64 => ir::TyKind::I64,
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT64 => {
                    ir::TyKind::UInt64
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT32 => ir::TyKind::I32,
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED64 => {
                    tags.insert(Fixed64);
                    ir::TyKind::UInt64
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED32 => {
                    tags.insert(Fixed32);
                    ir::TyKind::UInt32
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL => ir::TyKind::Bool,
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING => {
                    ir::TyKind::String
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_GROUP => todo!(),
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_BYTES => ir::TyKind::Bytes,
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT32 => {
                    ir::TyKind::UInt32
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED32 => {
                    tags.insert(SFixed32);
                    ir::TyKind::I32
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED64 => {
                    tags.insert(SFixed64);
                    ir::TyKind::I64
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT32 => {
                    tags.insert(SInt32);
                    ir::TyKind::I32
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT64 => {
                    tags.insert(SInt64);
                    ir::TyKind::I64
                }

                protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE
                | protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM => unreachable!(),
            };

            return ir::Ty {
                kind,
                tags: Arc::new(tags),
            };
        }

        panic!()
    }

    fn lower_enum(&self, e: &EnumDescriptorProto) -> ir::Item {
        ir::Item {
            tags: Default::default(),
            kind: ir::ItemKind::Enum(ir::Enum {
                name: e.name().into(),
                variants: e
                    .value
                    .iter()
                    .map(|v| ir::EnumVariant {
                        id: v.number,
                        name: v.name().into(),
                        discr: v.number.map(|v| v as i64),
                        tags: Default::default(),
                        fields: Default::default(),
                    })
                    .collect_vec(),
                repr: Some(EnumRepr::I32),
            }),
        }
    }

    fn lower_message(&self, message: &DescriptorProto) -> ir::Item {
        let fq_message_name = format!(
            "{}{}.{}",
            if self.cur_package.is_none() { "" } else { "." },
            self.cur_package.as_deref().unwrap_or(""),
            message.name()
        );

        let nested_messages = message
            .nested_type
            .iter()
            .map(|m| (format!("{}.{}", fq_message_name, m.name()), m))
            .collect::<FxHashMap<_, _>>();

        let mut fields = Vec::default();
        let mut oneof_fields = FxHashMap::default();

        message.field.iter().for_each(|field| {
            if field.proto3_optional.unwrap_or(false) {
                fields.push(field)
            } else if let Some(oneof_index) = field.oneof_index {
                oneof_fields
                    .entry(oneof_index)
                    .or_insert_with(Vec::default)
                    .push(field)
            } else {
                fields.push(field)
            }
        });

        let mut nested_items: Vec<_> = Default::default();

        message.oneof_decl.iter().enumerate().for_each(|(idx, d)| {
            let fields = oneof_fields.remove(&(idx as i32)).unwrap();
            nested_items.push(Arc::new(ir::Item {
                tags: Arc::new(crate::tags!(OneOf)),
                kind: ir::ItemKind::Enum(ir::Enum {
                    name: d.name().into(),
                    repr: None,
                    variants: fields
                        .iter()
                        .map(|f| ir::EnumVariant {
                            discr: None,
                            id: f.number,
                            name: f.name().into(),
                            fields: vec![self.lower_ty(
                                f.type_,
                                f.type_name.as_deref(),
                                &nested_messages,
                            )],
                            tags: Default::default(),
                        })
                        .collect_vec(),
                }),
            }));
        });

        nested_messages
            .iter()
            .filter(|(_, m)| !m.options.has_map_entry())
            .for_each(|(_, m)| nested_items.push(Arc::new(self.lower_message(m))));

        let item = ir::Item {
            tags: Default::default(),
            kind: ir::ItemKind::Message(ir::Message {
                fields: fields
                    .iter()
                    .map(|f| {
                        let mut ty =
                            self.lower_ty(f.type_, f.type_name.as_deref(), &nested_messages);

                        let is_map = matches!(ty.kind, TyKind::Map(_, _));
                        let repeated = !is_map && matches!(f.label(), Label::LABEL_REPEATED);

                        if repeated {
                            ty = ir::Ty {
                                kind: ir::TyKind::Vec(Arc::from(ty)),
                                tags: Default::default(),
                            }
                        }

                        let optional = !is_map && {
                            f.proto3_optional()
                                || (!repeated && matches!(f.type_(), Type::TYPE_MESSAGE))
                        };

                        let mut tags = Tags::default();
                        if repeated {
                            tags.insert(Repeated);
                        }

                        ir::Field {
                            id: f.number(),
                            name: f.name().into(),
                            ty,
                            tags: Arc::new(tags),
                            kind: if optional {
                                FieldKind::Optional
                            } else {
                                FieldKind::Required
                            },
                        }
                    })
                    .chain(message.oneof_decl.iter().map(|d| ir::Field {
                        name: d.name().into(),
                        id: -1,
                        ty: ir::Ty {
                            kind: ir::TyKind::Path(Path {
                                segments: Arc::from([d.name().into()]),
                            }),
                            tags: Default::default(),
                        },
                        tags: Arc::new(crate::tags!(OneOf)),
                        kind: ir::FieldKind::Optional,
                    }))
                    .collect(),
                name: message.name().into(),
            }),
        };

        if nested_items.is_empty() {
            item
        } else {
            let name = item.name().to_lower_camel_case();
            nested_items.push(Arc::new(item));
            Item {
                tags: Default::default(),
                kind: ir::ItemKind::Mod(ir::Mod {
                    name: Ident::new(name),
                    items: nested_items,
                }),
            }
        }
    }

    pub fn lower_service(&self, service: &ServiceDescriptorProto) -> ir::Item {
        ir::Item {
            tags: Default::default(),
            kind: ir::ItemKind::Service(ir::Service {
                name: service.name().into(),
                methods: service
                    .method
                    .iter()
                    .map(|m| {
                        let mut tags = Tags::default();
                        if m.client_streaming() {
                            tags.insert(ClientStreaming);
                        }
                        if m.server_streaming() {
                            tags.insert(ServerStreaming);
                        }
                        ir::Method {
                            name: m.name().into(),
                            tags: Arc::new(tags),
                            args: vec![ir::Arg {
                                name: "req".into(),
                                id: -1,
                                ty: self.lower_ty(
                                    None,
                                    m.input_type.as_deref(),
                                    &Default::default(),
                                ),
                            }],
                            oneway: false,
                            ret: self.lower_ty(None, m.output_type.as_deref(), &Default::default()),
                            exceptions: None,
                        }
                    })
                    .collect_vec(),
                extend: vec![],
            }),
        }
    }

    pub fn lower(
        &mut self,
        files: &[protobuf::descriptor::FileDescriptorProto],
    ) -> Vec<Arc<ir::File>> {
        files.iter().for_each(|f| {
            self.files
                .insert(f.name().to_string(), self.next_file_id.inc_one());
        });

        files
            .iter()
            .map(|f| {
                self.cur_package = f.package.clone();

                let file_id = *self.files.get(f.name()).unwrap();

                let package = self.str2path(
                    &f.package
                        .clone()
                        .unwrap_or_else(|| f.name().trim_end_matches(".proto").to_snake_case()),
                );

                let enums = f.enum_type.iter().map(|e| self.lower_enum(e));
                let messages = f.message_type.iter().map(|m| self.lower_message(m));
                let services = f.service.iter().map(|s| self.lower_service(s));

                let f = Arc::from(ir::File {
                    package,
                    uses: f
                        .dependency
                        .iter()
                        .map(|d| {
                            (
                                d.trim_end_matches(".proto")
                                    .split('/')
                                    .last()
                                    .unwrap()
                                    .into(),
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
                });

                self.cur_package = None;

                f
            })
            .collect::<Vec<_>>()
    }
}

impl Parser for ProtobufParser {
    fn input<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.inner.input(path);
    }

    fn include_dirs(&mut self, dirs: Vec<std::path::PathBuf>) {
        self.inner.includes(dirs);
    }

    fn parse(self) -> super::ParseResult {
        let descriptors = self.inner.parse_and_typecheck().unwrap().file_descriptors;

        super::ParseResult {
            files: Lower::default().lower(&descriptors),
        }
    }
}
