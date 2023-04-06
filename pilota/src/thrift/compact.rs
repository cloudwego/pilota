// SPDX-License-Identifier: MIT OR Apache-2.0
//
// https://github.com/apache/thrift/blob/ec5e17714a1f9da34173749fc01eea33c7f6af62/lib/rs/src/protocol/compact.rs

use std::{ops::Deref, str};

use bytes::{Bytes, BytesMut};
use faststr::FastStr;
use integer_encoding::VarInt;
use linkedbytes::LinkedBytes;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::{
    error::ProtocolErrorKind,
    new_protocol_error,
    rw_ext::{ReadExt, WriteExt},
    varint_ext::VarIntProcessor,
    DecodeError, DecodeErrorKind, EncodeError, ProtocolError, TAsyncInputProtocol,
    TFieldIdentifier, TInputProtocol, TLengthProtocol, TListIdentifier, TMapIdentifier,
    TMessageIdentifier, TMessageType, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType,
    INLINE_CAP, ZERO_COPY_THRESHOLD,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TCompactType {
    Stop = 0x00,
    BooleanTrue = 0x01,
    BooleanFalse = 0x02,
    Byte = 0x03,
    I16 = 0x04,
    I32 = 0x05,
    I64 = 0x06,
    Double = 0x07,
    Binary = 0x08,
    List = 0x09,
    Set = 0x0A,
    Map = 0x0B,
    Struct = 0x0C,
    Uuid = 0x0D,
}

const COMPACT_BOOLEAN_TRUE: u8 = TCompactType::BooleanTrue as u8;
const COMPACT_BOOLEAN_FALSE: u8 = TCompactType::BooleanFalse as u8;

impl TryFrom<u8> for TCompactType {
    type Error = ProtocolError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TCompactType::Stop),
            0x01 => Ok(TCompactType::BooleanTrue),
            0x02 => Ok(TCompactType::BooleanFalse),
            0x03 => Ok(TCompactType::Byte),
            0x04 => Ok(TCompactType::I16),
            0x05 => Ok(TCompactType::I32),
            0x06 => Ok(TCompactType::I64),
            0x07 => Ok(TCompactType::Double),
            0x08 => Ok(TCompactType::Binary),
            0x09 => Ok(TCompactType::List),
            0x0A => Ok(TCompactType::Set),
            0x0B => Ok(TCompactType::Map),
            0x0C => Ok(TCompactType::Struct),
            0x0D => Ok(TCompactType::Uuid),
            _ => Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid compact type {:?}", value),
            )),
        }
    }
}

impl TryFrom<TType> for TCompactType {
    type Error = ProtocolError;
    #[inline]
    fn try_from(value: TType) -> Result<Self, Self::Error> {
        match value {
            TType::Stop => Ok(Self::Stop),
            TType::Bool => Ok(Self::BooleanTrue),
            TType::I8 => Ok(Self::Byte),
            TType::I16 => Ok(Self::I16),
            TType::I32 => Ok(Self::I32),
            TType::I64 => Ok(Self::I64),
            TType::Double => Ok(Self::Double),
            TType::Binary => Ok(Self::Binary),
            TType::List => Ok(Self::List),
            TType::Set => Ok(Self::Set),
            TType::Map => Ok(Self::Map),
            TType::Struct => Ok(Self::Struct),
            TType::Uuid => Ok(Self::Uuid),
            _ => Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid ttype {:?}", value),
            )),
        }
    }
}

impl TryFrom<TCompactType> for TType {
    type Error = ProtocolError;
    #[inline]
    fn try_from(value: TCompactType) -> Result<Self, Self::Error> {
        match value {
            TCompactType::Stop => Ok(Self::Stop),
            TCompactType::BooleanTrue | TCompactType::BooleanFalse => Ok(Self::Bool),
            TCompactType::Byte => Ok(TType::I8),
            TCompactType::I16 => Ok(TType::I16),
            TCompactType::I32 => Ok(TType::I32),
            TCompactType::I64 => Ok(TType::I64),
            TCompactType::Double => Ok(TType::Double),
            TCompactType::Binary => Ok(TType::Binary),
            TCompactType::List => Ok(TType::List),
            TCompactType::Set => Ok(TType::Set),
            TCompactType::Map => Ok(TType::Map),
            TCompactType::Struct => Ok(TType::Struct),
            TCompactType::Uuid => Ok(TType::Uuid),
        }
    }
}

const COMPACT_PROTOCOL_ID: u8 = 0x082;
const COMPACT_VERSION: u8 = 1;
const COMPACT_VERSION_MASK: u8 = 0x1f;
const COMPACT_TYPE_MASK: u8 = 0x0E0;
const COMPACT_TYPE_SHIFT_AMOUNT: u8 = 5;

#[inline]
fn tcompact_get_ttype(ct: TCompactType) -> Result<TType, ProtocolError> {
    ct.try_into().map_err(|_| {
        new_protocol_error(
            ProtocolErrorKind::InvalidData,
            format!("don't know what type: {:?}", ct),
        )
    })
}

#[inline]
fn tcompact_get_compact(tt: TType) -> Result<TCompactType, ProtocolError> {
    tt.try_into().map_err(|_| {
        new_protocol_error(
            ProtocolErrorKind::InvalidData,
            format!("invalid ttype {:?}", tt),
        )
    })
}

pub struct TCompactOutputProtocol<T> {
    pub(crate) trans: T,

    // Identifier of the last field serialized for a struct.
    last_write_field_id: i16,
    // Stack of the last written field ids (new entry added each time a nested struct is written).
    write_field_id_stack: Vec<i16>,
    // Field identifier of the boolean field to be written.
    // Saved because boolean fields and their value are encoded in a single byte
    pending_write_bool_field_identifier: Option<TFieldIdentifier>,

    zero_copy: bool,
    zero_copy_len: usize,
}

impl<T> TCompactOutputProtocol<T> {
    // `zero_copy` only takes effect when `T` is [`BytesMut`] for input and
    // [`LinkedBytes`] for output.
    #[inline]
    pub fn new(trans: T, zero_copy: bool) -> Self {
        Self {
            trans,
            write_field_id_stack: Vec::with_capacity(24),
            last_write_field_id: 0,
            pending_write_bool_field_identifier: None,

            zero_copy,
            zero_copy_len: 0,
        }
    }

    fn assert_no_pending_bool_write(&self) {
        if let Some(ref f) = self.pending_write_bool_field_identifier {
            panic!("pending bool field {:?} not written", f);
        }
    }
}

macro_rules! write_field_header_len {
    ($self:expr, $ax:expr, $field_type:expr, $id:expr) => {
        let field_delta = $id - $self.last_write_field_id;
        if field_delta > 0 && field_delta < 15 {
            $ax += $self.write_byte_len(0);
        } else {
            $ax += $self.write_byte_len($field_type as u8);
            $ax += $self.write_i16_len($id);
        }
        $self.last_write_field_id = $id;
    };
}

impl<T> TLengthProtocol for TCompactOutputProtocol<T> {
    #[inline]
    fn write_message_begin_len(&mut self, ident: &TMessageIdentifier) -> usize {
        2 + VarInt::required_space(ident.sequence_number as u32)
            + self.write_faststr_len(&ident.name)
    }
    #[inline]
    fn write_message_end_len(&mut self) -> usize {
        self.assert_no_pending_bool_write();
        0
    }

    #[inline]
    fn write_struct_begin_len(&mut self, _ident: &TStructIdentifier) -> usize {
        self.write_field_id_stack.push(self.last_write_field_id);
        self.last_write_field_id = 0;
        0
    }
    #[inline]
    fn write_struct_end_len(&mut self) -> usize {
        self.assert_no_pending_bool_write();
        self.last_write_field_id = self
            .write_field_id_stack
            .pop()
            .ok_or_else(|| {
                DecodeError::new(
                    super::DecodeErrorKind::InvalidData,
                    "WriteStructEndLen called without matching WriteStructBeginLen",
                )
            })
            .unwrap();
        0
    }

    #[inline]
    fn write_field_begin_len(&mut self, field_type: TType, id: Option<i16>) -> usize {
        // `id` is an Option<i16> following trait [`TLengthProtocol`]
        // write_field_begin_len.
        match field_type {
            TType::Bool => {
                if self.pending_write_bool_field_identifier.is_some() {
                    panic!(
                        "should not have a pending bool while writing another bool with id: \
                        {:?}",
                        id,
                    )
                }
                self.pending_write_bool_field_identifier = Some(TFieldIdentifier {
                    name: None,
                    field_type: field_type,
                    id: id,
                });
                0
            }
            _ => {
                let tc_field_type = TCompactType::try_from(field_type).unwrap(); // this should never happen
                let mut ax = 0;
                write_field_header_len!(self, ax, tc_field_type, id.expect("expecting a field id"));
                ax
            }
        }
    }
    #[inline]
    fn write_field_end_len(&mut self) -> usize {
        self.assert_no_pending_bool_write();
        0
    }
    #[inline]
    fn write_field_stop_len(&mut self) -> usize {
        self.assert_no_pending_bool_write();
        self.write_byte_len(TType::Stop as u8)
    }

    #[inline]
    fn write_bool_len(&mut self, b: bool) -> usize {
        match self.pending_write_bool_field_identifier.take() {
            Some(pending) => {
                let field_id = pending.id.expect("bool field should have a field id");
                let tc_field_type = if b {
                    TCompactType::BooleanTrue
                } else {
                    TCompactType::BooleanFalse
                };
                let mut ax = 0;
                write_field_header_len!(self, ax, tc_field_type, field_id);
                ax
            }
            None => self.write_byte_len(if b {
                TCompactType::BooleanTrue as u8
            } else {
                TCompactType::BooleanFalse as u8
            }),
        }
    }

    #[inline]
    fn write_bytes_len(&mut self, b: &[u8]) -> usize {
        if self.zero_copy && b.len() >= ZERO_COPY_THRESHOLD {
            self.zero_copy_len += b.len();
        }
        VarInt::required_space(b.len() as u32) + b.len()
    }
    #[inline]
    fn write_byte_len(&mut self, _b: u8) -> usize {
        1
    }

    #[inline]
    fn write_uuid_len(&mut self, _u: [u8; 16]) -> usize {
        16
    }

    #[inline]
    fn write_i8_len(&mut self, _i: i8) -> usize {
        1
    }
    #[inline]
    fn write_i16_len(&mut self, i: i16) -> usize {
        VarInt::required_space(i)
    }
    #[inline]
    fn write_i32_len(&mut self, i: i32) -> usize {
        VarInt::required_space(i)
    }
    #[inline]
    fn write_i64_len(&mut self, i: i64) -> usize {
        VarInt::required_space(i)
    }
    #[inline]
    fn write_double_len(&mut self, d: f64) -> usize {
        d.to_le_bytes().len()
    }

