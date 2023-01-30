pub mod binary;
pub mod compact;
pub mod error;
pub mod rw_ext;
pub mod varint_ext;

use std::{ops::Deref, sync::Arc};

use bytes::{Buf, BufMut, Bytes};
pub use error::*;
use faststr::FastStr;
use tokio::io::AsyncRead;

pub use self::{binary::TAsyncBinaryProtocol, compact::TAsyncCompactProtocol};

const MAXIMUM_SKIP_DEPTH: i8 = 64;
const ZERO_COPY_THRESHOLD: usize = 4 * 1024; // 4KB
const INLINE_CAP: usize = 22;

lazy_static::lazy_static! {
    pub static ref VOID_IDENT: TStructIdentifier = TStructIdentifier { name: "void" };
}

#[async_trait::async_trait]
pub trait Message: Sized + Send {
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<(), Error>;

    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self, Error>;

    async fn decode_async<R>(protocol: &mut TAsyncBinaryProtocol<R>) -> Result<Self, Error>
    where
        R: AsyncRead + Unpin + Send;

    fn size<T: TLengthProtocol>(&self, protocol: &mut T) -> usize;
}

#[async_trait::async_trait]
impl<M: Message> Message for Box<M> {
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<(), Error> {
        self.deref().encode(protocol)
    }

    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self, Error> {
        Ok(Box::new(M::decode(protocol)?))
    }

    async fn decode_async<R>(protocol: &mut TAsyncBinaryProtocol<R>) -> Result<Self, Error>
    where
        R: AsyncRead + Unpin + Send,
    {
        Ok(Box::new(M::decode_async(protocol).await?))
    }

    fn size<T: TLengthProtocol>(&self, protocol: &mut T) -> usize {
        self.deref().size(protocol)
    }
}

#[async_trait::async_trait]
impl<M: Message + Send + Sync> Message for Arc<M> {
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<(), Error> {
        self.deref().encode(protocol)
    }

    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self, Error> {
        Ok(Arc::new(M::decode(protocol)?))
    }

    async fn decode_async<R>(protocol: &mut TAsyncBinaryProtocol<R>) -> Result<Self, Error>
    where
        R: AsyncRead + Unpin + Send,
    {
        Ok(Arc::new(M::decode_async(protocol).await?))
    }

    fn size<T: TLengthProtocol>(&self, protocol: &mut T) -> usize {
        self.deref().size(protocol)
    }
}

