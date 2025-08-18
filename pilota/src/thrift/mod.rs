pub mod binary;
pub mod binary_le;
pub mod binary_unsafe;
pub mod compact;
pub mod error;
pub mod rw_ext;
pub mod shmipc;
pub mod unknown;
pub mod varint_ext;

use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    ops::Deref,
    sync::Arc,
};

use bytes::{Buf, BufMut, Bytes};
pub use error::*;
use faststr::FastStr;

pub use self::{binary::TAsyncBinaryProtocol, compact::TAsyncCompactProtocol};
use crate::{AHashMap, AHashSet, assert_remaining, thrift::rw_ext::IOError};

const MAXIMUM_SKIP_DEPTH: i8 = 64;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
// According to the benchmark, 1KB is the suitable threshold for zero-copy on
// Apple Silicon.
const ZERO_COPY_THRESHOLD: usize = 1024;

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
// While 4KB is better for other platforms (mainly amd64 linux).
const ZERO_COPY_THRESHOLD: usize = 4 * 1024;

pub static VOID_IDENT: TStructIdentifier = TStructIdentifier { name: "void" };

pub trait Message: Sized + Send {
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<(), ThriftException>;

    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self, ThriftException>;

    fn decode_async<T: TAsyncInputProtocol>(
        protocol: &mut T,
    ) -> impl Future<Output = Result<Self, ThriftException>> + Send;

    fn size<T: TLengthProtocol>(&self, protocol: &mut T) -> usize;
}

impl<M: Message> Message for Box<M> {
    #[inline]
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<(), ThriftException> {
        self.deref().encode(protocol)
    }

    #[inline]
    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self, ThriftException> {
        Ok(Box::new(M::decode(protocol)?))
    }

    async fn decode_async<T: TAsyncInputProtocol>(
        protocol: &mut T,
    ) -> Result<Self, ThriftException> {
        Ok(Box::new(M::decode_async(protocol).await?))
    }

    #[inline]
    fn size<T: TLengthProtocol>(&self, protocol: &mut T) -> usize {
        self.deref().size(protocol)
    }
}

impl<M: Message + Send + Sync> Message for Arc<M> {
    #[inline]
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<(), ThriftException> {
        self.deref().encode(protocol)
    }

    #[inline]
    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self, ThriftException> {
        Ok(Arc::new(M::decode(protocol)?))
    }

    async fn decode_async<T: TAsyncInputProtocol>(
        protocol: &mut T,
    ) -> Result<Self, ThriftException> {
        Ok(Arc::new(M::decode_async(protocol).await?))
    }

    #[inline]
    fn size<T: TLengthProtocol>(&self, protocol: &mut T) -> usize {
        self.deref().size(protocol)
    }
}

