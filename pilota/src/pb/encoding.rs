//! Utility functions and types for encoding and decoding Protobuf types.
//!
//! Meant to be used only from `Message` implementations.

#![allow(clippy::implicit_hasher, clippy::ptr_arg)]
extern crate alloc;

use alloc::{collections::BTreeMap, format, string::String, vec::Vec};
use core::{cmp::min, convert::TryFrom, mem, str};

use ::bytes::{Buf, BufMut, Bytes};
use linkedbytes::LinkedBytes;

use super::{DecodeError, Message};

/// Encodes an integer value into LEB128 variable length format, and writes it
/// to the buffer. The buffer must have enough remaining space (maximum 10
/// bytes).
#[inline]
pub fn encode_varint(mut value: u64, buf: &mut LinkedBytes) {
    loop {
        if value < 0x80 {
            buf.put_u8(value as u8);
            break;
        } else {
            buf.put_u8(((value & 0x7F) | 0x80) as u8);
            value >>= 7;
        }
    }
}

/// Decodes a LEB128-encoded variable length integer from the buffer.
#[inline]
pub fn decode_varint(buf: &mut Bytes) -> Result<u64, DecodeError> {
    let bytes = buf.chunk();
    let len = bytes.len();
    if len == 0 {
        return Err(DecodeError::new("invalid varint"));
    }

    let byte = bytes[0];
    if byte < 0x80 {
        buf.advance(1);
        Ok(u64::from(byte))
    } else if len > 10 || bytes[len - 1] < 0x80 {
        let (value, advance) = decode_varint_slice(bytes)?;
        buf.advance(advance);
        Ok(value)
    } else {
        decode_varint_slow(buf)
    }
}

/// Decodes a LEB128-encoded variable length integer from the slice, returning
/// the value and the number of bytes read.
///
/// Based loosely on [`ReadVarint64FromArray`][1] with a varint overflow check
/// from [`ConsumeVarint`][2].
///
/// ## Safety
///
/// The caller must ensure that `bytes` is non-empty and either `bytes.len() >=
/// 10` or the last element in bytes is < `0x80`.
///
/// [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/io/coded_stream.cc#L365-L406
/// [2]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
#[inline]
fn decode_varint_slice(bytes: &[u8]) -> Result<(u64, usize), DecodeError> {
    // Fully unrolled varint decoding loop. Splitting into 32-bit pieces gives
    // better performance.

    // Use assertions to ensure memory safety, but it should always be optimized
    // after inline.
    assert!(!bytes.is_empty());
    assert!(bytes.len() > 10 || bytes[bytes.len() - 1] < 0x80);

    let mut b: u8 = unsafe { *bytes.get_unchecked(0) };
    let mut part0: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((u64::from(part0), 1));
    };
    part0 -= 0x80;
    b = unsafe { *bytes.get_unchecked(1) };
    part0 += u32::from(b) << 7;
    if b < 0x80 {
        return Ok((u64::from(part0), 2));
    };
    part0 -= 0x80 << 7;
    b = unsafe { *bytes.get_unchecked(2) };
    part0 += u32::from(b) << 14;
    if b < 0x80 {
        return Ok((u64::from(part0), 3));
    };
    part0 -= 0x80 << 14;
    b = unsafe { *bytes.get_unchecked(3) };
    part0 += u32::from(b) << 21;
    if b < 0x80 {
        return Ok((u64::from(part0), 4));
    };
    part0 -= 0x80 << 21;
    let value = u64::from(part0);

    b = unsafe { *bytes.get_unchecked(4) };
    let mut part1: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 5));
    };
    part1 -= 0x80;
    b = unsafe { *bytes.get_unchecked(5) };
    part1 += u32::from(b) << 7;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 6));
    };
    part1 -= 0x80 << 7;
    b = unsafe { *bytes.get_unchecked(6) };
    part1 += u32::from(b) << 14;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 7));
    };
    part1 -= 0x80 << 14;
    b = unsafe { *bytes.get_unchecked(7) };
    part1 += u32::from(b) << 21;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 8));
    };
    part1 -= 0x80 << 21;
    let value = value + ((u64::from(part1)) << 28);

    b = unsafe { *bytes.get_unchecked(8) };
    let mut part2: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((value + (u64::from(part2) << 56), 9));
    };
    part2 -= 0x80;
    b = unsafe { *bytes.get_unchecked(9) };
    part2 += u32::from(b) << 7;
    // Check for u64::MAX overflow. See [`ConsumeVarint`][1] for details.
    // [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
    if b < 0x02 {
        return Ok((value + (u64::from(part2) << 56), 10));
    };

    // We have overrun the maximum size of a varint (10 bytes) or the final byte
    // caused an overflow. Assume the data is corrupt.
    Err(DecodeError::new("invalid varint"))
}

/// Decodes a LEB128-encoded variable length integer from the buffer, advancing
/// the buffer as necessary.
///
/// Contains a varint overflow check from [`ConsumeVarint`][1].
///
/// [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
#[inline(never)]
#[cold]
fn decode_varint_slow(buf: &mut Bytes) -> Result<u64, DecodeError> {
    let mut value = 0;
    for count in 0..min(10, buf.remaining()) {
        let byte = buf.get_u8();
        value |= u64::from(byte & 0x7F) << (count * 7);
        if byte <= 0x7F {
            // Check for u64::MAX overflow. See [`ConsumeVarint`][1] for details.
            // [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
            if count == 9 && byte >= 0x02 {
                return Err(DecodeError::new("invalid varint"));
            } else {
                return Ok(value);
            }
        }
    }

    Err(DecodeError::new("invalid varint"))
}

/// Additional information passed to every decode/merge function.
///
/// The context should be passed by value and can be freely cloned. When passing
/// to a function which is decoding a nested object, then use `enter_recursion`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "no-recursion-limit", derive(Default))]
pub struct DecodeContext {
    /// How many times we can recurse in the current decode stack before we hit
    /// the recursion limit.
    ///
    /// The recursion limit is defined by `RECURSION_LIMIT` and cannot be
    /// customized. The recursion limit can be ignored by building the Prost
    /// crate with the `no-recursion-limit` feature.
    #[cfg(not(feature = "no-recursion-limit"))]
    recurse_count: u32,

    raw_bytes: Bytes,
    raw_bytes_cursor: usize,
}

impl DecodeContext {
    pub fn new(raw_bytes: Bytes) -> DecodeContext {
        let raw_bytes_cursor = raw_bytes.chunk().as_ptr() as usize;
        DecodeContext {
            recurse_count: super::RECURSION_LIMIT,
            raw_bytes,
            raw_bytes_cursor,
        }
    }
    /// Call this function before recursively decoding.
    ///
    /// There is no `exit` function since this function creates a new
    /// `DecodeContext` to be used at the next level of recursion. Continue
    /// to use the old context
    // at the previous level of recursion.
    #[cfg(not(feature = "no-recursion-limit"))]
    #[inline]
    pub(crate) fn enter_recursion(&mut self) -> &mut DecodeContext {
        self.recurse_count -= 1;
        self
    }

    #[cfg(not(feature = "no-recursion-limit"))]
    #[inline]
    pub(crate) fn exit_recursion(&mut self) -> &mut DecodeContext {
        self.recurse_count += 1;
        self
    }

    #[cfg(feature = "no-recursion-limit")]
    #[inline]
    pub(crate) fn enter_recursion(&mut self) -> &mut DecodeContext {
        self
    }

    #[cfg(feature = "no-recursion-limit")]
    #[inline]
    pub(crate) fn exit_recursion(&mut self) -> &mut DecodeContext {
        self
    }