pub trait TInputProtocol {
    type Buf: Buf;
    /// Read the beginning of a Thrift message.
    fn read_message_begin(&mut self) -> Result<TMessageIdentifier, Error>;
    /// Read the end of a Thrift message.
    fn read_message_end(&mut self) -> Result<(), Error>;
    /// Read the beginning of a Thrift struct.
    fn read_struct_begin(&mut self) -> Result<Option<TStructIdentifier>, Error>;
    /// Read the end of a Thrift struct.
    fn read_struct_end(&mut self) -> Result<(), Error>;
    /// Read the beginning of a Thrift struct field.
    fn read_field_begin(&mut self) -> Result<TFieldIdentifier, Error>;
    /// Read the end of a Thrift struct field.
    fn read_field_end(&mut self) -> Result<(), Error>;
    /// Read a bool.
    fn read_bool(&mut self) -> Result<bool, Error>;
    /// Read a binary.
    fn read_bytes(&mut self) -> Result<Bytes, Error>;
    /// Read a uuid.
    fn read_uuid(&mut self) -> Result<[u8; 16], Error>;
    /// Read a word.
    fn read_i8(&mut self) -> Result<i8, Error>;
    /// Read a 16-bit signed integer.
    fn read_i16(&mut self) -> Result<i16, Error>;
    /// Read a 32-bit signed integer.
    fn read_i32(&mut self) -> Result<i32, Error>;
    /// Read a 64-bit signed integer.
    fn read_i64(&mut self) -> Result<i64, Error>;
    /// Read a 64-bit float.
    fn read_double(&mut self) -> Result<f64, Error>;
    /// Read a fixed-length string (not null terminated).
    fn read_string(&mut self) -> Result<String, Error>;
    /// Read a faststr.
    fn read_faststr(&mut self) -> Result<FastStr, Error>;
    /// Read the beginning of a list.
    fn read_list_begin(&mut self) -> Result<TListIdentifier, Error>;
    /// Read the end of a list.
    fn read_list_end(&mut self) -> Result<(), Error>;
    /// Read the beginning of a set.
    fn read_set_begin(&mut self) -> Result<TSetIdentifier, Error>;
    /// Read the end of a set.
    fn read_set_end(&mut self) -> Result<(), Error>;
    /// Read the beginning of a map.
    fn read_map_begin(&mut self) -> Result<TMapIdentifier, Error>;
    /// Read the end of a map.
    fn read_map_end(&mut self) -> Result<(), Error>;
    /// Skip a field with type `field_type` recursively until the default
    /// maximum skip depth is reached.
    fn skip(&mut self, field_type: TType) -> Result<(), Error> {
        self.skip_till_depth(field_type, MAXIMUM_SKIP_DEPTH)
    }
    /// Skip a field with type `field_type` recursively up to `depth` levels.
    fn skip_till_depth(&mut self, field_type: TType, depth: i8) -> Result<(), Error> {
        if depth == 0 {
            return Err(new_protocol_error(
                ProtocolErrorKind::DepthLimit,
                format!("cannot parse past {:?}", field_type),
            ));
        }

        match field_type {
            TType::Bool => self.read_bool().map(|_| ()),
            TType::I8 => self.read_i8().map(|_| ()),
            TType::I16 => self.read_i16().map(|_| ()),
            TType::I32 => self.read_i32().map(|_| ()),
            TType::I64 => self.read_i64().map(|_| ()),
            TType::Double => self.read_double().map(|_| ()),
            TType::Binary => self.read_bytes().map(|_| ()),
            TType::Uuid => self.read_uuid().map(|_| ()),
            TType::Struct => {
                self.read_struct_begin()?;
                loop {
                    let field_ident = self.read_field_begin()?;
                    if field_ident.field_type == TType::Stop {
                        break;
                    }
                    self.skip_till_depth(field_ident.field_type, depth - 1)?;
                }
                self.read_struct_end()
            }
            TType::List => {
                let list_ident = self.read_list_begin()?;
                for _ in 0..list_ident.size {
                    self.skip_till_depth(list_ident.element_type, depth - 1)?;
                }
                self.read_list_end()
            }
            TType::Set => {
                let set_ident = self.read_set_begin()?;
                for _ in 0..set_ident.size {
                    self.skip_till_depth(set_ident.element_type, depth - 1)?;
                }
                self.read_set_end()
            }
            TType::Map => {
                let map_ident = self.read_map_begin()?;
                for _ in 0..map_ident.size {
                    let key_type = map_ident.key_type;
                    let val_type = map_ident.value_type;
                    self.skip_till_depth(key_type, depth - 1)?;
                    self.skip_till_depth(val_type, depth - 1)?;
                }
                self.read_map_end()
            }
            u => Err(new_protocol_error(
                ProtocolErrorKind::DepthLimit,
                format!("cannot skip field type {:?}", &u),
            )),
        }
    }

    // utility (DO NOT USE IN GENERATED CODE!!!!)
    //

    /// Read an unsigned byte.
    ///
    /// This method should **never** be used in generated code.
    fn read_byte(&mut self) -> Result<u8, Error>;

    /// Read a Vec<u8>.
    fn read_bytes_vec(&mut self) -> Result<Vec<u8>, Error>;

    fn buf_mut(&mut self) -> &mut Self::Buf;
}

pub trait TLengthProtocol {
    // size

    fn write_message_begin_len(&mut self, identifier: &TMessageIdentifier) -> usize;

    fn write_message_end_len(&mut self) -> usize;

    fn write_struct_begin_len(&mut self, identifier: &TStructIdentifier) -> usize;

    fn write_struct_end_len(&mut self) -> usize;

    fn write_field_begin_len(&mut self, identifier: &TFieldIdentifier) -> usize;

    fn write_field_end_len(&mut self) -> usize;

    fn write_field_stop_len(&mut self) -> usize;

    fn write_bool_len(&mut self, b: bool) -> usize;

    fn write_bytes_len(&mut self, b: &[u8]) -> usize;

    fn write_bytes_vec_len(&mut self, b: &[u8]) -> usize;

    fn write_byte_len(&mut self, b: u8) -> usize;

    fn write_uuid_len(&mut self, u: [u8; 16]) -> usize;