pub trait TInputProtocol: TLengthProtocol {
    type Buf: Buf;
    /// Read the beginning of a Thrift message.
    fn read_message_begin(&mut self) -> Result<TMessageIdentifier, ThriftException>;
    /// Read the end of a Thrift message.
    fn read_message_end(&mut self) -> Result<(), ThriftException>;
    /// Read the beginning of a Thrift struct.
    fn read_struct_begin(&mut self) -> Result<Option<TStructIdentifier>, ThriftException>;
    /// Read the end of a Thrift struct.
    fn read_struct_end(&mut self) -> Result<(), ThriftException>;
    /// Read the beginning of a Thrift struct field.
    fn read_field_begin(&mut self) -> Result<TFieldIdentifier, ThriftException>;
    /// Read the end of a Thrift struct field.
    fn read_field_end(&mut self) -> Result<(), ThriftException>;
    /// Read a bool.
    fn read_bool(&mut self) -> Result<bool, ThriftException>;
    /// Read a binary.
    fn read_bytes(&mut self) -> Result<Bytes, ThriftException>;
    /// Read a uuid.
    fn read_uuid(&mut self) -> Result<[u8; 16], ThriftException>;
    /// Read a word.
    fn read_i8(&mut self) -> Result<i8, ThriftException>;
    /// Read a 16-bit signed integer.
    fn read_i16(&mut self) -> Result<i16, ThriftException>;
    /// Read a 32-bit signed integer.
    fn read_i32(&mut self) -> Result<i32, ThriftException>;
    /// Read a 64-bit signed integer.
    fn read_i64(&mut self) -> Result<i64, ThriftException>;
    /// Read a 64-bit float.
    fn read_double(&mut self) -> Result<f64, ThriftException>;
    /// Read a fixed-length string (not null terminated).
    fn read_string(&mut self) -> Result<String, ThriftException>;
    /// Read a faststr.
    fn read_faststr(&mut self) -> Result<FastStr, ThriftException>;
    /// Read the beginning of a list.
    fn read_list_begin(&mut self) -> Result<TListIdentifier, ThriftException>;
    /// Read the end of a list.
    fn read_list_end(&mut self) -> Result<(), ThriftException>;
    /// Read the beginning of a set.
    fn read_set_begin(&mut self) -> Result<TSetIdentifier, ThriftException>;
    /// Read the end of a set.
    fn read_set_end(&mut self) -> Result<(), ThriftException>;
    /// Read the beginning of a map.
    fn read_map_begin(&mut self) -> Result<TMapIdentifier, ThriftException>;
    /// Read the end of a map.
    fn read_map_end(&mut self) -> Result<(), ThriftException>;
    /// Skip a field with type `field_type` recursively until the default
    /// maximum skip depth is reached.
    #[inline]
    fn skip(&mut self, field_type: TType) -> Result<usize, ThriftException> {
        self.skip_till_depth(field_type, MAXIMUM_SKIP_DEPTH)
    }
    /// Skip a field with type `field_type` recursively up to `depth` levels.
    fn skip_till_depth(&mut self, field_type: TType, depth: i8) -> Result<usize, ThriftException> {
        if depth == 0 {
            return Err(new_protocol_exception(
                ProtocolExceptionKind::DepthLimit,
                format!("cannot parse past {field_type:?}"),
            ));
        }
        let mut len = 0;

        match field_type {
            TType::Bool => {
                assert_remaining!(self.buf().remaining() >= 1);
                self.buf().advance(1);
                len += 1;
            }
            TType::I8 => {
                assert_remaining!(self.buf().remaining() >= 1);
                self.buf().advance(1);
                len += 1;
            }
            TType::I16 => {
                assert_remaining!(self.buf().remaining() >= 2);
                self.buf().advance(2);
                len += 2;
            }
            TType::I32 => {
                assert_remaining!(self.buf().remaining() >= 4);
                self.buf().advance(4);
                len += 4;
            }
            TType::I64 => {
                assert_remaining!(self.buf().remaining() >= 8);
                self.buf().advance(8);
                len += 8;
            }
            TType::Double => {
                assert_remaining!(self.buf().remaining() >= 8);
                self.buf().advance(8);
                len += 8;
            }
            TType::Binary => {
                let length = self.read_i32()?;
                assert_remaining!(self.buf().remaining() >= length as usize);
                self.buf().advance(length as usize);
                len += 4 + length as usize;
            }
            TType::Uuid => {
                assert_remaining!(self.buf().remaining() >= 16);
                self.buf().advance(16);
                len += 16;
            }
            TType::Struct => {
                self.read_struct_begin()?;
                len += self.struct_begin_len(&crate::thrift::VOID_IDENT);
                loop {
                    let field_ident = self.read_field_begin()?;
                    if field_ident.field_type == TType::Stop {
                        len += self.field_stop_len();
                        break;
                    } else {
                        len += self.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    len += self.skip_till_depth(field_ident.field_type, depth - 1)?;
                    self.read_field_end()?;
                    len += self.field_end_len();
                }
                self.read_struct_end()?;
                len += self.struct_end_len();
            }
            TType::List => {
                let list_ident = self.read_list_begin()?;
                len += self.list_begin_len(list_ident);
                for _ in 0..list_ident.size {
                    len += self.skip_till_depth(list_ident.element_type, depth - 1)?;
                }
                self.read_list_end()?;
                len += self.list_end_len();
            }
            TType::Set => {
                let set_ident = self.read_set_begin()?;
                len += self.set_begin_len(set_ident);
                for _ in 0..set_ident.size {
                    len += self.skip_till_depth(set_ident.element_type, depth - 1)?;
                }
                self.read_set_end()?;
                len += self.set_end_len();
            }
            TType::Map => {
                let map_ident = self.read_map_begin()?;
                len += self.map_begin_len(map_ident);
                for _ in 0..map_ident.size {
                    let key_type = map_ident.key_type;
                    let val_type = map_ident.value_type;
                    len += self.skip_till_depth(key_type, depth - 1)?;
                    len += self.skip_till_depth(val_type, depth - 1)?;
                }
                self.read_map_end()?;
                len += self.map_end_len();
            }
            u => {
                return Err(new_protocol_exception(
                    ProtocolExceptionKind::DepthLimit,
                    format!("cannot skip field type {:?}", &u),
                ));
            }
        };

        Ok(len)
    }

    // utility (DO NOT USE IN GENERATED CODE!!!!)
    //

    /// Read an unsigned byte.
    ///
    /// This method should **never** be used in generated code.
    fn read_byte(&mut self) -> Result<u8, ThriftException>;

    /// Read a Vec<u8>.
    fn read_bytes_vec(&mut self) -> Result<Vec<u8>, ThriftException>;

    fn get_bytes(&mut self, ptr: Option<*const u8>, len: usize) -> Result<Bytes, ThriftException>;

    #[doc(hidden)]
    fn buf(&mut self) -> &mut Self::Buf;
}

macro_rules! field_len {
    ($ttype:ty, $name:ident($($k:ident: $t:ty),*)) => {
        paste::paste! {
            #[inline]
            fn [<$name _field_len>](&mut self, id: Option<i16>, $($k: $t),*) -> usize {
                self.field_begin_len($ttype, id) + self.[<$name _len>]($($k),*) + self.field_end_len()
            }
        }
    };
}

macro_rules! set_field_len {
    ($name:ident($t:ty)) => {
        paste::paste! {
            #[inline]
            fn [<$name _field_len>]<T, F>(&mut self, id: Option<i16>, el_ttype: TType, els: &$t<T>, el_len: F,) -> usize
            where
                F: Fn(&mut Self, &T) -> usize,
            {
                self.field_begin_len(TType::Set, id)
                    + self.[<$name _len>](el_ttype, els, el_len)
                    + self.field_end_len()
            }

            #[inline]
            fn [<$name _len>]<T, F>(&mut self, el_ttype: TType, els: &$t<T>, el_len: F) -> usize
            where
                F: Fn(&mut Self, &T) -> usize,
            {
                self.set_begin_len(TSetIdentifier {
                    element_type: el_ttype,
                    size: els.len(),
                }) + els.iter().map(|el| el_len(self, el)).sum::<usize>() + self.set_end_len()
            }
        }
    };
}

macro_rules! map_field_len {
    ($name:ident($t:ty)) => {
        paste::paste! {
            #[inline]
            fn [<$name _field_len>]<K, V, FK, FV>(&mut self, id: Option<i16>, key_ttype: TType, val_ttype: TType, els: &$t<K, V>, key_len: FK, val_len: FV,) -> usize
            where
                FK: Fn(&mut Self, &K) -> usize,
                FV: Fn(&mut Self, &V) -> usize,
            {
                self.field_begin_len(TType::Map, id)
                    + self.[<$name _len>](key_ttype, val_ttype, els, key_len, val_len)
                    + self.field_end_len()
            }

            #[inline]
            fn [<$name _len>]<K, V, FK, FV>(&mut self, key_ttype: TType, val_ttype: TType, els: &$t<K, V>, key_len: FK, val_len: FV,) -> usize
            where
                FK: Fn(&mut Self, &K) -> usize,
                FV: Fn(&mut Self, &V) -> usize,
            {
                self.map_begin_len(TMapIdentifier {
                    key_type: key_ttype,
                    value_type: val_ttype,
                    size: els.len(),
                }) + els.iter().map(|(k, v)| key_len(self, k) + val_len(self, v)).sum::<usize>() + self.map_end_len()
            }
        }
    };
}

pub trait TLengthProtocolExt: TLengthProtocol + Sized {
    field_len!(TType::Bool, bool(b: bool));
    field_len!(TType::I8, i8(i: i8));
    field_len!(TType::I16, i16(i: i16));
    field_len!(TType::I32, i32(i: i32));
    field_len!(TType::I64, i64(i: i64));
    field_len!(TType::Double, double(d: f64));
    field_len!(TType::Binary, bytes(b: &[u8]));
    field_len!(TType::Binary, bytes_vec(b: &[u8]));
    field_len!(TType::Uuid, uuid(u: [u8; 16]));
    field_len!(TType::Binary, string(s: &str));
    field_len!(TType::Binary, faststr(s: &FastStr));
    field_len!(TType::I8, byte(b: u8));
    field_len!(TType::Void, void());

