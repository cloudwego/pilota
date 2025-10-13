use std::marker::PhantomData;

use bytes::Bytes;
use faststr::FastStr;

use crate::pb::{DecodeError, Message, message::EnumMessage};

pub trait OptionValueExtractor {
    type Value;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError>;
}

pub struct BoolOptionValueExtractor;

impl OptionValueExtractor for BoolOptionValueExtractor {
    type Value = bool;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::Varint(v) => Ok(v != 0),
            _ => Err(DecodeError::new("invalid value for bool")),
        }
    }
}

pub struct Int32OptionValueExtractor;

impl OptionValueExtractor for Int32OptionValueExtractor {
    type Value = i32;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::Varint(v) => Ok(v as i32),
            _ => Err(DecodeError::new("invalid value for i32")),
        }
    }
}

pub struct Int64OptionValueExtractor;

impl OptionValueExtractor for Int64OptionValueExtractor {
    type Value = i64;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::Varint(v) => Ok(v as i64),
            _ => Err(DecodeError::new("invalid value for i64")),
        }
    }
}

pub struct MessageOptionValueExtractor<T> {
    phantom: PhantomData<T>,
}

impl<T> OptionValueExtractor for MessageOptionValueExtractor<T>
where
    T: Message + Default + Clone,
{
    type Value = T;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::LengthDelimited(v) => T::decode(Bytes::copy_from_slice(v)),
            _ => Err(DecodeError::new("invalid value for pb Message")),
        }
    }
}

pub struct UInt32OptionValueExtractor;

impl OptionValueExtractor for UInt32OptionValueExtractor {
    type Value = u32;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::Varint(v) => Ok(v as u32),
            _ => Err(DecodeError::new("invalid value for u32")),
        }
    }
}

pub struct UInt64OptionValueExtractor;

impl OptionValueExtractor for UInt64OptionValueExtractor {
    type Value = u64;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::Varint(v) => Ok(v),
            _ => Err(DecodeError::new("invalid value for u64")),
        }
    }
}

pub struct FloatOptionValueExtractor;

impl OptionValueExtractor for FloatOptionValueExtractor {
    type Value = f32;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::Fixed32(v) => Ok(f32::from_bits(v)),
            _ => Err(DecodeError::new("invalid value for f32")),
        }
    }
}

pub struct DoubleOptionValueExtractor;

impl OptionValueExtractor for DoubleOptionValueExtractor {
    type Value = f64;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::Fixed64(v) => Ok(f64::from_bits(v)),
            _ => Err(DecodeError::new("invalid value for f64")),
        }
    }
}

pub struct StrOptionValueExtractor;

impl OptionValueExtractor for StrOptionValueExtractor {
    type Value = FastStr;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::LengthDelimited(v) => match std::str::from_utf8(v) {
                Ok(s) => Ok(FastStr::new(s)),
                Err(_) => Err(DecodeError::new("invalid value for string")),
            },
            _ => Err(DecodeError::new("invalid value for string")),
        }
    }
}
pub struct BytesOptionValueExtractor;

impl OptionValueExtractor for BytesOptionValueExtractor {
    type Value = Bytes;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError> {
        match value {
            protobuf::UnknownValueRef::LengthDelimited(v) => Ok(Bytes::copy_from_slice(v)),
            _ => Err(DecodeError::new("invalid value for bytes")),
        }
    }
}

/// CustomExtField is used to extract the value of option used in pb item
/// - field_number, the defined tag id number of the option
/// - M, the option extendee type, including
///     - FileOptions
///     - MessageOptions
///     - FieldOptions
///     - EnumOptions
///     - ServiceOptions
///     - MethodOptions
///     - OneofOptions
/// - V, the option value type, including
///     - BoolOptionValueExtractor
///     - Int32OptionValueExtractor
///     - Int64OptionValueExtractor
///     - UInt32OptionValueExtractor
///     - UInt64OptionValueExtractor
///     - FloatOptionValueExtractor
///     - DoubleOptionValueExtractor
///     - StrOptionValueExtractor
///     - BytesOptionValueExtractor
///     - MessageOptionValueExtractor
pub struct CustomExtField<M, V> {
    field_number: u32,
    phantom: PhantomData<(M, V)>,
}

impl<M, V> CustomExtField<M, V>
where
    M: protobuf::Message,
    V: OptionValueExtractor,
{
    pub const fn new(field_number: u32) -> Self {
        Self {
            field_number,
            phantom: PhantomData,
        }
    }

    pub fn get(&self, m: &M) -> Result<V::Value, DecodeError> {
        if let Some(u) = m.unknown_fields().get(self.field_number) {
            match V::get_from_unknown(u) {
                Ok(v) => Ok(v),
                Err(e) => Err(DecodeError::new(format!(
                    "invalid value for option {}: {e}",
                    self.field_number
                ))),
            }
        } else {
            Err(DecodeError::new("extension field not found"))
        }
    }
}

pub trait EnumValueExtractor {
    type Value;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError>;
}

impl<T> EnumValueExtractor for T
where
    T: EnumMessage,
{
    type Value = T;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<T, DecodeError> {
        match value {
            protobuf::UnknownValueRef::Varint(v) => {
                T::try_from_i32(v as i32).ok_or(DecodeError::new("invalid value for i32"))
            }
            _ => Err(DecodeError::new("invalid value for i32")),
        }
    }
}

/// CustomExtEnumField is used to extract the value of enum option used in pb item
/// - field_number, the defined tag id number of the option
/// - M, the option extendee type, including
///     - FileOptions
///     - MessageOptions
///     - FieldOptions
///     - EnumOptions
///     - ServiceOptions
///     - MethodOptions
///     - OneofOptions
/// - T, the enum value generic type, which implements EnumMessage
pub struct CustomExtEnumField<M, T> {
    field_number: u32,
    phantom: PhantomData<(M, T)>,
}

impl<M, T> CustomExtEnumField<M, T>
where
    M: protobuf::Message,
    T: EnumMessage,
{
    pub const fn new(field_number: u32) -> Self {
        Self {
            field_number,
            phantom: PhantomData,
        }
    }

    pub fn get(&self, m: &M) -> Result<T, DecodeError> {
        if let Some(u) = m.unknown_fields().get(self.field_number) {
            match T::get_from_unknown(u) {
                Ok(v) => Ok(v),
                Err(e) => Err(DecodeError::new(format!(
                    "invalid value for enum option {}: {e}",
                    self.field_number
                ))),
            }
        } else {
            Err(DecodeError::new("extension field not found"))
        }
    }
}