    fn write_i8_len(&mut self, i: i8) -> usize;

    fn write_i16_len(&mut self, i: i16) -> usize;

    fn write_i32_len(&mut self, i: i32) -> usize;

    fn write_i64_len(&mut self, i: i64) -> usize;

    fn write_double_len(&mut self, d: f64) -> usize;

    fn write_string_len(&mut self, s: &str) -> usize;

    fn write_faststr_len(&mut self, s: &FastStr) -> usize;

    fn write_list_begin_len(&mut self, identifier: &TListIdentifier) -> usize;

    fn write_list_end_len(&mut self) -> usize;

    fn write_set_begin_len(&mut self, identifier: &TSetIdentifier) -> usize;

    fn write_set_end_len(&mut self) -> usize;

    fn write_map_begin_len(&mut self, identifier: &TMapIdentifier) -> usize;

    fn write_map_end_len(&mut self) -> usize;

    /// The zero copy length used to calculate the recommended malloc length.
    fn zero_copy_len(&mut self) -> usize;

    /// Resets the zero copy length counter.
    fn reset(&mut self);
}

pub trait TOutputProtocol {
    type BufMut: BufMut;

    /// Write the beginning of a Thrift message.
    fn write_message_begin(&mut self, identifier: &TMessageIdentifier) -> Result<(), Error>;
    /// Write the end of a Thrift message.
    fn write_message_end(&mut self) -> Result<(), Error>;
    /// Write the beginning of a Thrift struct.
    fn write_struct_begin(&mut self, identifier: &TStructIdentifier) -> Result<(), Error>;
    /// Write the end of a Thrift struct.
    fn write_struct_end(&mut self) -> Result<(), Error>;
    /// Write the beginning of a Thrift field.
    fn write_field_begin(&mut self, field_type: TType, id: i16) -> Result<(), Error>;
    /// Write the end of a Thrift field.
    fn write_field_end(&mut self) -> Result<(), Error>;
    /// Write a STOP field indicating that all the fields in a struct have been
    /// written.
    fn write_field_stop(&mut self) -> Result<(), Error>;
    /// Write a bool.
    fn write_bool(&mut self, b: bool) -> Result<(), Error>;
    /// Write a fixed-length byte array.
    fn write_bytes(&mut self, b: Bytes) -> Result<(), Error>;
    /// Write a uuid.
    fn write_uuid(&mut self, u: [u8; 16]) -> Result<(), Error>;
    /// Write a Vec<u8>.
    fn write_bytes_vec(&mut self, b: &[u8]) -> Result<(), Error>;
    /// Write a byte.
    fn write_byte(&mut self, b: u8) -> Result<(), Error>;
    /// Write an 8-bit signed integer.
    fn write_i8(&mut self, i: i8) -> Result<(), Error>;
    /// Write a 16-bit signed integer.
    fn write_i16(&mut self, i: i16) -> Result<(), Error>;
    /// Write a 32-bit signed integer.
    fn write_i32(&mut self, i: i32) -> Result<(), Error>;
    /// Write a 64-bit signed integer.
    fn write_i64(&mut self, i: i64) -> Result<(), Error>;
    /// Write a 64-bit float.
    fn write_double(&mut self, d: f64) -> Result<(), Error>;
    /// Write a fixed-length string.
    fn write_string(&mut self, s: &str) -> Result<(), Error>;
    /// Write a fixed-length faststr.
    fn write_faststr(&mut self, s: FastStr) -> Result<(), Error>;
    /// Write the beginning of a list.
    fn write_list_begin(&mut self, identifier: &TListIdentifier) -> Result<(), Error>;
    /// Write the end of a list.
    fn write_list_end(&mut self) -> Result<(), Error>;
    /// Write the beginning of a set.
    fn write_set_begin(&mut self, identifier: &TSetIdentifier) -> Result<(), Error>;
    /// Write the end of a set.
    fn write_set_end(&mut self) -> Result<(), Error>;
    /// Write the beginning of a map.
    fn write_map_begin(&mut self, identifier: &TMapIdentifier) -> Result<(), Error>;
    /// Write the end of a map.
    fn write_map_end(&mut self) -> Result<(), Error>;
    /// Flush buffered bytes to the underlying transport.
    fn flush(&mut self) -> Result<(), Error>;

    fn reserve(&mut self, size: usize);

