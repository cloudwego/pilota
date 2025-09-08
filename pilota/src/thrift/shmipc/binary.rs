use std::{convert::TryInto, str};

use bytes::Bytes;
use faststr::FastStr;

use crate::thrift::{
    error::ProtocolExceptionKind,
    new_protocol_exception,
    rw_ext::{ReadExt, WriteExt},
    ProtocolException, TFieldIdentifier, TInputProtocol, TLengthProtocol, TListIdentifier,
    TMapIdentifier, TMessageIdentifier, TMessageType, TOutputProtocol, TSetIdentifier,
    TStructIdentifier, TType, ThriftException,
};

static VERSION_1: u32 = 0x80010000;
static VERSION_MASK: u32 = 0xffff0000;

pub struct TBinaryProtocol<T> {
    pub(crate) trans: T,
}

impl<T> TBinaryProtocol<T> {
    #[inline]
    pub fn new(trans: T) -> Self {
        Self { trans }
    }
}

#[inline]
fn field_type_from_u8(ttype: u8) -> Result<TType, ProtocolException> {
    let ttype: TType = ttype.try_into().map_err(|_| {
        ProtocolException::new(
            ProtocolExceptionKind::InvalidData,
            format!("invalid ttype {}", ttype),
        )
    })?;

    Ok(ttype)
}

impl<T> TLengthProtocol for TBinaryProtocol<T> {
    #[inline]
    fn message_begin_len(&mut self, identifier: &TMessageIdentifier) -> usize {
        self.i32_len(0) + self.faststr_len(&identifier.name) + self.i32_len(0)
    }

    #[inline]
    fn message_end_len(&mut self) -> usize {
        0
    }

    #[inline]
    fn struct_begin_len(&mut self, _identifier: &TStructIdentifier) -> usize {
        0
    }

    #[inline]
    fn struct_end_len(&mut self) -> usize {
        0
    }

    #[inline]
    fn field_begin_len(&mut self, _field_type: TType, _id: Option<i16>) -> usize {
        self.byte_len(0) + self.i16_len(0)
    }

    #[inline]
    fn field_end_len(&mut self) -> usize {
        0
    }

    #[inline]
    fn field_stop_len(&mut self) -> usize {
        self.byte_len(0)
    }

    #[inline]
    fn bool_len(&mut self, _b: bool) -> usize {
        self.i8_len(0)
    }

    #[inline]
    fn bytes_len(&mut self, b: &[u8]) -> usize {
        self.i32_len(0) + b.len()
    }

    #[inline]
    fn byte_len(&mut self, _b: u8) -> usize {
        1
    }

    #[inline]
    fn uuid_len(&mut self, _u: [u8; 16]) -> usize {
        16
    }

    #[inline]
    fn i8_len(&mut self, _i: i8) -> usize {
        1
    }

    #[inline]
    fn i16_len(&mut self, _i: i16) -> usize {
        2
    }

    #[inline]
    fn i32_len(&mut self, _i: i32) -> usize {
        4
    }

    #[inline]
    fn i64_len(&mut self, _i: i64) -> usize {
        8
    }

    #[inline]
    fn double_len(&mut self, _d: f64) -> usize {
        8
    }

    fn string_len(&mut self, s: &str) -> usize {
        self.i32_len(0) + s.len()
    }

    #[inline]
    fn faststr_len(&mut self, s: &FastStr) -> usize {
        self.i32_len(0) + s.len()
    }

    #[inline]
    fn list_begin_len(&mut self, _identifier: TListIdentifier) -> usize {
        self.byte_len(0) + self.i32_len(0)
    }

    #[inline]
    fn list_end_len(&mut self) -> usize {
        0
    }

    #[inline]
    fn set_begin_len(&mut self, _identifier: TSetIdentifier) -> usize {
        self.byte_len(0) + self.i32_len(0)
    }

    #[inline]
    fn set_end_len(&mut self) -> usize {
        0
    }

    #[inline]
    fn map_begin_len(&mut self, _identifier: TMapIdentifier) -> usize {
        self.byte_len(0) + self.byte_len(0) + self.i32_len(0)
    }

    #[inline]
    fn map_end_len(&mut self) -> usize {
        0
    }

    #[inline]
    fn bytes_vec_len(&mut self, b: &[u8]) -> usize {
        self.i32_len(0) + b.len()
    }
}

impl<'a> TOutputProtocol for TBinaryProtocol<&'a mut [u8]> {
    type BufMut = &'a mut [u8];

    #[inline]
    fn write_message_begin(
        &mut self,
        identifier: &TMessageIdentifier,
    ) -> Result<(), ThriftException> {
        let msg_type_u8: u8 = identifier.message_type.into();
        let version = (VERSION_1 | msg_type_u8 as u32) as i32;
        self.write_i32(version)?;
        self.write_faststr(identifier.name.clone())?;
        self.write_i32(identifier.sequence_number)?;
        Ok(())
    }