    #[inline]
    fn write_string_len(&mut self, s: &str) -> usize {
        VarInt::required_space(s.len() as u32) + s.len()
    }

    #[inline]
    fn write_faststr_len(&mut self, s: &FastStr) -> usize {
        if self.zero_copy && s.len() >= ZERO_COPY_THRESHOLD {
            self.zero_copy_len += s.len();
        }
        VarInt::required_space(s.len() as u32) + s.len()
    }

    #[inline]
    fn write_list_begin_len(&mut self, identifier: TListIdentifier) -> usize {
        if identifier.size <= 14 {
            self.write_byte_len(
                ((identifier.size as i32) << 4) as u8
                    | (tcompact_get_compact(identifier.element_type).unwrap() as u8),
            )
        } else {
            self.write_byte_len(
                0xF0 | (tcompact_get_compact(identifier.element_type).unwrap() as u8),
            ) + VarInt::required_space(identifier.size as u32)
        }
    }
    #[inline]
    fn write_list_end_len(&mut self) -> usize {
        0
    }

    #[inline]
    fn write_set_begin_len(&mut self, identifier: TSetIdentifier) -> usize {
        if identifier.size <= 14 {
            self.write_byte_len(
                ((identifier.size as i32) << 4) as u8
                    | (tcompact_get_compact(identifier.element_type).unwrap() as u8),
            )
        } else {
            self.write_byte_len(
                0xF0 | (tcompact_get_compact(identifier.element_type).unwrap() as u8),
            ) + VarInt::required_space(identifier.size as u32)
        }
    }
    #[inline]
    fn write_set_end_len(&mut self) -> usize {
        0
    }

    #[inline]
    fn write_map_begin_len(&mut self, identifier: TMapIdentifier) -> usize {
        if identifier.size == 0 {
            self.write_byte_len(TType::Stop as u8)
        } else {
            VarInt::required_space(identifier.size as u32)
                + self.write_byte_len(
                    (tcompact_get_compact(identifier.key_type).unwrap() as u8) << 4
                        | (tcompact_get_compact(identifier.value_type).unwrap()) as u8,
                )
        }
    }
    #[inline]
    fn write_map_end_len(&mut self) -> usize {
        0
    }

    #[inline]
    fn write_bytes_vec_len(&mut self, b: &[u8]) -> usize {
        self.write_bytes_len(b)
    }

    #[inline]
    fn zero_copy_len(&mut self) -> usize {
        self.zero_copy_len
    }

    #[inline]
    fn reset(&mut self) {
        self.zero_copy_len = 0;
    }
}

impl TCompactOutputProtocol<&mut BytesMut> {
    #[inline]
    fn write_varint<VI: VarInt>(&mut self, n: VI) -> Result<(), EncodeError> {
        let mut buf = [0u8; 10];
        let size = n.encode_var(&mut buf);
        self.trans.write_slice(&buf[0..size])?;
        Ok(())
    }

    #[inline]
    fn write_field_header(&mut self, field_type: TCompactType, id: i16) -> Result<(), EncodeError> {
        let field_delta = id - self.last_write_field_id;
        if field_delta > 0 && field_delta < 15 {
            self.write_byte(((field_delta as u8) << 4) | (field_type as u8))?;
        } else {
            self.write_byte(field_type as u8)?;
            self.write_i16(id)?;
        }
        self.last_write_field_id = id;
        Ok(())
    }

    #[inline]
    fn write_collection_begin(
        &mut self,
        element_type: TType,
        size: usize,
    ) -> Result<(), EncodeError> {
        if size <= 14 {
            self.write_byte(
                ((size as i32) << 4) as u8 | (tcompact_get_compact(element_type)? as u8),
            )?;
        } else {
            self.write_byte(0xF0 | (tcompact_get_compact(element_type)? as u8))?;
            self.write_varint(size as u32)?;
        }
        Ok(())
    }
}

impl TOutputProtocol for TCompactOutputProtocol<&mut BytesMut> {
    type BufMut = BytesMut;

    #[inline]
    fn write_message_begin(&mut self, identifier: &TMessageIdentifier) -> Result<(), EncodeError> {
        let mtype = identifier.message_type as u8;
        self.trans.write_slice(&[
            COMPACT_PROTOCOL_ID,
            (COMPACT_VERSION & COMPACT_VERSION_MASK)
                | ((mtype << COMPACT_TYPE_SHIFT_AMOUNT) & COMPACT_TYPE_MASK),
        ])?;
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(identifier.sequence_number as u32)?;
        self.write_faststr(identifier.name.clone())?;
        Ok(())
    }
    #[inline]
    fn write_message_end(&mut self) -> Result<(), EncodeError> {
        self.assert_no_pending_bool_write();
        Ok(())
    }

    #[inline]
    fn write_struct_begin(&mut self, _identifier: &TStructIdentifier) -> Result<(), EncodeError> {
        self.write_field_id_stack.push(self.last_write_field_id);
        self.last_write_field_id = 0;
        Ok(())
    }
    #[inline]
    fn write_struct_end(&mut self) -> Result<(), EncodeError> {
        self.assert_no_pending_bool_write();
        self.last_write_field_id = self.write_field_id_stack.pop().ok_or_else(|| {
            EncodeError::new(
                ProtocolErrorKind::InvalidData,
                "WriteStructEnd called without matching WriteStructBegin",
            )
        })?;
        Ok(())
    }

    #[inline]
    fn write_field_begin(&mut self, field_type: TType, id: i16) -> Result<(), EncodeError> {
        match field_type {
            TType::Bool => {
                if self.pending_write_bool_field_identifier.is_some() {
                    panic!(
                        "should not have a pending bool while writing another bool with id: \
                        {:?}",
                        id
                    )
                }
                self.pending_write_bool_field_identifier = Some(TFieldIdentifier {
                    name: None,
                    field_type,
                    id: Some(id),
                });
                Ok(())
            }
            _ => {
                let tc_field_type = TCompactType::try_from(field_type)?;
                self.write_field_header(tc_field_type, id)
            }
        }
    }
    #[inline]
    fn write_field_end(&mut self) -> Result<(), EncodeError> {
        self.assert_no_pending_bool_write();
        Ok(())
    }
    #[inline]
    fn write_field_stop(&mut self) -> Result<(), EncodeError> {
        self.assert_no_pending_bool_write();
        self.write_byte(TType::Stop as u8)?;
        Ok(())
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<(), EncodeError> {
        match self.pending_write_bool_field_identifier.take() {
            Some(pending) => {
                let field_id = pending.id.expect("bool field should have a field id");
                let tc_field_type = if b {
                    TCompactType::BooleanTrue
                } else {
                    TCompactType::BooleanFalse
                };
                self.write_field_header(tc_field_type, field_id)
            }
            None => self.write_byte(if b {
                TCompactType::BooleanTrue as u8
            } else {
                TCompactType::BooleanFalse as u8
            }),
        }
    }
    #[inline]
    fn write_bytes(&mut self, b: Bytes) -> Result<(), EncodeError> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(b.len() as u32)?;
        self.trans.write_slice(&b)?;
        Ok(())
    }
    #[inline]
    fn write_byte(&mut self, b: u8) -> Result<(), EncodeError> {
        self.trans.write_u8(b)?;
        Ok(())
    }

    #[inline]
    fn write_uuid(&mut self, u: [u8; 16]) -> Result<(), EncodeError> {
        self.trans.write_slice(&u)?;
        Ok(())
    }

    #[inline]
    fn write_i8(&mut self, i: i8) -> Result<(), EncodeError> {
        self.trans.write_i8(i)?;
        Ok(())
    }
    #[inline]
    fn write_i16(&mut self, i: i16) -> Result<(), EncodeError> {
        self.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), EncodeError> {
        self.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_i64(&mut self, i: i64) -> Result<(), EncodeError> {
        self.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_double(&mut self, d: f64) -> Result<(), EncodeError> {
        self.trans.write_f64(d)?;
        Ok(())
    }

    #[inline]
    fn write_string(&mut self, s: &str) -> Result<(), EncodeError> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(s.len() as u32)?;
        self.trans.write_slice(s.as_bytes())?;
        Ok(())
    }

    #[inline]
    fn write_faststr(&mut self, s: FastStr) -> Result<(), EncodeError> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(s.len() as u32)?;
        self.trans.write_slice(s.as_ref())?;
        Ok(())
    }