    #[inline]
    fn list_field_len<T, F>(
        &mut self,
        id: Option<i16>,
        el_ttype: TType,
        els: &[T],
        el_len: F,
    ) -> usize
    where
        F: Fn(&mut Self, &T) -> usize,
    {
        self.field_begin_len(TType::List, id)
            + self.list_len(el_ttype, els, el_len)
            + self.field_end_len()
    }

    #[inline]
    fn list_len<T, F>(&mut self, el_ttype: TType, els: &[T], len: F) -> usize
    where
        F: Fn(&mut Self, &T) -> usize,
    {
        self.list_begin_len(TListIdentifier {
            element_type: el_ttype,
            size: els.len(),
        }) + els.iter().map(|el| len(self, el)).sum::<usize>()
            + self.list_end_len()
    }

    set_field_len!(set(AHashSet));
    set_field_len!(btree_set(BTreeSet));

    #[inline]
    fn message_len<M: Message>(&mut self, id: Option<i16>, m: &M) -> usize {
        self.field_begin_len(TType::Struct, id) + m.size(self) + self.field_end_len()
    }

    map_field_len!(map(AHashMap));
    map_field_len!(btree_map(BTreeMap));

    #[inline]
    fn void_len(&mut self) -> usize {
        self.struct_begin_len(&crate::thrift::VOID_IDENT) + self.struct_end_len()
    }

