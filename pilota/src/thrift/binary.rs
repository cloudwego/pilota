use std::convert::TryInto;

use bytes::{Buf, Bytes, BytesMut};
use linkedbytes::LinkedBytes;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::{
    error::{Error, ProtocolErrorKind},
    new_protocol_error,
    rw_ext::{ReadExt, WriteExt},
    TFieldIdentifier, TInputProtocol, TLengthProtocol, TListIdentifier, TMapIdentifier,
    TMessageIdentifier, TMessageType, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType,
    MAXIMUM_SKIP_DEPTH, ZERO_COPY_THRESHOLD,
};

static VERSION_1: u32 = 0x80010000;
static VERSION_MASK: u32 = 0xffff0000;

pub struct TBinaryProtocol<T> {
    pub(crate) trans: T,
    zero_copy: bool,
}

impl<T> TBinaryProtocol<T> {
    /// `zero_copy` only takes effect when `T` is [`BytesMut`] for input and
    /// [`LinkedBytes`] for output.
    #[inline]
    pub fn new(trans: T, zero_copy: bool) -> Self {
        Self { trans, zero_copy }
    }
}

#[inline]
fn field_type_from_u8(ttype: u8) -> Result<TType, Error> {
    let ttype: TType = ttype.try_into().map_err(|_| {
        new_protocol_error(
            ProtocolErrorKind::InvalidData,
            format!("invalid ttype {}", ttype),
        )
    })?;

    Ok(ttype)
}

impl<T> TLengthProtocol for TBinaryProtocol<T> {
    #[inline]
    fn write_message_begin_len(&self, identifier: &TMessageIdentifier) -> usize {
        self.write_i32_len(0) + self.write_string_len(&identifier.name) + self.write_i32_len(0)
    }

    #[inline]
    fn write_message_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_struct_begin_len(&self, _identifier: &TStructIdentifier) -> usize {
        0
    }

    #[inline]
    fn write_struct_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_field_begin_len(&self, _identifier: &TFieldIdentifier) -> usize {
        self.write_byte_len(0) + self.write_i16_len(0)
    }

    #[inline]
    fn write_field_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_field_stop_len(&self) -> usize {
        self.write_byte_len(0)
    }

    #[inline]
    fn write_bool_len(&self, _b: bool) -> usize {
        self.write_i8_len(0)
    }

    #[inline]
    fn write_bytes_len(&self, b: &[u8]) -> usize {
        // FIXME: this will calc the wrong size if T is not LinkedBytes and zero copy is
        // enabled
        if self.zero_copy && b.len() >= ZERO_COPY_THRESHOLD {
            self.write_i32_len(0)
        } else {
            self.write_i32_len(0) + b.len()
        }
    }

    #[inline]
    fn write_byte_len(&self, _b: u8) -> usize {
        1
    }

    #[inline]
    fn write_uuid_len(&self, _u: [u8; 16]) -> usize {
        16
    }

    #[inline]
    fn write_i8_len(&self, _i: i8) -> usize {
        1
    }

    #[inline]
    fn write_i16_len(&self, _i: i16) -> usize {
        2
    }

    #[inline]
    fn write_i32_len(&self, _i: i32) -> usize {
        4
    }

    #[inline]
    fn write_i64_len(&self, _i: i64) -> usize {
        8
    }

    #[inline]
    fn write_double_len(&self, _d: f64) -> usize {
        8
    }

    #[inline]
    fn write_string_len(&self, s: &str) -> usize {
        self.write_i32_len(0) + s.len()
    }

    #[inline]
    fn write_list_begin_len(&self, _identifier: &TListIdentifier) -> usize {
        self.write_byte_len(0) + self.write_i32_len(0)
    }

    #[inline]
    fn write_list_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_set_begin_len(&self, _identifier: &TSetIdentifier) -> usize {
        self.write_byte_len(0) + self.write_i32_len(0)
    }

    #[inline]
    fn write_set_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_map_begin_len(&self, _identifier: &TMapIdentifier) -> usize {
        self.write_byte_len(0) + self.write_byte_len(0) + self.write_i32_len(0)
    }

    #[inline]
    fn write_map_end_len(&self) -> usize {
        0
    }

    #[inline]
    fn write_bytes_vec_len(&self, b: &[u8]) -> usize {
        self.write_bytes_len(b)
    }
}

impl TOutputProtocol for TBinaryProtocol<&mut BytesMut> {
    type BufMut = BytesMut;