    #[inline]
    fn write_list_begin(&mut self, identifier: TListIdentifier) -> Result<(), EncodeError> {
        self.write_collection_begin(identifier.element_type, identifier.size)?;
        Ok(())
    }
    #[inline]
    fn write_list_end(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    #[inline]
    fn write_set_begin(&mut self, identifier: TSetIdentifier) -> Result<(), EncodeError> {
        self.write_collection_begin(identifier.element_type, identifier.size)?;
        Ok(())
    }
    #[inline]
    fn write_set_end(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    #[inline]
    fn write_map_begin(&mut self, identifier: TMapIdentifier) -> Result<(), EncodeError> {
        if identifier.size == 0 {
            self.write_byte(TType::Stop as u8)?;
        } else {
            // element count is strictly positive as per the spec, so
            // cast i32 as u32 so that varint writing won't use zigzag encoding
            self.write_varint(identifier.size as u32)?;
            self.write_byte(
                (tcompact_get_compact(identifier.key_type)? as u8) << 4
                    | (tcompact_get_compact(identifier.value_type)?) as u8,
            )?
        }
        Ok(())
    }
    #[inline]
    fn write_map_end(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    #[inline]
    fn reserve(&mut self, size: usize) {
        self.trans.reserve(size)
    }

    #[inline]
    fn buf_mut(&mut self) -> &mut Self::BufMut {
        self.trans
    }

    #[inline]
    fn write_bytes_vec(&mut self, b: &[u8]) -> Result<(), EncodeError> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(b.len() as u32)?;
        self.trans.write_slice(b)?;
        Ok(())
    }
}

impl TCompactOutputProtocol<&mut LinkedBytes> {
    #[inline]
    fn write_varint<VI: VarInt>(&mut self, n: VI) -> Result<(), EncodeError> {
        let mut buf = [0u8; 10];
        let size = n.encode_var(&mut buf);
        self.trans.bytes_mut().write_slice(&buf[0..size])?;
        Ok(())
    }

    #[inline]
    fn write_field_header(&mut self, field_type: TCompactType, id: i16) -> Result<(), EncodeError> {
        let field_delta = id - self.last_write_field_id;
        if field_delta > 0 && field_delta < 15 {
            self.write_byte(((field_delta as u8) << 4) | (field_type as u8))?;
        } else {
            self.write_byte(field_type as u8)?;
            self.write_i16(id)?;
        }
        self.last_write_field_id = id;
        Ok(())
    }

    #[inline]
    fn write_collection_begin(
        &mut self,
        element_type: TType,
        size: usize,
    ) -> Result<(), EncodeError> {
        if size <= 14 {
            self.write_byte(
                ((size as i32) << 4) as u8 | (tcompact_get_compact(element_type)? as u8),
            )?;
        } else {
            self.write_byte(0xF0 | (tcompact_get_compact(element_type)? as u8))?;
            self.write_varint(size as u32)?;
        }
        Ok(())
    }
}

impl TOutputProtocol for TCompactOutputProtocol<&mut LinkedBytes> {
    type BufMut = LinkedBytes;

    #[inline]
    fn write_message_begin(&mut self, identifier: &TMessageIdentifier) -> Result<(), EncodeError> {
        let mtype = identifier.message_type as u8;
        self.trans.bytes_mut().write_slice(&[
            COMPACT_PROTOCOL_ID,
            (COMPACT_VERSION & COMPACT_VERSION_MASK)
                | ((mtype << COMPACT_TYPE_SHIFT_AMOUNT) & COMPACT_TYPE_MASK),
        ])?;
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(identifier.sequence_number as u32)?;
        self.write_faststr(identifier.name.clone())?;
        Ok(())
    }
    #[inline]
    fn write_message_end(&mut self) -> Result<(), EncodeError> {
        self.assert_no_pending_bool_write();
        Ok(())
    }

    #[inline]
    fn write_struct_begin(&mut self, _identifier: &TStructIdentifier) -> Result<(), EncodeError> {
        self.write_field_id_stack.push(self.last_write_field_id);
        self.last_write_field_id = 0;
        Ok(())
    }
    #[inline]
    fn write_struct_end(&mut self) -> Result<(), EncodeError> {
        self.assert_no_pending_bool_write();
        self.last_write_field_id = self.write_field_id_stack.pop().ok_or_else(|| {
            EncodeError::new(
                ProtocolErrorKind::InvalidData,
                "WriteStructEnd called without matching WriteStructBegin",
            )
        })?;
        Ok(())
    }

    #[inline]
    fn write_field_begin(&mut self, field_type: TType, id: i16) -> Result<(), EncodeError> {
        match field_type {
            TType::Bool => {
                if self.pending_write_bool_field_identifier.is_some() {
                    panic!(
                        "should not have a pending bool while writing another bool with id: \
                        {:?}",
                        id
                    )
                }
                self.pending_write_bool_field_identifier = Some(TFieldIdentifier {
                    name: None,
                    field_type,
                    id: Some(id),
                });
                Ok(())
            }
            _ => {
                let tc_field_type = TCompactType::try_from(field_type)?;
                self.write_field_header(tc_field_type, id)
            }
        }
    }
    #[inline]
    fn write_field_end(&mut self) -> Result<(), EncodeError> {
        self.assert_no_pending_bool_write();
        Ok(())
    }
    #[inline]
    fn write_field_stop(&mut self) -> Result<(), EncodeError> {
        self.assert_no_pending_bool_write();
        self.write_byte(TType::Stop as u8)?;
        Ok(())
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<(), EncodeError> {
        match self.pending_write_bool_field_identifier.take() {
            Some(pending) => {
                let field_id = pending.id.expect("bool field should have a field id");
                let tc_field_type = if b {
                    TCompactType::BooleanTrue
                } else {
                    TCompactType::BooleanFalse
                };
                self.write_field_header(tc_field_type, field_id)
            }
            None => {
                if b {
                    self.write_byte(TCompactType::BooleanTrue as u8)
                } else {
                    self.write_byte(TCompactType::BooleanFalse as u8)
                }
            }
        }
    }
    #[inline]
    fn write_bytes(&mut self, b: Bytes) -> Result<(), EncodeError> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(b.len() as u32)?;
        if self.zero_copy && b.len() >= ZERO_COPY_THRESHOLD {
            self.trans.insert(b);
            return Ok(());
        }
        self.trans.bytes_mut().write_slice(&b)?;
        Ok(())
    }
    #[inline]
    fn write_byte(&mut self, b: u8) -> Result<(), EncodeError> {
        self.trans.bytes_mut().write_u8(b)?;
        Ok(())
    }

    #[inline]
    fn write_uuid(&mut self, u: [u8; 16]) -> Result<(), EncodeError> {
        self.trans.bytes_mut().write_slice(&u)?;
        Ok(())
    }

    #[inline]
    fn write_i8(&mut self, i: i8) -> Result<(), EncodeError> {
        self.trans.bytes_mut().write_i8(i)?;
        Ok(())
    }
    #[inline]
    fn write_i16(&mut self, i: i16) -> Result<(), EncodeError> {
        self.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), EncodeError> {
        self.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_i64(&mut self, i: i64) -> Result<(), EncodeError> {
        self.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_double(&mut self, d: f64) -> Result<(), EncodeError> {
        self.trans.bytes_mut().write_f64(d)?;
        Ok(())
    }

    #[inline]
    fn write_string(&mut self, s: &str) -> Result<(), EncodeError> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(s.len() as u32)?;
        self.trans.bytes_mut().write_slice(s.as_bytes())?;
        Ok(())
    }

    #[inline]
    fn write_faststr(&mut self, s: FastStr) -> Result<(), EncodeError> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(s.len() as u32)?;
        if self.zero_copy && s.len() <= ZERO_COPY_THRESHOLD {
            self.trans.insert_faststr(s);
            return Ok(());
        }
        self.trans.bytes_mut().write_slice(s.as_ref())?;
        Ok(())
    }

    #[inline]
    fn write_list_begin(&mut self, identifier: TListIdentifier) -> Result<(), EncodeError> {
        self.write_collection_begin(identifier.element_type, identifier.size)?;
        Ok(())
    }
    #[inline]
    fn write_list_end(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    #[inline]
    fn write_set_begin(&mut self, identifier: TSetIdentifier) -> Result<(), EncodeError> {
        self.write_collection_begin(identifier.element_type, identifier.size)?;
        Ok(())
    }
    #[inline]
    fn write_set_end(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    #[inline]
    fn write_map_begin(&mut self, identifier: TMapIdentifier) -> Result<(), EncodeError> {
        if identifier.size == 0 {
            self.write_byte(TType::Stop as u8)?;
        } else {
            // element count is strictly positive as per the spec, so
            // cast i32 as u32 so that varint writing won't use zigzag encoding
            self.write_varint(identifier.size as u32)?;
            self.write_byte(
                (tcompact_get_compact(identifier.key_type)? as u8) << 4
                    | (tcompact_get_compact(identifier.value_type)?) as u8,
            )?
        }
        Ok(())
    }
    #[inline]
    fn write_map_end(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), EncodeError> {
        Ok(())
    }

    #[inline]
    fn reserve(&mut self, size: usize) {
        self.trans.reserve(size)
    }

    #[inline]
    fn buf_mut(&mut self) -> &mut Self::BufMut {
        self.trans
    }

    #[inline]
    fn write_bytes_vec(&mut self, b: &[u8]) -> Result<(), EncodeError> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.write_varint(b.len() as u32)?;
        self.trans.bytes_mut().write_slice(b)?;
        Ok(())
    }
}

pub struct TAsyncCompactProtocol<R> {
    reader: R,

    last_read_field_id: i16,
    read_field_id_stack: Vec<i16>,
    pending_read_bool_value: Option<bool>,
}

#[async_trait::async_trait]
impl<R> TAsyncInputProtocol for TAsyncCompactProtocol<R>
where
    R: AsyncRead + Unpin + Send,
{
    async fn read_message_begin(&mut self) -> Result<TMessageIdentifier, DecodeError> {
        let compact_id = self.read_byte().await?;
        if compact_id != COMPACT_PROTOCOL_ID {
            return Err(DecodeError::new(
                DecodeErrorKind::BadVersion,
                format!("invalid compact protocol header {:?}", compact_id),
            ));
        }

        let type_and_byte = self.read_byte().await?;
        let version = type_and_byte & COMPACT_VERSION_MASK;
        if version != COMPACT_VERSION {
            return Err(DecodeError::new(
                DecodeErrorKind::BadVersion,
                format!("cannot process compact protocol version {:?}", version),
            ));
        }

        // NOTE: unsigned right shift will pad with 0s
        let type_id = type_and_byte >> 5;
        let message_type = TMessageType::try_from(type_id).map_err(|_| {
            DecodeError::new(
                DecodeErrorKind::InvalidData,
                format!("invalid message type {}", type_id),
            )
        })?;

        // writing side wrote signed sequence number as u32 to avoid zigzag encoding
        let sequence_number = self.read_varint_async::<u32>().await? as i32;
        let name = self.read_faststr().await?;

        Ok(TMessageIdentifier::new(name, message_type, sequence_number))
    }

    #[inline]
    async fn read_message_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    #[inline]
    async fn read_struct_begin(&mut self) -> Result<Option<TStructIdentifier>, DecodeError> {
        self.read_field_id_stack.push(self.last_read_field_id);
        self.last_read_field_id = 0;
        Ok(None)
    }

    #[inline]
    async fn read_struct_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    // #[inline]
    async fn read_field_begin(&mut self) -> Result<TFieldIdentifier, DecodeError> {
        // we can read at least one byte, which is:
        // - the type
        // - the field id delta and the type
        let field_type = self.read_byte().await?;
        let field_delta = (field_type & 0xF0) >> 4;
        let field_type = match field_type & 0x0F {
            COMPACT_BOOLEAN_TRUE => {
                self.pending_read_bool_value = Some(true);
                Ok(TType::Bool)
            }
            COMPACT_BOOLEAN_FALSE => {
                self.pending_read_bool_value = Some(false);
                Ok(TType::Bool)
            }
            ttu8 => TType::try_from(TCompactType::try_from(ttu8)?),
        }?;
        match field_type {
            TType::Stop => Ok(TFieldIdentifier::new::<Option<&'static str>, Option<i16>>(
                None,
                TType::Stop,
                None,
            )),
            _ => {
                if field_delta != 0 {
                    self.last_read_field_id += field_delta as i16;
                } else {
                    self.last_read_field_id = self.read_i16().await?;
                }
                Ok(TFieldIdentifier::new::<Option<&'static str>, i16>(
                    None,
                    field_type,
                    self.last_read_field_id,
                ))
            }
        }
    }

    #[inline]
    async fn read_field_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    #[inline]
    async fn read_bool(&mut self) -> Result<bool, DecodeError> {
        match self.pending_read_bool_value.take() {
            Some(b) => Ok(b),
            None => {
                let b: TCompactType = self.read_byte().await?.try_into()?;
                match b {
                    TCompactType::BooleanTrue => Ok(true),
                    TCompactType::BooleanFalse => Ok(false),
                    unkn => Err(DecodeError::new(
                        DecodeErrorKind::InvalidData,
                        format!("cannot convert {:?} into bool", unkn),
                    )),
                }
            }
        }
    }

    #[inline]
    async fn read_bytes(&mut self) -> Result<Bytes, DecodeError> {
        self.read_bytes_vec().await.map(Bytes::from)
    }

    #[inline]
    async fn read_bytes_vec(&mut self) -> Result<Vec<u8>, DecodeError> {
        let size = self.read_varint_async::<u32>().await? as usize;
        // FIXME: use maybe_uninit?
        let mut v = vec![0; size];
        self.reader.read_exact(&mut v).await?;
        Ok(v)
    }

    #[inline]
    async fn read_uuid(&mut self) -> Result<[u8; 16], DecodeError> {
        let mut uuid = [0; 16];
        self.reader.read_exact(&mut uuid).await?;
        Ok(uuid)
    }

    #[inline]
    async fn read_string(&mut self) -> Result<String, DecodeError> {
        let v = self.read_bytes_vec().await?;
        Ok(unsafe { String::from_utf8_unchecked(v) })
    }

    #[inline]
    async fn read_faststr(&mut self) -> Result<FastStr, DecodeError> {
        self.read_string().await.map(FastStr::from_string)
    }

    #[inline]
    async fn read_byte(&mut self) -> Result<u8, DecodeError> {
        Ok(self.reader.read_u8().await?)
    }

    #[inline]
    async fn read_i8(&mut self) -> Result<i8, DecodeError> {
        Ok(self.reader.read_i8().await?)
    }

    #[inline]
    async fn read_i16(&mut self) -> Result<i16, DecodeError> {
        Ok(self.read_varint_async::<i16>().await?)
    }

    #[inline]
    async fn read_i32(&mut self) -> Result<i32, DecodeError> {
        Ok(self.read_varint_async::<i32>().await?)
    }

    #[inline]
    async fn read_i64(&mut self) -> Result<i64, DecodeError> {
        Ok(self.read_varint_async::<i64>().await?)
    }

    #[inline]
    async fn read_double(&mut self) -> Result<f64, DecodeError> {
        Ok(self.reader.read_f64_le().await?)
    }

    #[inline]
    async fn read_list_begin(&mut self) -> Result<TListIdentifier, DecodeError> {
        let (element_type, element_count) = self.read_collection_begin().await?;
        Ok(TListIdentifier {
            element_type,
            size: element_count,
        })
    }

    #[inline]
    async fn read_list_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    #[inline]
    async fn read_set_begin(&mut self) -> Result<TSetIdentifier, DecodeError> {
        let (element_type, element_count) = self.read_collection_begin().await?;
        Ok(TSetIdentifier {
            element_type,
            size: element_count,
        })
    }

    #[inline]
    async fn read_set_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    #[inline]
    async fn read_map_begin(&mut self) -> Result<TMapIdentifier, DecodeError> {
        let element_count = self.read_varint_async::<u32>().await? as i32;
        if element_count == 0 {
            Ok(TMapIdentifier::new(TType::Stop, TType::Stop, 0))
        } else {
            let type_header = self.read_byte().await?;
            let key_type = tcompact_get_ttype(((type_header & 0xF0) >> 4).try_into()?)?;
            let val_type = tcompact_get_ttype((type_header & 0x0F).try_into()?)?;

            Ok(TMapIdentifier::new(
                key_type,
                val_type,
                element_count as usize,
            ))
        }
    }

    #[inline]
    async fn read_map_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }
}

