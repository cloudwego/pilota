use std::{convert::TryInto, ptr, slice, str};

use bytes::{Buf, Bytes};
use faststr::FastStr;
use smallvec::SmallVec;

use crate::thrift::{
    error::ProtocolExceptionKind, new_protocol_exception, ProtocolException, TFieldIdentifier,
    TInputProtocol, TLengthProtocol, TListIdentifier, TMapIdentifier, TMessageIdentifier,
    TMessageType, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType, ThriftException,
    BINARY_BASIC_TYPE_FIXED_SIZE,
};

static VERSION_1: u32 = 0x80010000;
static VERSION_MASK: u32 = 0xffff0000;

const FIELD_BEGIN_LEN: usize = 3;

pub struct TBinaryUnsafeOutputProtocol<'a> {
    pub(crate) trans: &'a mut [u8],
    pub(crate) index: usize,
}

impl<'a> TBinaryUnsafeOutputProtocol<'a> {
    #[inline]
    pub fn new(trans: &'a mut [u8]) -> Self {
        Self { trans, index: 0 }
    }

    #[doc(hidden)]
    pub fn index(&self) -> usize {
        self.index
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

impl<'a> TLengthProtocol for TBinaryUnsafeOutputProtocol<'a> {
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

impl<'a> TOutputProtocol for TBinaryUnsafeOutputProtocol<'a> {
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
        unsafe {
            *self.trans.get_unchecked_mut(self.index) = field_type as u8;
            let buf: &mut [u8; 2] = self
                .trans
                .get_unchecked_mut(self.index + 1..self.index + 3)
                .try_into()
                .unwrap_unchecked();
            *buf = id.to_be_bytes();
            self.index += 3;
        }
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
        unsafe {
            ptr::copy_nonoverlapping(b.as_ptr(), self.trans.as_mut_ptr().add(self.index), b.len());
            self.index += b.len();
        }
        Ok(())
    }

    #[inline]
    fn write_byte(&mut self, b: u8) -> Result<(), ThriftException> {
        unsafe {
            *self.trans.get_unchecked_mut(self.index) = b;
            self.index += 1;
        }
        Ok(())
    }

    #[inline]
    fn write_uuid(&mut self, u: [u8; 16]) -> Result<(), ThriftException> {
        unsafe {
            let buf: &mut [u8; 16] = self
                .trans
                .get_unchecked_mut(self.index..self.index + 16)
                .try_into()
                .unwrap_unchecked();
            *buf = u;
            self.index += 16;
        }
        Ok(())
    }

    #[inline]
    fn write_i8(&mut self, i: i8) -> Result<(), ThriftException> {
        unsafe {
            *self.trans.get_unchecked_mut(self.index) = *i.to_be_bytes().get_unchecked(0);
            self.index += 1;
        }
        Ok(())
    }

    #[inline]
    fn write_i16(&mut self, i: i16) -> Result<(), ThriftException> {
        unsafe {
            let buf: &mut [u8; 2] = self
                .trans
                .get_unchecked_mut(self.index..self.index + 2)
                .try_into()
                .unwrap_unchecked();
            *buf = i.to_be_bytes();
            self.index += 2;
        }
        Ok(())
    }

    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), ThriftException> {
        unsafe {
            let buf: &mut [u8; 4] = self
                .trans
                .get_unchecked_mut(self.index..self.index + 4)
                .try_into()
                .unwrap_unchecked();
            *buf = i.to_be_bytes();
            self.index += 4;
        }
        Ok(())
    }

    #[inline]
    fn write_i64(&mut self, i: i64) -> Result<(), ThriftException> {
        unsafe {
            let buf: &mut [u8; 8] = self
                .trans
                .get_unchecked_mut(self.index..self.index + 8)
                .try_into()
                .unwrap_unchecked();
            *buf = i.to_be_bytes();
            self.index += 8;
        }
        Ok(())
    }

    #[inline]
    fn write_double(&mut self, d: f64) -> Result<(), ThriftException> {
        unsafe {
            let buf: &mut [u8; 8] = self
                .trans
                .get_unchecked_mut(self.index..self.index + 8)
                .try_into()
                .unwrap_unchecked();
            *buf = d.to_bits().to_be_bytes();
            self.index += 8;
        }
        Ok(())
    }

    #[inline]
    fn write_string(&mut self, s: &str) -> Result<(), ThriftException> {
        self.write_i32(s.len() as i32)?;
        unsafe {
            ptr::copy_nonoverlapping(s.as_ptr(), self.trans.as_mut_ptr().add(self.index), s.len());
            self.index += s.len();
        }
        Ok(())
    }

    #[inline]
    fn write_faststr(&mut self, s: FastStr) -> Result<(), ThriftException> {
        self.write_i32(s.len() as i32)?;
        unsafe {
            ptr::copy_nonoverlapping(s.as_ptr(), self.trans.as_mut_ptr().add(self.index), s.len());
            self.index += s.len();
        }
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
        unsafe {
            ptr::copy_nonoverlapping(b.as_ptr(), self.trans.as_mut_ptr().add(self.index), b.len());
            self.index += b.len();
        }
        Ok(())
    }

    #[inline]
    fn buf_mut(&mut self) -> &mut Self::BufMut {
        &mut self.trans
    }
}

pub struct TBinaryUnsafeInputProtocol<'a> {
    pub(crate) trans: &'a mut Bytes,
    pub(crate) buf: &'a [u8],
    pub(crate) index: usize,
}