    #[inline]
    fn struct_field_len<M: Message>(&mut self, id: Option<i16>, m: &M) -> usize {
        self.field_begin_len(TType::Struct, id) + self.struct_len(m) + self.field_end_len()
    }

    #[inline]
    fn struct_len<M: Message>(&mut self, m: &M) -> usize {
        m.size(self)
    }
}

impl<T> TLengthProtocolExt for T where T: TLengthProtocol {}

pub trait TLengthProtocol {
    // size

    fn message_begin_len(&mut self, identifier: &TMessageIdentifier) -> usize;

    fn message_end_len(&mut self) -> usize;

    fn struct_begin_len(&mut self, identifier: &TStructIdentifier) -> usize;

    fn struct_end_len(&mut self) -> usize;

    fn field_begin_len(&mut self, field_type: TType, id: Option<i16>) -> usize;

    fn field_end_len(&mut self) -> usize;

    fn field_stop_len(&mut self) -> usize;

    fn bool_len(&mut self, b: bool) -> usize;

    fn bytes_len(&mut self, b: &[u8]) -> usize;

    fn bytes_vec_len(&mut self, b: &[u8]) -> usize;

    fn byte_len(&mut self, b: u8) -> usize;

    fn uuid_len(&mut self, u: [u8; 16]) -> usize;

    fn i8_len(&mut self, i: i8) -> usize;

    fn i16_len(&mut self, i: i16) -> usize;

    fn i32_len(&mut self, i: i32) -> usize;

    fn i64_len(&mut self, i: i64) -> usize;

    fn double_len(&mut self, d: f64) -> usize;

    fn string_len(&mut self, s: &str) -> usize;

    fn faststr_len(&mut self, s: &FastStr) -> usize;

    fn list_begin_len(&mut self, identifier: TListIdentifier) -> usize;

    fn list_end_len(&mut self) -> usize;

    fn set_begin_len(&mut self, identifier: TSetIdentifier) -> usize;

    fn set_end_len(&mut self) -> usize;

    fn map_begin_len(&mut self, identifier: TMapIdentifier) -> usize;

    fn map_end_len(&mut self) -> usize;

    /// The zero copy length used to calculate the recommended malloc length.
    fn zero_copy_len(&mut self) -> usize {
        0
    }

