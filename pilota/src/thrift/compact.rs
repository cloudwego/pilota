// SPDX-License-Identifier: MIT OR Apache-2.0
//
// https://github.com/apache/thrift/blob/ec5e17714a1f9da34173749fc01eea33c7f6af62/lib/rs/src/protocol/compact.rs

use bytes::BytesMut;
use integer_encoding::{VarInt, VarIntAsyncReader};
use tokio::io::{AsyncRead, AsyncReadExt};

use super::{
    error::{Error, ProtocolErrorKind},
    new_protocol_error,
    rw_ext::{ReadExt, WriteExt},
    TFieldIdentifier, TInputProtocol, TLengthProtocol, TListIdentifier, TMapIdentifier,
    TMessageIdentifier, TMessageType, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType,
    MAXIMUM_SKIP_DEPTH,
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
}

const COMPACT_BOOLEAN_TRUE: u8 = TCompactType::BooleanTrue as u8;
const COMPACT_BOOLEAN_FALSE: u8 = TCompactType::BooleanFalse as u8;

impl TryFrom<u8> for TCompactType {
    type Error = Error;
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
            _ => Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid compact type {:?}", value),
            )),
        }
    }
}

impl TryFrom<TType> for TCompactType {
    type Error = Error;
    #[inline]
    fn try_from(value: TType) -> Result<Self, Self::Error> {
        match value {
            TType::Stop => Ok(Self::Stop),
            TType::Bool => Ok(Self::BooleanTrue),
            TType::I08 => Ok(Self::Byte),
            TType::I16 => Ok(Self::I16),
            TType::I32 => Ok(Self::I32),
            TType::I64 => Ok(Self::I64),
            TType::Double => Ok(Self::Double),
            TType::String => Ok(Self::Binary),
            TType::List => Ok(Self::List),
            TType::Set => Ok(Self::Set),
            TType::Map => Ok(Self::Map),
            TType::Struct => Ok(Self::Struct),
            _ => Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid ttype {:?}", value),
            )),
        }
    }
}

impl TryFrom<TCompactType> for TType {
    type Error = Error;
    #[inline]
    fn try_from(value: TCompactType) -> Result<Self, Self::Error> {
        match value {
            TCompactType::Stop => Ok(Self::Stop),
            TCompactType::BooleanTrue => Ok(Self::Bool),
            TCompactType::BooleanFalse => Ok(Self::Bool),
            TCompactType::Byte => Ok(TType::I08),
            TCompactType::I16 => Ok(TType::I16),
            TCompactType::I32 => Ok(TType::I32),
            TCompactType::I64 => Ok(TType::I64),
            TCompactType::Double => Ok(TType::Double),
            TCompactType::Binary => Ok(TType::String),
            TCompactType::List => Ok(TType::List),
            TCompactType::Set => Ok(TType::Set),
            TCompactType::Map => Ok(TType::Map),
            TCompactType::Struct => Ok(TType::Struct),
            // _ => Err(new_protocol_error(ProtocolErrorKind::InvalidData, format!("invalid
            // tcompacttype {:?}", value)))
        }
    }
}

static COMPACT_PROTOCOL_ID: u8 = 0x082;
static COMPACT_VERSION: u8 = 1;
static COMPACT_VERSION_MASK: u8 = 0x1f;
static COMPACT_TYPE_MASK: u8 = 0x0E0;
// static COMPACT_TYPE_BITS: u8 = 0x07;
static COMPACT_TYPE_SHIFT_AMOUNT: u8 = 5;

#[inline]
fn tcompact_get_ttype(ct: TCompactType) -> Result<TType, Error> {
    Ok(ct.try_into().map_err(|_| {
        new_protocol_error(
            ProtocolErrorKind::InvalidData,
            format!("don't know what type: {:?}", ct),
        )
    })?)
}