impl<R> TAsyncCompactProtocol<R>
where
    R: AsyncRead + Unpin + Send,
{
    pub fn new(reader: R) -> TAsyncCompactProtocol<R> {
        Self {
            reader,
            last_read_field_id: 0,
            read_field_id_stack: Vec::new(),
            pending_read_bool_value: None,
        }
    }

    #[inline]
    async fn read_collection_begin(&mut self) -> Result<(TType, usize), DecodeError> {
        let header = self.read_byte().await?;
        let element_type = tcompact_get_ttype((header & 0x0F).try_into()?)?;

        let possible_element_count = (header & 0xF0) >> 4;
        let element_count = if possible_element_count != 15 {
            possible_element_count as i32
        } else {
            self.read_varint_async::<u32>().await? as i32
        };
        Ok((element_type, element_count as usize))
    }

    #[inline]
    async fn read_varint_async<VI: VarInt>(&mut self) -> Result<VI, DecodeError> {
        let mut p = VarIntProcessor::new::<VI>();
        while !p.finished() {
            let read = self.reader.read_u8().await?;
            p.push(read)?;
        }
        p.decode()
            .ok_or_else(|| DecodeError::new(DecodeErrorKind::InvalidData, "can't decode varint"))
    }
}

pub struct TCompactInputProtocol<T> {
    pub(crate) trans: T,

    // Identifier of the last field deserialized for a struct.
    last_read_field_id: i16,
    // Stack of the last read field ids (a new entry is added each time a nested struct is read).
    read_field_id_stack: Vec<i16>,
    // Boolean value for a field.
    // Saved because boolean fields and their value are encoded in a single byte,
    // and reading the field only occurs after the field id is read.
    pending_read_bool_value: Option<bool>,
}

impl<T> TCompactInputProtocol<T> {
    pub fn new(trans: T) -> Self {
        Self {
            trans,
            last_read_field_id: 0,
            read_field_id_stack: Vec::with_capacity(24),
            pending_read_bool_value: None,
        }
    }
}

impl TCompactInputProtocol<&mut BytesMut> {
    #[inline]
    fn read_varint<VI: VarInt>(&mut self) -> Result<VI, DecodeError> {
        let mut p = VarIntProcessor::new::<VI>();
        while !p.finished() {
            let read = self.trans.read_u8()?;
            p.push(read)?;
        }
        p.decode()
            .ok_or_else(|| DecodeError::new(DecodeErrorKind::InvalidData, "can't decode varint"))
    }

    #[inline]
    fn read_collection_begin(&mut self) -> Result<(TType, usize), DecodeError> {
        let header = self.read_byte()?;
        let element_type = tcompact_get_ttype((header & 0x0F).try_into()?)?;

        let possible_element_count = (header & 0xF0) >> 4;
        let element_count = if possible_element_count != 15 {
            possible_element_count as i32
        } else {
            self.read_varint::<u32>()? as i32
        };
        Ok((element_type, element_count as usize))
    }
}

impl TInputProtocol for TCompactInputProtocol<&mut BytesMut> {
    type Buf = BytesMut;

    fn read_message_begin(&mut self) -> Result<TMessageIdentifier, DecodeError> {
        let compact_id = self.read_byte()?;
        if compact_id != COMPACT_PROTOCOL_ID {
            return Err(DecodeError::new(
                DecodeErrorKind::InvalidData,
                format!("invalid compact protocol header {:?}", compact_id),
            ));
        }

        let type_and_byte = self.read_byte()?;
        let version = type_and_byte & COMPACT_VERSION_MASK;
        if version != COMPACT_VERSION {
            return Err(DecodeError::new(
                DecodeErrorKind::InvalidData,
                format!("cannot process compact protocol version {:?}", version),
            ));
        }

        // NOTE: unsigned right shift will pad with 0s
        let type_id = type_and_byte >> 5;
        let message_type = TMessageType::try_from(type_id).map_err(|_| {
            DecodeError::new(
                DecodeErrorKind::InvalidData,
                format!("invalid message type {:?}", type_id),
            )
        })?;

        // writing side wrote signed sequence number as u32 to avoid zigzag encoding
        let sequence_number = self.read_varint::<u32>()? as i32;
        let name = self.read_faststr()?;

        Ok(TMessageIdentifier::new(name, message_type, sequence_number))
    }

    #[inline]
    fn read_message_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    #[inline]
    fn read_struct_begin(&mut self) -> Result<Option<TStructIdentifier>, DecodeError> {
        self.read_field_id_stack.push(self.last_read_field_id);
        self.last_read_field_id = 0;
        Ok(None)
    }

