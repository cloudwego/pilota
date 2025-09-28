use std::sync::Arc;

use crate::{ir, symbol::Ident, ty::Ty};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExtendeeIndex {
    pub extendee_kind: ExtendeeKind,
    pub tag_id: u32,
}

impl From<ir::ext::pb::ExtendeeIndex> for ExtendeeIndex {
    fn from(index: ir::ext::pb::ExtendeeIndex) -> Self {
        ExtendeeIndex {
            extendee_kind: index.extendee_kind.into(),
            tag_id: index.tag_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Extendee {
    pub name: Ident,
    pub index: ExtendeeIndex,
    pub extendee_ty: ExtendeeType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExtendeeType {
    pub field_ty: FieldType,
    pub item_ty: Ty,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum ExtendeeKind {
    File,
    Message,
    Field,
    Enum,
    EnumValue,
    Service,
    Method,
    Oneof,
}

impl From<ir::ext::pb::ExtendeeKind> for ExtendeeKind {
    fn from(kind: ir::ext::pb::ExtendeeKind) -> Self {
        match kind {
            ir::ext::pb::ExtendeeKind::File => ExtendeeKind::File,
            ir::ext::pb::ExtendeeKind::Message => ExtendeeKind::Message,
            ir::ext::pb::ExtendeeKind::Field => ExtendeeKind::Field,
            ir::ext::pb::ExtendeeKind::Enum => ExtendeeKind::Enum,
            ir::ext::pb::ExtendeeKind::EnumValue => ExtendeeKind::EnumValue,
            ir::ext::pb::ExtendeeKind::Service => ExtendeeKind::Service,
            ir::ext::pb::ExtendeeKind::Method => ExtendeeKind::Method,
            ir::ext::pb::ExtendeeKind::Oneof => ExtendeeKind::Oneof,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FieldType {
    Bool,
    Int32,
    Int64,
    UInt32,
    UInt64,
    Float,
    Double,
    String,
    Bytes,
    Message,
    Enum,
}

impl From<ir::ext::pb::FieldType> for FieldType {
    fn from(field_ty: ir::ext::pb::FieldType) -> Self {
        match field_ty {
            ir::ext::pb::FieldType::Bool => FieldType::Bool,
            ir::ext::pb::FieldType::Int32 => FieldType::Int32,
            ir::ext::pb::FieldType::Int64 => FieldType::Int64,
            ir::ext::pb::FieldType::UInt32 => FieldType::UInt32,
            ir::ext::pb::FieldType::UInt64 => FieldType::UInt64,
            ir::ext::pb::FieldType::Float => FieldType::Float,
            ir::ext::pb::FieldType::Double => FieldType::Double,
            ir::ext::pb::FieldType::String => FieldType::String,
            ir::ext::pb::FieldType::Bytes => FieldType::Bytes,
            ir::ext::pb::FieldType::Message => FieldType::Message,
            ir::ext::pb::FieldType::Enum => FieldType::Enum,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileExts {
    pub extendees: Vec<Arc<Extendee>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModExts {
    pub extendees: Vec<Arc<Extendee>>,
}