    #[inline]
    fn write_message_begin(&mut self, identifier: &TMessageIdentifier) -> Result<(), Error> {
        let msg_type_u8: u8 = identifier.message_type.into();
        let version = (VERSION_1 | msg_type_u8 as u32) as i32;
        self.write_i32(version)?;
        self.write_string(&identifier.name)?;
        self.write_i32(identifier.sequence_number)?;
        Ok(())
    }

    #[inline]
    fn write_message_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_struct_begin(&mut self, _: &TStructIdentifier) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_struct_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_field_begin(&mut self, field_type: TType, id: i16) -> Result<(), Error> {
        let mut data: [u8; 3] = [0; 3];
        data[0] = field_type as u8;
        let id = id.to_be_bytes();
        data[1] = id[0];
        data[2] = id[1];
        self.trans.write_slice(&data)?;
        Ok(())
    }

    #[inline]
    fn write_field_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_field_stop(&mut self) -> Result<(), Error> {
        self.write_byte(TType::Stop as u8)
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<(), Error> {
        if b {
            self.write_i8(1)
        } else {
            self.write_i8(0)
        }
    }

    #[inline]
    fn write_bytes(&mut self, b: Bytes) -> Result<(), Error> {
        self.write_i32(b.len() as i32)?;
        self.trans.write_slice(&b)?;
        Ok(())
    }

    #[inline]
    fn write_byte(&mut self, b: u8) -> Result<(), Error> {
        self.trans.write_u8(b)?;
        Ok(())
    }

    #[inline]
    fn write_uuid(&mut self, u: [u8; 16]) -> Result<(), Error> {
        self.trans.write_slice(&u)?;
        Ok(())
    }

    #[inline]
    fn write_i8(&mut self, i: i8) -> Result<(), Error> {
        self.trans.write_i8(i)?;
        Ok(())
    }

    #[inline]
    fn write_i16(&mut self, i: i16) -> Result<(), Error> {
        self.trans.write_i16(i)?;
        Ok(())
    }

    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), Error> {
        self.trans.write_i32(i)?;
        Ok(())
    }

    #[inline]
    fn write_i64(&mut self, i: i64) -> Result<(), Error> {
        self.trans.write_i64(i)?;
        Ok(())
    }

    #[inline]
    fn write_double(&mut self, d: f64) -> Result<(), Error> {
        self.trans.write_f64(d)?;
        Ok(())
    }

    #[inline]
    fn write_string(&mut self, s: &str) -> Result<(), Error> {
        self.write_i32(s.len() as i32)?;
        self.trans.write_slice(s.as_bytes())?;
        Ok(())
    }

    #[inline]
    fn write_list_begin(&mut self, identifier: &TListIdentifier) -> Result<(), Error> {
        self.write_byte(identifier.element_type.into())?;
        self.write_i32(identifier.size as i32)
    }

    #[inline]
    fn write_list_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_set_begin(&mut self, identifier: &TSetIdentifier) -> Result<(), Error> {
        self.write_byte(identifier.element_type.into())?;
        self.write_i32(identifier.size as i32)
    }

    #[inline]
    fn write_set_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_map_begin(&mut self, identifier: &TMapIdentifier) -> Result<(), Error> {
        let key_type = identifier.key_type;
        self.write_byte(key_type.into())?;
        let val_type = identifier.value_type;
        self.write_byte(val_type.into())?;
        self.write_i32(identifier.size as i32)
    }

    #[inline]
    fn write_map_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn reserve(&mut self, size: usize) {
        self.trans.reserve(size)
    }

    #[inline]
    fn buf_mut(&mut self) -> &mut BytesMut {
        self.trans
    }

    #[inline]
    fn write_bytes_vec(&mut self, b: &[u8]) -> Result<(), Error> {
        self.write_i32(b.len() as i32)?;
        self.trans.write_slice(b)?;
        Ok(())
    }
}

impl TOutputProtocol for TBinaryProtocol<&mut LinkedBytes> {
    type BufMut = LinkedBytes;

    #[inline]
    fn write_message_begin(&mut self, identifier: &TMessageIdentifier) -> Result<(), Error> {
        let msg_type_u8: u8 = identifier.message_type.into();
        let version = (VERSION_1 | msg_type_u8 as u32) as i32;
        self.write_i32(version)?;
        self.write_string(&identifier.name)?;
        self.write_i32(identifier.sequence_number)?;
        Ok(())
    }

    #[inline]
    fn write_message_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_struct_begin(&mut self, _: &TStructIdentifier) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_struct_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_field_begin(&mut self, field_type: TType, id: i16) -> Result<(), Error> {
        let mut data: [u8; 3] = [0; 3];
        data[0] = field_type as u8;
        let id = id.to_be_bytes();
        data[1] = id[0];
        data[2] = id[1];
        self.trans.bytes_mut().write_slice(&data)?;
        Ok(())
    }