    /// Checks whether the recursion limit has been reached in the stack of
    /// decodes described by the `DecodeContext` at `self.ctx`.
    ///
    /// Returns `Ok<()>` if it is ok to continue recursing.
    /// Returns `Err<DecodeError>` if the recursion limit has been reached.
    #[cfg(not(feature = "no-recursion-limit"))]
    #[inline]
    pub(crate) fn limit_reached(&self) -> Result<(), DecodeError> {
        if self.recurse_count == 0 {
            Err(DecodeError::new("recursion limit reached"))
        } else {
            Ok(())
        }
    }

    #[cfg(feature = "no-recursion-limit")]
    #[inline]
    #[allow(clippy::unnecessary_wraps)] // needed in other features
    pub(crate) fn limit_reached(&self) -> Result<(), DecodeError> {
        Ok(())
    }

    pub fn raw_bytes_split_to(&mut self, len: usize) -> Bytes {
        let split = self.raw_bytes.split_to(len);
        self.raw_bytes_cursor += len;
        split
    }

    pub fn raw_bytes_cursor(&self) -> usize {
        self.raw_bytes_cursor
    }

    pub fn advance_raw_bytes(&mut self, n: usize) {
        self.raw_bytes.advance(n);
        self.raw_bytes_cursor += n;
    }
}

/// Returns the encoded length of the value in LEB128 variable length format.
/// The returned value will be between 1 and 10, inclusive.
#[inline]
pub fn encoded_len_varint(value: u64) -> usize {
    // Based on [VarintSize64][1].
    // [1]: https://github.com/google/protobuf/blob/3.3.x/src/google/protobuf/io/coded_stream.h#L1301-L1309
    ((((value | 1).leading_zeros() ^ 63) * 9 + 73) / 64) as usize
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WireType {
    Varint = 0,
    SixtyFourBit = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    ThirtyTwoBit = 5,
}

pub const MIN_TAG: u32 = 1;
pub const MAX_TAG: u32 = (1 << 29) - 1;

impl TryFrom<u64> for WireType {
    type Error = DecodeError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::SixtyFourBit),
            2 => Ok(WireType::LengthDelimited),
            3 => Ok(WireType::StartGroup),
            4 => Ok(WireType::EndGroup),
            5 => Ok(WireType::ThirtyTwoBit),
            _ => Err(DecodeError::new(format!(
                "invalid wire type value: {value}"
            ))),
        }
    }
}

/// Encodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline]
pub fn encode_key(tag: u32, wire_type: WireType, buf: &mut LinkedBytes) {
    debug_assert!((MIN_TAG..=MAX_TAG).contains(&tag));
    let key = (tag << 3) | wire_type as u32;
    encode_varint(u64::from(key), buf);
}

/// Decodes a Protobuf field key, which consists of a wire type designator and
/// the field tag.
#[inline(always)]
pub fn decode_key(buf: &mut Bytes) -> Result<(u32, WireType), DecodeError> {
    let key: u64 = decode_varint(buf)?;
    if key > u64::from(u32::MAX) {
        return Err(DecodeError::new(format!("invalid key value: {key}")));
    }
    let wire_type = WireType::try_from(key & 0x07)?;
    let tag = key as u32 >> 3;

    if tag < MIN_TAG {
        return Err(DecodeError::new("invalid tag value: 0"));
    }

    Ok((tag, wire_type))
}

/// Returns the width of an encoded Protobuf field key with the given tag.
/// The returned width will be between 1 and 5 bytes (inclusive).
#[inline]
pub fn key_len(tag: u32) -> usize {
    encoded_len_varint(u64::from(tag << 3))
}

/// Checks that the expected wire type matches the actual wire type,
/// or returns an error result.
#[inline]
pub fn check_wire_type(expected: WireType, actual: WireType) -> Result<(), DecodeError> {
    if expected != actual {
        return Err(DecodeError::new(format!(
            "invalid wire type: {actual:?} (expected {expected:?})",
        )));
    }
    Ok(())
}

/// Helper function which abstracts reading a length delimiter prefix followed
/// by decoding values until the length of bytes is exhausted.
pub fn merge_loop<T, M>(
    value: &mut T,
    buf: &mut Bytes,
    ctx: &mut DecodeContext,
    mut merge: M,
) -> Result<(), DecodeError>
where
    M: FnMut(&mut T, &mut Bytes, &mut DecodeContext) -> Result<(), DecodeError>,
{
    let len = decode_varint(buf)?;
    let remaining = buf.remaining();
    if len > remaining as u64 {
        return Err(DecodeError::new("buffer underflow"));
    }

    let cur = buf.chunk().as_ptr();
    let last = ctx.raw_bytes_cursor();
    ctx.advance_raw_bytes(cur as usize - last);

    let limit = remaining - len as usize;
    while buf.remaining() > limit {
        merge(value, buf, ctx)?;
    }

    if buf.remaining() != limit {
        return Err(DecodeError::new("delimited length exceeded"));
    }
    Ok(())
}

pub fn skip_field(
    wire_type: WireType,
    tag: u32,
    buf: &mut Bytes,
    ctx: &mut DecodeContext,
) -> Result<(), DecodeError> {
    ctx.limit_reached()?;
    let len = match wire_type {
        WireType::Varint => decode_varint(buf).map(|_| 0)?,
        WireType::ThirtyTwoBit => 4,
        WireType::SixtyFourBit => 8,
        WireType::LengthDelimited => decode_varint(buf)?,
        WireType::StartGroup => loop {
            let (inner_tag, inner_wire_type) = decode_key(buf)?;
            match inner_wire_type {
                WireType::EndGroup => {
                    if inner_tag != tag {
                        return Err(DecodeError::new("unexpected end group tag"));
                    }
                    break 0;
                }
                _ => {
                    ctx.enter_recursion();
                    skip_field(inner_wire_type, inner_tag, buf, ctx)?;
                    ctx.exit_recursion();
                }
            }
        },
        WireType::EndGroup => return Err(DecodeError::new("unexpected end group tag")),
    };

    if len > buf.remaining() as u64 {
        return Err(DecodeError::new("buffer underflow"));
    }

    buf.advance(len as usize);
    Ok(())
}

/// Helper macro which emits an `encode_repeated` function for the type.
macro_rules! encode_repeated {
    ($ty:ty) => {
        pub fn encode_repeated(tag: u32, values: &[$ty], buf: &mut LinkedBytes) {
            for value in values {
                encode(tag, value, buf);
            }
        }
    };
}

/// Helper macro which emits a `merge_repeated` function for the numeric type.
macro_rules! merge_repeated_numeric {
    ($ty:ty,
     $wire_type:expr_2021,
     $merge:ident,
     $merge_repeated:ident) => {
        pub fn $merge_repeated(
            wire_type: crate::pb::encoding::WireType,
            values: &mut Vec<$ty>,
            buf: &mut Bytes,
            ctx: &mut DecodeContext,
        ) -> Result<(), DecodeError> {
            if wire_type == WireType::LengthDelimited {
                // Packed.
                merge_loop(values, buf, ctx, |values, buf, ctx| {
                    let mut value = Default::default();
                    $merge($wire_type, &mut value, buf, ctx)?;
                    values.push(value);
                    Ok(())
                })
            } else {
                // Unpacked.
                check_wire_type($wire_type, wire_type)?;
                let mut value = Default::default();
                $merge(wire_type, &mut value, buf, ctx)?;
                values.push(value);
                Ok(())
            }
        }
    };
}