#[inline]
fn tcompact_get_compact(tt: TType) -> Result<TCompactType, Error> {
    Ok(tt.try_into().map_err(|_| {
        new_protocol_error(
            ProtocolErrorKind::InvalidData,
            format!("invalid ttype {:?}", tt),
        )
    })?)
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
}

impl<T> TCompactOutputProtocol<T> {
    pub fn new(trans: T) -> Self {
        Self {
            trans,
            write_field_id_stack: Vec::with_capacity(24),
            last_write_field_id: 0,
            pending_write_bool_field_identifier: None,
        }
    }
}

impl<T> TLengthProtocol for TCompactOutputProtocol<T> {
    #[inline]
    fn write_message_begin_len(&self, ident: &TMessageIdentifier) -> usize {
        2 + VarInt::required_space(ident.sequence_number as u32)
    }
    #[inline]
    fn write_message_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_struct_begin_len(&self, _ident: &TStructIdentifier) -> usize {
        0
    }
    #[inline]
    fn write_struct_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_field_begin_len(&self, _ident: &TFieldIdentifier) -> usize {
        todo!()
    }
    #[inline]
    fn write_field_end_len(&self) -> usize {
        0
    }
    #[inline]
    fn write_field_stop_len(&self) -> usize {
        1
    }

    #[inline]
    fn write_bool_len(&self, _b: bool) -> usize {
        todo!()
    }
    #[inline]
    fn write_bytes_len(&self, b: &[u8]) -> usize {
        let sz = VarInt::required_space(b.len() as u32);
        if b.len() > 0 {
            sz + b.len()
        } else {
            sz
        }
    }
    #[inline]
    fn write_byte_len(&self, _b: u8) -> usize {
        1
    }
    #[inline]
    fn write_i8_len(&self, _i: i8) -> usize {
        1
    }
    #[inline]
    fn write_i16_len(&self, i: i16) -> usize {
        VarInt::required_space(i)
    }
    #[inline]
    fn write_i32_len(&self, i: i32) -> usize {
        VarInt::required_space(i)
    }
    #[inline]
    fn write_i64_len(&self, i: i64) -> usize {
        VarInt::required_space(i)
    }
    #[inline]
    fn write_double_len(&self, d: f64) -> usize {
        d.to_le_bytes().len()
    }

    #[inline]
    fn write_string_len(&self, s: &str) -> usize {
        self.write_bytes_len(s.as_bytes())
    }

    #[inline]
    fn write_list_begin_len(&self, ident: &TListIdentifier) -> usize {
        if ident.size <= 14 {
            1
        } else {
            1 + VarInt::required_space(ident.size as u32)
        }
    }
    #[inline]
    fn write_list_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_set_begin_len(&self, ident: &TSetIdentifier) -> usize {
        if ident.size <= 14 {
            1
        } else {
            1 + VarInt::required_space(ident.size as u32)
        }
    }
    #[inline]
    fn write_set_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_map_begin_len(&self, ident: &TMapIdentifier) -> usize {
        if ident.size == 0 {
            1
        } else {
            1 + VarInt::required_space(ident.size as u32)
        }
    }
    #[inline]
    fn write_map_end_len(&self) -> usize {
        0
    }
}

impl TCompactOutputProtocol<&mut BytesMut> {
    #[inline]
    fn write_field_header(&mut self, field_type: TCompactType, id: i16) -> Result<(), Error> {
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
    fn write_collection_begin(&mut self, element_type: TType, size: usize) -> Result<(), Error> {
        if size <= 14 {
            self.write_byte(
                ((size as i32) << 4) as u8 | (tcompact_get_compact(element_type)? as u8),
            )?;
        } else {
            self.write_byte(0xF0 | (tcompact_get_compact(element_type)? as u8))?;
            self.trans.write_varint(size as u32)?;
        }
        Ok(())
    }

    fn assert_no_pending_bool_write(&self) {
        if let Some(ref f) = self.pending_write_bool_field_identifier {
            panic!("pending bool field {:?} not written", f);
        }
    }
}

impl TOutputProtocol for TCompactOutputProtocol<&mut BytesMut> {
    type Buf = BytesMut;

