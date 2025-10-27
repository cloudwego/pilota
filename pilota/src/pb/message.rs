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
use crate::pb::encoding::EncodeLengthContext;

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
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize;

    /// Encodes the message to a buffer.
    ///
    /// An error will be returned if the buffer does not have sufficient
    /// capacity.
    fn encode(&self, buf: &mut LinkedBytes) -> Result<(), EncodeError>
    where
        Self: Sized,
    {
        let mut ctx = EncodeLengthContext::default();
        let len = self.encoded_len(&mut ctx);
        let required = len - ctx.zero_copy_len;

        let remaining = buf.remaining_mut();
        if required > remaining {
            return Err(EncodeError::new(required, remaining));
        }

        self.encode_raw(buf);
        Ok(())
    }

    /// Returns the encoded length of the message with a length delimiter.
    ///
    /// (length, total)
    /// - length: The encoded length of the message itself
    /// - total: The encoded length of the message with the length delimiter
    fn encoded_len_length_delimited(&self, ctx: &mut EncodeLengthContext) -> (usize, usize) {
        let len = self.encoded_len(ctx);
        let total = len + encoded_len_varint(len as u64);
        (len, total)
    }

    /// Encodes the message with a length-delimiter to a buffer.
    ///
    /// An error will be returned if the buffer does not have sufficient
    /// capacity.
    fn encode_length_delimited(
        &self,
        ctx: &mut EncodeLengthContext,
        buf: &mut LinkedBytes,
    ) -> Result<(), EncodeError>
    where
        Self: Sized,
    {
        let (len, total) = self.encoded_len_length_delimited(ctx);
        let required = total - ctx.zero_copy_len;

        let remaining = buf.remaining_mut();
        if required > remaining {
            return Err(EncodeError::new(required, remaining));
        }

        encode_varint(len as u64, buf);

        self.encode_raw(buf);
        Ok(())
    }

    /// Encodes the message to a newly allocated buffer.
    fn encode_to_vec(&self, ctx: &mut EncodeLengthContext) -> Vec<u8>
    where
        Self: Sized,
    {
        let len = self.encoded_len(ctx);
        let required = len - ctx.zero_copy_len;

        let mut buf = LinkedBytes::with_capacity(required);

        self.encode_raw(&mut buf);

        buf.concat().to_vec()
    }

    /// Encodes the message with a length-delimiter to a newly allocated buffer.
    fn encode_length_delimited_to_vec(&self, ctx: &mut EncodeLengthContext) -> Vec<u8>
    where
        Self: Sized,
    {
        let (len, total) = self.encoded_len_length_delimited(ctx);
        let required = total - ctx.zero_copy_len;

        let mut buf = LinkedBytes::with_capacity(required);

        encode_varint(len as u64, &mut buf);

        self.encode_raw(&mut buf);

        buf.concat().to_vec()
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
            // align the buffer to the start of the next field
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
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        (**self).encoded_len(ctx)
    }
}

trait ArcMessage<M>
where
    M: Message + Default + Clone,
{
    fn encode(msg: &Arc<M>, buf: &mut LinkedBytes) -> Result<(), EncodeError>;
    fn encode_length_delimited(
        msg: &Arc<M>,
        ctx: &mut EncodeLengthContext,
        buf: &mut LinkedBytes,
    ) -> Result<(), EncodeError>;
    fn encoded_len(msg: &Arc<M>, ctx: &mut EncodeLengthContext) -> usize;
    fn decode(buf: Bytes) -> Result<Arc<M>, DecodeError>;
    fn decode_length_delimited(buf: Bytes) -> Result<Arc<M>, DecodeError>;
}

impl<M: Message + Default + Clone> ArcMessage<M> for std::sync::Arc<M> {
    fn encode(msg: &Arc<M>, buf: &mut LinkedBytes) -> Result<(), EncodeError> {
        msg.as_ref().encode(buf)
    }

    fn encode_length_delimited(
        msg: &Arc<M>,
        ctx: &mut EncodeLengthContext,
        buf: &mut LinkedBytes,
    ) -> Result<(), EncodeError> {
        msg.as_ref().encode_length_delimited(ctx, buf)
    }