    /// Resets the zero copy length counter.
    fn reset(&mut self) {}
}

macro_rules! write_field {
    ($ttype:ty, $name:ident($($k:ident: $t:ty),*)) => {
        paste::paste! {
            #[inline]
            fn [<write_ $name _field>](&mut self, id: i16, $($k: $t),*) -> Result<(), ThriftException> {
                self.write_field_begin($ttype, id)?;
                self.[<write_ $name>]($($k),*)?;
                self.write_field_end()?;
                Ok(())
            }
        }
    };
}

macro_rules! write_set_field {
    ($name:ident($t:ty)) => {
        paste::paste! {
            #[inline]
            fn [<write_ $name _field>]<T, F>(&mut self, id: i16, el_ttype: TType, els: &$t<T>, encode: F,) -> Result<(), ThriftException>
            where
                F: Fn(&mut Self, &T) -> Result<(), ThriftException>,
            {
                self.write_field_begin(TType::Set, id)?;
                self.[<write_ $name>](el_ttype, els, encode)?;
                self.write_field_end()
            }

            #[inline]
            fn [<write_ $name>]<T, F>(&mut self, el_ttype: TType, els: &$t<T>, encode: F,) -> Result<(), ThriftException>
            where
                F: Fn(&mut Self, &T) -> Result<(), ThriftException>,
            {
                self.write_set_begin(TSetIdentifier {
                    element_type: el_ttype,
                    size: els.len(),
                })?;
                for el in els {
                    encode(self, el)?;
                }
                self.write_set_end()
            }
        }
    };
}

macro_rules! write_map_field {
    ($name:ident($t:ty)) => {
        paste::paste! {
            #[inline]
            fn [<write_ $name _field>]<K, V, FK, FV>(&mut self, id: i16, key_ttype: TType, val_ttype: TType, els: &$t<K, V>, key_encode: FK, val_encode: FV,) -> Result<(), ThriftException>
            where
                FK: Fn(&mut Self, &K) -> Result<(), ThriftException>,
                FV: Fn(&mut Self, &V) -> Result<(), ThriftException>,
            {
                self.write_field_begin(TType::Map, id)?;
                self.[<write_ $name>](key_ttype, val_ttype, els, key_encode, val_encode)?;
                self.write_field_end()
            }

            #[inline]
            fn [<write_ $name>]<K, V, FK, FV>(&mut self, key_ttype: TType, val_ttype: TType, els: &$t<K, V>, key_encode: FK, val_encode: FV,) -> Result<(), ThriftException>
            where
                FK: Fn(&mut Self, &K) -> Result<(), ThriftException>,
                FV: Fn(&mut Self, &V) -> Result<(), ThriftException>,
            {
                self.write_map_begin(TMapIdentifier {
                    key_type: key_ttype,
                    value_type: val_ttype,
                    size: els.len(),
                })?;
                for (k, v) in els {
                    key_encode(self, k)?;
                    val_encode(self, v)?;
                }
                self.write_map_end()
            }
        }
    };
}

pub trait TOutputProtocolExt: TOutputProtocol + Sized {
    write_field!(TType::Bool, bool(b: bool));
    write_field!(TType::I8, i8(i: i8));
    write_field!(TType::I16, i16(i: i16));
    write_field!(TType::I32, i32(i: i32));
    write_field!(TType::I64, i64(i: i64));
    write_field!(TType::Double, double(d: f64));
    write_field!(TType::Binary, bytes(b: Bytes));
    write_field!(TType::Binary, bytes_vec(b: &[u8]));
    write_field!(TType::Uuid, uuid(u: [u8; 16]));
    write_field!(TType::Binary, string(s: &str));
    write_field!(TType::Binary, faststr(s: FastStr));
    write_field!(TType::Void, void());
    write_field!(TType::I8, byte(b: u8));

    #[inline]
    fn write_list_field<T, F>(
        &mut self,
        id: i16,
        el_ttype: TType,
        els: &[T],
        encode: F,
    ) -> Result<(), ThriftException>
    where
        F: Fn(&mut Self, &T) -> Result<(), ThriftException>,
    {
        self.write_field_begin(TType::List, id)?;
        self.write_list(el_ttype, els, encode)?;
        self.write_field_end()
    }

    #[inline]
    fn write_list<T, F>(
        &mut self,
        el_ttype: TType,
        els: &[T],
        encode: F,
    ) -> Result<(), ThriftException>
    where
        F: Fn(&mut Self, &T) -> Result<(), ThriftException>,
    {
        self.write_list_begin(TListIdentifier {
            element_type: el_ttype,
            size: els.len(),
        })?;
        for el in els {
            encode(self, el)?
        }
        self.write_list_end()
    }

    write_set_field!(set(AHashSet));
    write_set_field!(btree_set(BTreeSet));

    #[inline]
    fn write_struct_field<M: Message>(
        &mut self,
        id: i16,
        m: &M,
        ty: TType,
    ) -> Result<(), ThriftException> {
        self.write_field_begin(ty, id)?;
        self.write_struct(m)?;
        self.write_field_end()
    }

    #[inline]
    fn write_struct<M: Message>(&mut self, m: &M) -> Result<(), ThriftException> {
        m.encode(self)
    }

    write_map_field!(map(AHashMap));
    write_map_field!(btree_map(BTreeMap));

    #[inline]
    fn write_void(&mut self) -> Result<(), ThriftException> {
        self.write_struct_begin(&crate::thrift::VOID_IDENT)?;
        self.write_struct_end()
    }
}

impl<T> TOutputProtocolExt for T where T: TOutputProtocol {}

pub trait TOutputProtocol: TLengthProtocol {
    type BufMut: BufMut;