/// Macro which emits a module containing a set of encoding functions for a
/// variable width numeric type.
macro_rules! varint {
    ($ty:ty,
     $proto_ty:ident) => (
        varint!($ty,
                $proto_ty,
                to_uint64(value) { *value as u64 },
                from_uint64(value) { value as $ty });
    );

    ($ty:ty,
     $proto_ty:ident,
     to_uint64($to_uint64_value:ident) $to_uint64:expr_2021,
     from_uint64($from_uint64_value:ident) $from_uint64:expr_2021) => (

         pub mod $proto_ty {
            use crate::pb::encoding::*;

            pub fn encode(tag: u32, $to_uint64_value: &$ty, buf: &mut LinkedBytes) {
                encode_key(tag, WireType::Varint, buf);
                encode_varint($to_uint64, buf);
            }

            pub fn merge(wire_type: WireType, value: &mut $ty, buf: &mut Bytes, _ctx: &mut DecodeContext) -> Result<(), DecodeError> {
                check_wire_type(WireType::Varint, wire_type)?;
                let $from_uint64_value = decode_varint(buf)?;
                *value = $from_uint64;
                Ok(())
            }

            encode_repeated!($ty);

            pub fn encode_packed(tag: u32, values: &[$ty], buf: &mut LinkedBytes) {
                if values.is_empty() { return; }

                encode_key(tag, WireType::LengthDelimited, buf);
                let len: usize = values.iter().map(|$to_uint64_value| {
                    encoded_len_varint($to_uint64)
                }).sum();
                encode_varint(len as u64, buf);

                for $to_uint64_value in values {
                    encode_varint($to_uint64, buf);
                }
            }

            merge_repeated_numeric!($ty, WireType::Varint, merge, merge_repeated);

            #[inline]
            pub fn encoded_len(tag: u32, $to_uint64_value: &$ty) -> usize {
                key_len(tag) + encoded_len_varint($to_uint64)
            }

            #[inline]
            pub fn encoded_len_repeated(tag: u32, values: &[$ty]) -> usize {
                key_len(tag) * values.len() + values.iter().map(|$to_uint64_value| {
                    encoded_len_varint($to_uint64)
                }).sum::<usize>()
            }

            #[inline]
            pub fn encoded_len_packed(tag: u32, values: &[$ty]) -> usize {
                if values.is_empty() {
                    0
                } else {
                    let len = values.iter()
                                    .map(|$to_uint64_value| encoded_len_varint($to_uint64))
                                    .sum::<usize>();
                    key_len(tag) + encoded_len_varint(len as u64) + len
                }
            }

            #[cfg(test)]
            mod test {
                use proptest::prelude::*;

                use crate::pb::encoding::$proto_ty::*;
                use crate::pb::encoding::test::{
                    check_collection_type,
                    check_type,
                };

                proptest! {
                    #[test]
                    fn check(value: $ty, tag in MIN_TAG..=MAX_TAG) {
                        check_type(value, tag, WireType::Varint,
                                   encode, merge, encoded_len)?;
                    }
                    #[test]
                    fn check_repeated(value: Vec<$ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_collection_type(value, tag, WireType::Varint,
                                              encode_repeated, merge_repeated,
                                              encoded_len_repeated)?;
                    }
                    #[test]
                    fn check_packed(value: Vec<$ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_collection_type(value, tag, WireType::LengthDelimited,
                                   encode_packed, merge_repeated,
                                   encoded_len_packed)?;
                    }
                }
            }
         }

    );
}
varint!(bool, bool,
        to_uint64(value) u64::from(*value),
        from_uint64(value) value != 0);
// varint!(i32, int32);

pub mod int32 {

    use crate::pb::encoding::*;

    pub fn encode<T: Into<i32> + Copy>(tag: u32, value: &T, buf: &mut LinkedBytes) {
        let value: i32 = (*value).into();
        encode_key(tag, WireType::Varint, buf);
        encode_varint(value as u64, buf);
    }