    #[inline]
    fn write_message_begin(&mut self, identifier: &TMessageIdentifier) -> Result<(), Error> {
        let mtype = identifier.message_type as u8;
        self.trans.write_slice(&[
            COMPACT_PROTOCOL_ID,
            (COMPACT_VERSION & COMPACT_VERSION_MASK)
                | ((mtype << COMPACT_TYPE_SHIFT_AMOUNT) & COMPACT_TYPE_MASK),
        ])?;
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        self.trans.write_varint(identifier.sequence_number as u32)?;
        self.write_string(&identifier.name)?;
        Ok(())
    }
    #[inline]
    fn write_message_end(&mut self) -> Result<(), Error> {
        self.assert_no_pending_bool_write();
        Ok(())
    }

    #[inline]
    fn write_struct_begin(&mut self, _identifier: &TStructIdentifier) -> Result<(), Error> {
        self.write_field_id_stack.push(self.last_write_field_id);
        self.last_write_field_id = 0;
        Ok(())
    }
    #[inline]
    fn write_struct_end(&mut self) -> Result<(), Error> {
        self.assert_no_pending_bool_write();
        if self.write_field_id_stack.len() <= 0 {
            return Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                "WriteStructEnd called without matching WriteStructBegin",
            ));
        }

        self.last_write_field_id = self.write_field_id_stack.pop().unwrap();
        Ok(())
    }

    #[inline]
    fn write_field_begin(&mut self, field_type: TType, id: i16) -> Result<(), Error> {
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
                    field_type: field_type,
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
    fn write_field_end(&mut self) -> Result<(), Error> {
        self.assert_no_pending_bool_write();
        Ok(())
    }
    #[inline]
    fn write_field_stop(&mut self) -> Result<(), Error> {
        self.assert_no_pending_bool_write();
        self.write_byte(TType::Stop as u8)?;
        Ok(())
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<(), Error> {
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
    fn write_bytes(&mut self, b: &[u8]) -> Result<(), Error> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        let size = b.len() as u32;
        self.trans.write_varint(size)?;
        if size == 0 {
            return Ok(());
        }
        self.trans.write_slice(b)?;
        Ok(())
    }
    #[inline]
    fn write_byte(&mut self, b: u8) -> Result<(), Error> {
        self.trans.write_u8(b)?;
        Ok(())
    }
    #[inline]
    fn write_i8(&mut self, i: i8) -> Result<(), Error> {
        self.trans.write_i8(i)?;
        Ok(())
    }
    #[inline]
    fn write_i16(&mut self, i: i16) -> Result<(), Error> {
        self.trans.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), Error> {
        self.trans.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_i64(&mut self, i: i64) -> Result<(), Error> {
        self.trans.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_double(&mut self, d: f64) -> Result<(), Error> {
        self.trans.write_f64(d)?;
        Ok(())
    }
    #[inline]
    fn write_string(&mut self, s: &str) -> Result<(), Error> {
        self.write_bytes(s.as_bytes())
    }

    #[inline]
    fn write_list_begin(&mut self, identifier: &TListIdentifier) -> Result<(), Error> {
        self.write_collection_begin(identifier.element_type, identifier.size)?;
        Ok(())
    }
    #[inline]
    fn write_list_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_set_begin(&mut self, identifier: &TSetIdentifier) -> Result<(), Error> {
        self.write_collection_begin(identifier.element_type, identifier.size)?;
        Ok(())
    }
    #[inline]
    fn write_set_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_map_begin(&mut self, identifier: &TMapIdentifier) -> Result<(), Error> {
        if identifier.size == 0 {
            self.write_byte(TType::Stop as u8)?;
        } else {
            // element count is strictly positive as per the spec, so
            // cast i32 as u32 so that varint writing won't use zigzag encoding
            self.trans.write_varint(identifier.size as u32)?;
            self.write_byte(
                (tcompact_get_compact(identifier.key_type.into())? as u8) << 4
                    | (tcompact_get_compact(identifier.value_type.into())?) as u8,
            )?
        }
        Ok(())
    }
    #[inline]
    fn write_map_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn reserve(&mut self, size: usize) {
        self.trans.reserve(size)
    }

    fn buf_mut(&mut self) -> &mut Self::Buf {
        self.trans
    }
}

pub struct TAsyncCompactProtocol<R> {
    reader: R,

    last_read_field_id: i16,
    read_field_id_stack: Vec<i16>,
    pending_read_bool_value: Option<bool>,
}

impl<R> TAsyncCompactProtocol<R>
where
    R: AsyncRead + Unpin + Send + VarIntAsyncReader,
{
    pub fn new(reader: R) -> TAsyncCompactProtocol<R> {
        Self {
            reader,
            last_read_field_id: 0,
            read_field_id_stack: Vec::new(),
            pending_read_bool_value: None,
        }
    }

    pub async fn read_message_begin(&mut self) -> Result<TMessageIdentifier, Error> {
        let compact_id = self.read_byte().await?;
        if compact_id != COMPACT_PROTOCOL_ID {
            return Err(new_protocol_error(
                ProtocolErrorKind::BadVersion,
                format!("invalid compact protocol header {:?}", compact_id),
            ));
        }

        let type_and_byte = self.read_byte().await?;
        let version = type_and_byte & COMPACT_VERSION_MASK;
        if version != COMPACT_VERSION {
            return Err(new_protocol_error(
                ProtocolErrorKind::BadVersion,
                format!("cannot process compact protocol version {:?}", version),
            ));
        }

        // NOTE: unsigned right shift will pad with 0s
        let type_id = type_and_byte >> 5;
        let message_type = TMessageType::try_from(type_id).map_err(|_| {
            new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid message type {}", type_id),
            )
        })?;

        // writing side wrote signed sequence number as u32 to avoid zigzag encoding
        let sequence_number = self.reader.read_varint_async::<u32>().await? as i32;
        let name = self.read_string().await?;

        Ok(TMessageIdentifier::new(
            smol_str::SmolStr::new(name),
            message_type,
            sequence_number,
        ))
    }

    #[inline]
    pub async fn read_message_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    pub async fn read_struct_begin(&mut self) -> Result<Option<TStructIdentifier>, Error> {
        self.read_field_id_stack.push(self.last_read_field_id);
        self.last_read_field_id = 0;
        Ok(None)
    }

    #[inline]
    pub async fn read_struct_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    // #[inline]
    pub async fn read_field_begin(&mut self) -> Result<TFieldIdentifier, Error> {
        // we can read at least one byte, which is:
        // - the type
        // - the field id delta and the type
        let field_type = self.read_byte().await?;
        let field_delta = (field_type & 0xF0) >> 4;
        let field_type = match (field_type & 0x0F) as u8 {
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
    pub async fn read_field_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    pub async fn read_bool(&mut self) -> Result<bool, Error> {
        match self.pending_read_bool_value.take() {
            Some(b) => Ok(b),
            None => {
                let b: TCompactType = self.read_byte().await?.try_into()?;
                match b {
                    TCompactType::BooleanTrue => Ok(true),
                    TCompactType::BooleanFalse => Ok(false),
                    unkn => Err(new_protocol_error(
                        ProtocolErrorKind::InvalidData,
                        format!("cannot convert {:?} into bool", unkn),
                    )),
                }
            }
        }
    }

    #[inline]
    pub async fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.reader.read_varint_async::<u32>().await?;
        let mut buf = vec![0u8; size as usize];
        self.reader.read_exact(&mut buf).await?;
        Ok(buf)
    }

    #[inline]
    pub async fn read_string(&mut self) -> Result<String, Error> {
        let v = self.read_bytes().await?;
        Ok(unsafe { String::from_utf8_unchecked(v) })
    }

    #[inline]
    pub async fn read_byte(&mut self) -> Result<u8, Error> {
        Ok(self.reader.read_u8().await?)
    }

    #[inline]
    pub async fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(self.reader.read_i8().await?)
    }
    #[inline]
    pub async fn read_i16(&mut self) -> Result<i16, Error> {
        Ok(self.reader.read_varint_async::<i16>().await?)
    }
    #[inline]
    pub async fn read_i32(&mut self) -> Result<i32, Error> {
        Ok(self.reader.read_varint_async::<i32>().await?)
    }
    #[inline]
    pub async fn read_i64(&mut self) -> Result<i64, Error> {
        Ok(self.reader.read_varint_async::<i64>().await?)
    }
    #[inline]
    pub async fn read_double(&mut self) -> Result<f64, Error> {
        Ok(self.reader.read_f64_le().await?)
    }

    #[inline]
    async fn read_collection_begin(&mut self) -> Result<(TType, usize), Error> {
        let header = self.read_byte().await?;
        let element_type = tcompact_get_ttype((header & 0x0F).try_into()?)?;

        let possible_element_count = (header & 0xF0) >> 4;
        let element_count = if possible_element_count != 15 {
            possible_element_count as i32
        } else {
            self.reader.read_varint_async::<u32>().await? as i32
        };
        Ok((element_type, element_count as usize))
    }

    // #[inline]
    pub async fn read_list_begin(&mut self) -> Result<TListIdentifier, Error> {
        let (element_type, element_count) = self.read_collection_begin().await?;
        Ok(TListIdentifier {
            element_type: element_type,
            size: element_count,
        })
    }
    #[inline]
    pub async fn read_list_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    // #[inline]
    pub async fn read_set_begin(&mut self) -> Result<TSetIdentifier, Error> {
        let (element_type, element_count) = self.read_collection_begin().await?;
        Ok(TSetIdentifier {
            element_type: element_type,
            size: element_count,
        })
    }
    #[inline]
    pub async fn read_set_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn read_map_begin(&mut self) -> Result<TMapIdentifier, Error> {
        let element_count = self.reader.read_varint_async::<u32>().await? as i32;
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
    pub async fn read_map_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Skip a field with type `field_type` recursively until the default
    /// maximum skip depth is reached.
    #[inline]
    pub async fn skip(&mut self, field_type: TType) -> Result<(), Error> {
        self.skip_till_depth(field_type, MAXIMUM_SKIP_DEPTH).await
    }

    #[async_recursion::async_recursion]
    /// Skip a field with type `field_type` recursively up to `depth` levels.
    async fn skip_till_depth(&mut self, field_type: TType, depth: i8) -> Result<(), Error> {
        if depth == 0 {
            return Err(new_protocol_error(
                ProtocolErrorKind::DepthLimit,
                format!("cannot parse past {:?}", field_type),
            ));
        }

        match field_type {
            TType::Bool => self.read_bool().await.map(|_| ()),
            TType::I08 => self.read_i8().await.map(|_| ()),
            TType::I16 => self.read_i16().await.map(|_| ()),
            TType::I32 => self.read_i32().await.map(|_| ()),
            TType::I64 => self.read_i64().await.map(|_| ()),
            TType::Double => self.read_double().await.map(|_| ()),
            TType::String => self.read_string().await.map(|_| ()),
            TType::Struct => {
                self.read_struct_begin().await?;
                loop {
                    let field_ident = self.read_field_begin().await?;
                    if field_ident.field_type == TType::Stop {
                        break;
                    }
                    self.skip_till_depth(field_ident.field_type, depth - 1)
                        .await?;
                }
                self.read_struct_end().await
            }
            TType::List => {
                let list_ident = self.read_list_begin().await?;
                for _ in 0..list_ident.size {
                    self.skip_till_depth(list_ident.element_type, depth - 1)
                        .await?;
                }
                self.read_list_end().await
            }
            TType::Set => {
                let set_ident = self.read_set_begin().await?;
                for _ in 0..set_ident.size {
                    self.skip_till_depth(set_ident.element_type, depth - 1)
                        .await?;
                }
                self.read_set_end().await
            }
            TType::Map => {
                let map_ident = self.read_map_begin().await?;
                for _ in 0..map_ident.size {
                    let key_type = map_ident.key_type;
                    let val_type = map_ident.value_type;
                    self.skip_till_depth(key_type, depth - 1).await?;
                    self.skip_till_depth(val_type, depth - 1).await?;
                }
                self.read_map_end().await
            }
            u => Err(new_protocol_error(
                ProtocolErrorKind::DepthLimit,
                format!("cannot skip field type {:?}", &u),
            )),
        }
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
    fn read_collection_begin(&mut self) -> Result<(TType, usize), Error> {
        let header = self.read_byte()?;
        let element_type = tcompact_get_ttype((header & 0x0F).try_into()?)?;

        let possible_element_count = (header & 0xF0) >> 4;
        let element_count = if possible_element_count != 15 {
            possible_element_count as i32
        } else {
            self.trans.read_varint::<u32>()? as i32
        };
        Ok((element_type, element_count as usize))
    }
}

impl TInputProtocol for TCompactInputProtocol<&mut BytesMut> {
    type Buf = BytesMut;

    fn read_message_begin(&mut self) -> Result<TMessageIdentifier, Error> {
        let compact_id = self.read_byte()?;
        if compact_id != COMPACT_PROTOCOL_ID {
            return Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid compact protocol header {:?}", compact_id),
            ));
        }

        let type_and_byte = self.read_byte()?;
        let version = type_and_byte & COMPACT_VERSION_MASK;
        if version != COMPACT_VERSION {
            return Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("cannot process compact protocol version {:?}", version),
            ));
        }

        // NOTE: unsigned right shift will pad with 0s
        let type_id = type_and_byte >> 5;
        let message_type = TMessageType::try_from(type_id).map_err(|_| {
            new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid message type {:?}", type_id),
            )
        })?;

        // writing side wrote signed sequence number as u32 to avoid zigzag encoding
        let sequence_number = self.trans.read_varint::<u32>()? as i32;
        let name = self.read_string()?;

        Ok(TMessageIdentifier::new(
            smol_str::SmolStr::new(name),
            message_type,
            sequence_number,
        ))
    }

    #[inline]
    fn read_message_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn read_struct_begin(&mut self) -> Result<Option<TStructIdentifier>, Error> {
        self.read_field_id_stack.push(self.last_read_field_id);
        self.last_read_field_id = 0;
        Ok(None)
    }

    #[inline]
    fn read_struct_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    // #[inline]
    fn read_field_begin(&mut self) -> Result<TFieldIdentifier, Error> {
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
    fn read_field_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn read_bool(&mut self) -> Result<bool, Error> {
        match self.pending_read_bool_value.take() {
            Some(b) => Ok(b),
            None => {
                let b: TCompactType = self.read_byte()?.try_into()?;
                match b {
                    TCompactType::BooleanTrue => Ok(true),
                    TCompactType::BooleanFalse => Ok(false),
                    unkn => Err(new_protocol_error(
                        ProtocolErrorKind::InvalidData,
                        format!("cannot convert {:?} into bool", unkn),
                    )),
                }
            }
        }
    }

    #[inline]
    fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let size = self.trans.read_varint::<u32>()?;
        let mut buf = vec![0u8; size as usize];
        self.trans.read_to_slice(&mut buf)?;
        Ok(buf)
    }

    #[inline]
    fn read_string(&mut self) -> Result<String, Error> {
        let v = self.read_bytes()?;
        Ok(unsafe { String::from_utf8_unchecked(v) })
    }

    #[inline]
    fn read_byte(&mut self) -> Result<u8, Error> {
        Ok(self.trans.read_u8()?)
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(self.trans.read_i8()?)
    }
    #[inline]
    fn read_i16(&mut self) -> Result<i16, Error> {
        Ok(self.trans.read_varint::<i16>()?)
    }
    #[inline]
    fn read_i32(&mut self) -> Result<i32, Error> {
        Ok(self.trans.read_varint::<i32>()?)
    }
    #[inline]
    fn read_i64(&mut self) -> Result<i64, Error> {
        Ok(self.trans.read_varint::<i64>()?)
    }

    #[inline]
    fn read_double(&mut self) -> Result<f64, Error> {
        Ok(self.trans.read_f64_le()?)
    }

    // #[inline]
    fn read_list_begin(&mut self) -> Result<TListIdentifier, Error> {
        let (element_type, element_count) = self.read_collection_begin()?;
        Ok(TListIdentifier {
            element_type: element_type,
            size: element_count,
        })
    }
    #[inline]
    fn read_list_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    // #[inline]
    fn read_set_begin(&mut self) -> Result<TSetIdentifier, Error> {
        let (element_type, element_count) = self.read_collection_begin()?;
        Ok(TSetIdentifier {
            element_type: element_type,
            size: element_count,
        })
    }
    #[inline]
    fn read_set_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    // #[inline]
    fn read_map_begin(&mut self) -> Result<TMapIdentifier, Error> {
        let element_count = self.trans.read_varint::<u32>()? as i32;
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
    fn read_map_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn buf_mut(&mut self) -> &mut Self::Buf {
        self.trans
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use bytes::{Buf, BufMut, BytesMut};
    type TCompactInputProt<'a> = TCompactInputProtocol<&'a mut BytesMut>;
    type TCompactOutputProt<'a> = TCompactOutputProtocol<&'a mut BytesMut>;

    use super::{TCompactInputProtocol, TCompactOutputProtocol};
    use crate::thrift::{
        Error, TFieldIdentifier, TInputProtocol, TMessageIdentifier, TMessageType, TOutputProtocol,
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

    fn test_input_prot<'a>(trans: &'a mut BytesMut) -> TCompactInputProt<'a> {
        TCompactInputProt::new(trans)
    }
    fn test_output_prot<'a>(trans: &'a mut BytesMut) -> TCompactOutputProt<'a> {
        TCompactOutputProt::new(trans)
    }

    #[test]
    fn must_write_message_begin_largest_maximum_positive_sequence_number() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut i_prot = test_input_prot(&mut trans);

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
            let mut o_prot = test_output_prot(&mut trans);

            let val: i64 = ((1u64 << i) - 1) as i64;
            o_prot.write_field_begin(TType::I64, 1).unwrap();
            o_prot.write_i64(val).unwrap();
            o_prot.write_field_end().unwrap();
            o_prot.flush().unwrap();
            // println!("trans {:?}", trans);
            let mut i_prot = test_input_prot(&mut trans);
            i_prot.read_field_begin().unwrap();
            assert_eq!(val, i_prot.read_i64().unwrap());
        }
    }

    #[test]
    fn must_round_trip_message_begin() {
        let mut trans = BytesMut::new();

        let mut o_prot = test_output_prot(&mut trans);
        let ident = TMessageIdentifier::new("service_call".into(), TMessageType::Call, 1_283_948);
        assert_success!(o_prot.write_message_begin(&ident));

        let mut i_prot = test_input_prot(&mut trans);
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
        let mut o_prot = test_output_prot(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with tiny field ids
        // since they're small the field ids will be encoded as deltas

        // since this is the first field (and it's zero) it gets the full varint write
        assert_success!(o_prot.write_field_begin(TType::I08, 0));
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
        let mut o_prot = test_output_prot(&mut trans);

        // no bytes should be written however
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // write three fields with tiny field ids
        // since they're small the field ids will be encoded as deltas

        // since this is the first field (and it's zero) it gets the full varint write
        let field_ident_1 = TFieldIdentifier::new("foo", TType::I08, 0);
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

        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        assert_success!(o_prot.write_field_begin(TType::String, 6));
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
        let mut o_prot = test_output_prot(&mut trans);

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
        let field_ident_3 = TFieldIdentifier::new("foo", TType::String, 6);
        assert_success!(
            o_prot.write_field_begin(field_ident_3.field_type, field_ident_3.id.unwrap())
        );
        assert_success!(o_prot.write_field_end());

        // now, finish the struct off
        assert_success!(o_prot.write_field_stop());
        assert_success!(o_prot.write_struct_end());

        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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

        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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

        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        assert_success!(o_prot.write_field_begin(TType::I08, 7));
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
        let mut o_prot = test_output_prot(&mut trans);

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
        let field_ident_3 = TFieldIdentifier::new("foo", TType::I08, 7);
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

        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        assert_success!(o_prot.write_field_begin(TType::I08, 27));
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
        let mut o_prot = test_output_prot(&mut trans);

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
        let field_ident_4 = TFieldIdentifier::new("foo", TType::I08, 27);
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

        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I64, 1));
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::String, 21));
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since this delta > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::Double, 7));
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I08, 10));
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
        let mut o_prot = test_output_prot(&mut trans);

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
        let field_ident_2 = TFieldIdentifier::new("foo", TType::String, 21);
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
        let field_ident_4 = TFieldIdentifier::new("foo", TType::I08, 10);
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

        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

        // start containing struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // containing struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I64, 1));
        assert_success!(o_prot.write_field_end());

        // containing struct
        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::String, 21));
        assert_success!(o_prot.write_field_end());

        // start contained struct
        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));

        // contained struct
        // since this delta > 15 it gets a full write
        assert_success!(o_prot.write_field_begin(TType::Double, 21));
        assert_success!(o_prot.write_field_end());

        // contained struct
        // since the delta is > 0 and < 15 it gets a delta write
        assert_success!(o_prot.write_field_begin(TType::I08, 27));
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
        let mut o_prot = test_output_prot(&mut trans);

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
        let field_ident_2 = TFieldIdentifier::new("foo", TType::String, 21);
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
        let field_ident_4 = TFieldIdentifier::new("foo", TType::I08, 27);
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

        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

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

        let mut i_prot = test_input_prot(&mut trans);

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
        let mut o_prot = test_output_prot(&mut trans);

        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));
        assert_success!(o_prot.write_field_begin(TType::Bool, 1));
        o_prot.write_field_end().unwrap();
    }

    #[test]
    #[should_panic]
    fn must_fail_if_write_stop_field_without_writing_bool_value() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot(&mut trans);

        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));
        assert_success!(o_prot.write_field_begin(TType::Bool, 1));
        o_prot.write_field_stop().unwrap();
    }

    #[test]
    #[should_panic]
    fn must_fail_if_write_struct_end_without_writing_bool_value() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot(&mut trans);

        assert_success!(o_prot.write_struct_begin(&TStructIdentifier::new("foo")));
        assert_success!(o_prot.write_field_begin(TType::Bool, 1));
        o_prot.write_struct_end().unwrap();
    }

    #[test]
    #[should_panic]
    fn must_fail_if_write_struct_end_without_any_fields() {
        let mut trans = BytesMut::new();
        let mut o_prot = test_output_prot(&mut trans);
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
        F: FnMut(&mut TCompactOutputProtocol<&mut B>) -> Result<(), Error>,
    {
        let mut o_prot = TCompactOutputProtocol::new(&mut trans);
        assert!(write_fn(&mut o_prot).is_ok());

        let mut out = Vec::new();
        o_prot.trans.reader().read_to_end(&mut out).unwrap();
        assert_eq!(out.len(), 0);
    }
}