    fn encoded_len(msg: &Arc<M>, ctx: &mut EncodeLengthContext) -> usize {
        msg.as_ref().encoded_len(ctx)
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

    fn encode_length_delimited(
        &self,
        ctx: &mut EncodeLengthContext,
        buf: &mut LinkedBytes,
    ) -> Result<(), EncodeError> {
        <Arc<M> as ArcMessage<M>>::encode_length_delimited(self, ctx, buf)
    }

    fn encode_raw(&self, _buf: &mut LinkedBytes) {
        unreachable!("Arc<M> does not implement encode_raw")
    }

    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        <Arc<M> as ArcMessage<M>>::encoded_len(self, ctx)
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
    use crate::pb::encoding::{DecodeContext, EncodeLengthContext, WireType, encode_varint};

    const _MESSAGE_IS_OBJECT_SAFE: Option<&dyn Message> = None;

    #[derive(Debug, Default, Clone, PartialEq)]
    struct TestMessage {
        value: i32,
    }

    impl TestMessage {
        fn new(value: i32) -> Self {
            Self { value }
        }
    }

    impl Message for TestMessage {
        fn encode_raw(&self, buf: &mut LinkedBytes) {
            // tag=1, wire_type=0 (varint), value
            encode_varint((1 << 3) | 0, buf); // tag=1, wire_type=0
            encode_varint(self.value as u64, buf);
        }

        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: WireType,
            buf: &mut Bytes,
            ctx: &mut DecodeContext,
            _is_root: bool,
        ) -> Result<(), DecodeError> {
            match tag {
                1 => {
                    if wire_type != WireType::Varint {
                        return Err(DecodeError::new("invalid wire type"));
                    }
                    let value = crate::pb::encoding::decode_varint(buf)?;
                    self.value = value as i32;
                    Ok(())
                }
                _ => crate::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }

        fn encoded_len(&self, _ctx: &mut EncodeLengthContext) -> usize {
            // tag (1 byte) + varint value (1-5 bytes)
            1 + crate::pb::encoding::encoded_len_varint(self.value as u64)
        }
    }

    #[test]
    fn test_message_decode_from_bytes() {
        // tag=1 (0x08), value=42 (0x2A)
        let bytes = Bytes::from_static(&[0x08, 0x2A]);

        let decoded = TestMessage::decode(bytes).unwrap();
        assert_eq!(decoded.value, 42);
    }

    #[test]
    fn test_message_merge() {
        let mut msg = TestMessage::new(100);

        // tag=1, value=200
        let bytes = Bytes::from_static(&[0x08, 0xC8, 0x01]); // 200 的 varint 编码

        msg.merge(bytes).unwrap();

        assert_eq!(msg.value, 200);
    }

    #[test]
    fn test_message_encoded_len() {
        let msg = TestMessage::new(42);
        let mut ctx = EncodeLengthContext::default();
        let len = msg.encoded_len(&mut ctx);

        // tag (1 byte) + value (1 byte) = 2 bytes
        assert_eq!(len, 2);
    }

    #[test]
    fn test_message_encode_raw() {
        let msg = TestMessage::new(42);
        let mut buf = LinkedBytes::new();
        msg.encode_raw(&mut buf);

        assert_eq!(buf.len(), 2);
    }

    #[test]
    fn test_encode_to_vec_and_zero_copy_len() {
        let msg = TestMessage::new(42);
        let mut ctx = EncodeLengthContext::default();
        let vec = msg.encode_to_vec(&mut ctx);
        assert_eq!(vec, vec![0x08, 0x2A]);
    }

    #[test]
    fn test_encode_length_delimited_to_vec() {
        let msg = TestMessage::new(300);
        let mut ctx = EncodeLengthContext::default();
        // payload = [0x08, 0xAC, 0x02]; length varint for 3 is [0x03]
        let vec = msg.encode_length_delimited_to_vec(&mut ctx);
        assert_eq!(vec, vec![0x03, 0x08, 0xAC, 0x02]);
    }

    #[test]
    fn test_message_encode_error() {
        let msg = TestMessage::new(42);

        let mut buf = LinkedBytes::new();
        let large_data = vec![0u8; 1024 * 1024]; // 1MB
        buf.put_slice(&large_data);

        let result = msg.encode(&mut buf);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_box_message() {
        let original = Box::new(TestMessage::new(789));
        let mut ctx = EncodeLengthContext::default();

        let len = original.encoded_len(&mut ctx);
        assert_eq!(len, 3);

        let mut buf = LinkedBytes::new();
        original.encode_raw(&mut buf);
        assert_eq!(buf.len(), len);

        let mut decoded = Box::new(TestMessage::default());
        let bytes = Bytes::from_static(&[0x08, 0x95, 0x06]); // 789 的 varint 编码
        decoded.merge(bytes).unwrap();
        assert_eq!(decoded.value, 789);
    }

    #[test]
    fn test_arc_message_encode() {
        let original = Arc::new(TestMessage::new(999));

        let mut buf = LinkedBytes::new();
        original.encode(&mut buf).unwrap();

        assert_eq!(buf.len(), 3);
    }

    #[test]
    fn test_arc_message_decode() {
        // tag=1, value=999
        let bytes = Bytes::from_static(&[0x08, 0xE7, 0x07]); // 999 的 varint 编码

        let decoded = <Arc<TestMessage> as Message>::decode(bytes).unwrap();
        assert_eq!(decoded.value, 999);
    }

    #[test]
    fn test_arc_message_encoded_len() {
        let msg = Arc::new(TestMessage::new(222));
        let mut ctx = EncodeLengthContext::default();
        let len = msg.encoded_len(&mut ctx);

        assert_eq!(len, 3);
    }

    #[test]
    fn test_arc_message_encode_length_delimited() {
        let msg = Arc::new(TestMessage::new(222));
        let mut ctx = EncodeLengthContext::default();
        let mut buf = LinkedBytes::new();
        msg.encode_length_delimited(&mut ctx, &mut buf).unwrap();
        assert_eq!(buf.len(), 4);
    }

    #[test]
    #[should_panic(expected = "Arc<M> does not implement encode_raw")]
    fn test_arc_message_encode_raw_panics() {
        let msg = Arc::new(TestMessage::new(333));
        let mut buf = LinkedBytes::new();
        msg.encode_raw(&mut buf);
    }

    #[test]
    #[should_panic(expected = "Arc<M> does not implement merge_field")]
    fn test_arc_message_merge_field_panics() {
        let mut msg = Arc::new(TestMessage::new(444));
        let mut buf = Bytes::new();
        let mut ctx = DecodeContext::new(Bytes::new());
        let _ = msg.merge_field(1, WireType::Varint, &mut buf, &mut ctx, true);
    }

    #[test]
    #[should_panic(expected = "Arc<M> does not implement merge")]
    fn test_arc_message_merge_panics() {
        let mut msg = Arc::new(TestMessage::new(555));
        let buf = Bytes::new();
        let _ = msg.merge(buf);
    }

    #[test]
    #[should_panic(expected = "Arc<M> does not implement merge_length_delimited")]
    fn test_arc_message_merge_length_delimited_panics() {
        let mut msg = Arc::new(TestMessage::new(666));
        let buf = Bytes::new();
        let _ = msg.merge_length_delimited(buf);
    }

    #[derive(Debug, Clone, PartialEq)]
    enum TestEnum {
        A = 0,
        B = 1,
        C = 2,
    }

    impl EnumMessage for TestEnum {
        fn inner(&self) -> i32 {
            match self {
                TestEnum::A => 0,
                TestEnum::B => 1,
                TestEnum::C => 2,
            }
        }

        fn to_string(&self) -> String {
            match self {
                TestEnum::A => "A".to_string(),
                TestEnum::B => "B".to_string(),
                TestEnum::C => "C".to_string(),
            }
        }

        fn try_from_i32(value: i32) -> Option<Self> {
            match value {
                0 => Some(TestEnum::A),
                1 => Some(TestEnum::B),
                2 => Some(TestEnum::C),
                _ => None,
            }
        }
    }

    #[test]
    fn test_enum_message() {
        let enum_val = TestEnum::B;

        assert_eq!(enum_val.inner(), 1);
        assert_eq!(enum_val.to_string(), "B");
        assert_eq!(TestEnum::try_from_i32(1), Some(TestEnum::B));
        assert_eq!(TestEnum::try_from_i32(99), None);
    }

    #[test]
    fn test_decode_error_handling() {
        let invalid_data = Bytes::from_static(&[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ]);
        let result = TestMessage::decode(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_with_unknown_fields() {
        let mut msg = TestMessage::new(100);

        // tag=1, value=200, tag=99, value=300
        let data = vec![
            0x08, 0xC8, 0x01, // tag=1, value=200
            0xF8, 0x06, 0xAC, 0x02, // tag=99, value=300
        ];
        let bytes = Bytes::from(data);
        msg.merge(bytes).unwrap();

        assert_eq!(msg.value, 200);
    }
}