impl<'a> TBinaryUnsafeInputProtocol<'a> {
    /// # Safety
    ///
    /// The 'trans' MUST have enough capacity to read from or write to.
    #[inline]
    pub unsafe fn new(trans: &'a mut Bytes) -> Self {
        let buf = slice::from_raw_parts(trans.as_ptr(), trans.len());
        Self {
            trans,
            buf,
            index: 0,
        }
    }

    #[doc(hidden)]
    pub fn index(&self) -> usize {
        self.index
    }

    #[doc(hidden)]
    fn advance(&mut self, len: usize) {
        self.trans.advance(len);
        self.buf.advance(len);
        self.index -= len;
    }
}

impl<'a> TLengthProtocol for TBinaryUnsafeInputProtocol<'a> {
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

struct SkipData {
    pub ttype: [TType; 2],
    pub len: u32,
}

macro_rules! skip_stack_pop {
    ($stack: expr) => {
        let top: &mut SkipData = $stack.last_mut().unwrap();
        top.len -= 1;
        if (top.len == 0) {
            $stack.pop();
        }
    };
}

impl<'a> TInputProtocol for TBinaryUnsafeInputProtocol<'a> {
    type Buf = Bytes;

    fn read_message_begin(&mut self) -> Result<TMessageIdentifier, ThriftException> {
        let size = self.read_i32()?;

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
        self.advance(self.index);
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
        let len = self.read_i32()?;
        self.advance(self.index);
        let val = Bytes::copy_from_slice(&self.trans.split_to(len as usize));
        self.buf = unsafe { slice::from_raw_parts(self.trans.as_ptr(), self.trans.len()) };
        Ok(val)
    }

    #[inline]
    fn get_bytes(
        &mut self,
        ptr: Option<*const u8>,
        mut len: usize,
    ) -> Result<Bytes, ThriftException> {
        if ptr.is_none() {
            len -= self.index;
            self.advance(self.index);
        }
        self.index = 0;
        let val = Bytes::copy_from_slice(&self.trans.split_to(len));
        self.buf = unsafe { slice::from_raw_parts(self.trans.as_ptr(), self.trans.len()) };

        Ok(val)
    }

