use std::marker::PhantomData;

use bytes::Bytes;

use crate::pb::{DecodeError, Message, message::EnumMessage};

pub trait OptionValueExtractor {
    type Value;
    fn get_from_unknown(value: protobuf::UnknownValueRef) -> Result<Self::Value, DecodeError>;
}

impl<T> OptionValueExtractor for T
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

pub struct CustomExtField<M, T> {
    field_number: u32,
    phantom: PhantomData<(M, T)>,
}

impl<M, T> CustomExtField<M, T>
where
    M: protobuf::Message,
    T: Message + Default + Clone,
{
    pub const fn new(field_number: u32) -> Self {
        Self {
            field_number,
            phantom: PhantomData,
        }
    }

    pub fn get(&self, m: &M) -> Result<T, DecodeError> {
        if let Some(u) = m.unknown_fields().get(self.field_number) {
            T::get_from_unknown(u)
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
            T::get_from_unknown(u)
        } else {
            Err(DecodeError::new("extension field not found"))
        }
    }
}