    #[inline]
    fn write_message_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn write_struct_begin(&mut self, _: &TStructIdentifier) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn write_struct_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn write_field_begin(&mut self, field_type: TType, id: i16) -> Result<(), ThriftException> {
        let mut data: [u8; 3] = [0; 3];
        data[0] = field_type as u8;
        let id = id.to_be_bytes();
        data[1] = id[0];
        data[2] = id[1];
        self.trans.write_slice(&data);
        Ok(())
    }

    #[inline]
    fn write_field_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn write_field_stop(&mut self) -> Result<(), ThriftException> {
        self.write_byte(TType::Stop as u8)
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<(), ThriftException> {
        if b {
            self.write_i8(1)
        } else {
            self.write_i8(0)
        }
    }

    #[inline]
    fn write_bytes(&mut self, b: Bytes) -> Result<(), ThriftException> {
        self.write_i32(b.len() as i32)?;
        self.write_bytes_without_len(b)
    }

    #[inline]
    fn write_bytes_without_len(&mut self, b: Bytes) -> Result<(), ThriftException> {
        self.trans.write_slice(&b);
        Ok(())
    }

    #[inline]
    fn write_byte(&mut self, b: u8) -> Result<(), ThriftException> {
        self.trans.write_u8(b);
        Ok(())
    }

    #[inline]
    fn write_uuid(&mut self, u: [u8; 16]) -> Result<(), ThriftException> {
        self.trans.write_slice(&u);
        Ok(())
    }

    #[inline]
    fn write_i8(&mut self, i: i8) -> Result<(), ThriftException> {
        self.trans.write_i8(i);
        Ok(())
    }

    #[inline]
    fn write_i16(&mut self, i: i16) -> Result<(), ThriftException> {
        self.trans.write_i16(i);
        Ok(())
    }

    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), ThriftException> {
        self.trans.write_i32(i);
        Ok(())
    }

    #[inline]
    fn write_i64(&mut self, i: i64) -> Result<(), ThriftException> {
        self.trans.write_i64(i);
        Ok(())
    }

    #[inline]
    fn write_double(&mut self, d: f64) -> Result<(), ThriftException> {
        self.trans.write_f64(d);
        Ok(())
    }

    #[inline]
    fn write_string(&mut self, s: &str) -> Result<(), ThriftException> {
        self.write_i32(s.len() as i32)?;
        self.trans.write_slice(s.as_bytes());
        Ok(())
    }

    #[inline]
    fn write_faststr(&mut self, s: FastStr) -> Result<(), ThriftException> {
        self.write_i32(s.len() as i32)?;
        self.trans.write_slice(s.as_ref());
        Ok(())
    }

    #[inline]
    fn write_list_begin(&mut self, identifier: TListIdentifier) -> Result<(), ThriftException> {
        self.write_byte(identifier.element_type.into())?;
        self.write_i32(identifier.size as i32)
    }

    #[inline]
    fn write_list_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn write_set_begin(&mut self, identifier: TSetIdentifier) -> Result<(), ThriftException> {
        self.write_byte(identifier.element_type.into())?;
        self.write_i32(identifier.size as i32)
    }

    #[inline]
    fn write_set_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn write_map_begin(&mut self, identifier: TMapIdentifier) -> Result<(), ThriftException> {
        let key_type = identifier.key_type;
        self.write_byte(key_type.into())?;
        let val_type = identifier.value_type;
        self.write_byte(val_type.into())?;
        self.write_i32(identifier.size as i32)
    }

    #[inline]
    fn write_map_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn write_bytes_vec(&mut self, b: &[u8]) -> Result<(), ThriftException> {
        self.write_i32(b.len() as i32)?;
        self.trans.write_slice(b);
        Ok(())
    }

    #[inline]
    fn buf_mut(&mut self) -> &mut Self::BufMut {
        &mut self.trans
    }
}

impl TInputProtocol for TBinaryProtocol<&mut Bytes> {
    type Buf = Bytes;

    fn read_message_begin(&mut self) -> Result<TMessageIdentifier, ThriftException> {
        let size = self.trans.read_i32()?;

        if size > 0 {
            return Err(new_protocol_exception(
                ProtocolExceptionKind::BadVersion,
                "Missing version in ReadMessageBegin".to_string(),
            ));
        }
        let type_u8 = (size & 0xf) as u8;

        let message_type = TMessageType::try_from(type_u8).map_err(|_| {
            new_protocol_exception(
                ProtocolExceptionKind::InvalidData,
                format!("invalid message type {}", type_u8),
            )
        })?;

        let version = size & (VERSION_MASK as i32);
        if version != (VERSION_1 as i32) {
            return Err(new_protocol_exception(
                ProtocolExceptionKind::BadVersion,
                "Bad version in ReadMessageBegin",
            ));
        }

        let name = self.read_faststr()?;

        let sequence_number = self.read_i32()?;
        Ok(TMessageIdentifier::new(name, message_type, sequence_number))
    }