    #[inline]
    fn read_uuid(&mut self) -> Result<[u8; 16], ThriftException> {
        let u;
        unsafe {
            u = self
                .buf
                .get_unchecked(self.index..self.index + 16)
                .try_into()
                .unwrap_unchecked();
            self.index += 16;
        }
        Ok(u)
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8, ThriftException> {
        unsafe {
            let val = *self.buf.get_unchecked(self.index) as i8;
            self.index += 1;
            Ok(val)
        }
    }

    #[inline]
    fn read_i16(&mut self) -> Result<i16, ThriftException> {
        unsafe {
            let val = self.buf.get_unchecked(self.index..self.index + 2);
            self.index += 2;
            Ok(i16::from_be_bytes(val.try_into().unwrap_unchecked()))
        }
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32, ThriftException> {
        unsafe {
            let val = self.buf.get_unchecked(self.index..self.index + 4);
            self.index += 4;
            Ok(i32::from_be_bytes(val.try_into().unwrap_unchecked()))
        }
    }

    #[inline]
    fn read_i64(&mut self) -> Result<i64, ThriftException> {
        unsafe {
            let val = self.buf.get_unchecked(self.index..self.index + 8);
            self.index += 8;
            Ok(i64::from_be_bytes(val.try_into().unwrap_unchecked()))
        }
    }

    #[inline]
    fn read_double(&mut self) -> Result<f64, ThriftException> {
        unsafe {
            let val = self.buf.get_unchecked(self.index..self.index + 8);
            self.index += 8;
            Ok(f64::from_bits(u64::from_be_bytes(
                val.try_into().unwrap_unchecked(),
            )))
        }
    }

    #[inline]
    fn read_string(&mut self) -> Result<String, ThriftException> {
        unsafe {
            let len = self.read_i32().unwrap_unchecked();
            let val = str::from_utf8_unchecked(
                self.buf
                    .get_unchecked(self.index..self.index + len as usize),
            )
            .to_string();
            self.index += len as usize;
            Ok(val)
        }
    }

    #[inline]
    fn read_faststr(&mut self) -> Result<FastStr, ThriftException> {
        unsafe {
            let len = self.read_i32().unwrap_unchecked() as usize;
            self.advance(self.index);
            let bytes = Bytes::copy_from_slice(&self.trans.split_to(len));
            self.buf = slice::from_raw_parts(self.trans.as_ptr(), self.trans.len());
            Ok(FastStr::from_bytes_unchecked(bytes))
        }
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
        unsafe {
            let val = *self.buf.get_unchecked(self.index);
            self.index += 1;
            Ok(val)
        }
    }

    #[inline]
    fn read_bytes_vec(&mut self) -> Result<Vec<u8>, ThriftException> {
        let len = self.read_i32()? as usize;
        self.advance(self.index);
        let val = self.trans.split_to(len).into();
        self.buf = unsafe { slice::from_raw_parts(self.trans.as_ptr(), self.trans.len()) };
        Ok(val)
    }

    #[inline]
    fn buf(&mut self) -> &mut Self::Buf {
        self.trans
    }

    #[inline]
    fn skip(&mut self, field_type: TType) -> Result<usize, ThriftException> {
        debug_assert!(self.index >= FIELD_BEGIN_LEN);

        self.advance(self.index - FIELD_BEGIN_LEN);
        self.buf = unsafe { slice::from_raw_parts(self.trans.as_ptr(), self.trans.len()) };

        self.skip_till_depth(field_type, crate::thrift::MAXIMUM_SKIP_DEPTH)
    }

    /// Skip a field with type `field_type` iterativly.
    fn skip_till_depth(&mut self, field_type: TType, _: i8) -> Result<usize, ThriftException> {
        let mut ttype = field_type;
        let mut len: usize = 0;
        let mut stack: SmallVec<[SkipData; 8]> = SmallVec::<[SkipData; 8]>::new();

        if field_type == TType::Struct {
            stack.push(SkipData {
                ttype: [TType::Struct, TType::Struct],
                len: 1,
            });
        }

        loop {
            match ttype {
                TType::Bool => {
                    self.index += 1;
                    len += 1;
                }
                TType::I8 => {
                    self.index += 1;
                    len += 1;
                }
                TType::I16 => {
                    self.index += 2;
                    len += 2;
                }
                TType::I32 => {
                    self.index += 4;
                    len += 4;
                }
                TType::I64 => {
                    self.index += 8;
                    len += 8;
                }
                TType::Double => {
                    self.index += 8;
                    len += 8;
                }
                TType::Binary => {
                    let length = unsafe { self.read_i32().unwrap_unchecked() };
                    len += 4 + length as usize;
                    self.index += length as usize;
                }
                TType::Uuid => {
                    self.index += 16;
                    len += 16;
                }
                TType::Struct => {
                    let field_ident = self.read_field_begin()?;
                    if field_ident.field_type == TType::Stop {
                        len += self.field_stop_len();
                        skip_stack_pop!(stack);
                    } else {
                        len += self.field_begin_len(field_ident.field_type, field_ident.id);
                        let fixed_size =
                            BINARY_BASIC_TYPE_FIXED_SIZE[field_ident.field_type as usize];
                        if fixed_size > 0 {
                            // fastpath
                            self.index += fixed_size;
                            len += fixed_size;
                            continue;
                        } else {
                            stack.push(SkipData {
                                ttype: [field_ident.field_type, field_ident.field_type],
                                len: 1,
                            });
                        }
                    }
                }
                TType::List => {
                    let list_ident = self.read_list_begin()?;
                    len += self.list_begin_len(list_ident);
                    if list_ident.size != 0 {
                        let fixed_size =
                            BINARY_BASIC_TYPE_FIXED_SIZE[list_ident.element_type as usize];
                        if fixed_size > 0 {
                            // fastpath
                            self.index += fixed_size * list_ident.size;
                            len += fixed_size * list_ident.size;
                        } else {
                            // slowpath
                            stack.push(SkipData {
                                ttype: [list_ident.element_type, list_ident.element_type],
                                len: list_ident.size as u32,
                            });
                        }
                    }
                }
                TType::Set => {
                    let set_ident = self.read_set_begin()?;
                    len += self.set_begin_len(set_ident);
                    if set_ident.size != 0 {
                        let fixed_size =
                            BINARY_BASIC_TYPE_FIXED_SIZE[set_ident.element_type as usize];
                        if fixed_size > 0 {
                            // fastpath
                            self.index += fixed_size * set_ident.size;
                            len += fixed_size * set_ident.size;
                        } else {
                            // slowpath
                            stack.push(SkipData {
                                ttype: [set_ident.element_type, set_ident.element_type],
                                len: set_ident.size as u32,
                            });
                        }
                    }
                }
                TType::Map => {
                    let map_ident = self.read_map_begin()?;
                    len += self.map_begin_len(map_ident);
                    if map_ident.size > 0 {
                        let key_fixed_size =
                            BINARY_BASIC_TYPE_FIXED_SIZE[map_ident.key_type as usize];
                        let val_fixed_size =
                            BINARY_BASIC_TYPE_FIXED_SIZE[map_ident.value_type as usize];
                        if key_fixed_size > 0 && val_fixed_size > 0 {
                            // fastpath
                            let skip_size = (key_fixed_size + val_fixed_size) * map_ident.size;
                            self.index += skip_size;
                            len += skip_size;
                        } else {
                            // slowpath
                            stack.push(SkipData {
                                ttype: [map_ident.key_type, map_ident.value_type],
                                len: (map_ident.size * 2) as u32,
                            });
                        }
                    }
                }
                u => {
                    return Err(new_protocol_exception(
                        ProtocolExceptionKind::DepthLimit,
                        format!("cannot skip field type {:?}", &u),
                    ))
                }
            };

            if stack.is_empty() {
                return Ok(len);
            }

            let top = stack.last().unwrap();
            ttype = top.ttype[(top.len & 1) as usize];
            if ttype != TType::Struct {
                skip_stack_pop!(stack);
            }
        }
    }
}
