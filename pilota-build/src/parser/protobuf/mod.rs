use std::{collections::HashMap, path::PathBuf, sync::Arc};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use normpath::PathExt;
use protobuf::descriptor::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, EnumDescriptorProto, ServiceDescriptorProto,
};

use super::Parser;
use crate::{
    index::Idx,
    ir::{self, FieldKind, Item, Path, TyKind},
    symbol::{EnumRepr, FileId, Ident, IdentName},
    tags::{
        protobuf::{ClientStreaming, OneOf, ProstType, Repeated, ServerStreaming},
        PilotaName, Tags,
    },
    ty::StringRepr,
};

#[derive(Default)]
pub struct ProtobufParser {
    inner: protobuf_parse::Parser,
    include_dirs: Vec<PathBuf>,
    input_files: FxHashSet<PathBuf>,
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
        ir::Path {
            segments: Arc::from_iter(s.split('.').map(Ident::from)),
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

            return ir::Ty {
                kind: ir::TyKind::Path(self.str2path(&name[1..])),
                tags: Default::default(),
            };
        }
        let Some(ty) = type_ else {
            panic!()
        };

        let mut tags = Tags::default();
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
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING => {
                tags.insert(StringRepr::String);
                ir::TyKind::String
            }
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

    fn lower_message(
        &self,
        message: &DescriptorProto,
        parent_messages: &mut Vec<String>,
    ) -> ir::Item {
        let fq_message_name = format!(
            "{}{}.{}{}",
            if self.cur_package.is_none() { "" } else { "." },
            self.cur_package.as_deref().unwrap_or(""),
            {
                let mut s = String::new();
                parent_messages.iter().for_each(|m| {
                    s.push_str(m);
                    s.push_str(".")
                });
                s
            },
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

        let mut extra_fields = Vec::default();

        message.oneof_decl.iter().enumerate().for_each(|(idx, d)| {
            if let Some(fields) = oneof_fields.remove(&(idx as i32)) {
                nested_items.push(Arc::new(ir::Item {
                    related_items: Default::default(),
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

                extra_fields.push(ir::Field {
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
                });
            }
        });

        parent_messages.push(message.name().into());

        nested_messages
            .iter()
            .filter(|(_, m)| !m.options.has_map_entry())
            .for_each(|(_, m)| nested_items.push(Arc::new(self.lower_message(m, parent_messages))));

        parent_messages.pop();

        message
            .enum_type
            .iter()
            .for_each(|e| nested_items.push(Arc::new(self.lower_enum(e))));

        let item = ir::Item {
            related_items: Default::default(),
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

                        let optional = !is_map
                            && ({
                                f.proto3_optional()
                                    || (!repeated && matches!(f.type_(), Type::TYPE_MESSAGE))
                            } || f.label() == Label::LABEL_OPTIONAL);

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
                    .chain(extra_fields)
                    .collect(),
                name: message.name().into(),
            }),
        };

        if nested_items.is_empty() {
            item
        } else {
            let name = item.name().clone();
            nested_items.push(Arc::new(item));
            let mut tags = Tags::default();
            tags.insert(PilotaName(name.0.mod_ident().clone()));
            Item {
                related_items: Default::default(),
                tags: Arc::from(tags),
                kind: ir::ItemKind::Mod(ir::Mod {
                    name: Ident { sym: name },
                    items: nested_items,
                }),
            }
        }
    }

    pub fn lower_service(&self, service: &ServiceDescriptorProto) -> ir::Item {
        ir::Item {
            tags: Default::default(),
            related_items: Default::default(),
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
                                tags: Arc::new(Tags::default()),
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
        let mut file_map = HashMap::with_capacity(files.len());
        files.iter().for_each(|f| {
            self.files
                .insert(f.name().to_string(), self.next_file_id.inc_one());
            file_map.insert(f.name(), f);
        });

        files
            .iter()
            .map(|f| {
                self.cur_package = f.package.clone();

                let file_id = *self.files.get(f.name()).unwrap();

                let package = self.str2path(&f.package());

                let enums = f.enum_type.iter().map(|e| self.lower_enum(e));
                let messages = f
                    .message_type
                    .iter()
                    .map(|m| self.lower_message(m, &mut Vec::new()));
                let services = f.service.iter().map(|s| self.lower_service(s));

                let f = Arc::from(ir::File {
                    package: package.clone(),
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
                });

                self.cur_package = None;

                f
            })
            .collect::<Vec<_>>()
    }
}

impl Parser for ProtobufParser {
    fn input<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.input_files
            .insert(path.as_ref().normalize().unwrap().into_path_buf());
        self.inner.input(path);
    }

    fn include_dirs(&mut self, dirs: Vec<std::path::PathBuf>) {
        self.include_dirs = dirs.clone();
        self.inner.includes(dirs);
    }

    fn parse(self) -> super::ParseResult {
        let descriptors = self.inner.parse_and_typecheck().unwrap().file_descriptors;

        let mut input_file_ids = vec![];

        let mut lower = Lower::default();

        let files = lower.lower(&descriptors);

        let mut file_ids = FxHashMap::default();

        descriptors.iter().for_each(|f| {
            self.include_dirs.iter().for_each(|p| {
                let path = p.join(f.name());
                if path.exists() {
                    println!("cargo:rerun-if-changed={}", path.display());
                    file_ids.insert(
                        Arc::from(path.normalize().unwrap().into_path_buf()),
                        *lower.files.get(f.name()).unwrap(),
                    );
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
        }
    }
}
