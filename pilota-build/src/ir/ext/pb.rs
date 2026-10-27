use std::sync::Arc;

use crate::{
    ir::{Path, Ty},
    parser::protobuf::WellKnownFileName,
    symbol::Ident,
};

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

/// A custom option's raw protobuf wire value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UsedOptionValue {
    Fixed32(u32),
    Fixed64(u64),
    Varint(u64),
    LengthDelimited(Vec<u8>),
}

impl<'a> From<protobuf::UnknownValueRef<'a>> for UsedOptionValue {
    fn from(value: protobuf::UnknownValueRef<'a>) -> Self {
        match value {
            protobuf::UnknownValueRef::Fixed32(value) => Self::Fixed32(value),
            protobuf::UnknownValueRef::Fixed64(value) => Self::Fixed64(value),
            protobuf::UnknownValueRef::Varint(value) => Self::Varint(value),
            protobuf::UnknownValueRef::LengthDelimited(value) => {
                Self::LengthDelimited(value.to_vec())
            }
        }
    }
}

/// One use of a protobuf custom option, including its raw wire value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UsedOption {
    pub index: ExtendeeIndex,
    pub value: UsedOptionValue,
}

#[derive(Clone, Debug)]
pub struct UsedOptions(pub Vec<UsedOption>);

impl UsedOptions {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns every value associated with this extension index.
    pub fn values_for(&self, index: &ExtendeeIndex) -> impl Iterator<Item = &UsedOptionValue> {
        self.0
            .iter()
            .filter_map(move |option| (option.index == *index).then_some(&option.value))
    }

    pub fn from_pb_unknown_fields(
        extendee_kind: ExtendeeKind,
        unknown_fields: &protobuf::UnknownFields,
    ) -> Self {
        Self(
            unknown_fields
                .iter()
                .map(|(tag_id, value)| UsedOption {
                    index: ExtendeeIndex {
                        extendee_kind,
                        tag_id,
                    },
                    value: value.into(),
                })
                .collect(),
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
/// - parent, the parent path of nested message, enum and oneof
#[derive(Clone, Debug)]
pub struct ItemExts {
    pub used_options: UsedOptions,
    pub parent: Option<Path>,
}

impl ItemExts {
    pub fn has_used_options(&self) -> bool {
        !self.used_options.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_custom_option_wire_values_and_repetitions() {
        let mut unknown_fields = protobuf::UnknownFields::new();
        unknown_fields.add_varint(10_001, 0x10016f);
        unknown_fields.add_varint(10_001, 1);
        unknown_fields.add_fixed32(10_002, 0x3f80_0000);
        unknown_fields.add_fixed64(10_003, 0x3ff0_0000_0000_0000);
        unknown_fields.add_length_delimited(10_004, vec![0x61, 0x62]);

        let used = UsedOptions::from_pb_unknown_fields(ExtendeeKind::Service, &unknown_fields);

        let repeated_index = ExtendeeIndex {
            extendee_kind: ExtendeeKind::Service,
            tag_id: 10_001,
        };
        let mut values = used.values_for(&repeated_index);
        assert_eq!(values.next(), Some(&UsedOptionValue::Varint(0x10016f)));
        assert_eq!(values.next(), Some(&UsedOptionValue::Varint(1)));
        assert_eq!(values.next(), None);

        assert_eq!(
            used.values_for(&ExtendeeIndex {
                extendee_kind: ExtendeeKind::Service,
                tag_id: 10_002,
            })
            .next(),
            Some(&UsedOptionValue::Fixed32(0x3f80_0000))
        );
        assert_eq!(
            used.values_for(&ExtendeeIndex {
                extendee_kind: ExtendeeKind::Service,
                tag_id: 10_003,
            })
            .next(),
            Some(&UsedOptionValue::Fixed64(0x3ff0_0000_0000_0000))
        );
        assert_eq!(
            used.values_for(&ExtendeeIndex {
                extendee_kind: ExtendeeKind::Service,
                tag_id: 10_004,
            })
            .next(),
            Some(&UsedOptionValue::LengthDelimited(vec![0x61, 0x62]))
        );
    }
}
