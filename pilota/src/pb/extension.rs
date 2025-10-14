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

/// CustomExtEnumField is used to extract the value of enum option used in pb
/// item
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

#[cfg(test)]
mod tests {
    use crate::pb::{Message, encoding::DecodeContext};
    use linkedbytes::LinkedBytes;
    use protobuf::Message as PbMessage;

    use super::*;

    #[test]
    fn test_custom_ext_bool_field() {
        let field: CustomExtField<protobuf::descriptor::FileOptions, BoolOptionValueExtractor> =
            CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields().add_varint(1, 1);
        let v = field.get(&m).unwrap();
        assert_eq!(v, true);
    }

    #[test]
    fn test_custom_ext_int32_field() {
        let field: CustomExtField<protobuf::descriptor::FileOptions, Int32OptionValueExtractor> =
            CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields().add_varint(1, 1);
        let v = field.get(&m).unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_custom_ext_int64_field() {
        let field: CustomExtField<protobuf::descriptor::FileOptions, Int64OptionValueExtractor> =
            CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields().add_varint(1, 1);
        let v = field.get(&m).unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_custom_ext_uint32_field() {
        let field: CustomExtField<protobuf::descriptor::FileOptions, UInt32OptionValueExtractor> =
            CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields().add_varint(1, 1);
        let v = field.get(&m).unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_custom_ext_uint64_field() {
        let field: CustomExtField<protobuf::descriptor::FileOptions, UInt64OptionValueExtractor> =
            CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields().add_varint(1, 1);
        let v = field.get(&m).unwrap();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_custom_ext_float_field() {
        let field: CustomExtField<protobuf::descriptor::FileOptions, FloatOptionValueExtractor> =
            CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields().add_fixed32(1, 1.0f32.to_bits());
        let v = field.get(&m).unwrap();
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_custom_ext_double_field() {
        let field: CustomExtField<protobuf::descriptor::FileOptions, DoubleOptionValueExtractor> =
            CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields().add_fixed64(1, 1.0f64.to_bits());
        let v = field.get(&m).unwrap();
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_custom_ext_str_field() {
        let field: CustomExtField<protobuf::descriptor::FileOptions, StrOptionValueExtractor> =
            CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields()
            .add_length_delimited(1, b"hello".to_vec());
        let v = field.get(&m).unwrap();
        assert_eq!(v, "hello");
    }

    #[test]
    fn test_custom_ext_bytes_field() {
        let field: CustomExtField<protobuf::descriptor::FileOptions, BytesOptionValueExtractor> =
            CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields()
            .add_length_delimited(1, b"hello".to_vec());
        let v = field.get(&m).unwrap();
        assert_eq!(v, Bytes::copy_from_slice(b"hello"));
    }

    #[test]
    fn test_custom_ext_message_field() {
        use crate::{Buf as _, BufMut as _};
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub a: ::std::option::Option<bytes::Bytes>,
        }
        impl crate::pb::Message for A {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self
                    .a
                    .as_ref()
                    .map_or(0, |value| crate::pb::encoding::bytes::encoded_len(1, value))
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut crate::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.a.as_ref() {
                    crate::pb::encoding::bytes::encode(1, _pilota_inner_value, buf);
                };
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: crate::pb::encoding::WireType,
                buf: &mut crate::Bytes,
                ctx: &mut crate::pb::encoding::DecodeContext,
            ) -> ::core::result::Result<(), crate::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(A);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.a;
                        crate::pb::encoding::bytes::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(a));
                            error
                        })
                    }
                    _ => crate::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }

        let field: CustomExtField<
            protobuf::descriptor::FileOptions,
            MessageOptionValueExtractor<A>,
        > = CustomExtField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        let mut buf = crate::LinkedBytes::new();
        A::default().encode(&mut buf).unwrap();
        m.mut_unknown_fields()
            .add_length_delimited(1, buf.bytes().clone().freeze().to_vec());
        let v = field.get(&m).unwrap();
        assert_eq!(v, A::default());
    }

    #[test]
    fn test_custom_ext_enum_field() {
        use crate::{Buf as _, BufMut as _};

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct PaymentType(i32);

        impl PaymentType {
            pub const PAYMENT_UNKNOWN: Self = Self(0);
            pub const PAYMENT_CREDIT: Self = Self(1);
            pub const PAYMENT_DEBIT: Self = Self(2);
            pub const PAYMENT_BANK_TRANSFER: Self = Self(3);
            pub const PAYMENT_CRYPTO: Self = Self(4);
            pub const PAYMENT_CHECK: Self = Self(5);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("PAYMENT_UNKNOWN"),
                    Self(1) => ::std::string::String::from("PAYMENT_CREDIT"),
                    Self(2) => ::std::string::String::from("PAYMENT_DEBIT"),
                    Self(3) => ::std::string::String::from("PAYMENT_BANK_TRANSFER"),
                    Self(4) => ::std::string::String::from("PAYMENT_CRYPTO"),
                    Self(5) => ::std::string::String::from("PAYMENT_CHECK"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => Some(Self::PAYMENT_UNKNOWN),
                    1 => Some(Self::PAYMENT_CREDIT),
                    2 => Some(Self::PAYMENT_DEBIT),
                    3 => Some(Self::PAYMENT_BANK_TRANSFER),
                    4 => Some(Self::PAYMENT_CRYPTO),
                    5 => Some(Self::PAYMENT_CHECK),
                    _ => None,
                }
            }
        }

        impl crate::pb::EnumMessage for PaymentType {
            fn inner(&self) -> i32 {
                self.inner()
            }

            fn to_string(&self) -> ::std::string::String {
                self.to_string()
            }

            fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                PaymentType::try_from_i32(value)
            }
        }

        impl ::std::convert::From<i32> for PaymentType {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<PaymentType> for i32 {
            fn from(value: PaymentType) -> i32 {
                value.0
            }
        }

        let field: CustomExtEnumField<protobuf::descriptor::FileOptions, PaymentType> =
            CustomExtEnumField::new(1);
        let mut m = protobuf::descriptor::FileOptions::new();
        m.mut_unknown_fields().add_varint(1, 1);
        let v = field.get(&m).unwrap();
        assert_eq!(v, PaymentType::PAYMENT_CREDIT);
    }
}