    pub fn merge<T: TryFrom<i32>>(
        wire_type: WireType,
        value: &mut T,
        buf: &mut Bytes,
        _ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError>
    where
        T::Error: Into<DecodeError>,
    {
        *value = inner_merge(wire_type, buf, _ctx)?;
        Ok(())
    }

    pub fn inner_merge<T: TryFrom<i32>>(
        wire_type: WireType,
        buf: &mut Bytes,
        _ctx: &mut DecodeContext,
    ) -> Result<T, DecodeError>
    where
        T::Error: Into<DecodeError>,
    {
        check_wire_type(WireType::Varint, wire_type)?;
        let from_value = decode_varint(buf)?;
        T::try_from(from_value as i32).map_err(|err| err.into())
    }

    pub fn encode_repeated<T: Into<i32> + Copy>(tag: u32, values: &[T], buf: &mut LinkedBytes) {
        for value in values {
            encode(tag, value, buf);
        }
    }

    pub fn encode_packed(tag: u32, values: &[i32], buf: &mut LinkedBytes) {
        if values.is_empty() {
            return;
        }

        encode_key(tag, WireType::LengthDelimited, buf);
        let len: usize = values
            .iter()
            .map(|value| encoded_len_varint(*value as u64))
            .sum();
        encode_varint(len as u64, buf);

        for value in values {
            encode_varint(*value as u64, buf);
        }
    }

    pub fn merge_repeated<T: TryFrom<i32>>(
        wire_type: crate::pb::encoding::WireType,
        values: &mut Vec<T>,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError>
    where
        T::Error: Into<crate::pb::DecodeError>,
    {
        if wire_type == WireType::LengthDelimited {
            // Packed.
            merge_loop(values, buf, ctx, |values, buf, ctx| {
                values.push(inner_merge(WireType::Varint, buf, ctx)?);
                Ok(())
            })
        } else {
            // Unpacked.
            check_wire_type(WireType::Varint, wire_type)?;
            values.push(inner_merge(wire_type, buf, ctx)?);
            Ok(())
        }
    }

    #[inline]
    pub fn encoded_len<T: Into<i32> + Copy>(tag: u32, value: &T) -> usize {
        key_len(tag) + encoded_len_varint((*value).into() as u64)
    }

    #[inline]
    pub fn encoded_len_repeated<V: Into<i32> + Copy>(tag: u32, values: &[V]) -> usize {
        key_len(tag) * values.len()
            + values
                .iter()
                .map(|value| encoded_len_varint((*value).into() as u64))
                .sum::<usize>()
    }

    #[inline]
    pub fn encoded_len_packed(tag: u32, values: &[i32]) -> usize {
        if values.is_empty() {
            0
        } else {
            let len = values
                .iter()
                .map(|value| encoded_len_varint(*value as u64))
                .sum::<usize>();
            key_len(tag) + encoded_len_varint(len as u64) + len
        }
    }
}
varint!(i64, int64);
varint!(u32, uint32);
varint!(u64, uint64);
varint!(i32, sint32,
to_uint64(value) {
    ((value << 1) ^ (value >> 31)) as u32 as u64
},
from_uint64(value) {
    let value = value as u32;
    ((value >> 1) as i32) ^ (-((value & 1) as i32))
});
varint!(i64, sint64,
to_uint64(value) {
    ((value << 1) ^ (value >> 63)) as u64
},
from_uint64(value) {
    ((value >> 1) as i64) ^ (-((value & 1) as i64))
});

/// Macro which emits a module containing a set of encoding functions for a
/// fixed width numeric type.
macro_rules! fixed_width {
    ($ty:ty,
     $width:expr_2021,
     $wire_type:expr_2021,
     $proto_ty:ident,
     $put:ident,
     $get:ident) => {
        pub mod $proto_ty {
            use crate::pb::encoding::*;

            pub fn encode(tag: u32, value: &$ty, buf: &mut LinkedBytes) {
                encode_key(tag, $wire_type, buf);
                buf.$put(*value);
            }

            pub fn merge(
                wire_type: WireType,
                value: &mut $ty,
                buf: &mut Bytes,
                _ctx: &mut DecodeContext,
            ) -> Result<(), DecodeError> {
                check_wire_type($wire_type, wire_type)?;
                if buf.remaining() < $width {
                    return Err(DecodeError::new("buffer underflow"));
                }
                *value = buf.$get();
                Ok(())
            }

            encode_repeated!($ty);

            pub fn encode_packed(tag: u32, values: &[$ty], buf: &mut LinkedBytes) {
                if values.is_empty() {
                    return;
                }

                encode_key(tag, WireType::LengthDelimited, buf);
                let len = values.len() as u64 * $width;
                encode_varint(len as u64, buf);

                for value in values {
                    buf.$put(*value);
                }
            }

            merge_repeated_numeric!($ty, $wire_type, merge, merge_repeated);

            #[inline]
            pub fn encoded_len(tag: u32, _: &$ty) -> usize {
                key_len(tag) + $width
            }

            #[inline]
            pub fn encoded_len_repeated(tag: u32, values: &[$ty]) -> usize {
                (key_len(tag) + $width) * values.len()
            }

            #[inline]
            pub fn encoded_len_packed(tag: u32, values: &[$ty]) -> usize {
                if values.is_empty() {
                    0
                } else {
                    let len = $width * values.len();
                    key_len(tag) + encoded_len_varint(len as u64) + len
                }
            }

            #[cfg(test)]
            mod test {
                use proptest::prelude::*;

                use super::{
                    super::test::{check_collection_type, check_type},
                    *,
                };

                proptest! {
                    #[test]
                    fn check(value: $ty, tag in MIN_TAG..=MAX_TAG) {
                        check_type(value, tag, $wire_type,
                                   encode, merge, encoded_len)?;
                    }
                    #[test]
                    fn check_repeated(value: Vec<$ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_collection_type(value, tag, $wire_type,
                                              encode_repeated, merge_repeated,
                                              encoded_len_repeated)?;
                    }
                    #[test]
                    fn check_packed(value: Vec<$ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_collection_type(value, tag, WireType::LengthDelimited,
                                   encode_packed, merge_repeated,
                                   encoded_len_packed)?;
                    }
                }
            }
        }
    };
}
fixed_width!(
    f32,
    4,
    WireType::ThirtyTwoBit,
    float,
    put_f32_le,
    get_f32_le
);
fixed_width!(
    f64,
    8,
    WireType::SixtyFourBit,
    double,
    put_f64_le,
    get_f64_le
);
fixed_width!(
    u32,
    4,
    WireType::ThirtyTwoBit,
    fixed32,
    put_u32_le,
    get_u32_le
);
fixed_width!(
    u64,
    8,
    WireType::SixtyFourBit,
    fixed64,
    put_u64_le,
    get_u64_le
);
fixed_width!(
    i32,
    4,
    WireType::ThirtyTwoBit,
    sfixed32,
    put_i32_le,
    get_i32_le
);
fixed_width!(
    i64,
    8,
    WireType::SixtyFourBit,
    sfixed64,
    put_i64_le,
    get_i64_le
);

/// Macro which emits encoding functions for a length-delimited type.
macro_rules! length_delimited {
    ($ty:ty) => {
        encode_repeated!($ty);

        pub fn merge_repeated(
            wire_type: WireType,
            values: &mut Vec<$ty>,
            buf: &mut Bytes,
            ctx: &mut DecodeContext,
        ) -> Result<(), DecodeError> {
            check_wire_type(WireType::LengthDelimited, wire_type)?;
            let mut value = Default::default();
            merge(wire_type, &mut value, buf, ctx)?;
            values.push(value);
            Ok(())
        }

        #[inline]
        pub fn encoded_len(tag: u32, value: &$ty) -> usize {
            key_len(tag) + encoded_len_varint(value.len() as u64) + value.len()
        }

        #[inline]
        pub fn encoded_len_repeated(tag: u32, values: &[$ty]) -> usize {
            key_len(tag) * values.len()
                + values
                    .iter()
                    .map(|value| encoded_len_varint(value.len() as u64) + value.len())
                    .sum::<usize>()
        }
    };
}

pub mod string {
    use std::borrow::Borrow;

    use super::*;

    pub fn encode<T: Borrow<str>>(tag: u32, value: &T, buf: &mut LinkedBytes) {
        let value = value.borrow();
        encode_key(tag, WireType::LengthDelimited, buf);
        encode_varint(value.len() as u64, buf);
        buf.put_slice(value.as_bytes());
    }
    pub fn merge<S: From<String>>(
        wire_type: WireType,
        value: &mut S,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        // ## Unsafety
        //
        // `string::merge` reuses `bytes::merge`, with an additional check of utf-8
        // well-formedness. If the utf-8 is not well-formed, or if any other error
        // occurs, then the string is cleared, so as to avoid leaking a string
        // field with invalid data.
        //
        // This implementation uses the unsafe `String::as_mut_vec` method instead of
        // the safe alternative of temporarily swapping an empty `String` into
        // the field, because it results in up to 10% better performance on the
        // protobuf message decoding benchmarks.
        //
        // It's required when using `String::as_mut_vec` that invalid utf-8 data not be
        // leaked into the backing `String`. To enforce this, even in the event
        // of a panic in `bytes::merge` or in the buf implementation, a drop
        // guard is used.
        unsafe {
            struct DropGuard<'a>(&'a mut Vec<u8>);
            impl<'a> Drop for DropGuard<'a> {
                #[inline]
                fn drop(&mut self) {
                    self.0.clear();
                }
            }

            let mut empty = String::new();

            let drop_guard = DropGuard(empty.as_mut_vec());
            bytes::merge(wire_type, drop_guard.0, buf, ctx)?;
            match str::from_utf8(drop_guard.0) {
                Ok(_) => {
                    // Success; do not clear the bytes.
                    mem::forget(drop_guard);
                    *value = S::from(empty);
                    Ok(())
                }
                Err(_) => Err(DecodeError::new(
                    "invalid string value: data is not UTF-8 encoded",
                )),
            }
        }
    }

    pub fn encode_repeated<T: Borrow<str>>(tag: u32, values: &[T], buf: &mut LinkedBytes) {
        for value in values {
            encode(tag, value, buf);
        }
    }

    pub fn merge_repeated<T: From<String>>(
        wire_type: WireType,
        values: &mut Vec<T>,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        let mut value = Default::default();
        merge(wire_type, &mut value, buf, ctx)?;
        values.push(T::from(value));
        Ok(())
    }

    #[inline]
    pub fn encoded_len<T: Borrow<str>>(tag: u32, value: &T) -> usize {
        let value = value.borrow();
        key_len(tag) + encoded_len_varint(value.len() as u64) + value.len()
    }

    #[inline]
    pub fn encoded_len_repeated<T: Borrow<str>>(tag: u32, values: &[T]) -> usize {
        key_len(tag) * values.len()
            + values
                .iter()
                .map(|value| {
                    let value = value.borrow();
                    encoded_len_varint(value.len() as u64) + value.len()
                })
                .sum::<usize>()
    }

    #[cfg(test)]
    mod test {
        use proptest::prelude::*;

        use super::{
            super::test::{check_collection_type, check_type},
            *,
        };

        proptest! {
            #[test]
            fn check(value: String, tag in MIN_TAG..=MAX_TAG) {
                super::test::check_type(value, tag, WireType::LengthDelimited,
                                        encode, merge, encoded_len::<String>)?;
            }
            #[test]
            fn check_repeated(value: Vec<String>, tag in MIN_TAG..=MAX_TAG) {
                super::test::check_collection_type(value, tag, WireType::LengthDelimited,
                                                   encode_repeated, merge_repeated,
                                                   encoded_len_repeated)?;
            }
        }
    }
}

pub mod faststr {
    use std::borrow::Borrow;