    /// Write the beginning of a Thrift message.
    fn write_message_begin(
        &mut self,
        identifier: &TMessageIdentifier,
    ) -> Result<(), ThriftException>;
    /// Write the end of a Thrift message.
    fn write_message_end(&mut self) -> Result<(), ThriftException>;
    /// Write the beginning of a Thrift struct.
    fn write_struct_begin(&mut self, identifier: &TStructIdentifier)
    -> Result<(), ThriftException>;
    /// Write the end of a Thrift struct.
    fn write_struct_end(&mut self) -> Result<(), ThriftException>;
    /// Write the beginning of a Thrift field.
    fn write_field_begin(&mut self, field_type: TType, id: i16) -> Result<(), ThriftException>;
    /// Write the end of a Thrift field.
    fn write_field_end(&mut self) -> Result<(), ThriftException>;
    /// Write a STOP field indicating that all the fields in a struct have been
    /// written.
    fn write_field_stop(&mut self) -> Result<(), ThriftException>;
    /// Write a bool.
    fn write_bool(&mut self, b: bool) -> Result<(), ThriftException>;
    /// Write a fixed-length byte array.
    fn write_bytes(&mut self, b: Bytes) -> Result<(), ThriftException>;
    fn write_bytes_without_len(&mut self, b: Bytes) -> Result<(), ThriftException>;
    /// Write a uuid.
    fn write_uuid(&mut self, u: [u8; 16]) -> Result<(), ThriftException>;
    /// Write a Vec<u8>.
    fn write_bytes_vec(&mut self, b: &[u8]) -> Result<(), ThriftException>;
    /// Write a byte.
    fn write_byte(&mut self, b: u8) -> Result<(), ThriftException>;
    /// Write an 8-bit signed integer.
    fn write_i8(&mut self, i: i8) -> Result<(), ThriftException>;
    /// Write a 16-bit signed integer.
    fn write_i16(&mut self, i: i16) -> Result<(), ThriftException>;
    /// Write a 32-bit signed integer.
    fn write_i32(&mut self, i: i32) -> Result<(), ThriftException>;
    /// Write a 64-bit signed integer.
    fn write_i64(&mut self, i: i64) -> Result<(), ThriftException>;
    /// Write a 64-bit float.
    fn write_double(&mut self, d: f64) -> Result<(), ThriftException>;
    /// Write a fixed-length string.
    fn write_string(&mut self, s: &str) -> Result<(), ThriftException>;
    /// Write a fixed-length faststr.
    fn write_faststr(&mut self, s: FastStr) -> Result<(), ThriftException>;
    /// Write the beginning of a list.
    fn write_list_begin(&mut self, identifier: TListIdentifier) -> Result<(), ThriftException>;
    /// Write the end of a list.
    fn write_list_end(&mut self) -> Result<(), ThriftException>;
    /// Write the beginning of a set.
    fn write_set_begin(&mut self, identifier: TSetIdentifier) -> Result<(), ThriftException>;
    /// Write the end of a set.
    fn write_set_end(&mut self) -> Result<(), ThriftException>;
    /// Write the beginning of a map.
    fn write_map_begin(&mut self, identifier: TMapIdentifier) -> Result<(), ThriftException>;
    /// Write the end of a map.
    fn write_map_end(&mut self) -> Result<(), ThriftException>;
    /// Flush buffered bytes to the underlying transport.
    fn flush(&mut self) -> Result<(), ThriftException>;

    #[doc(hidden)]
    fn buf_mut(&mut self) -> &mut Self::BufMut;
}

pub trait TAsyncInputProtocol: Send {
    /// Read the beginning of a Thrift message.
    fn read_message_begin(
        &mut self,
    ) -> impl Future<Output = Result<TMessageIdentifier, ThriftException>> + Send;

    /// Read the end of a Thrift message.
    fn read_message_end(&mut self) -> impl Future<Output = Result<(), ThriftException>> + Send;

    /// Read the beginning of a Thrift struct.
    fn read_struct_begin(
        &mut self,
    ) -> impl Future<Output = Result<Option<TStructIdentifier>, ThriftException>> + Send;

    /// Read the end of a Thrift struct.
    fn read_struct_end(&mut self) -> impl Future<Output = Result<(), ThriftException>> + Send;

    /// Read the beginning of a Thrift struct field.
    fn read_field_begin(
        &mut self,
    ) -> impl Future<Output = Result<TFieldIdentifier, ThriftException>> + Send;

    /// Read the end of a Thrift struct field.
    fn read_field_end(&mut self) -> impl Future<Output = Result<(), ThriftException>> + Send;

    /// Read a bool.
    fn read_bool(&mut self) -> impl Future<Output = Result<bool, ThriftException>> + Send;

    /// Read a binary.
    fn read_bytes(&mut self) -> impl Future<Output = Result<Bytes, ThriftException>> + Send;

    /// Read a binary, return `Vec<u8>`
    fn read_bytes_vec(&mut self) -> impl Future<Output = Result<Vec<u8>, ThriftException>> + Send;

    /// Read a uuid.
    fn read_uuid(&mut self) -> impl Future<Output = Result<[u8; 16], ThriftException>> + Send;

    /// Read a string.
    fn read_string(&mut self) -> impl Future<Output = Result<String, ThriftException>> + Send;

    /// Read a string, return `FastStr`
    fn read_faststr(&mut self) -> impl Future<Output = Result<FastStr, ThriftException>> + Send;

