use std::convert::TryInto;

use bytes::BytesMut;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::{
    error::{new_protocol_error, Error, ProtocolErrorKind},
    rw_ext::{ReadExt, WriteExt},
    TFieldIdentifier, TInputProtocol, TLengthProtocol, TListIdentifier, TMapIdentifier,
    TMessageIdentifier, TMessageType, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType,
    MAXIMUM_SKIP_DEPTH,
};

static VERSION_1: u32 = 0x80010000;
static VERSION_MASK: u32 = 0xffff0000;

pub struct TBinaryProtocol<T> {
    pub(crate) trans: T,
}

impl<T> TBinaryProtocol<T> {
    pub fn new(trans: T) -> Self {
        Self { trans }
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
        self.write_i32_len(0) + b.len()
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
        self.write_bytes_len(s.as_bytes())
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
}

macro_rules! impl_output_bytes_mut {
    ($t: ty) => {
        impl TOutputProtocol for TBinaryProtocol<$t> {
            type Buf = BytesMut;

            #[inline]
            fn write_message_begin(
                &mut self,
                identifier: &TMessageIdentifier,
            ) -> Result<(), Error> {
                let msg_type_u8: u8 = identifier.message_type.into();
                let version = (VERSION_1 | msg_type_u8 as u32) as i32;
                self.write_i32(version as i32)?;
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

            fn write_field_begin(&mut self, identifier: &TFieldIdentifier) -> Result<(), Error> {
                if identifier.field_type != TType::Stop {
                    if identifier.id.is_some() {
                        let mut data: [u8; 3] = [0; 3];
                        data[0] = identifier.field_type.into();
                        let id = identifier.id.unwrap().to_be_bytes();
                        data[1] = id[0];
                        data[2] = id[1];
                        self.trans.write_slice(&data)?;
                    } else {
                        return Err(new_protocol_error(
                            ProtocolErrorKind::Unknown,
                            format!(
                                "cannot write identifier {:?} without sequence number",
                                &identifier
                            ),
                        ));
                    }
                } else {
                    self.write_byte(identifier.field_type.into())?;
                }
                Ok(())
            }

            #[inline]
            fn write_field_end(&mut self) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn write_field_stop(&mut self) -> Result<(), Error> {
                self.write_byte(TType::Stop.into())
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
            fn write_bytes(&mut self, b: &[u8]) -> Result<(), Error> {
                self.write_i32(b.len() as i32)?;
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
                self.write_bytes(s.as_bytes())
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

            fn reserve(&mut self, size: usize) {
                self.trans.reserve(size)
            }

            fn buf_mut(&mut self) -> &mut BytesMut {
                self.trans
            }
        }
    };
}

impl_output_bytes_mut!(&mut BytesMut);

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
        let v = self.read_bytes().await?;
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
    pub async fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let len = self.reader.read_i32().await? as usize;
        // FIXME: use maybe_uninit?
        let mut v = vec![0; len];
        self.reader.read_exact(&mut v).await?;
        Ok(v)
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
    fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let s = self.read_string()?;
        Ok(s.into_bytes())
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

    fn buf_mut(&mut self) -> &mut Self::Buf {
        self.trans
    }
}