    use ::faststr::FastStr;

    use super::*;

    pub fn encode<T: Borrow<str>>(tag: u32, value: &T, buf: &mut LinkedBytes) {
        let value = value.borrow();
        encode_key(tag, WireType::LengthDelimited, buf);
        encode_varint(value.len() as u64, buf);
        buf.put_slice(value.as_bytes());
    }
    pub fn merge(
        wire_type: WireType,
        value: &mut FastStr,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut bytes = Bytes::new();

        bytes::merge(wire_type, &mut bytes, buf, ctx)?;
        *value = unsafe { FastStr::from_bytes_unchecked(bytes) };
        Ok(())
    }

    pub fn encode_repeated<T: Borrow<str>>(tag: u32, values: &[T], buf: &mut LinkedBytes) {
        for value in values {
            encode(tag, value, buf);
        }
    }

    pub fn merge_repeated(
        wire_type: WireType,
        values: &mut Vec<FastStr>,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        let mut value = Default::default();
        merge(wire_type, &mut value, buf, ctx)?;
        values.push(value);
        Ok(())
    }

    #[inline]
    pub fn encoded_len<T: Borrow<str>>(tag: u32, value: &T) -> usize {
        let value = value.borrow();
        key_len(tag) + encoded_len_varint(value.len() as u64) + value.len()
    }

    #[inline]
    pub fn encoded_len_repeated<T: Borrow<str>>(tag: u32, values: &[T]) -> usize {
        key_len(tag) * values.len()
            + values
                .iter()
                .map(|value| {
                    let value = value.borrow();
                    encoded_len_varint(value.len() as u64) + value.len()
                })
                .sum::<usize>()
    }
}

pub trait BytesAdapter: sealed::BytesAdapter {}

mod sealed {
    use bytes::Bytes;
    use linkedbytes::LinkedBytes;

    pub trait BytesAdapter: Default + Sized + 'static {
        fn len(&self) -> usize;

        /// Replace contents of this buffer with the contents of another buffer.
        fn replace_with(&mut self, buf: Bytes);

        /// Appends this buffer to the (contents of) other buffer.
        fn append_to(&self, buf: &mut LinkedBytes);

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }
}

impl BytesAdapter for Bytes {}

impl sealed::BytesAdapter for Bytes {
    #[inline]
    fn len(&self) -> usize {
        Buf::remaining(self)
    }

    #[inline]
    fn replace_with(&mut self, buf: Bytes) {
        *self = buf;
    }

    #[inline]
    fn append_to(&self, buf: &mut LinkedBytes) {
        buf.put(self.clone())
    }
}

impl BytesAdapter for Vec<u8> {}

impl sealed::BytesAdapter for Vec<u8> {
    #[inline]
    fn len(&self) -> usize {
        Vec::len(self)
    }

    #[inline]
    fn replace_with(&mut self, buf: Bytes) {
        self.clear();
        self.reserve(buf.remaining());
        self.put(buf);
    }

    #[inline]
    fn append_to(&self, buf: &mut LinkedBytes) {
        buf.put(self.as_slice())
    }
}

pub mod bytes {
    use super::*;

    pub fn encode<A>(tag: u32, value: &A, buf: &mut LinkedBytes)
    where
        A: BytesAdapter,
    {
        encode_key(tag, WireType::LengthDelimited, buf);
        encode_varint(value.len() as u64, buf);
        value.append_to(buf);
    }

    pub fn merge<A>(
        wire_type: WireType,
        value: &mut A,
        buf: &mut Bytes,
        _ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError>
    where
        A: BytesAdapter,
    {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        let len = decode_varint(buf)?;
        if len > buf.remaining() as u64 {
            return Err(DecodeError::new("buffer underflow"));
        }
        let len = len as usize;

        // Clear the existing value. This follows from the following rule in the
        // encoding guide[1]:
        //
        // > Normally, an encoded message would never have more than one instance of a
        // > non-repeated
        // > field. However, parsers are expected to handle the case in which they do.
        // > For numeric
        // > types and strings, if the same field appears multiple times, the parser
        // > accepts the
        // > last value it sees.
        //
        // [1]: https://developers.google.com/protocol-buffers/docs/encoding#optional
        //
        // This is intended for A being Bytes so it is zero-copy.
        // Some combinations of A being other BytesAdapter types may cause one copy.
        value.replace_with(buf.split_to(len));
        Ok(())
    }

    length_delimited!(impl BytesAdapter);

    #[cfg(test)]
    mod test {
        use proptest::prelude::*;

        use super::{
            super::test::{check_collection_type, check_type},
            *,
        };

        proptest! {
            #[test]
            fn check_vec(value: Vec<u8>, tag in MIN_TAG..=MAX_TAG) {
                super::test::check_type::<Vec<u8>>(value, tag, WireType::LengthDelimited,
                                                            encode, merge, encoded_len)?;
            }

            #[test]
            fn check_bytes(value: Vec<u8>, tag in MIN_TAG..=MAX_TAG) {
                let value = Bytes::from(value);
                super::test::check_type::<Bytes>(value, tag, WireType::LengthDelimited,
                                                        encode, merge, encoded_len)?;
            }

            #[test]
            fn check_repeated_vec(value: Vec<Vec<u8>>, tag in MIN_TAG..=MAX_TAG) {
                super::test::check_collection_type(value, tag, WireType::LengthDelimited,
                                                   encode_repeated, merge_repeated,
                                                   encoded_len_repeated)?;
            }

            #[test]
            fn check_repeated_bytes(value: Vec<Vec<u8>>, tag in MIN_TAG..=MAX_TAG) {
                let value = value.into_iter().map(Bytes::from).collect();
                super::test::check_collection_type(value, tag, WireType::LengthDelimited,
                                                   encode_repeated, merge_repeated,
                                                   encoded_len_repeated)?;
            }
        }
    }
}

pub mod message {
    use super::*;

    pub fn encode<M>(tag: u32, msg: &M, buf: &mut LinkedBytes)
    where
        M: Message,
    {
        encode_key(tag, WireType::LengthDelimited, buf);
        encode_varint(msg.encoded_len() as u64, buf);
        msg.encode_raw(buf);
    }