    #[inline]
    fn write_field_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_field_stop(&mut self) -> Result<(), Error> {
        self.write_byte(TType::Stop as u8)
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<(), Error> {
        if b {
            self.write_i8(1)
        } else {
            self.write_i8(0)
        }
    }

    #[inline]
    fn write_bytes(&mut self, b: Bytes) -> Result<(), Error> {
        self.write_i32(b.len() as i32)?;
        if self.zero_copy && b.len() >= ZERO_COPY_THRESHOLD {
            self.trans.insert(b);
            return Ok(());
        }
        self.trans.bytes_mut().write_slice(&b)?;
        Ok(())
    }

    #[inline]
    fn write_byte(&mut self, b: u8) -> Result<(), Error> {
        self.trans.bytes_mut().write_u8(b)?;
        Ok(())
    }

    #[inline]
    fn write_uuid(&mut self, u: [u8; 16]) -> Result<(), Error> {
        self.trans.bytes_mut().write_slice(&u)?;
        Ok(())
    }

    #[inline]
    fn write_i8(&mut self, i: i8) -> Result<(), Error> {
        self.trans.bytes_mut().write_i8(i)?;
        Ok(())
    }

    #[inline]
    fn write_i16(&mut self, i: i16) -> Result<(), Error> {
        self.trans.bytes_mut().write_i16(i)?;
        Ok(())
    }

    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), Error> {
        self.trans.bytes_mut().write_i32(i)?;
        Ok(())
    }

    #[inline]
    fn write_i64(&mut self, i: i64) -> Result<(), Error> {
        self.trans.bytes_mut().write_i64(i)?;
        Ok(())
    }

    #[inline]
    fn write_double(&mut self, d: f64) -> Result<(), Error> {
        self.trans.bytes_mut().write_f64(d)?;
        Ok(())
    }

    #[inline]
    fn write_string(&mut self, s: &str) -> Result<(), Error> {
        self.write_i32(s.len() as i32)?;
        self.trans.bytes_mut().write_slice(s.as_bytes())?;
        Ok(())
    }

    #[inline]
    fn write_list_begin(&mut self, identifier: &TListIdentifier) -> Result<(), Error> {
        self.write_byte(identifier.element_type.into())?;
        self.write_i32(identifier.size as i32)
    }

    #[inline]
    fn write_list_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_set_begin(&mut self, identifier: &TSetIdentifier) -> Result<(), Error> {
        self.write_byte(identifier.element_type.into())?;
        self.write_i32(identifier.size as i32)
    }

    #[inline]
    fn write_set_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn write_map_begin(&mut self, identifier: &TMapIdentifier) -> Result<(), Error> {
        let key_type = identifier.key_type;
        self.write_byte(key_type.into())?;
        let val_type = identifier.value_type;
        self.write_byte(val_type.into())?;
        self.write_i32(identifier.size as i32)
    }

    #[inline]
    fn write_map_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn reserve(&mut self, size: usize) {
        self.trans.reserve(size)
    }

    #[inline]
    fn buf_mut(&mut self) -> &mut LinkedBytes {
        self.trans
    }

    #[inline]
    fn write_bytes_vec(&mut self, b: &[u8]) -> Result<(), Error> {
        self.write_i32(b.len() as i32)?;
        self.trans.bytes_mut().write_slice(b)?;
        Ok(())
    }
}

pub struct TAsyncBinaryProtocol<R> {
    reader: R,
}

