extern crate alloc;

use alloc::boxed::Box;
use core::fmt::Debug;
use std::sync::Arc;

use bytes::{Buf, BufMut, Bytes};
use linkedbytes::LinkedBytes;

use super::{
    DecodeError, EncodeError,
    encoding::{DecodeContext, WireType, decode_key, encode_varint, encoded_len_varint, message},
};

/// A Protocol Buffers message.
pub trait Message: Debug + Send + Sync {
    /// Encodes the message to a buffer.
    ///
    /// This method will panic if the buffer has insufficient capacity.
    ///
    /// Meant to be used only by `Message` implementations.
    #[doc(hidden)]
    fn encode_raw(&self, buf: &mut LinkedBytes)
    where
        Self: Sized;

    /// Decodes a field from a buffer, and merges it into `self`.
    ///
    /// Meant to be used only by `Message` implementations.
    #[doc(hidden)]
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        is_root: bool,
    ) -> Result<(), DecodeError>
    where
        Self: Sized;

    /// Returns the encoded length of the message without a length delimiter.
    fn encoded_len(&self) -> usize;

    /// Encodes the message to a buffer.
    ///
    /// An error will be returned if the buffer does not have sufficient
    /// capacity.
    fn encode(&self, buf: &mut LinkedBytes) -> Result<(), EncodeError>
    where
        Self: Sized,
    {
        let required = self.encoded_len();
        let remaining = buf.remaining_mut();
        if required > buf.remaining_mut() {
            return Err(EncodeError::new(required, remaining));
        }

        self.encode_raw(buf);
        Ok(())
    }

    /// Encodes the message with a length-delimiter to a buffer.
    ///
    /// An error will be returned if the buffer does not have sufficient
    /// capacity.
    fn encode_length_delimited(&self, buf: &mut LinkedBytes) -> Result<(), EncodeError>
    where
        Self: Sized,
    {
        let len = self.encoded_len();
        let required = len + encoded_len_varint(len as u64);
        let remaining = buf.remaining_mut();
        if required > remaining {
            return Err(EncodeError::new(required, remaining));
        }
        encode_varint(len as u64, buf);
        self.encode_raw(buf);
        Ok(())
    }

    /// Decodes an instance of the message from a buffer.
    ///
    /// The entire buffer will be consumed.
    fn decode(buf: Bytes) -> Result<Self, DecodeError>
    where
        Self: Default,
    {
        let mut message = Self::default();
        Self::merge(&mut message, buf).map(|_| message)
    }

    /// Decodes a length-delimited instance of the message from the buffer.
    fn decode_length_delimited(buf: Bytes) -> Result<Self, DecodeError>
    where
        Self: Default,
    {
        let mut message = Self::default();
        message.merge_length_delimited(buf)?;
        Ok(message)
    }

    /// Decodes an instance of the message from a buffer, and merges it into
    /// `self`.
    ///
    /// The entire buffer will be consumed.
    fn merge(&mut self, mut buf: Bytes) -> Result<(), DecodeError>
    where
        Self: Sized,
    {
        let mut ctx = DecodeContext::new(buf.clone());
        while buf.has_remaining() {
            ctx.align_with_buf(&buf);
            let (tag, wire_type) = decode_key(&mut buf)?;
            self.merge_field(tag, wire_type, &mut buf, &mut ctx, true)?;
        }
        Ok(())
    }

    /// Decodes a length-delimited instance of the message from buffer, and
    /// merges it into `self`.
    fn merge_length_delimited(&mut self, mut buf: Bytes) -> Result<(), DecodeError>
    where
        Self: Sized,
    {
        let mut ctx = DecodeContext::new(buf.clone());
        message::merge(WireType::LengthDelimited, self, &mut buf, &mut ctx)
    }
}

impl<M> Message for Box<M>
where
    M: Message,
{
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        (**self).encode_raw(buf)
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        is_root: bool,
    ) -> Result<(), DecodeError> {
        (**self).merge_field(tag, wire_type, buf, ctx, is_root)
    }
    fn encoded_len(&self) -> usize {
        (**self).encoded_len()
    }
}

trait ArcMessage<M>
where
    M: Message + Default + Clone,
{
    fn encode(msg: &Arc<M>, buf: &mut LinkedBytes) -> Result<(), EncodeError>;
    fn encode_length_delimited(msg: &Arc<M>, buf: &mut LinkedBytes) -> Result<(), EncodeError>;
    fn encoded_len(msg: &Arc<M>) -> usize;
    fn decode(buf: Bytes) -> Result<Arc<M>, DecodeError>;
    fn decode_length_delimited(buf: Bytes) -> Result<Arc<M>, DecodeError>;
}

impl<M: Message + Default + Clone> ArcMessage<M> for std::sync::Arc<M> {
    fn encode(msg: &Arc<M>, buf: &mut LinkedBytes) -> Result<(), EncodeError> {
        msg.encode(buf)
    }

    fn encode_length_delimited(msg: &Arc<M>, buf: &mut LinkedBytes) -> Result<(), EncodeError> {
        msg.encode_length_delimited(buf)
    }

    fn encoded_len(msg: &Arc<M>) -> usize {
        msg.encoded_len()
    }

    fn decode(buf: Bytes) -> Result<Arc<M>, DecodeError> {
        let message = M::decode(buf)?;
        Ok(Arc::new(message))
    }

    fn decode_length_delimited(buf: Bytes) -> Result<Arc<M>, DecodeError> {
        let message = M::decode_length_delimited(buf)?;
        Ok(Arc::new(message))
    }
}

impl<M> Message for Arc<M>
where
    M: Message + Default + Clone,
{
    fn encode(&self, buf: &mut LinkedBytes) -> Result<(), EncodeError> {
        <Arc<M> as ArcMessage<M>>::encode(self, buf)
    }

    fn encode_length_delimited(&self, buf: &mut LinkedBytes) -> Result<(), EncodeError> {
        <Arc<M> as ArcMessage<M>>::encode_length_delimited(self, buf)
    }

    fn encode_raw(&self, _buf: &mut LinkedBytes) {
        unreachable!("Arc<M> does not implement encode_raw")
    }

    fn encoded_len(&self) -> usize {
        <Arc<M> as ArcMessage<M>>::encoded_len(self)
    }

    fn decode(buf: Bytes) -> Result<Arc<M>, DecodeError>
    where
        M: Default + Clone,
    {
        <Arc<M> as ArcMessage<M>>::decode(buf)
    }

    fn decode_length_delimited(buf: Bytes) -> Result<Arc<M>, DecodeError> {
        <Arc<M> as ArcMessage<M>>::decode_length_delimited(buf)
    }

    fn merge_field(
        &mut self,
        _tag: u32,
        _wire_type: WireType,
        _buf: &mut Bytes,
        _ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        unreachable!("Arc<M> does not implement merge_field")
    }

    fn merge_length_delimited(&mut self, _buf: Bytes) -> Result<(), DecodeError> {
        unreachable!("Arc<M> does not implement merge_length_delimited")
    }

    fn merge(&mut self, _buf: Bytes) -> Result<(), DecodeError> {
        unreachable!("Arc<M> does not implement merge")
    }
}

pub trait EnumMessage: Sized {
    fn inner(&self) -> i32;

    fn to_string(&self) -> ::std::string::String;

    fn try_from_i32(value: i32) -> ::std::option::Option<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    const _MESSAGE_IS_OBJECT_SAFE: Option<&dyn Message> = None;
}