    pub fn merge<M>(
        wire_type: WireType,
        msg: &mut M,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message,
    {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        ctx.limit_reached()?;
        ctx.enter_recursion();
        merge_loop(
            msg,
            buf,
            ctx,
            |msg: &mut M, buf: &mut Bytes, ctx: &mut DecodeContext| {
                let (tag, wire_type) = decode_key(buf)?;
                msg.merge_field(tag, wire_type, buf, ctx)
            },
        )?;
        ctx.exit_recursion();
        Ok(())
    }

    pub fn encode_repeated<M>(tag: u32, messages: &[M], buf: &mut LinkedBytes)
    where
        M: Message,
    {
        for msg in messages {
            encode(tag, msg, buf);
        }
    }

    pub fn merge_repeated<M>(
        wire_type: WireType,
        messages: &mut Vec<M>,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message + Default,
    {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        let mut msg = M::default();
        merge(WireType::LengthDelimited, &mut msg, buf, ctx)?;
        messages.push(msg);
        Ok(())
    }

    #[inline]
    pub fn encoded_len<M>(tag: u32, msg: &M) -> usize
    where
        M: Message,
    {
        let len = msg.encoded_len();
        key_len(tag) + encoded_len_varint(len as u64) + len
    }

    #[inline]
    pub fn encoded_len_repeated<M>(tag: u32, messages: &[M]) -> usize
    where
        M: Message,
    {
        key_len(tag) * messages.len()
            + messages
                .iter()
                .map(Message::encoded_len)
                .map(|len| len + encoded_len_varint(len as u64))
                .sum::<usize>()
    }
}

pub mod arc_message {
    use std::sync::Arc;

    use super::*;

    pub fn encode<M>(tag: u32, msg: &Arc<M>, buf: &mut LinkedBytes)
    where
        M: Message,
    {
        message::encode(tag, msg.as_ref(), buf)
    }

    pub fn merge<M>(
        wire_type: WireType,
        value: &mut Arc<M>,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message + Default,
    {
        let mut msg = M::default();
        message::merge(wire_type, &mut msg, buf, ctx)?;
        *value = Arc::new(msg);
        Ok(())
    }

    pub fn encode_repeated<M>(tag: u32, messages: &[Arc<M>], buf: &mut LinkedBytes)
    where
        M: Message,
    {
        for msg in messages {
            encode(tag, msg, buf);
        }
    }

    pub fn merge_repeated<M>(
        wire_type: WireType,
        messages: &mut Vec<Arc<M>>,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message + Default,
    {
        check_wire_type(WireType::LengthDelimited, wire_type)?;
        let mut msg = M::default();
        message::merge(wire_type, &mut msg, buf, ctx)?;
        messages.push(Arc::new(msg));
        Ok(())
    }

    #[inline]
    pub fn encoded_len<M>(tag: u32, msg: &Arc<M>) -> usize
    where
        M: Message,
    {
        message::encoded_len(tag, msg.as_ref())
    }

    #[inline]
    pub fn encoded_len_repeated<M>(tag: u32, messages: &[Arc<M>]) -> usize
    where
        M: Message,
    {
        key_len(tag) * messages.len()
            + messages
                .iter()
                .map(|msg| {
                    let len = msg.encoded_len();
                    encoded_len_varint(len as u64) + len
                })
                .sum::<usize>()
    }
}

pub mod group {
    use super::*;

    pub fn encode<M>(tag: u32, msg: &M, buf: &mut LinkedBytes)
    where
        M: Message,
    {
        encode_key(tag, WireType::StartGroup, buf);
        msg.encode_raw(buf);
        encode_key(tag, WireType::EndGroup, buf);
    }

    pub fn merge<M>(
        tag: u32,
        wire_type: WireType,
        msg: &mut M,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message,
    {
        check_wire_type(WireType::StartGroup, wire_type)?;

        ctx.limit_reached()?;
        loop {
            let (field_tag, field_wire_type) = decode_key(buf)?;
            if field_wire_type == WireType::EndGroup {
                if field_tag != tag {
                    return Err(DecodeError::new("unexpected end group tag"));
                }
                return Ok(());
            }

            ctx.enter_recursion();
            M::merge_field(msg, field_tag, field_wire_type, buf, ctx)?;
            ctx.exit_recursion();
        }
    }

    pub fn encode_repeated<M>(tag: u32, messages: &[M], buf: &mut LinkedBytes)
    where
        M: Message,
    {
        for msg in messages {
            encode(tag, msg, buf);
        }
    }

    pub fn merge_repeated<M>(
        tag: u32,
        wire_type: WireType,
        messages: &mut Vec<M>,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError>
    where
        M: Message + Default,
    {
        check_wire_type(WireType::StartGroup, wire_type)?;
        let mut msg = M::default();
        merge(tag, WireType::StartGroup, &mut msg, buf, ctx)?;
        messages.push(msg);
        Ok(())
    }

    #[inline]
    pub fn encoded_len<M>(tag: u32, msg: &M) -> usize
    where
        M: Message,
    {
        2 * key_len(tag) + msg.encoded_len()
    }

    #[inline]
    pub fn encoded_len_repeated<M>(tag: u32, messages: &[M]) -> usize
    where
        M: Message,
    {
        2 * key_len(tag) * messages.len() + messages.iter().map(Message::encoded_len).sum::<usize>()
    }
}

/// Rust doesn't have a `Map` trait, so macros are currently the best way to be
/// generic over `HashMap` and `BTreeMap`.
macro_rules! map {
    ($map_ty:ident) => {
        use core::hash::Hash;

        use crate::pb::encoding::*;

        /// Generic protobuf map encode function.
        pub fn encode<K, V, KE, KL, VE, VL>(
            key_encode: KE,
            key_encoded_len: KL,
            val_encode: VE,
            val_encoded_len: VL,
            tag: u32,
            values: &$map_ty<K, V>,
            buf: &mut LinkedBytes,
        ) where
            K: Default + Eq + Hash + Ord,
            V: Default + PartialEq,
            KE: Fn(u32, &K, &mut LinkedBytes),
            KL: Fn(u32, &K) -> usize,
            VE: Fn(u32, &V, &mut LinkedBytes),
            VL: Fn(u32, &V) -> usize,
        {
            encode_with_default(
                key_encode,
                key_encoded_len,
                val_encode,
                val_encoded_len,
                &V::default(),
                tag,
                values,
                buf,
            )
        }

        /// Generic protobuf map merge function.
        pub fn merge<K, V, KM, VM>(
            key_merge: KM,
            val_merge: VM,
            values: &mut $map_ty<K, V>,
            buf: &mut Bytes,
            ctx: &mut DecodeContext,
        ) -> Result<(), DecodeError>
        where
            K: Default + Eq + Hash + Ord,
            V: Default,
            KM: Fn(WireType, &mut K, &mut Bytes, &mut DecodeContext) -> Result<(), DecodeError>,
            VM: Fn(WireType, &mut V, &mut Bytes, &mut DecodeContext) -> Result<(), DecodeError>,
        {
            merge_with_default(key_merge, val_merge, V::default(), values, buf, ctx)
        }

        /// Generic protobuf map encode function.
        pub fn encoded_len<K, V, KL, VL>(
            key_encoded_len: KL,
            val_encoded_len: VL,
            tag: u32,
            values: &$map_ty<K, V>,
        ) -> usize
        where
            K: Default + Eq + Hash + Ord,
            V: Default + PartialEq,
            KL: Fn(u32, &K) -> usize,
            VL: Fn(u32, &V) -> usize,
        {
            encoded_len_with_default(key_encoded_len, val_encoded_len, &V::default(), tag, values)
        }

        /// Generic protobuf map encode function with an overridden value default.
        ///
        /// This is necessary because enumeration values can have a default value other
        /// than 0 in proto2.
        #[allow(clippy::too_many_arguments)]
        pub fn encode_with_default<K, V, KE, KL, VE, VL>(
            key_encode: KE,
            key_encoded_len: KL,
            val_encode: VE,
            val_encoded_len: VL,
            val_default: &V,
            tag: u32,
            values: &$map_ty<K, V>,
            buf: &mut LinkedBytes,
        ) where
            K: Default + Eq + Hash + Ord,
            V: PartialEq,
            KE: Fn(u32, &K, &mut LinkedBytes),
            KL: Fn(u32, &K) -> usize,
            VE: Fn(u32, &V, &mut LinkedBytes),
            VL: Fn(u32, &V) -> usize,
        {
            for (key, val) in values.iter() {
                let skip_key = !cfg!(feature = "pb-encode-default-value") && key == &K::default();
                let skip_val = !cfg!(feature = "pb-encode-default-value") && val == val_default;

                let len = (if skip_key { 0 } else { key_encoded_len(1, key) })
                    + (if skip_val { 0 } else { val_encoded_len(2, val) });

                encode_key(tag, WireType::LengthDelimited, buf);
                encode_varint(len as u64, buf);
                if !skip_key {
                    key_encode(1, key, buf);
                }
                if !skip_val {
                    val_encode(2, val, buf);
                }
            }
        }

        /// Generic protobuf map merge function with an overridden value default.
        ///
        /// This is necessary because enumeration values can have a default value other
        /// than 0 in proto2.
        pub fn merge_with_default<K, V, KM, VM>(
            key_merge: KM,
            val_merge: VM,
            val_default: V,
            values: &mut $map_ty<K, V>,
            buf: &mut Bytes,
            ctx: &mut DecodeContext,
        ) -> Result<(), DecodeError>
        where
            K: Default + Eq + Hash + Ord,
            KM: Fn(WireType, &mut K, &mut Bytes, &mut DecodeContext) -> Result<(), DecodeError>,
            VM: Fn(WireType, &mut V, &mut Bytes, &mut DecodeContext) -> Result<(), DecodeError>,
        {
            let mut key = Default::default();
            let mut val = val_default;
            ctx.limit_reached()?;
            ctx.enter_recursion();
            merge_loop(
                &mut (&mut key, &mut val),
                buf,
                ctx,
                |&mut (ref mut key, ref mut val), buf, ctx| {
                    let (tag, wire_type) = decode_key(buf)?;
                    match tag {
                        1 => key_merge(wire_type, key, buf, ctx),
                        2 => val_merge(wire_type, val, buf, ctx),
                        _ => skip_field(wire_type, tag, buf, ctx),
                    }
                },
            )?;
            ctx.exit_recursion();
            values.insert(key, val);

            Ok(())
        }

        /// Generic protobuf map encode function with an overridden value default.
        ///
        /// This is necessary because enumeration values can have a default value other
        /// than 0 in proto2.
        pub fn encoded_len_with_default<K, V, KL, VL>(
            key_encoded_len: KL,
            val_encoded_len: VL,
            val_default: &V,
            tag: u32,
            values: &$map_ty<K, V>,
        ) -> usize
        where
            K: Default + Eq + Hash + Ord,
            V: PartialEq,
            KL: Fn(u32, &K) -> usize,
            VL: Fn(u32, &V) -> usize,
        {
            let skip_default_value = !cfg!(feature = "pb-encode-default-value");

            key_len(tag) * values.len()
                + values
                    .iter()
                    .map(|(key, val)| {
                        let len = (if key == &K::default() && skip_default_value {
                            0
                        } else {
                            key_encoded_len(1, key)
                        }) + (if val == val_default && skip_default_value {
                            0
                        } else {
                            val_encoded_len(2, val)
                        });
                        encoded_len_varint(len as u64) + len
                    })
                    .sum::<usize>()
        }
    };
}

pub mod hash_map {
    use crate::AHashMap;
    map!(AHashMap);
}

pub mod btree_map {
    map!(BTreeMap);
}

#[cfg(test)]
mod test {
    use alloc::string::ToString;
    use core::{fmt::Debug, u64};

    use ::bytes::Bytes;
    use proptest::{prelude::*, test_runner::TestCaseResult};

    use crate::pb::encoding::*;

    pub fn check_type<T>(
        value: T,
        tag: u32,
        wire_type: WireType,
        encode: fn(u32, &T, &mut LinkedBytes),
        merge: fn(WireType, &mut T, &mut Bytes, &mut DecodeContext) -> Result<(), DecodeError>,
        encoded_len: fn(u32, &T) -> usize,
    ) -> TestCaseResult
    where
        T: Debug + Default + PartialEq,
    {
        prop_assume!((MIN_TAG..=MAX_TAG).contains(&tag));

        let expected_len = encoded_len(tag, &value);

        let mut buf = LinkedBytes::with_capacity(expected_len);
        encode(tag, &value, &mut buf);

        let mut buf = buf.bytes().clone().freeze();

        prop_assert_eq!(
            buf.remaining(),
            expected_len,
            "encoded_len wrong; expected: {}, actual: {}",
            expected_len,
            buf.remaining()
        );

        if !buf.has_remaining() {
            // Short circuit for empty packed values.
            return Ok(());
        }

        let (decoded_tag, decoded_wire_type) =
            decode_key(&mut buf).map_err(|error| TestCaseError::fail(error.to_string()))?;
        prop_assert_eq!(
            tag,
            decoded_tag,
            "decoded tag does not match; expected: {}, actual: {}",
            tag,
            decoded_tag
        );

        prop_assert_eq!(
            wire_type,
            decoded_wire_type,
            "decoded wire type does not match; expected: {:?}, actual: {:?}",
            wire_type,
            decoded_wire_type,
        );

        match wire_type {
            WireType::SixtyFourBit if buf.remaining() != 8 => Err(TestCaseError::fail(format!(
                "64bit wire type illegal remaining: {}, tag: {}",
                buf.remaining(),
                tag
            ))),
            WireType::ThirtyTwoBit if buf.remaining() != 4 => Err(TestCaseError::fail(format!(
                "32bit wire type illegal remaining: {}, tag: {}",
                buf.remaining(),
                tag
            ))),
            _ => Ok(()),
        }?;

        let mut roundtrip_value = T::default();
        let mut ctx = DecodeContext::new(buf.clone());
        merge(wire_type, &mut roundtrip_value, &mut buf, &mut ctx)
            .map_err(|error| TestCaseError::fail(error.to_string()))?;

        prop_assert!(
            !buf.has_remaining(),
            "expected buffer to be empty, remaining: {}",
            buf.remaining()
        );

        prop_assert_eq!(value, roundtrip_value);

        Ok(())
    }

    pub fn check_collection_type<T, Item, E, M, L>(
        value: T,
        tag: u32,
        wire_type: WireType,
        encode: E,
        mut merge: M,
        encoded_len: L,
    ) -> TestCaseResult
    where
        T: Debug + Default + PartialEq + AsRef<[Item]>,
        E: FnOnce(u32, &[Item], &mut LinkedBytes),
        M: FnMut(WireType, &mut T, &mut Bytes, &mut DecodeContext) -> Result<(), DecodeError>,
        L: FnOnce(u32, &[Item]) -> usize,
    {
        prop_assume!((MIN_TAG..=MAX_TAG).contains(&tag));

        let expected_len = encoded_len(tag, value.as_ref());

        let mut buf = LinkedBytes::with_capacity(expected_len);
        encode(tag, value.as_ref(), &mut buf);

        let mut buf = buf.bytes().clone().freeze();

        prop_assert_eq!(
            buf.remaining(),
            expected_len,
            "encoded_len wrong; expected: {}, actual: {}",
            expected_len,
            buf.remaining()
        );

        let mut roundtrip_value = Default::default();
        while buf.has_remaining() {
            let (decoded_tag, decoded_wire_type) =
                decode_key(&mut buf).map_err(|error| TestCaseError::fail(error.to_string()))?;

            prop_assert_eq!(
                tag,
                decoded_tag,
                "decoded tag does not match; expected: {}, actual: {}",
                tag,
                decoded_tag
            );

            prop_assert_eq!(
                wire_type,
                decoded_wire_type,
                "decoded wire type does not match; expected: {:?}, actual: {:?}",
                wire_type,
                decoded_wire_type
            );

            let mut ctx = DecodeContext::new(buf.clone());
            merge(wire_type, &mut roundtrip_value, &mut buf, &mut ctx)
                .map_err(|error| TestCaseError::fail(error.to_string()))?;
        }

        prop_assert_eq!(value, roundtrip_value);

        Ok(())
    }

    #[test]
    fn string_merge_invalid_utf8() {
        let mut s = String::new();
        let mut buf = Bytes::from_static(b"\x02\x80\x80");

        let mut ctx = DecodeContext::new(buf.clone());
        let r = string::merge(WireType::LengthDelimited, &mut s, &mut buf, &mut ctx);
        r.expect_err("must be an error");
        assert!(s.is_empty());
    }

    #[test]
    fn varint() {
        fn check(value: u64, encoded: &mut Bytes) {
            // Small buffer.
            let mut buf = LinkedBytes::with_capacity(1);
            encode_varint(value, &mut buf);
            assert_eq!(*buf.bytes(), *encoded);

            // Large buffer.
            let mut buf = LinkedBytes::with_capacity(100);
            encode_varint(value, &mut buf);
            assert_eq!(*buf.bytes(), *encoded);

            assert_eq!(encoded_len_varint(value), encoded.len());

            let roundtrip_value = decode_varint(&mut encoded.clone()).expect("decoding failed");
            assert_eq!(value, roundtrip_value);

            let roundtrip_value = decode_varint_slow(encoded).expect("slow decoding failed");
            assert_eq!(value, roundtrip_value);
        }

        check(2u64.pow(0) - 1, &mut Bytes::from_static(b"\x00"));
        check(2u64.pow(0), &mut Bytes::from_static(b"\x01"));

        check(2u64.pow(7) - 1, &mut Bytes::from_static(b"\x7F"));
        check(2u64.pow(7), &mut Bytes::from_static(b"\x80\x01"));
        check(300, &mut Bytes::from_static(b"\xAC\x02"));

        check(2u64.pow(14) - 1, &mut Bytes::from_static(b"\xFF\x7F"));
        check(2u64.pow(14), &mut Bytes::from_static(b"\x80\x80\x01"));

        check(2u64.pow(21) - 1, &mut Bytes::from_static(b"\xFF\xFF\x7F"));
        check(2u64.pow(21), &mut Bytes::from_static(b"\x80\x80\x80\x01"));

        check(
            2u64.pow(28) - 1,
            &mut Bytes::from_static(b"\xFF\xFF\xFF\x7F"),
        );
        check(
            2u64.pow(28),
            &mut Bytes::from_static(b"\x80\x80\x80\x80\x01"),
        );

        check(
            2u64.pow(35) - 1,
            &mut Bytes::from_static(b"\xFF\xFF\xFF\xFF\x7F"),
        );
        check(
            2u64.pow(35),
            &mut Bytes::from_static(b"\x80\x80\x80\x80\x80\x01"),
        );

        check(
            2u64.pow(42) - 1,
            &mut Bytes::from_static(b"\xFF\xFF\xFF\xFF\xFF\x7F"),
        );
        check(
            2u64.pow(42),
            &mut Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x01"),
        );

        check(
            2u64.pow(49) - 1,
            &mut Bytes::from_static(b"\xFF\xFF\xFF\xFF\xFF\xFF\x7F"),
        );
        check(
            2u64.pow(49),
            &mut Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x01"),
        );

        check(
            2u64.pow(56) - 1,
            &mut Bytes::from_static(b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F"),
        );
        check(
            2u64.pow(56),
            &mut Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80\x01"),
        );

        check(
            2u64.pow(63) - 1,
            &mut Bytes::from_static(b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F"),
        );
        check(
            2u64.pow(63),
            &mut Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x01"),
        );

        check(
            u64::MAX,
            &mut Bytes::from_static(b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01"),
        );
    }

    #[test]
    fn varint_overflow() {
        let mut u64_max_plus_one = Bytes::from_static(b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02");

        decode_varint(&mut u64_max_plus_one).expect_err("decoding u64::MAX + 1 succeeded");
        decode_varint_slow(&mut u64_max_plus_one)
            .expect_err("slow decoding u64::MAX + 1 succeeded");
    }

    /// This big bowl o' macro soup generates an encoding property test for each
    /// combination of map type, scalar map key, and value type.
    /// TODO: these tests take a long time to compile, can this be improved?
    #[cfg(feature = "std")]
    macro_rules! map_tests {
        (keys: $keys:tt,
         vals: $vals:tt) => {
            mod hash_map {
                map_tests!(@private HashMap, hash_map, $keys, $vals);
            }
            mod btree_map {
                map_tests!(@private BTreeMap, btree_map, $keys, $vals);
            }
        };

        (@private $map_type:ident,
                  $mod_name:ident,
                  [$(($key_ty:ty, $key_proto:ident)),*],
                  $vals:tt) => {
            $(
                mod $key_proto {
                    use std::collections::$map_type;

                    use proptest::prelude::*;

                    use crate::encoding::*;
                    use crate::encoding::test::check_collection_type;

                    map_tests!(@private $map_type, $mod_name, ($key_ty, $key_proto), $vals);
                }
            )*
        };

        (@private $map_type:ident,
                  $mod_name:ident,
                  ($key_ty:ty, $key_proto:ident),
                  [$(($val_ty:ty, $val_proto:ident)),*]) => {
            $(
                proptest! {
                    #[test]
                    fn $val_proto(values: $map_type<$key_ty, $val_ty>, tag in MIN_TAG..=MAX_TAG) {
                        check_collection_type(values, tag, WireType::LengthDelimited,
                                              |tag, values, buf| {
                                                  $mod_name::encode($key_proto::encode,
                                                                    $key_proto::encoded_len,
                                                                    $val_proto::encode,
                                                                    $val_proto::encoded_len,
                                                                    tag,
                                                                    values,
                                                                    buf)
                                              },
                                              |wire_type, values, buf, ctx| {
                                                  check_wire_type(WireType::LengthDelimited, wire_type)?;
                                                  $mod_name::merge($key_proto::merge,
                                                                   $val_proto::merge,
                                                                   values,
                                                                   buf,
                                                                   ctx)
                                              },
                                              |tag, values| {
                                                  $mod_name::encoded_len($key_proto::encoded_len,
                                                                         $val_proto::encoded_len,
                                                                         tag,
                                                                         values)
                                              })?;
                    }
                }
             )*
        };
    }

    #[cfg(feature = "std")]
    map_tests!(keys: [
        (i32, int32),
        (i64, int64),
        (u32, uint32),
        (u64, uint64),
        (i32, sint32),
        (i64, sint64),
        (u32, fixed32),
        (u64, fixed64),
        (i32, sfixed32),
        (i64, sfixed64),
        (bool, bool),
        (String, string)
    ],
    vals: [
        (f32, float),
        (f64, double),
        (i32, int32),
        (i64, int64),
        (u32, uint32),
        (u64, uint64),
        (i32, sint32),
        (i64, sint64),
        (u32, fixed32),
        (u64, fixed64),
        (i32, sfixed32),
        (i64, sfixed64),
        (bool, bool),
        (String, string),
        (Vec<u8>, bytes)
    ]);
}