    /// Read a byte.
    fn read_byte(&mut self) -> impl Future<Output = Result<u8, ThriftException>> + Send;

    /// Read a word.
    fn read_i8(&mut self) -> impl Future<Output = Result<i8, ThriftException>> + Send;

    /// Read a 16-bit signed integer.
    fn read_i16(&mut self) -> impl Future<Output = Result<i16, ThriftException>> + Send;

    /// Read a 32-bit signed integer.
    fn read_i32(&mut self) -> impl Future<Output = Result<i32, ThriftException>> + Send;

    /// Read a 64-bit signed integer.
    fn read_i64(&mut self) -> impl Future<Output = Result<i64, ThriftException>> + Send;

    /// Read a 64-bit float.
    fn read_double(&mut self) -> impl Future<Output = Result<f64, ThriftException>> + Send;

    /// Read the beginning of a list.
    fn read_list_begin(
        &mut self,
    ) -> impl Future<Output = Result<TListIdentifier, ThriftException>> + Send;

    /// Read the end of a list.
    fn read_list_end(&mut self) -> impl Future<Output = Result<(), ThriftException>> + Send;

    /// Read the beginning of a set.
    fn read_set_begin(
        &mut self,
    ) -> impl Future<Output = Result<TSetIdentifier, ThriftException>> + Send;

    /// Read the end of a set.
    fn read_set_end(&mut self) -> impl Future<Output = Result<(), ThriftException>> + Send;

    /// Read the beginning of a map.
    fn read_map_begin(
        &mut self,
    ) -> impl Future<Output = Result<TMapIdentifier, ThriftException>> + Send;

    /// Read the end of a map.
    fn read_map_end(&mut self) -> impl Future<Output = Result<(), ThriftException>> + Send;

    /// Skip a field with type `field_type` recursively until the default
    /// maximum skip depth is reached.
    #[inline]
    fn skip(
        &mut self,
        field_type: TType,
    ) -> impl Future<Output = Result<(), ThriftException>> + Send {
        self.skip_till_depth(field_type, MAXIMUM_SKIP_DEPTH)
    }

    /// Skip a field with type `field_type` recursively up to `depth` levels.
    #[async_recursion::async_recursion]
    async fn skip_till_depth(
        &mut self,
        field_type: TType,
        depth: i8,
    ) -> Result<(), ThriftException> {
        // async move {
        if depth == 0 {
            return Err(new_protocol_exception(
                ProtocolExceptionKind::DepthLimit,
                format!("cannot parse past {field_type:?}"),
            ));
        }

        match field_type {
            TType::Bool => self.read_bool().await.map(|_| ()),
            TType::I8 => self.read_i8().await.map(|_| ()),
            TType::I16 => self.read_i16().await.map(|_| ()),
            TType::I32 => self.read_i32().await.map(|_| ()),
            TType::I64 => self.read_i64().await.map(|_| ()),
            TType::Double => self.read_double().await.map(|_| ()),
            TType::Binary => self.read_string().await.map(|_| ()),
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
            u => Err(new_protocol_exception(
                ProtocolExceptionKind::DepthLimit,
                format!("cannot skip field type {:?}", &u),
            )),
        }
    }
}

// Thrift struct identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TStructIdentifier {
    /// Name of the encoded Thrift struct.
    pub name: &'static str,
}

impl TStructIdentifier {
    /// Create a `TStructIdentifier` for a struct named `name`.
    pub fn new(name: &'static str) -> TStructIdentifier {
        TStructIdentifier { name }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TType {
    Stop = 0,
    Void = 1,
    Bool = 2,
    I8 = 3,
    Double = 4,
    I16 = 6,
    I32 = 8,
    I64 = 10,
    Binary = 11,
    Struct = 12,
    Map = 13,
    Set = 14,
    List = 15,
    Uuid = 16,
}

impl From<TType> for u8 {
    #[inline]
    fn from(ttype: TType) -> Self {
        ttype as u8
    }
}

static TTYPE_LOOKUP: [Option<TType>; 17] = [
    Some(TType::Stop),
    Some(TType::Void),
    Some(TType::Bool),
    Some(TType::I8),
    Some(TType::Double),
    None,
    Some(TType::I16),
    None,
    Some(TType::I32),
    None,
    Some(TType::I64),
    Some(TType::Binary),
    Some(TType::Struct),
    Some(TType::Map),
    Some(TType::Set),
    Some(TType::List),
    Some(TType::Uuid),
];

impl TryFrom<u8> for TType {
    type Error = ThriftException;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match TTYPE_LOOKUP.get(value as usize) {
            Some(Some(ttype)) => Ok(*ttype),
            _ => Err(new_protocol_exception(
                ProtocolExceptionKind::InvalidData,
                format!("invalid ttype {value}"),
            )),
        }
    }
}

const BINARY_BASIC_TYPE_FIXED_SIZE: [usize; 17] = [
    0,  // TType::Stop
    0,  // TType::Void
    1,  // TType::Bool
    1,  // TType::I8
    8,  // TType::Double
    0,  // NAN
    2,  // TType::I16
    0,  // NAN
    4,  // TType::I32
    0,  // NAN
    8,  // TType::I64
    0,  // TType::Binary
    0,  // TType::Struct
    0,  // TType::Map
    0,  // TType::List
    0,  // TType::Set
    16, // TType::Uuid
];

/// Thrift message types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TMessageType {
    /// Service-call request.
    Call = 1,
    /// Service-call response.
    Reply = 2,
    /// Unexpected error in the remote service.
    Exception = 3,
    /// One-way service-call request (no response is expected).
    OneWay = 4,
}

