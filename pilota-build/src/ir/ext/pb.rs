use std::sync::Arc;

use crate::{ir::Ty, parser::protobuf::WellKnownFileName, symbol::Ident};

/// Extension for protobuf custom options
/// As defined in [google/protobuf/descriptor.proto](https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto#L124), the extendee is following the syntax of [FieldDescriptorProto](https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto#L243)
/// The extension is used to store the definition of the custom option, for
/// example
///
/// ```proto
/// extend {extendee_kind} {
///     optional {extendee_ty} {name} = {tag_id};
/// }
/// ```
/// - name, the name of the custom option
/// - index, the index of the extendee, including extendee_kind and tag_id
/// - extendee_ty, the type supported by protobuf syntax of custom option type
#[derive(Clone, Debug)]
pub struct Extendee {
    pub name: Ident,
    pub index: ExtendeeIndex,
    pub extendee_ty: ExtendeeType,
}

/// Index of the extendee, uniquely identify the extendee
/// - extendee_kind, the extendee level, including file, message, field, enum,
///   enum value, service, method
/// - tag_id, the tag id of the extendee
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct ExtendeeIndex {
    pub extendee_kind: ExtendeeKind,
    pub tag_id: u32,
}

/// The type of the extendee
/// - field_ty, the field type defined in [field type](https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto#L244)
/// - item_ty, the concrete type defined idl
#[derive(Clone, Debug)]
pub struct ExtendeeType {
    pub field_ty: FieldType,
    pub item_ty: Ty,
}

/// The field type defined in [field type](https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/descriptor.proto#L244)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// The extendee level, including file, message, field, enum, enum value,
/// service, method
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

#[derive(Clone, Debug)]
pub struct UsedOptions(pub Vec<ExtendeeIndex>);

impl UsedOptions {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn from_pb_unknown_fields(
        extendee_kind: ExtendeeKind,
        unknown_fields: &protobuf::UnknownFields,
    ) -> Self {
        Self(
            unknown_fields
                .iter()
                .map(|(k, _)| ExtendeeIndex {
                    extendee_kind,
                    tag_id: k,
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Extendees(pub Vec<Arc<Extendee>>);

impl Extendees {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// The extension for file
/// - extendees, the nested extendees
/// - used_options, the used options
#[derive(Clone, Debug)]
pub struct FileExts {
    pub well_known_file_name: WellKnownFileName,
    pub extendees: Extendees,
    pub used_options: UsedOptions,
}

impl FileExts {
    pub fn has_extendees(&self) -> bool {
        !self.extendees.is_empty()
    }

    pub fn has_used_options(&self) -> bool {
        !self.used_options.is_empty()
    }
}

/// The extension for mod
/// - extendees, the nested extendees
#[derive(Clone, Debug)]
pub struct ModExts {
    pub extendees: Extendees,
}

impl ModExts {
    pub fn has_extendees(&self) -> bool {
        !self.extendees.is_empty()
    }
}

/// The extension for item
/// - used_options, the used options
#[derive(Clone, Debug)]
pub struct ItemExts {
    pub used_options: UsedOptions,
}

impl ItemExts {
    pub fn has_used_options(&self) -> bool {
        !self.used_options.is_empty()
    }
}