    #[inline]
    fn read_struct_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    // #[inline]
    fn read_field_begin(&mut self) -> Result<TFieldIdentifier, DecodeError> {
        // we can read at least one byte, which is:
        // - the type
        // - the field id delta and the type
        let field_type = self.read_byte()?;
        let field_delta = (field_type & 0xF0) >> 4;
        let field_type = match field_type & 0x0F {
            COMPACT_BOOLEAN_TRUE => {
                self.pending_read_bool_value = Some(true);
                Ok(TType::Bool)
            }
            COMPACT_BOOLEAN_FALSE => {
                self.pending_read_bool_value = Some(false);
                Ok(TType::Bool)
            }
            ttu8 => TType::try_from(TCompactType::try_from(ttu8)?),
        }?;
        match field_type {
            TType::Stop => Ok(TFieldIdentifier::new::<Option<&'static str>, Option<i16>>(
                None,
                TType::Stop,
                None,
            )),
            _ => {
                if field_delta != 0 {
                    self.last_read_field_id += field_delta as i16;
                } else {
                    self.last_read_field_id = self.read_i16()?;
                }
                Ok(TFieldIdentifier::new::<Option<&'static str>, i16>(
                    None,
                    field_type,
                    self.last_read_field_id,
                ))
            }
        }
    }

    #[inline]
    fn read_field_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    #[inline]
    fn read_bool(&mut self) -> Result<bool, DecodeError> {
        match self.pending_read_bool_value.take() {
            Some(b) => Ok(b),
            None => {
                let b: TCompactType = self.read_byte()?.try_into()?;
                match b {
                    TCompactType::BooleanTrue => Ok(true),
                    TCompactType::BooleanFalse => Ok(false),
                    unkn => Err(DecodeError::new(
                        DecodeErrorKind::InvalidData,
                        format!("cannot convert {:?} into bool", unkn),
                    )),
                }
            }
        }
    }

    #[inline]
    fn read_bytes(&mut self) -> Result<Bytes, DecodeError> {
        let size = self.read_varint::<u32>()?;
        Ok(self.trans.split_to(size as usize).freeze())
    }

    #[inline]
    fn read_uuid(&mut self) -> Result<[u8; 16], DecodeError> {
        let mut u = [0; 16];
        self.trans.read_to_slice(&mut u)?;
        Ok(u)
    }

    #[inline]
    fn read_string(&mut self) -> Result<String, DecodeError> {
        let size = self.read_varint::<u32>()? as usize;
        Ok(self.trans.read_to_string(size)?)
    }

    #[inline]
    fn read_faststr(&mut self) -> Result<FastStr, DecodeError> {
        let size = self.read_varint::<u32>()? as usize;
        let bytes = self.trans.split_to(size);
        if size > INLINE_CAP {
            unsafe { return Ok(FastStr::from_bytes_mut_unchecked(bytes)) }
        }
        unsafe { Ok(FastStr::new_inline(str::from_utf8_unchecked(bytes.deref()))) }
    }

    #[inline]
    fn read_byte(&mut self) -> Result<u8, DecodeError> {
        Ok(self.trans.read_u8()?)
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8, DecodeError> {
        Ok(self.trans.read_i8()?)
    }
    #[inline]
    fn read_i16(&mut self) -> Result<i16, DecodeError> {
        self.read_varint::<i16>()
    }
    #[inline]
    fn read_i32(&mut self) -> Result<i32, DecodeError> {
        self.read_varint::<i32>()
    }
    #[inline]
    fn read_i64(&mut self) -> Result<i64, DecodeError> {
        self.read_varint::<i64>()
    }

    #[inline]
    fn read_double(&mut self) -> Result<f64, DecodeError> {
        Ok(self.trans.read_f64_le()?)
    }

    #[inline]
    fn read_list_begin(&mut self) -> Result<TListIdentifier, DecodeError> {
        let (element_type, element_count) = self.read_collection_begin()?;
        Ok(TListIdentifier {
            element_type,
            size: element_count,
        })
    }
    #[inline]
    fn read_list_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    #[inline]
    fn read_set_begin(&mut self) -> Result<TSetIdentifier, DecodeError> {
        let (element_type, element_count) = self.read_collection_begin()?;
        Ok(TSetIdentifier {
            element_type,
            size: element_count,
        })
    }

    #[inline]
    fn read_set_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    // #[inline]
    fn read_map_begin(&mut self) -> Result<TMapIdentifier, DecodeError> {
        let element_count = self.read_varint::<u32>()? as i32;
        if element_count == 0 {
            Ok(TMapIdentifier::new(TType::Stop, TType::Stop, 0))
        } else {
            let type_header = self.read_byte()?;
            let key_type = tcompact_get_ttype(((type_header & 0xF0) >> 4).try_into()?)?;
            let val_type = tcompact_get_ttype((type_header & 0x0F).try_into()?)?;

            Ok(TMapIdentifier::new(
                key_type,
                val_type,
                element_count as usize,
            ))
        }
    }

    #[inline]
    fn read_map_end(&mut self) -> Result<(), DecodeError> {
        Ok(())
    }

    fn buf_mut(&mut self) -> &mut Self::Buf {
        self.trans
    }

    #[inline]
    fn read_bytes_vec(&mut self) -> Result<Vec<u8>, DecodeError> {
        let size = self.read_varint::<u32>()? as usize;

        Ok(self.trans.split_to(size).into())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use bytes::{Buf, BufMut, Bytes, BytesMut};
    use linkedbytes::LinkedBytes;

    use super::{TCompactInputProtocol, TCompactOutputProtocol};
    use crate::thrift::{
        EncodeError, TFieldIdentifier, TInputProtocol, TLengthProtocol, TListIdentifier,
        TMapIdentifier, TMessageIdentifier, TMessageType, TOutputProtocol, TSetIdentifier,
        TStructIdentifier, TType,
    };

    #[cfg(test)]
    macro_rules! assert_success {
        ($e: expr) => {{
            let res = $e;
            assert!(res.is_ok());
            res.unwrap()
        }};
    }
    #[cfg(test)]
    macro_rules! assert_eq_written_bytes {
        ($o_prot:ident, $expected_bytes:ident) => {{
            assert_eq!(&$o_prot.trans[..], &$expected_bytes);
        }};
    }

    fn test_input_prot_bytesmut<'a>(
        trans: &'a mut BytesMut,
    ) -> TCompactInputProtocol<&'a mut BytesMut> {
        TCompactInputProtocol::new(trans)
    }
    fn test_output_prot_bytesmut<'a>(
        trans: &'a mut BytesMut,
    ) -> TCompactOutputProtocol<&'a mut BytesMut> {
        TCompactOutputProtocol::new(trans, false)
    }

    fn test_input_prot_linkedbytes<'a>(
        trans: &'a mut LinkedBytes,
    ) -> TCompactInputProtocol<&'a mut LinkedBytes> {
        TCompactInputProtocol::new(trans)
    }
    fn test_output_prot_linkedbytes<'a>(
        trans: &'a mut LinkedBytes,
    ) -> TCompactOutputProtocol<&'a mut LinkedBytes> {
        TCompactOutputProtocol::new(trans, false)
    }

    #[test]
    fn must_have_same_length_written() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);
        macro_rules! mteq {
            ($o:expr, $exp:expr) => {
                assert_eq!($exp, $o.trans.len());
                $o.trans.clear();
            };
        }

        // message
        let identifier = &TMessageIdentifier::new("foo".into(), TMessageType::Call, 1);
        o_prot.write_message_begin(identifier).unwrap();
        let exp = o_prot.write_message_begin_len(identifier);
        mteq!(o_prot, exp);
        o_prot.write_message_end().unwrap();
        let exp = o_prot.write_message_end_len();
        mteq!(o_prot, exp);

        // struct
        let identifier = &TStructIdentifier::new("foo");
        o_prot.write_struct_begin(identifier).unwrap();
        let exp = o_prot.write_struct_begin_len(identifier);
        mteq!(o_prot, exp);
        o_prot.write_struct_end().unwrap();
        let exp = o_prot.write_struct_end_len();
        mteq!(o_prot, exp);

        // === START [ field test ] ===
        let (field_type, field_id) = (TType::I64, 0);
        o_prot.write_field_begin(field_type, field_id).unwrap(); // first id = 0
        mteq!(
            o_prot,
            o_prot.write_field_begin_len(field_type, Some(field_id))
        );
        o_prot.write_field_end().unwrap();
        mteq!(o_prot, o_prot.write_field_end_len());

        // trigger 3 bytes write, field ID delta > 0b1111
        // 1 byte (field header) + 2 bytes (I16, for field ID)
        let (field_type, field_id) = (TType::Binary, 16);
        o_prot.write_field_begin(field_type, field_id).unwrap();
        mteq!(
            o_prot,
            o_prot.write_field_begin_len(field_type, Some(field_id))
        );

        // bare write bool
        o_prot.write_bool(false).unwrap();
        mteq!(o_prot, o_prot.write_bool_len(false));
        // write bool with field
        let (field_type, field_id) = (TType::Bool, 17);
        let mut ax = 0;
        o_prot.write_field_begin(field_type, field_id).unwrap();
        let _pending_bool_field_ident = o_prot.pending_write_bool_field_identifier.take();
        ax += o_prot.write_field_begin_len(field_type, Some(field_id));
        //
        o_prot.write_bool(false).unwrap();
        ax += o_prot.write_bool_len(false);
        o_prot.write_field_end().unwrap();
        ax += o_prot.write_field_end_len();
        mteq!(o_prot, ax);

        o_prot.write_field_stop().unwrap();
        mteq!(o_prot, o_prot.write_field_stop_len());
        // === END [ field test ] ===

        o_prot.write_byte(0xff).unwrap();
        mteq!(o_prot, o_prot.write_byte_len(0xff));
        o_prot.write_i8(-1).unwrap();
        mteq!(o_prot, o_prot.write_i8_len(-1));

        o_prot.write_i16(-1).unwrap();
        mteq!(o_prot, o_prot.write_i16_len(-1));

        o_prot.write_i32(-1).unwrap();
        mteq!(o_prot, o_prot.write_i32_len(-1));

        o_prot.write_i64(-1).unwrap();
        mteq!(o_prot, o_prot.write_i64_len(-1));

        o_prot.write_double(13.37f64).unwrap();
        mteq!(o_prot, o_prot.write_double_len(13.37f64));

        let identifier = 0xf00baau64.to_le_bytes().to_vec();
        o_prot.write_bytes(Bytes::from(identifier.clone())).unwrap();
        mteq!(o_prot, o_prot.write_bytes_len(&identifier[..]));

        let identifier = [0u8; 16];
        o_prot.write_uuid(identifier).unwrap();
        mteq!(o_prot, o_prot.write_uuid_len(identifier));

        let identifier = "foobar";
        o_prot.write_faststr(identifier.into()).unwrap();
        mteq!(o_prot, o_prot.write_faststr_len(&identifier.into()));

        let mut identifier = TListIdentifier::new(TType::I16, 0);
        o_prot.write_list_begin(identifier).unwrap();
        mteq!(o_prot, o_prot.write_list_begin_len(identifier));
        o_prot.write_list_end().unwrap();
        mteq!(o_prot, o_prot.write_list_end_len());
        identifier.size = 1;
        o_prot.write_list_begin(identifier).unwrap();
        mteq!(o_prot, o_prot.write_list_begin_len(identifier));
        o_prot.write_list_end().unwrap();
        mteq!(o_prot, o_prot.write_list_end_len());

        let mut identifier = TSetIdentifier::new(TType::I16, 0);
        o_prot.write_set_begin(identifier).unwrap();
        mteq!(o_prot, o_prot.write_set_begin_len(identifier));
        o_prot.write_set_end().unwrap();
        mteq!(o_prot, o_prot.write_set_end_len());
        identifier.size = 1;
        o_prot.write_set_begin(identifier).unwrap();
        mteq!(o_prot, o_prot.write_set_begin_len(identifier));
        o_prot.write_set_end().unwrap();
        mteq!(o_prot, o_prot.write_set_end_len());

        let mut identifier = TMapIdentifier::new(TType::Binary, TType::I64, 0);
        o_prot.write_map_begin(identifier).unwrap();
        mteq!(o_prot, o_prot.write_map_begin_len(identifier));
        o_prot.write_map_end().unwrap();
        mteq!(o_prot, o_prot.write_map_end_len());
        identifier.size = 1;
        o_prot.write_map_begin(identifier).unwrap();
        mteq!(o_prot, o_prot.write_map_begin_len(identifier));
        o_prot.write_map_end().unwrap();
        mteq!(o_prot, o_prot.write_map_end_len());
    }

    #[test]
    fn must_write_message_begin_largest_maximum_positive_sequence_number() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_message_begin(&TMessageIdentifier::new(
            "bar".into(),
            TMessageType::Reply,
            i32::MAX
        )));

        #[rustfmt::skip]
        let expected: [u8; 11] = [
            0x82, /* protocol ID */
            0x41, /* message type | protocol version */
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0x07, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x62,
            0x61,
            0x72 /* "bar" */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_read_message_begin_largest_maximum_positive_sequence_number() {
        let mut trans = BytesMut::new();
        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        #[rustfmt::skip]
        let source_bytes: [u8; 11] = [
            0x82, /* protocol ID */
            0x41, /* message type | protocol version */
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0x07, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x62,
            0x61,
            0x72 /* "bar" */,
        ];

        i_prot.trans.put_slice(&source_bytes);

        let expected = TMessageIdentifier::new("bar".into(), TMessageType::Reply, i32::MAX);
        let res = assert_success!(i_prot.read_message_begin());

        assert_eq!(&expected, &res);
    }

    #[test]
    fn must_write_message_begin_positive_sequence_number_0() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_message_begin(&TMessageIdentifier::new(
            "foo".into(),
            TMessageType::Call,
            431
        )));

        #[rustfmt::skip]
        let expected: [u8; 8] = [
            0x82, /* protocol ID */
            0x21, /* message type | protocol version */
            0xAF,
            0x03, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x66,
            0x6F,
            0x6F /* "foo" */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_read_message_begin_positive_sequence_number_0() {
        let mut trans = BytesMut::new();
        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        #[rustfmt::skip]
        let source_bytes: [u8; 8] = [
            0x82, /* protocol ID */
            0x21, /* message type | protocol version */
            0xAF,
            0x03, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x66,
            0x6F,
            0x6F /* "foo" */,
        ];

        i_prot.trans.put_slice(&source_bytes);

        let expected = TMessageIdentifier::new("foo".into(), TMessageType::Call, 431);
        let res = assert_success!(i_prot.read_message_begin());

        assert_eq!(&expected, &res);
    }

    #[test]
    fn must_write_message_begin_positive_sequence_number_1() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_message_begin(&TMessageIdentifier::new(
            "bar".into(),
            TMessageType::Reply,
            991_828
        )));

        #[rustfmt::skip]
        let expected: [u8; 9] = [
            0x82, /* protocol ID */
            0x41, /* message type | protocol version */
            0xD4,
            0xC4,
            0x3C, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x62,
            0x61,
            0x72 /* "bar" */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_read_message_begin_positive_sequence_number_1() {
        let mut trans = BytesMut::new();
        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        #[rustfmt::skip]
        let source_bytes: [u8; 9] = [
            0x82, /* protocol ID */
            0x41, /* message type | protocol version */
            0xD4,
            0xC4,
            0x3C, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x62,
            0x61,
            0x72 /* "bar" */,
        ];

        i_prot.trans.put_slice(&source_bytes);

        let expected = TMessageIdentifier::new("bar".into(), TMessageType::Reply, 991_828);
        let res = assert_success!(i_prot.read_message_begin());

        assert_eq!(&expected, &res);
    }

    #[test]
    fn must_write_message_begin_zero_sequence_number() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_message_begin(&TMessageIdentifier::new(
            "bar".into(),
            TMessageType::Reply,
            0
        )));

        #[rustfmt::skip]
        let expected: [u8; 7] = [
            0x82, /* protocol ID */
            0x41, /* message type | protocol version */
            0x00, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x62,
            0x61,
            0x72 /* "bar" */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_read_message_begin_zero_sequence_number() {
        let mut trans = BytesMut::new();
        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        #[rustfmt::skip]
        let source_bytes: [u8; 7] = [
            0x82, /* protocol ID */
            0x41, /* message type | protocol version */
            0x00, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x62,
            0x61,
            0x72 /* "bar" */,
        ];

        i_prot.trans.put_slice(&source_bytes);

        let expected = TMessageIdentifier::new("bar".into(), TMessageType::Reply, 0);
        let res = assert_success!(i_prot.read_message_begin());

        assert_eq!(&expected, &res);
    }

    #[test]
    fn must_write_message_begin_largest_minimum_negative_sequence_number() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_message_begin(&TMessageIdentifier::new(
            "bar".into(),
            TMessageType::Reply,
            i32::MIN
        )));

        // two's complement notation of i32::MIN =
        // 1000_0000_0000_0000_0000_0000_0000_0000
        #[rustfmt::skip]
        let expected: [u8; 11] = [
            0x82, /* protocol ID */
            0x41, /* message type | protocol version */
            0x80,
            0x80,
            0x80,
            0x80,
            0x08, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x62,
            0x61,
            0x72 /* "bar" */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_read_message_begin_largest_minimum_negative_sequence_number() {
        let mut trans = BytesMut::new();
        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // two's complement notation of i32::MIN =
        // 1000_0000_0000_0000_0000_0000_0000_0000
        #[rustfmt::skip]
        let source_bytes: [u8; 11] = [
            0x82, /* protocol ID */
            0x41, /* message type | protocol version */
            0x80,
            0x80,
            0x80,
            0x80,
            0x08, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x62,
            0x61,
            0x72 /* "bar" */,
        ];

        i_prot.trans.put_slice(&source_bytes);

        let expected = TMessageIdentifier::new("bar".into(), TMessageType::Reply, i32::MIN);
        let res = assert_success!(i_prot.read_message_begin());

        assert_eq!(&expected, &res);
    }

    #[test]
    fn must_write_message_begin_negative_sequence_number_0() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_message_begin(&TMessageIdentifier::new(
            "foo".into(),
            TMessageType::Call,
            -431
        )));

        // signed two's complement of -431 = 1111_1111_1111_1111_1111_1110_0101_0001
        #[rustfmt::skip]
        let expected: [u8; 11] = [
            0x82, /* protocol ID */
            0x21, /* message type | protocol version */
            0xD1,
            0xFC,
            0xFF,
            0xFF,
            0x0F, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x66,
            0x6F,
            0x6F /* "foo" */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_read_message_begin_negative_sequence_number_0() {
        let mut trans = BytesMut::new();
        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // signed two's complement of -431 = 1111_1111_1111_1111_1111_1110_0101_0001
        #[rustfmt::skip]
        let source_bytes: [u8; 11] = [
            0x82, /* protocol ID */
            0x21, /* message type | protocol version */
            0xD1,
            0xFC,
            0xFF,
            0xFF,
            0x0F, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x66,
            0x6F,
            0x6F /* "foo" */,
        ];

        i_prot.trans.put_slice(&source_bytes);

        let expected = TMessageIdentifier::new("foo".into(), TMessageType::Call, -431);
        let res = assert_success!(i_prot.read_message_begin());

        assert_eq!(&expected, &res);
    }

    #[test]
    fn must_write_message_begin_negative_sequence_number_1() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_message_begin(&TMessageIdentifier::new(
            "foo".into(),
            TMessageType::Call,
            -73_184_125
        )));

        // signed two's complement of -73184125 =
        // 1111_1011_1010_0011_0100_1100_1000_0011
        #[rustfmt::skip]
        let expected: [u8; 11] = [
            0x82, /* protocol ID */
            0x21, /* message type | protocol version */
            0x83,
            0x99,
            0x8D,
            0xDD,
            0x0F, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x66,
            0x6F,
            0x6F /* "foo" */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_read_message_begin_negative_sequence_number_1() {
        let mut trans = BytesMut::new();
        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // signed two's complement of -73184125 =
        // 1111_1011_1010_0011_0100_1100_1000_0011
        #[rustfmt::skip]
        let source_bytes: [u8; 11] = [
            0x82, /* protocol ID */
            0x21, /* message type | protocol version */
            0x83,
            0x99,
            0x8D,
            0xDD,
            0x0F, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x66,
            0x6F,
            0x6F /* "foo" */,
        ];

        i_prot.trans.put_slice(&source_bytes);

        let expected = TMessageIdentifier::new("foo".into(), TMessageType::Call, -73_184_125);
        let res = assert_success!(i_prot.read_message_begin());

        assert_eq!(&expected, &res);
    }

    #[test]
    fn must_write_message_begin_negative_sequence_number_2() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_message_begin(&TMessageIdentifier::new(
            "foo".into(),
            TMessageType::Call,
            -1_073_741_823
        )));

        // signed two's complement of -1073741823 =
        // 1100_0000_0000_0000_0000_0000_0000_0001
        #[rustfmt::skip]
        let expected: [u8; 11] = [
            0x82, /* protocol ID */
            0x21, /* message type | protocol version */
            0x81,
            0x80,
            0x80,
            0x80,
            0x0C, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x66,
            0x6F,
            0x6F /* "foo" */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_read_message_begin_negative_sequence_number_2() {
        let mut trans = BytesMut::new();
        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // signed two's complement of -1073741823 =
        // 1100_0000_0000_0000_0000_0000_0000_0001
        #[rustfmt::skip]
        let source_bytes: [u8; 11] = [
            0x82, /* protocol ID */
            0x21, /* message type | protocol version */
            0x81,
            0x80,
            0x80,
            0x80,
            0x0C, /* non-zig-zag varint sequence number */
            0x03, /* message-name length */
            0x66,
            0x6F,
            0x6F, /* "foo" */
        ];

        i_prot.trans.put_slice(&source_bytes);

        let expected = TMessageIdentifier::new("foo".into(), TMessageType::Call, -1_073_741_823);
        let res = assert_success!(i_prot.read_message_begin());

        assert_eq!(&expected, &res);
    }

    #[test]
    fn must_round_trip_upto_i64_maxvalue() {
        // See https://issues.apache.org/jira/browse/THRIFT-5131
        for i in 0..64 {
            let mut trans = BytesMut::new();
            let mut o_prot = test_output_prot_bytesmut(&mut trans);

            let val: i64 = ((1u64 << i) - 1) as i64;
            o_prot.write_field_begin(TType::I64, 1).unwrap();
            o_prot.write_i64(val).unwrap();
            o_prot.write_field_end().unwrap();
            o_prot.flush().unwrap();
            // println!("trans {:?}", trans);
            let mut i_prot = test_input_prot_bytesmut(&mut trans);
            i_prot.read_field_begin().unwrap();
            assert_eq!(val, i_prot.read_i64().unwrap());
        }
    }

    #[test]
    fn must_round_trip_message_begin() {
        let mut trans = BytesMut::new();

        let mut o_prot = test_output_prot_bytesmut(&mut trans);
        let ident = TMessageIdentifier::new("service_call".into(), TMessageType::Call, 1_283_948);
        assert_success!(o_prot.write_message_begin(&ident));

        let mut i_prot = test_input_prot_bytesmut(&mut trans);
        let res = assert_success!(i_prot.read_message_begin());
        assert_eq!(&res, &ident);
    }

    #[test]
    fn must_write_message_end() {
        assert_no_write(BytesMut::new(), |o| o.write_message_end());
    }

    // NOTE: structs and fields are tested together
    //

    #[test]
    fn must_write_struct_with_delta_fields() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with tiny field ids
        // since they're small the field ids will be encoded as deltas

        // since this is the first field (and it's zero) it gets the full varint write
        assert_success!(o_prot.write_field_begin(TType::I8, 0));
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it can be encoded as a delta
        assert_success!(o_prot.write_field_begin(TType::I16, 4));
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it can be encoded as a delta
        assert_success!(o_prot.write_field_begin(TType::List, 9));
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        #[rustfmt::skip]
        let expected: [u8; 5] = [
            0x03, /* field type */
            0x00, /* first field id */
            0x44, /* field delta (4) | field type */
            0x59, /* field delta (5) | field type */
            0x00 /* field stop */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_round_trip_struct_with_delta_fields() {
        // let (mut i_prot, mut o_prot) = test_objects();

        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with tiny field ids
        // since they're small the field ids will be encoded as deltas

        // since this is the first field (and it's zero) it gets the full varint write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::I8, 0);
        assert_success!(
            o_prot.write_field_begin(field_ident_1.field_type, field_ident_1.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it can be encoded as a delta
        let field_ident_2 = TFieldIdentifier::new("foo", TType::I16, 4);
        assert_success!(
            o_prot.write_field_begin(field_ident_2.field_type, field_ident_2.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it can be encoded as a delta
        let field_ident_3 = TFieldIdentifier::new("foo", TType::List, 9);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // read the struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_1 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_1,
            TFieldIdentifier {
                name: None,
                ..field_ident_1
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_2 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_2,
            TFieldIdentifier {
                name: None,
                ..field_ident_2
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_3 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_3,
            TFieldIdentifier {
                name: None,
                ..field_ident_3
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_4 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_4,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );

        assert_success!(i_prot.read_struct_end());
    }

    #[test]
    fn must_write_struct_with_non_zero_initial_field_and_delta_fields() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with tiny field ids
        // since they're small the field ids will be encoded as deltas

        // gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I32, 1));
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it can be encoded as a delta
        assert_success!(o_prot.write_field_begin(TType::Set, 2));
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it can be encoded as a delta
        assert_success!(o_prot.write_field_begin(TType::Binary, 6));
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        #[rustfmt::skip]
        let expected: [u8; 4] = [
            0x15, /* field delta (1) | field type */
            0x1A, /* field delta (1) | field type */
            0x48, /* field delta (4) | field type */
            0x00 /* field stop */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_round_trip_struct_with_non_zero_initial_field_and_delta_fields() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with tiny field ids
        // since they're small the field ids will be encoded as deltas

        // gets a delta write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::I32, 1);
        assert_success!(
            o_prot.write_field_begin(field_ident_1.field_type, field_ident_1.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it can be encoded as a delta
        let field_ident_2 = TFieldIdentifier::new("foo", TType::Set, 2);
        assert_success!(
            o_prot.write_field_begin(field_ident_2.field_type, field_ident_2.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it can be encoded as a delta
        let field_ident_3 = TFieldIdentifier::new("foo", TType::Binary, 6);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // read the struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_1 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_1,
            TFieldIdentifier {
                name: None,
                ..field_ident_1
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_2 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_2,
            TFieldIdentifier {
                name: None,
                ..field_ident_2
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_3 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_3,
            TFieldIdentifier {
                name: None,
                ..field_ident_3
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_4 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_4,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );

        assert_success!(i_prot.read_struct_end());
    }

    #[test]
    fn must_write_struct_with_long_fields() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with field ids that cannot be encoded as deltas

        // since this is the first field (and it's zero) it gets the full varint write
        assert_success!(o_prot.write_field_begin(TType::I32, 0));
        assert_success!(o_prot.write_field_end());

        // since this delta is > 15 it is encoded as a zig-zag varint
        assert_success!(o_prot.write_field_begin(TType::I64, 16));
        assert_success!(o_prot.write_field_end());

        // since this delta is > 15 it is encoded as a zig-zag varint
        assert_success!(o_prot.write_field_begin(TType::Set, 99));
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        #[rustfmt::skip]
        let expected: [u8; 8] = [
            0x05, /* field type */
            0x00, /* first field id */
            0x06, /* field type */
            0x20, /* zig-zag varint field id */
            0x0A, /* field type */
            0xC6,
            0x01, /* zig-zag varint field id */
            0x00 /* field stop */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[test]
    fn must_round_trip_struct_with_long_fields() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with field ids that cannot be encoded as deltas

        // since this is the first field (and it's zero) it gets the full varint write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::I32, 0);
        assert_success!(
            o_prot.write_field_begin(field_ident_1.field_type, field_ident_1.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this delta is > 15 it is encoded as a zig-zag varint
        let field_ident_2 = TFieldIdentifier::new("foo", TType::I64, 16);
        assert_success!(
            o_prot.write_field_begin(field_ident_2.field_type, field_ident_2.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this delta is > 15 it is encoded as a zig-zag varint
        let field_ident_3 = TFieldIdentifier::new("foo", TType::Set, 99);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // read the struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_1 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_1,
            TFieldIdentifier {
                name: None,
                ..field_ident_1
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_2 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_2,
            TFieldIdentifier {
                name: None,
                ..field_ident_2
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_3 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_3,
            TFieldIdentifier {
                name: None,
                ..field_ident_3
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_4 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_4,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );

        assert_success!(i_prot.read_struct_end());
    }

    #[test]
    fn must_write_struct_with_mix_of_long_and_delta_fields() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with field ids that cannot be encoded as deltas

        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I64, 1));
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I32, 9));
        assert_success!(o_prot.write_field_end());

        // since this delta is > 15 it is encoded as a zig-zag varint
        assert_success!(o_prot.write_field_begin(TType::Set, 1000));
        assert_success!(o_prot.write_field_end());

        // since this delta is > 15 it is encoded as a zig-zag varint
        assert_success!(o_prot.write_field_begin(TType::Set, 2001));
        assert_success!(o_prot.write_field_end());

        // since this is only 3 up from the previous it is recorded as a delta
        assert_success!(o_prot.write_field_begin(TType::Set, 2004));
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        #[rustfmt::skip]
        let expected: [u8; 10] = [
            0x16, /* field delta (1) | field type */
            0x85, /* field delta (8) | field type */
            0x0A, /* field type */
            0xD0,
            0x0F, /* zig-zag varint field id */
            0x0A, /* field type */
            0xA2,
            0x1F, /* zig-zag varint field id */
            0x3A, /* field delta (3) | field type */
            0x00 /* field stop */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn must_round_trip_struct_with_mix_of_long_and_delta_fields() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        let struct_ident = TStructIdentifier::new("foo");
        assert_success!(o_prot.write_struct_begin(&struct_ident));

        // write three fields with field ids that cannot be encoded as deltas

        // since the delta is > 0 and < 15 it gets a delta write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::I64, 1);
        assert_success!(
            o_prot.write_field_begin(field_ident_1.field_type, field_ident_1.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it gets a delta write
        let field_ident_2 = TFieldIdentifier::new("foo", TType::I32, 9);
        assert_success!(
            o_prot.write_field_begin(field_ident_2.field_type, field_ident_2.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this delta is > 15 it is encoded as a zig-zag varint
        let field_ident_3 = TFieldIdentifier::new("foo", TType::Set, 1000);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this delta is > 15 it is encoded as a zig-zag varint
        let field_ident_4 = TFieldIdentifier::new("foo", TType::Set, 2001);
        assert_success!(
            o_prot.write_field_begin(field_ident_4.field_type, field_ident_4.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // since this is only 3 up from the previous it is recorded as a delta
        let field_ident_5 = TFieldIdentifier::new("foo", TType::Set, 2004);
        assert_success!(
            o_prot.write_field_begin(field_ident_5.field_type, field_ident_5.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // read the struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_1 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_1,
            TFieldIdentifier {
                name: None,
                ..field_ident_1
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_2 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_2,
            TFieldIdentifier {
                name: None,
                ..field_ident_2
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_3 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_3,
            TFieldIdentifier {
                name: None,
                ..field_ident_3
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_4 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_4,
            TFieldIdentifier {
                name: None,
                ..field_ident_4
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_5 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_5,
            TFieldIdentifier {
                name: None,
                ..field_ident_5
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_6 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_6,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );

        assert_success!(i_prot.read_struct_end());
    }

    #[test]
    fn must_write_nested_structs_0() {
        // last field of the containing struct is a delta
        // first field of the the contained struct is a delta

        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I64, 1));
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I32, 9));
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I8, 7));
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::Double, 24));
        assert_success!(o_prot.write_field_end());

        // end contained struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        // end containing struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        #[rustfmt::skip]
        let expected: [u8; 7] = [
            0x16, /* field delta (1) | field type */
            0x85, /* field delta (8) | field type */
            0x73, /* field delta (7) | field type */
            0x07, /* field type */
            0x30, /* zig-zag varint field id */
            0x00, /* field stop - contained */
            0x00 /* field stop - containing */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn must_round_trip_nested_structs_0() {
        // last field of the containing struct is a delta
        // first field of the the contained struct is a delta

        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::I64, 1);
        assert_success!(
            o_prot.write_field_begin(field_ident_1.field_type, field_ident_1.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 0 and < 15 it gets a delta write
        let field_ident_2 = TFieldIdentifier::new("foo", TType::I32, 9);
        assert_success!(
            o_prot.write_field_begin(field_ident_2.field_type, field_ident_2.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        let field_ident_3 = TFieldIdentifier::new("foo", TType::I8, 7);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since this delta > 15 it gets a full write
        let field_ident_4 = TFieldIdentifier::new("foo", TType::Double, 24);
        assert_success!(
            o_prot.write_field_begin(field_ident_4.field_type, field_ident_4.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // end contained struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        // end containing struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // read containing struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_1 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_1,
            TFieldIdentifier {
                name: None,
                ..field_ident_1
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_2 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_2,
            TFieldIdentifier {
                name: None,
                ..field_ident_2
            }
        );
        assert_success!(i_prot.read_field_end());

        // read contained struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_3 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_3,
            TFieldIdentifier {
                name: None,
                ..field_ident_3
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_4 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_4,
            TFieldIdentifier {
                name: None,
                ..field_ident_4
            }
        );
        assert_success!(i_prot.read_field_end());

        // end contained struct
        let read_ident_6 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_6,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );
        assert_success!(i_prot.read_struct_end());

        // end containing struct
        let read_ident_7 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_7,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );
        assert_success!(i_prot.read_struct_end());
    }

    #[test]
    fn must_write_nested_structs_1() {
        // last field of the containing struct is a delta
        // first field of the the contained struct is a full write

        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I64, 1));
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I32, 9));
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::Double, 24));
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I8, 27));
        assert_success!(o_prot.write_field_end());

        // end contained struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        // end containing struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        #[rustfmt::skip]
        let expected: [u8; 7] = [
            0x16, /* field delta (1) | field type */
            0x85, /* field delta (8) | field type */
            0x07, /* field type */
            0x30, /* zig-zag varint field id */
            0x33, /* field delta (3) | field type */
            0x00, /* field stop - contained */
            0x00 /* field stop - containing */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn must_round_trip_nested_structs_1() {
        // last field of the containing struct is a delta
        // first field of the the contained struct is a full write

        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::I64, 1);
        assert_success!(
            o_prot.write_field_begin(field_ident_1.field_type, field_ident_1.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 0 and < 15 it gets a delta write
        let field_ident_2 = TFieldIdentifier::new("foo", TType::I32, 9);
        assert_success!(
            o_prot.write_field_begin(field_ident_2.field_type, field_ident_2.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since this delta > 15 it gets a full write
        let field_ident_3 = TFieldIdentifier::new("foo", TType::Double, 24);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        let field_ident_4 = TFieldIdentifier::new("foo", TType::I8, 27);
        assert_success!(
            o_prot.write_field_begin(field_ident_4.field_type, field_ident_4.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // end contained struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        // end containing struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // read containing struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_1 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_1,
            TFieldIdentifier {
                name: None,
                ..field_ident_1
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_2 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_2,
            TFieldIdentifier {
                name: None,
                ..field_ident_2
            }
        );
        assert_success!(i_prot.read_field_end());

        // read contained struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_3 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_3,
            TFieldIdentifier {
                name: None,
                ..field_ident_3
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_4 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_4,
            TFieldIdentifier {
                name: None,
                ..field_ident_4
            }
        );
        assert_success!(i_prot.read_field_end());

        // end contained struct
        let read_ident_6 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_6,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );
        assert_success!(i_prot.read_struct_end());

        // end containing struct
        let read_ident_7 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_7,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );
        assert_success!(i_prot.read_struct_end());
    }

    #[test]
    fn must_write_nested_structs_2() {
        // last field of the containing struct is a full write
        // first field of the the contained struct is a delta write

        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I64, 1));
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::Binary, 21));
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since this delta > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::Double, 7));
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I8, 10));
        assert_success!(o_prot.write_field_end());

        // end contained struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        // end containing struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        #[rustfmt::skip]
        let expected: [u8; 7] = [
            0x16, /* field delta (1) | field type */
            0x08, /* field type */
            0x2A, /* zig-zag varint field id */
            0x77, /* field delta(7) | field type */
            0x33, /* field delta (3) | field type */
            0x00, /* field stop - contained */
            0x00 /* field stop - containing */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn must_round_trip_nested_structs_2() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::I64, 1);
        assert_success!(
            o_prot.write_field_begin(field_ident_1.field_type, field_ident_1.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 15 it gets a full write
        let field_ident_2 = TFieldIdentifier::new("foo", TType::Binary, 21);
        assert_success!(
            o_prot.write_field_begin(field_ident_2.field_type, field_ident_2.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since this delta > 0 and < 15 it gets a delta write
        let field_ident_3 = TFieldIdentifier::new("foo", TType::Double, 7);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        let field_ident_4 = TFieldIdentifier::new("foo", TType::I8, 10);
        assert_success!(
            o_prot.write_field_begin(field_ident_4.field_type, field_ident_4.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // end contained struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        // end containing struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // read containing struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_1 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_1,
            TFieldIdentifier {
                name: None,
                ..field_ident_1
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_2 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_2,
            TFieldIdentifier {
                name: None,
                ..field_ident_2
            }
        );
        assert_success!(i_prot.read_field_end());

        // read contained struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_3 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_3,
            TFieldIdentifier {
                name: None,
                ..field_ident_3
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_4 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_4,
            TFieldIdentifier {
                name: None,
                ..field_ident_4
            }
        );
        assert_success!(i_prot.read_field_end());

        // end contained struct
        let read_ident_6 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_6,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );
        assert_success!(i_prot.read_struct_end());

        // end containing struct
        let read_ident_7 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_7,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );
        assert_success!(i_prot.read_struct_end());
    }

    #[test]
    fn must_write_nested_structs_3() {
        // last field of the containing struct is a full write
        // first field of the the contained struct is a full write

        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I64, 1));
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::Binary, 21));
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::Double, 21));
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I8, 27));
        assert_success!(o_prot.write_field_end());

        // end contained struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        // end containing struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        #[rustfmt::skip]
        let expected: [u8; 8] = [
            0x16, /* field delta (1) | field type */
            0x08, /* field type */
            0x2A, /* zig-zag varint field id */
            0x07, /* field type */
            0x2A, /* zig-zag varint field id */
            0x63, /* field delta (6) | field type */
            0x00, /* field stop - contained */
            0x00 /* field stop - containing */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn must_round_trip_nested_structs_3() {
        // last field of the containing struct is a full write
        // first field of the the contained struct is a full write

        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::I64, 1);
        assert_success!(
            o_prot.write_field_begin(field_ident_1.field_type, field_ident_1.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 15 it gets a full write
        let field_ident_2 = TFieldIdentifier::new("foo", TType::Binary, 21);
        assert_success!(
            o_prot.write_field_begin(field_ident_2.field_type, field_ident_2.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since this delta > 15 it gets a full write
        let field_ident_3 = TFieldIdentifier::new("foo", TType::Double, 21);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        let field_ident_4 = TFieldIdentifier::new("foo", TType::I8, 27);
        assert_success!(
            o_prot.write_field_begin(field_ident_4.field_type, field_ident_4.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // end contained struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        // end containing struct
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // read containing struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_1 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_1,
            TFieldIdentifier {
                name: None,
                ..field_ident_1
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_2 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_2,
            TFieldIdentifier {
                name: None,
                ..field_ident_2
            }
        );
        assert_success!(i_prot.read_field_end());

        // read contained struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_3 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_3,
            TFieldIdentifier {
                name: None,
                ..field_ident_3
            }
        );
        assert_success!(i_prot.read_field_end());

        let read_ident_4 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_4,
            TFieldIdentifier {
                name: None,
                ..field_ident_4
            }
        );
        assert_success!(i_prot.read_field_end());

        // end contained struct
        let read_ident_6 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_6,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );
        assert_success!(i_prot.read_struct_end());

        // end containing struct
        let read_ident_7 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_7,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );
        assert_success!(i_prot.read_struct_end());
    }

    #[test]
    fn must_write_bool_field() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with field ids that cannot be encoded as deltas

        // since the delta is > 0 and < 16 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::Bool, 1));
        assert_success!(o_prot.write_bool(true));
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::Bool, 9));
        assert_success!(o_prot.write_bool(false));
        assert_success!(o_prot.write_field_end());

        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::Bool, 26));
        assert_success!(o_prot.write_bool(true));
        assert_success!(o_prot.write_field_end());

        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::Bool, 45));
        assert_success!(o_prot.write_bool(false));
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        #[rustfmt::skip]
        let expected: [u8; 7] = [
            0x11, /* field delta (1) | true */
            0x82, /* field delta (8) | false */
            0x01, /* true */
            0x34, /* field id */
            0x02, /* false */
            0x5A, /* field id */
            0x00 /* stop field */,
        ];

        assert_eq_written_bytes!(o_prot, expected);
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn must_round_trip_bool_field() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        // no bytes should be written however
        let struct_ident = TStructIdentifier::new("foo");
        assert_success!(o_prot.write_struct_begin(&struct_ident));

        // write two fields

        // since the delta is > 0 and < 16 it gets a delta write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::Bool, 1);
        assert_success!(
            o_prot.write_field_begin(field_ident_1.field_type, field_ident_1.id.unwrap())
        );
        assert_success!(o_prot.write_bool(true));
        assert_success!(o_prot.write_field_end());

        // since this delta > 0 and < 15 it gets a delta write
        let field_ident_2 = TFieldIdentifier::new("foo", TType::Bool, 9);
        assert_success!(
            o_prot.write_field_begin(field_ident_2.field_type, field_ident_2.id.unwrap())
        );
        assert_success!(o_prot.write_bool(false));
        assert_success!(o_prot.write_field_end());

        // since this delta > 15 it gets a full write
        let field_ident_3 = TFieldIdentifier::new("foo", TType::Bool, 26);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_bool(true));
        assert_success!(o_prot.write_field_end());

        // since this delta > 15 it gets a full write
        let field_ident_4 = TFieldIdentifier::new("foo", TType::Bool, 45);
        assert_success!(
            o_prot.write_field_begin(field_ident_4.field_type, field_ident_4.id.unwrap())
        );
        assert_success!(o_prot.write_bool(false));
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot_bytesmut(&mut trans);

        // read the struct back
        assert_success!(i_prot.read_struct_begin());

        let read_ident_1 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_1,
            TFieldIdentifier {
                name: None,
                ..field_ident_1
            }
        );
        let read_value_1 = assert_success!(i_prot.read_bool());
        assert_eq!(read_value_1, true);
        assert_success!(i_prot.read_field_end());

        let read_ident_2 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_2,
            TFieldIdentifier {
                name: None,
                ..field_ident_2
            }
        );
        let read_value_2 = assert_success!(i_prot.read_bool());
        assert_eq!(read_value_2, false);
        assert_success!(i_prot.read_field_end());

        let read_ident_3 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_3,
            TFieldIdentifier {
                name: None,
                ..field_ident_3
            }
        );
        let read_value_3 = assert_success!(i_prot.read_bool());
        assert_eq!(read_value_3, true);
        assert_success!(i_prot.read_field_end());

        let read_ident_4 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_4,
            TFieldIdentifier {
                name: None,
                ..field_ident_4
            }
        );
        let read_value_4 = assert_success!(i_prot.read_bool());
        assert_eq!(read_value_4, false);
        assert_success!(i_prot.read_field_end());

        let read_ident_5 = assert_success!(i_prot.read_field_begin());
        assert_eq!(
            read_ident_5,
            TFieldIdentifier {
                name: None,
                field_type: TType::Stop,
                id: None,
            }
        );

        assert_success!(i_prot.read_struct_end());
    }

    #[test]
    #[should_panic]
    fn must_fail_if_write_field_end_without_writing_bool_value() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));
        assert_success!(o_prot.write_field_begin(TType::Bool, 1));
        o_prot.write_field_end().unwrap();
    }

    #[test]
    #[should_panic]
    fn must_fail_if_write_stop_field_without_writing_bool_value() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));
        assert_success!(o_prot.write_field_begin(TType::Bool, 1));
        o_prot.write_field_stop().unwrap();
    }

    #[test]
    #[should_panic]
    fn must_fail_if_write_struct_end_without_writing_bool_value() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);

        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));
        assert_success!(o_prot.write_field_begin(TType::Bool, 1));
        o_prot.write_struct_end().unwrap();
    }

    #[test]
    #[should_panic]
    fn must_fail_if_write_struct_end_without_any_fields() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot_bytesmut(&mut trans);
        o_prot.write_struct_end().unwrap();
    }

    #[test]
    fn must_write_field_end() {
        let trans = BytesMut::new();
        assert_no_write(trans, |o| o.write_field_end());
    }

    fn assert_no_write<B, F>(mut trans: B, mut write_fn: F)
    where
        B: bytes::Buf,
        F: FnMut(&mut TCompactOutputProtocol<&mut B>) -> Result<(), EncodeError>,
    {
        let mut o_prot = TCompactOutputProtocol::new(&mut trans, false);
        assert!(write_fn(&mut o_prot).is_ok());

        let mut out = Vec::new();
        o_prot.trans.reader().read_to_end(&mut out).unwrap();
        assert_eq!(out.len(), 0);
    }
}