    #[inline]
    fn read_message_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn read_struct_begin(&mut self) -> Result<Option<TStructIdentifier>, ThriftException> {
        Ok(None)
    }

    #[inline]
    fn read_struct_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn read_field_begin(&mut self) -> Result<TFieldIdentifier, ThriftException> {
        let field_type_byte = self.read_byte()?;
        let field_type = field_type_byte.try_into().map_err(|_| {
            new_protocol_exception(
                ProtocolExceptionKind::InvalidData,
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
    fn read_field_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn read_bool(&mut self) -> Result<bool, ThriftException> {
        let b = self.read_i8()?;
        match b {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    #[inline]
    fn read_bytes(&mut self) -> Result<Bytes, ThriftException> {
        let len = self.trans.read_i32()?;
        Ok(Bytes::copy_from_slice(&self.trans.split_to(len as usize)))
    }

    #[inline]
    fn get_bytes(&mut self, ptr: Option<*const u8>, len: usize) -> Result<Bytes, ThriftException> {
        if let Some(ptr) = ptr {
            Ok(Bytes::copy_from_slice(unsafe {
                std::slice::from_raw_parts(ptr, len)
            }))
        } else {
            Ok(Bytes::copy_from_slice(&self.trans.split_to(len)))
        }
    }

    #[inline]
    fn read_uuid(&mut self) -> Result<[u8; 16], ThriftException> {
        let mut u = [0; 16];
        self.trans.read_to_slice(&mut u)?;
        Ok(u)
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8, ThriftException> {
        Ok(self.trans.read_i8()?)
    }

    #[inline]
    fn read_i16(&mut self) -> Result<i16, ThriftException> {
        Ok(self.trans.read_i16()?)
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32, ThriftException> {
        Ok(self.trans.read_i32()?)
    }

    #[inline]
    fn read_i64(&mut self) -> Result<i64, ThriftException> {
        Ok(self.trans.read_i64()?)
    }

    #[inline]
    fn read_double(&mut self) -> Result<f64, ThriftException> {
        Ok(self.trans.read_f64()?)
    }

    #[inline]
    fn read_string(&mut self) -> Result<String, ThriftException> {
        let len = self.trans.read_i32()?;
        Ok(self.trans.read_to_string(len as usize)?)
    }

    #[inline]
    fn read_faststr(&mut self) -> Result<FastStr, ThriftException> {
        let len = self.trans.read_i32()? as usize;
        let bytes = Bytes::copy_from_slice(&self.trans.split_to(len));
        unsafe { Ok(FastStr::from_bytes_unchecked(bytes)) }
    }

    #[inline]
    fn read_list_begin(&mut self) -> Result<TListIdentifier, ThriftException> {
        let element_type: TType = self.read_byte().and_then(|n| Ok(field_type_from_u8(n)?))?;
        let size = self.read_i32()?;
        Ok(TListIdentifier::new(element_type, size as usize))
    }

    #[inline]
    fn read_list_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn read_set_begin(&mut self) -> Result<TSetIdentifier, ThriftException> {
        let element_type: TType = self.read_byte().and_then(|n| Ok(field_type_from_u8(n)?))?;
        let size = self.read_i32()?;
        Ok(TSetIdentifier::new(element_type, size as usize))
    }

    #[inline]
    fn read_set_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn read_map_begin(&mut self) -> Result<TMapIdentifier, ThriftException> {
        let key_type: TType = self.read_byte().and_then(|n| Ok(field_type_from_u8(n)?))?;
        let value_type: TType = self.read_byte().and_then(|n| Ok(field_type_from_u8(n)?))?;
        let size = self.read_i32()?;
        Ok(TMapIdentifier::new(key_type, value_type, size as usize))
    }

    #[inline]
    fn read_map_end(&mut self) -> Result<(), ThriftException> {
        Ok(())
    }

    #[inline]
    fn read_byte(&mut self) -> Result<u8, ThriftException> {
        Ok(self.trans.read_u8()?)
    }

    #[inline]
    fn read_bytes_vec(&mut self) -> Result<Vec<u8>, ThriftException> {
        let len = self.trans.read_i32()? as usize;
        Ok(self.trans.split_to(len).into())
    }

    #[inline]
    fn buf(&mut self) -> &mut Self::Buf {
        self.trans
    }
}