    fn buf_mut(&mut self) -> &mut Self::BufMut;
}


#[async_trait::async_trait]
pub trait TAsyncInputProtocol {
    
    /// Read the beginning of a Thrift message.
    async fn read_message_begin(&mut self) -> Result<TMessageIdentifier, Error>;

    /// Read the end of a Thrift message.
    async fn read_message_end(&mut self) -> Result<(), Error>;

    /// Read the beginning of a Thrift struct.   
    async fn read_struct_begin(&mut self) -> Result<Option<TStructIdentifier>, Error>;

    /// Read the end of a Thrift struct.
    async fn read_struct_end(&mut self) -> Result<(), Error>;

    /// Read the beginning of a Thrift struct field.
    async fn read_field_begin(&mut self) -> Result<TFieldIdentifier, Error>;

    /// Read the end of a Thrift struct field.
    async fn read_field_end(&mut self) -> Result<(), Error>;

    /// Read a bool.
    async fn read_bool(&mut self) -> Result<bool, Error>;

    /// Read a binary.
    async fn read_bytes(&mut self) -> Result<Bytes, Error>;

    /// Read a binary, return `Vec<u8>`
    async fn read_bytes_vec(&mut self) -> Result<Vec<u8>, Error>;

    /// Read a uuid.
    async fn read_uuid(&mut self) -> Result<[u8; 16], Error>;

    /// Read a string.
    async fn read_string(&mut self) -> Result<String, Error>;

    /// Read a string, return `FastStr`
    async fn read_faststr(&mut self) -> Result<FastStr, Error>;
    
    /// Read a byte.
    async fn read_byte(&mut self) -> Result<u8, Error>;

    /// Read a word.
    async fn read_i8(&mut self) -> Result<i8, Error>;

    /// Read a 16-bit signed integer.
    async fn read_i16(&mut self) -> Result<i16, Error>;

    /// Read a 32-bit signed integer.
    async fn read_i32(&mut self) -> Result<i32, Error>;

    /// Read a 64-bit signed integer.
    async fn read_i64(&mut self) -> Result<i64, Error>;

    /// Read a 64-bit float.
    async fn read_double(&mut self) -> Result<f64, Error>;

    /// Read the beginning of a list.
    async fn read_list_begin(&mut self) -> Result<TListIdentifier, Error>;
    
    /// Read the end of a list.
    async fn read_list_end(&mut self) -> Result<(), Error>;

    /// Read the beginning of a set.
    async fn read_set_begin(&mut self) -> Result<TSetIdentifier, Error>;

    /// Read the end of a set.
    async fn read_set_end(&mut self) -> Result<(), Error>;

    /// Read the beginning of a map.
    async fn read_map_begin(&mut self) -> Result<TMapIdentifier, Error>;
    
    /// Read the end of a map.
    async fn read_map_end(&mut self) -> Result<(), Error>;

    /// Skip a field with type `field_type` recursively until the default
    /// maximum skip depth is reached.
    #[inline]
    async fn skip(&mut self, field_type: TType) -> Result<(), Error> {
        self.skip_till_depth(field_type, MAXIMUM_SKIP_DEPTH).await
    }

    // conflict with async_trait macro on trait: #[async_recursion::async_recursion]
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
            u => Err(new_protocol_error(
                ProtocolErrorKind::DepthLimit,
                format!("cannot skip field type {:?}", &u),
            )),
        }
    }

}

// pub trait TAsyncOutputProtocol {
// }

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

impl TryFrom<u8> for TType {
    type Error = Error;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TType::Stop),
            1 => Ok(TType::Void),
            2 => Ok(TType::Bool),
            3 => Ok(TType::I8),
            4 => Ok(TType::Double),
            6 => Ok(TType::I16),
            8 => Ok(TType::I32),
            10 => Ok(TType::I64),
            11 => Ok(TType::Binary),
            12 => Ok(TType::Struct),
            13 => Ok(TType::Map),
            14 => Ok(TType::Set),
            15 => Ok(TType::List),
            16 => Ok(TType::Uuid),
            _ => Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid ttype {}", value),
            )),
        }
    }
}

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
    type Error = Error;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TMessageType::Call),
            2 => Ok(TMessageType::Reply),
            3 => Ok(TMessageType::Exception),
            4 => Ok(TMessageType::OneWay),
            _ => Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                format!("invalid tmessage type {}", value),
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
