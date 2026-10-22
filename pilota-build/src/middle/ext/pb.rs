use std::sync::Arc;

use crate::{ir, parser::protobuf::WellKnownFileName, rir::Path, symbol::Ident, ty::Ty};

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
pub enum UsedOptionValue {
    Fixed32(u32),
    Fixed64(u64),
    Varint(u64),
    LengthDelimited(Vec<u8>),
}

impl From<ir::ext::pb::UsedOptionValue> for UsedOptionValue {
    fn from(value: ir::ext::pb::UsedOptionValue) -> Self {
        match value {
            ir::ext::pb::UsedOptionValue::Fixed32(value) => Self::Fixed32(value),
            ir::ext::pb::UsedOptionValue::Fixed64(value) => Self::Fixed64(value),
            ir::ext::pb::UsedOptionValue::Varint(value) => Self::Varint(value),
            ir::ext::pb::UsedOptionValue::LengthDelimited(value) => Self::LengthDelimited(value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UsedOption {
    pub index: ExtendeeIndex,
    pub value: UsedOptionValue,
}

impl From<&ir::ext::pb::UsedOption> for UsedOption {
    fn from(option: &ir::ext::pb::UsedOption) -> Self {
        Self {
            index: option.index.into(),
            value: option.value.clone().into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UsedOptions(pub Vec<UsedOption>);

impl UsedOptions {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn indices(&self) -> impl Iterator<Item = &ExtendeeIndex> {
        self.0.iter().map(|option| &option.index)
    }

    /// Returns every value associated with this extension index.
    pub fn values_for(&self, index: &ExtendeeIndex) -> impl Iterator<Item = &UsedOptionValue> {
        self.0
            .iter()
            .filter_map(move |option| (option.index == *index).then_some(&option.value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Extendees(pub Vec<Arc<Extendee>>);

impl Extendees {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileExts {
    pub well_known_file_name: WellKnownFileName,
    pub extendees: Extendees,
    pub used_options: UsedOptions,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModExts {
    pub extendees: Extendees,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ItemExts {
    pub used_options: UsedOptions,
    pub parent: Option<Path>,
}
