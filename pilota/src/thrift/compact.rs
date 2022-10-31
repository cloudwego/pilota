use bytes::BytesMut;
use integer_encoding::{VarInt, VarIntAsyncReader, VarIntReader, VarIntWriter};
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
static COMPACT_TYPE_BITS: u8 = 0x07;
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
        2 + VarInt::required_space::<u32>(ident.sequence_number)
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
        let sz = VarInt::required_space::<u32>(b.len() as u32);
        if b.len() > 0 {
            sz + b.len()
        } else {
            sz
        }
    }
    #[inline]
    fn write_byte_len(&self, b: u8) -> usize {
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
        self.write_bytes_len(s)
    }

    #[inline]
    fn write_list_begin_len(&self, ident: &TListIdentifier) -> usize {
        if ident.size <= 14 {
            1
        } else {
            1 + VarInt::required_space::<u32>(ident.size as u32)
        }
    }
    #[inline]
    fn write_list_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_set_begin_len(&self, ident: &TSetIdentifier) -> usize {
        self.write_list_begin_len(ident)
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
            1 + VarInt::required_space::<u32>(ident.size as u32)
        }
    }
    #[inline]
    fn write_map_end_len(&self) -> usize {
        0
    }
}

impl TCompactOutputProtocol<&mut BytesMut> {
    #[inline]
    fn write_varint<I: VarInt>(&mut self, value: I) -> Result<(), Error> {
        VarIntWriter::write_varint::<I>(&mut self, value)?;
        Ok(())
    }

    // TODO(ii64): constraint for `field_type` ?
    #[inline]
    fn write_field_header(&mut self, field_type: u8, id: i16) -> Result<(), Error> {
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
            self.write_varint(size as u32)?;
        }
        Ok(())
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
        self.write_varint(identifier.sequence_number as u32)?;
        self.write_string(&identifier.name)?;
        Ok(())
    }
    #[inline]
    fn write_message_end(&mut self) -> Result<(), Error> {
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
            _ => self.write_field_header(field_type as u8, id),
        }
    }
    #[inline]
    fn write_field_end(&mut self) -> Result<(), Error> {
        Ok(())
    }
    #[inline]
    fn write_field_stop(&mut self) -> Result<(), Error> {
        self.write_byte(TType::Stop as u8)?;
        Ok(())
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<(), Error> {
        match self.pending_write_bool_field_identifier.take() {
            Some(pending) => {
                let field_id = pending.id.expect("bool field should have a field id");
                let field_type_as_u8: u8 = if b { TCompactType::BooleanTrue } else { TCompactType::BooleanFalse };
                self.write_field_header(field_type_as_u8, field_id)
            }
            None => {
                if b {
                    self.write_byte(TCompactType::BooleanTrue)
                } else {
                    self.write_byte(TCompactType::BooleanFalse)
                }
            }
        }
    }
    #[inline]
    fn write_bytes(&mut self, b: &[u8]) -> Result<(), Error> {
        // length is strictly positive as per the spec, so
        // cast i32 as u32 so that varint writing won't use zigzag encoding
        let size = b.len() as u32;
        self.write_varint(size)?;
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
        self.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), Error> {
        self.write_varint(i)?;
        Ok(())
    }
    #[inline]
    fn write_i64(&mut self, i: i64) -> Result<(), Error> {
        self.write_varint(i)?;
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
            self.write_varint(identifier.size as u32)?;
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
        let field_type = match field_type & 0x0F {
            TCompactType::BooleanTrue => {
                self.pending_read_bool_value = Some(true);
                Ok(TType::Bool)
            }
            TCompactType::BooleanFalse => {
                self.pending_read_bool_value = Some(false);
                Ok(TType::Bool)
            }
            ttu8 => TType::try_from(ttu8),
        }?;
        match field_type {
            TType::Stop => Ok(TFieldIdentifier::new::<Option<&'static str>, i16>(
                None,
                TType::Stop,
                0,
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
                let b = self.read_byte().await?;
                match b {
                    TCompactType::BooleanTrue => Ok(true),
                    TCompactType::BooleanFalse => Ok(false),
                    unkn => Err(new_protocol_error(
                        ProtocolErrorKind::InvalidData,
                        format!("cannot convert {} into bool", unkn),
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

impl TCompactInputProtocol<&mut BytesMut> {
    #[inline]
    fn read_collection_begin(&mut self) -> Result<(TType, usize), Error> {
        let header = self.read_byte()?;
        let element_type = tcompact_get_ttype((header & 0x0F).try_into()?)?;

        let possible_element_count = (header & 0xF0) >> 4;
        let element_count = if possible_element_count != 15 {
            possible_element_count as i32
        } else {
            VarIntReader::read_varint::<u32>(&mut self)? as i32
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
        let sequence_number = self.reader.read_varint_async::<u32>()? as i32;
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
            TCompactType::BooleanTrue => {
                self.pending_read_bool_value = Some(true);
                Ok(TType::Bool)
            }
            TCompactType::BooleanFalse => {
                self.pending_read_bool_value = Some(false);
                Ok(TType::Bool)
            }
            ttu8 => TType::try_from(ttu8),
        }?;
        match field_type {
            TType::Stop => Ok(TFieldIdentifier::new::<Option<&'static str>, i16>(
                None,
                TType::Stop,
                0,
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
                let b = self.read_byte()?;
                match b {
                    TCompactType::BooleanTrue => Ok(true),
                    TCompactType::BooleanFalse => Ok(false),
                    unkn => Err(new_protocol_error(
                        ProtocolErrorKind::InvalidData,
                        format!("cannot convert {} into bool", unkn),
                    )),
                }
            }
        }
    }

    #[inline]
    fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let size = VarIntReader::read_varint::<u32>(&mut self)?;
        let mut buf = vec![0u8; size as usize];
        self.trans.read
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
        Ok(VarIntReader::read_varint::<i16>(&mut self)?)
    }
    #[inline]
    fn read_i32(&mut self) -> Result<i32, Error> {
        Ok(VarIntReader::read_varint::<i32>(&mut self)?)
    }
    #[inline]
    fn read_i64(&mut self) -> Result<i64, Error> {
        Ok(VarIntReader::read_varint::<i64>(&mut self)?)
    }

    #[inline]
    fn read_double(&mut self) -> Result<f64, Error> {
        Ok(self.trans.read_f32_le()?)
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
        let element_count = VarIntReader::read_varint::<u32>(&mut self)? as i32;
        if element_count == 0 {
            Ok(TMapIdentifier::new(TType::Stop, TType::Stop, 0))
        } else {
            let type_header = self.read_byte()?;
            let key_type = tcompact_get_ttype(((type_header & 0xF0) >> 4).try_into()?)?;
            let val_type = tcompact_get_ttype((type_header & 0x0F.try_into()?))?;

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