impl TryFrom<u8> for TMessageType {
    type Error = ThriftException;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TMessageType::Call),
            2 => Ok(TMessageType::Reply),
            3 => Ok(TMessageType::Exception),
            4 => Ok(TMessageType::OneWay),
            _ => Err(new_protocol_exception(
                ProtocolExceptionKind::InvalidData,
                format!("invalid tmessage type {value}"),
            )),
        }
    }
}

impl From<TMessageType> for u8 {
    fn from(t: TMessageType) -> Self {
        t as u8
    }
}

/// Thrift message identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TMessageIdentifier {
    /// Service call the message is associated with.
    pub name: FastStr,
    /// Message type.
    pub message_type: TMessageType,
    /// Ordered sequence number identifying the message.
    pub sequence_number: i32,
}

impl TMessageIdentifier {
    /// Create a `TMessageIdentifier` for a Thrift service-call named `name`
    /// with message type `message_type` and sequence number `sequence_number`.
    pub fn new(
        name: FastStr,
        message_type: TMessageType,
        sequence_number: i32,
    ) -> TMessageIdentifier {
        TMessageIdentifier {
            name,
            message_type,
            sequence_number,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct TListIdentifier {
    /// Type of the elements in the list.
    pub element_type: TType,
    /// Number of elements in the list.
    pub size: usize,
}

impl TListIdentifier {
    /// Create a `TListIdentifier` for a list with `size` elements of type
    /// `element_type`.
    pub fn new(element_type: TType, size: usize) -> TListIdentifier {
        TListIdentifier { element_type, size }
    }
}

/// Thrift set identifier.
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct TSetIdentifier {
    /// Type of the elements in the set.
    pub element_type: TType,
    /// Number of elements in the set.
    pub size: usize,
}

impl TSetIdentifier {
    /// Create a `TSetIdentifier` for a set with `size` elements of type
    /// `element_type`.
    pub fn new(element_type: TType, size: usize) -> TSetIdentifier {
        TSetIdentifier { element_type, size }
    }
}

/// Thrift field identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TFieldIdentifier {
    /// Name of the Thrift field.
    ///
    /// `None` if it's not sent over the wire.
    pub name: Option<&'static str>,
    /// Field type.
    ///
    /// This may be a primitive, container, or a struct.
    pub field_type: TType,
    /// Thrift field id.
    ///
    /// `None` only if `field_type` is `TType::Stop`.
    pub id: Option<i16>,
}

impl TFieldIdentifier {
    /// Create a `TFieldIdentifier` for a field named `name` with type
    /// `field_type` and field id `id`.
    ///
    /// `id` should be `None` if `field_type` is `TType::Stop`.
    pub fn new<N, I>(name: N, field_type: TType, id: I) -> TFieldIdentifier
    where
        N: Into<Option<&'static str>>,
        I: Into<Option<i16>>,
    {
        TFieldIdentifier {
            name: name.into(),
            field_type,
            id: id.into(),
        }
    }
}

/// Thrift map identifier.
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct TMapIdentifier {
    /// Map key type.
    pub key_type: TType,
    /// Map value type.
    pub value_type: TType,
    /// Number of entries in the map.
    pub size: usize,
}

impl TMapIdentifier {
    /// Create a `TMapIdentifier` for a map with `size` entries of type
    /// `key_type -> value_type`.
    pub fn new<K, V>(key_type: K, value_type: V, size: usize) -> TMapIdentifier
    where
        K: Into<TType>,
        V: Into<TType>,
    {
        TMapIdentifier {
            key_type: key_type.into(),
            value_type: value_type.into(),
            size,
        }
    }
}