impl<R> TAsyncBinaryProtocol<R>
where
    R: AsyncRead + Unpin + Send,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    // https://github.com/apache/thrift/blob/master/doc/specs/thrift-binary-protocol.md
    pub async fn read_message_begin(&mut self) -> Result<TMessageIdentifier, Error> {
        let size = self.reader.read_i32().await?;
        if size > 0 {
            return Err(new_protocol_error(
                ProtocolErrorKind::BadVersion,
                "Missing version in ReadMessageBegin".to_string(),
            ));
        }
        let type_u8 = (size & 0xf) as u8;

        let message_type = TMessageType::try_from(type_u8).map_err(|_| {
            new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid message type {}", type_u8),
            )
        })?;

        let version = size & (VERSION_MASK as i32);
        if version != (VERSION_1 as i32) {
            return Err(new_protocol_error(
                ProtocolErrorKind::BadVersion,
                "Bad version in ReadMessageBegin",
            ));
        }

        let name = self.read_string().await?;

        let sequence_number = self.read_i32().await?;
        Ok(TMessageIdentifier::new(
            smol_str::SmolStr::new(name),
            message_type,
            sequence_number,
        ))
    }

    #[inline]
    pub async fn read_i32(&mut self) -> Result<i32, Error> {
        Ok(self.reader.read_i32().await?)
    }

    #[inline]
    pub async fn read_string(&mut self) -> Result<String, Error> {
        let len = self.reader.read_i32().await? as usize;
        // FIXME: use maybe_uninit?
        let mut v = vec![0; len];
        self.reader.read_exact(&mut v).await?;
        Ok(unsafe { String::from_utf8_unchecked(v) })
    }

    #[inline]
    pub async fn read_message_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    pub async fn read_struct_begin(&mut self) -> Result<Option<TStructIdentifier>, Error> {
        Ok(None)
    }

    #[inline]
    pub async fn read_struct_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    pub async fn read_field_begin(&mut self) -> Result<TFieldIdentifier, Error> {
        let field_type_byte = self.read_byte().await?;
        let field_type = field_type_byte.try_into().map_err(|_| {
            new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid ttype {}", field_type_byte),
            )
        })?;
        let id = match field_type {
            TType::Stop => Ok(0),
            _ => self.read_i16().await,
        }?;
        Ok(TFieldIdentifier::new::<Option<&'static str>, i16>(
            None, field_type, id,
        ))
    }

    #[inline]
    pub async fn read_field_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    pub async fn read_bool(&mut self) -> Result<bool, Error> {
        let b = self.read_i8().await?;
        match b {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[inline]
    pub async fn read_bytes(&mut self) -> Result<Bytes, Error> {
        let len = self.reader.read_i32().await? as usize;
        // FIXME: use maybe_uninit?
        let mut v = vec![0; len];
        self.reader.read_exact(&mut v).await?;
        Ok(Bytes::from(v))
    }

    #[inline]
    pub async fn read_bytes_vec(&mut self) -> Result<Vec<u8>, Error> {
        let len = self.reader.read_i32().await? as usize;
        // FIXME: use maybe_uninit?
        let mut v = vec![0; len];
        self.reader.read_exact(&mut v).await?;
        Ok(v)
    }

    #[inline]
    pub async fn read_uuid(&mut self) -> Result<[u8; 16], Error> {
        let mut uuid = [0; 16];
        self.reader.read_exact(&mut uuid).await?;
        Ok(uuid)
    }

    #[inline]
    pub async fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(self.reader.read_i8().await?)
    }

    #[inline]
    pub async fn read_i16(&mut self) -> Result<i16, Error> {
        Ok(self.reader.read_i16().await?)
    }

    #[inline]
    pub async fn read_i64(&mut self) -> Result<i64, Error> {
        Ok(self.reader.read_i64().await?)
    }

    #[inline]
    pub async fn read_double(&mut self) -> Result<f64, Error> {
        Ok(self.reader.read_f64().await?)
    }

    #[inline]
    pub async fn read_list_begin(&mut self) -> Result<TListIdentifier, Error> {
        let element_type: TType = self.read_byte().await.and_then(field_type_from_u8)?;
        let size = self.read_i32().await?;
        Ok(TListIdentifier::new(element_type, size as usize))
    }

    #[inline]
    pub async fn read_list_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    pub async fn read_set_begin(&mut self) -> Result<TSetIdentifier, Error> {
        let element_type: TType = self.read_byte().await.and_then(field_type_from_u8)?;
        let size = self.read_i32().await?;
        Ok(TSetIdentifier::new(element_type, size as usize))
    }

    #[inline]
    pub async fn read_set_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    pub async fn read_map_begin(&mut self) -> Result<TMapIdentifier, Error> {
        let key_type: TType = self.read_byte().await.and_then(field_type_from_u8)?;
        let value_type: TType = self.read_byte().await.and_then(field_type_from_u8)?;
        let size = self.read_i32().await?;
        Ok(TMapIdentifier::new(key_type, value_type, size as usize))
    }

    #[inline]
    pub async fn read_map_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    pub async fn read_byte(&mut self) -> Result<u8, Error> {
        Ok(self.reader.read_u8().await?)
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
            TType::I8 => self.read_i8().await.map(|_| ()),
            TType::I16 => self.read_i16().await.map(|_| ()),
            TType::I32 => self.read_i32().await.map(|_| ()),
            TType::I64 => self.read_i64().await.map(|_| ()),
            TType::Double => self.read_double().await.map(|_| ()),
            TType::Binary => self.read_bytes().await.map(|_| ()),
            TType::Uuid => self.read_uuid().await.map(|_| ()),
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

impl TInputProtocol for TBinaryProtocol<&mut BytesMut> {
    type Buf = BytesMut;

    fn read_message_begin(&mut self) -> Result<TMessageIdentifier, Error> {
        let size = self.trans.read_i32()?;

        if size > 0 {
            return Err(new_protocol_error(
                ProtocolErrorKind::BadVersion,
                "Missing version in ReadMessageBegin".to_string(),
            ));
        }
        let type_u8 = (size & 0xf) as u8;

        let message_type = TMessageType::try_from(type_u8).map_err(|_| {
            new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid message type {}", type_u8),
            )
        })?;

        let version = size & (VERSION_MASK as i32);
        if version != (VERSION_1 as i32) {
            return Err(new_protocol_error(
                ProtocolErrorKind::BadVersion,
                "Bad version in ReadMessageBegin",
            ));
        }

        let name = self.read_string()?;

        let sequence_number = self.read_i32()?;
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
        Ok(None)
    }

    #[inline]
    fn read_struct_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn read_field_begin(&mut self) -> Result<TFieldIdentifier, Error> {
        let field_type_byte = self.read_byte()?;
        let field_type = field_type_byte.try_into().map_err(|_| {
            new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid ttype {}", field_type_byte),
            )
        })?;
        let id = match field_type {
            TType::Stop => Ok(0),
            _ => self.read_i16(),
        }?;
        Ok(TFieldIdentifier::new::<Option<&'static str>, i16>(
            None, field_type, id,
        ))
    }

    #[inline]
    fn read_field_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn read_bool(&mut self) -> Result<bool, Error> {
        let b = self.read_i8()?;
        match b {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[inline]
    fn read_bytes(&mut self) -> Result<Bytes, Error> {
        let len = self.trans.read_i32()?;
        // first, drop the previous bytes
        self.trans.advance(self.trans.len());
        // then, split and freeze it
        Ok(self.trans.split_to(len as usize).freeze())
    }

    #[inline]
    fn read_uuid(&mut self) -> Result<[u8; 16], Error> {
        let mut u = [0; 16];
        self.trans.read_to_slice(&mut u)?;
        Ok(u)
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(self.trans.read_i8()?)
    }

    #[inline]
    fn read_i16(&mut self) -> Result<i16, Error> {
        Ok(self.trans.read_i16()?)
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32, Error> {
        Ok(self.trans.read_i32()?)
    }

    #[inline]
    fn read_i64(&mut self) -> Result<i64, Error> {
        Ok(self.trans.read_i64()?)
    }

    #[inline]
    fn read_double(&mut self) -> Result<f64, Error> {
        Ok(self.trans.read_f64()?)
    }

    #[inline]
    fn read_string(&mut self) -> Result<String, Error> {
        let len = self.trans.read_i32()?;
        Ok(self.trans.read_to_string(len as usize)?)
    }

    #[inline]
    fn read_list_begin(&mut self) -> Result<TListIdentifier, Error> {
        let element_type: TType = self.read_byte().and_then(field_type_from_u8)?;
        let size = self.read_i32()?;
        Ok(TListIdentifier::new(element_type, size as usize))
    }

    #[inline]
    fn read_list_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn read_set_begin(&mut self) -> Result<TSetIdentifier, Error> {
        let element_type: TType = self.read_byte().and_then(field_type_from_u8)?;
        let size = self.read_i32()?;
        Ok(TSetIdentifier::new(element_type, size as usize))
    }

    #[inline]
    fn read_set_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn read_map_begin(&mut self) -> Result<TMapIdentifier, Error> {
        let key_type: TType = self.read_byte().and_then(field_type_from_u8)?;
        let value_type: TType = self.read_byte().and_then(field_type_from_u8)?;
        let size = self.read_i32()?;
        Ok(TMapIdentifier::new(key_type, value_type, size as usize))
    }

    #[inline]
    fn read_map_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    #[inline]
    fn read_byte(&mut self) -> Result<u8, Error> {
        Ok(self.trans.read_u8()?)
    }

    #[inline]
    fn buf_mut(&mut self) -> &mut Self::Buf {
        self.trans
    }

    #[inline]
    #[allow(clippy::uninit_vec)]
    fn read_bytes_vec(&mut self) -> Result<Vec<u8>, Error> {
        let len = self.trans.read_i32()? as usize;
        let mut vec = Vec::with_capacity(len);
        unsafe { vec.set_len(len) }
        self.trans.read_to_slice(vec.as_mut_slice())?;
        Ok(vec)
    }
}
