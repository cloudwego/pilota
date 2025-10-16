//! Protocol Buffers well-known wrapper types.
//!
//! This module provides implementations of `Message` for Rust standard library
//! types which correspond to a Protobuf well-known wrapper type. The remaining
//! well-known types are defined in the `prost-types` crate in order to avoid a
//! cyclic dependency between `prost` and `prost-build`.
extern crate alloc;

use alloc::{string::String, vec::Vec};

use ::bytes::Bytes;
use linkedbytes::LinkedBytes;

use super::{
    DecodeError, Message,
    encoding::{
        DecodeContext, EncodeLengthContext, WireType, bool, bytes, double, float, int32, int64,
        skip_field, string, uint32, uint64,
    },
};
use crate::pb::ZERO_COPY_THRESHOLD;

/// `google.protobuf.BoolValue`
impl Message for bool {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self {
            bool::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            bool::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, _ctx: &mut EncodeLengthContext) -> usize {
        if *self { 2 } else { 0 }
    }
}

/// `google.protobuf.UInt32Value`
impl Message for u32 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0 {
            uint32::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            uint32::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        if *self != 0 {
            uint32::encoded_len(ctx, 1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.UInt64Value`
impl Message for u64 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0 {
            uint64::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            uint64::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        if *self != 0 {
            uint64::encoded_len(ctx, 1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.Int32Value`
impl Message for i32 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0 {
            int32::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            int32::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        if *self != 0 {
            int32::encoded_len(ctx, 1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.Int64Value`
impl Message for i64 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0 {
            int64::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            int64::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        if *self != 0 {
            int64::encoded_len(ctx, 1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.FloatValue`
impl Message for f32 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0.0 {
            float::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            float::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        if *self != 0.0 {
            float::encoded_len(ctx, 1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.DoubleValue`
impl Message for f64 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0.0 {
            double::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            double::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        if *self != 0.0 {
            double::encoded_len(ctx, 1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.StringValue`
impl Message for String {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if !self.is_empty() {
            string::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            string::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        if !self.is_empty() {
            string::encoded_len(ctx, 1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.BytesValue`
impl Message for Vec<u8> {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if !self.is_empty() {
            bytes::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            bytes::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        if !self.is_empty() {
            bytes::encoded_len(ctx, 1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.BytesValue`
impl Message for Bytes {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if !self.is_empty() {
            bytes::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            bytes::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self, ctx: &mut EncodeLengthContext) -> usize {
        if !self.is_empty() {
            if self.len() >= ZERO_COPY_THRESHOLD {
                ctx.zero_copy_len += self.len();
            }
            bytes::encoded_len(ctx, 1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.Empty`
impl Message for () {
    fn encode_raw(&self, _buf: &mut LinkedBytes) {}
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
        _is_root: bool,
    ) -> Result<(), DecodeError> {
        skip_field(wire_type, tag, buf, ctx)
    }
    fn encoded_len(&self, _ctx: &mut EncodeLengthContext) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use ::bytes::Bytes;
    use linkedbytes::LinkedBytes;

    use super::*;
    use crate::pb::encoding::{DecodeContext, EncodeLengthContext, WireType};

    // Helper function to create a test buffer
    fn create_test_buffer() -> LinkedBytes {
        LinkedBytes::new()
    }

    // Helper function to create test bytes
    fn create_test_bytes(data: &[u8]) -> Bytes {
        Bytes::copy_from_slice(data)
    }

    #[test]
    fn test_bool_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test true value
        let true_val = true;
        true_val.encode_raw(&mut buf);
        assert_eq!(true_val.encoded_len(&mut ctx), 2);

        // Test false value (should not encode)
        let false_val = false;
        let mut buf2 = create_test_buffer();
        false_val.encode_raw(&mut buf2);
        assert_eq!(false_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_bool_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        // Test decoding true
        let mut bool_val = false;
        let mut buf = create_test_bytes(&[0x01]); // value=1 (without tag, just the value)
        bool_val
            .merge_field(1, WireType::Varint, &mut buf, &mut ctx, true)
            .unwrap();
        assert!(bool_val);

        // Test decoding false
        let mut bool_val = true;
        let mut buf = create_test_bytes(&[0x00]); // value=0 (without tag, just the value)
        bool_val
            .merge_field(1, WireType::Varint, &mut buf, &mut ctx, true)
            .unwrap();
        assert!(!bool_val);
    }

    #[test]
    fn test_u32_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test non-zero value
        let val = 42u32;
        val.encode_raw(&mut buf);
        assert!(val.encoded_len(&mut ctx) > 0);

        // Test zero value (should not encode)
        let zero_val = 0u32;
        let mut buf2 = create_test_buffer();
        zero_val.encode_raw(&mut buf2);
        assert_eq!(zero_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_u32_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = 0u32;
        let mut buf = create_test_bytes(&[0x2A]); // value=42 (without tag, just the value)
        val.merge_field(1, WireType::Varint, &mut buf, &mut ctx, true)
            .unwrap();
        assert_eq!(val, 42);
    }

    #[test]
    fn test_u64_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test non-zero value
        let val = 123456789u64;
        val.encode_raw(&mut buf);
        assert!(val.encoded_len(&mut ctx) > 0);

        // Test zero value (should not encode)
        let zero_val = 0u64;
        let mut buf2 = create_test_buffer();
        zero_val.encode_raw(&mut buf2);
        assert_eq!(zero_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_u64_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = 0u64;
        let mut buf = create_test_bytes(&[0x95, 0x9A, 0xEF, 0x3A]); // value=123456789 (without tag, just the value)
        val.merge_field(1, WireType::Varint, &mut buf, &mut ctx, true)
            .unwrap();
        assert_eq!(val, 123456789);
    }

    #[test]
    fn test_i32_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test non-zero value
        let val = -42i32;
        val.encode_raw(&mut buf);
        assert!(val.encoded_len(&mut ctx) > 0);

        // Test zero value (should not encode)
        let zero_val = 0i32;
        let mut buf2 = create_test_buffer();
        zero_val.encode_raw(&mut buf2);
        assert_eq!(zero_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_i32_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = 0i32;
        let mut buf = create_test_bytes(&[0xd6, 0xff, 0xff, 0xff, 0x0f]); // value=-42 (varint encoded, without tag, just the value)
        val.merge_field(1, WireType::Varint, &mut buf, &mut ctx, true)
            .unwrap();
        assert_eq!(val, -42);
    }

    #[test]
    fn test_i64_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test non-zero value
        let val = -123456789i64;
        val.encode_raw(&mut buf);
        assert!(val.encoded_len(&mut ctx) > 0);

        // Test zero value (should not encode)
        let zero_val = 0i64;
        let mut buf2 = create_test_buffer();
        zero_val.encode_raw(&mut buf2);
        assert_eq!(zero_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_i64_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = 0i64;
        let mut buf =
            create_test_bytes(&[0xeb, 0xe5, 0x90, 0xc5, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]); // value=-123456789 (varint encoded, without tag, just the value)
        val.merge_field(1, WireType::Varint, &mut buf, &mut ctx, true)
            .unwrap();
        assert_eq!(val, -123456789);
    }

    #[test]
    fn test_f32_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test non-zero value
        let val = 3.14f32;
        val.encode_raw(&mut buf);
        assert!(val.encoded_len(&mut ctx) > 0);

        // Test zero value (should not encode)
        let zero_val = 0.0f32;
        let mut buf2 = create_test_buffer();
        zero_val.encode_raw(&mut buf2);
        assert_eq!(zero_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_f32_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = 0.0f32;
        let mut buf = create_test_bytes(&[0xC3, 0xF5, 0x48, 0x40]); // value=3.14 (IEEE 754, without tag, just the value)
        val.merge_field(1, WireType::ThirtyTwoBit, &mut buf, &mut ctx, true)
            .unwrap();
        assert!((val - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_f64_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test non-zero value
        let val = 3.14159265359f64;
        val.encode_raw(&mut buf);
        assert!(val.encoded_len(&mut ctx) > 0);

        // Test zero value (should not encode)
        let zero_val = 0.0f64;
        let mut buf2 = create_test_buffer();
        zero_val.encode_raw(&mut buf2);
        assert_eq!(zero_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_f64_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = 0.0f64;
        let mut buf = create_test_bytes(&[0xea, 0x2e, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40]); // value=3.14159265359 (IEEE 754, without tag, just the value)
        val.merge_field(1, WireType::SixtyFourBit, &mut buf, &mut ctx, true)
            .unwrap();
        assert!((val - 3.14159265359).abs() < 0.00000000001);
    }

    #[test]
    fn test_string_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test non-empty string
        let val = String::from("hello");
        val.encode_raw(&mut buf);
        assert!(val.encoded_len(&mut ctx) > 0);

        // Test empty string (should not encode)
        let empty_val = String::new();
        let mut buf2 = create_test_buffer();
        empty_val.encode_raw(&mut buf2);
        assert_eq!(empty_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_string_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = String::new();
        let mut buf = create_test_bytes(&[0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F]); // length=5, "hello" (without tag, just the length and data)
        val.merge_field(1, WireType::LengthDelimited, &mut buf, &mut ctx, true)
            .unwrap();
        assert_eq!(val, "hello");
    }

    #[test]
    fn test_vec_u8_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test non-empty vector
        let val = vec![1, 2, 3, 4, 5];
        val.encode_raw(&mut buf);
        assert!(val.encoded_len(&mut ctx) > 0);

        // Test empty vector (should not encode)
        let empty_val = vec![];
        let mut buf2 = create_test_buffer();
        empty_val.encode_raw(&mut buf2);
        assert_eq!(empty_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_vec_u8_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = vec![];
        let mut buf = create_test_bytes(&[0x05, 0x01, 0x02, 0x03, 0x04, 0x05]); // length=5, data (without tag, just the length and data)
        val.merge_field(1, WireType::LengthDelimited, &mut buf, &mut ctx, true)
            .unwrap();
        assert_eq!(val, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bytes_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        // Test non-empty bytes
        let val = Bytes::from(vec![1, 2, 3, 4, 5]);
        val.encode_raw(&mut buf);
        assert!(val.encoded_len(&mut ctx) > 0);

        // Test empty bytes (should not encode)
        let empty_val = Bytes::new();
        let mut buf2 = create_test_buffer();
        empty_val.encode_raw(&mut buf2);
        assert_eq!(empty_val.encoded_len(&mut ctx), 0);
        assert_eq!(buf2.len(), 0);
    }

    #[test]
    fn test_bytes_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = Bytes::new();
        let mut buf = create_test_bytes(&[0x05, 0x01, 0x02, 0x03, 0x04, 0x05]); // length=5, data (without tag, just the length and data)
        val.merge_field(1, WireType::LengthDelimited, &mut buf, &mut ctx, true)
            .unwrap();
        assert_eq!(val, Bytes::from(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_unit_message_encoding() {
        let mut buf = create_test_buffer();
        let mut ctx = EncodeLengthContext::default();

        let val = ();
        val.encode_raw(&mut buf);
        assert_eq!(val.encoded_len(&mut ctx), 0);
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_unit_message_decoding() {
        let mut ctx = DecodeContext::new(Bytes::new());

        let mut val = ();
        let mut buf = create_test_bytes(&[0x01]); // Some field value
        val.merge_field(1, WireType::Varint, &mut buf, &mut ctx, true)
            .unwrap();
        // Unit type should skip all fields
    }

    #[test]
    fn test_unknown_field_handling() {
        let mut ctx = DecodeContext::new(Bytes::new());

        // Test that unknown fields are skipped
        let mut val = 42u32;
        let mut buf = create_test_bytes(&[0x01]); // value=1 (unknown field)
        val.merge_field(2, WireType::Varint, &mut buf, &mut ctx, true)
            .unwrap();
        assert_eq!(val, 42); // Value should remain unchanged
    }

    #[test]
    fn test_zero_copy_threshold() {
        // Test small bytes (below threshold)
        let mut ctx1 = EncodeLengthContext::default();
        let small_bytes = Bytes::from(vec![1, 2, 3]);
        let len1 = small_bytes.encoded_len(&mut ctx1);
        assert_eq!(ctx1.zero_copy_len, 0);

        // Test large bytes (above threshold) - use a size that's definitely above any
        // threshold
        let mut ctx2 = EncodeLengthContext::default();
        let large_bytes = Bytes::from(vec![0; 4096]); // Use 4KB to be above any threshold
        let len2 = large_bytes.encoded_len(&mut ctx2);
        assert!(ctx2.zero_copy_len > 0); // Should be above threshold
        assert!(len2 > len1);
    }

    #[test]
    fn test_roundtrip_encoding_decoding() {
        // Test bool
        let original_bool = true;
        let mut buf = create_test_buffer();
        original_bool.encode_raw(&mut buf);

        let mut decoded_bool = false;
        let mut ctx = DecodeContext::new(Bytes::new());
        let buf_bytes = buf.concat().freeze();
        // Skip the tag (first byte) and just pass the value
        let mut value_bytes = buf_bytes.slice(1..);
        decoded_bool
            .merge_field(1, WireType::Varint, &mut value_bytes, &mut ctx, true)
            .unwrap();
        assert_eq!(original_bool, decoded_bool);

        // Test u32
        let original_u32 = 12345u32;
        let mut buf = create_test_buffer();
        original_u32.encode_raw(&mut buf);

        let mut decoded_u32 = 0u32;
        let mut ctx = DecodeContext::new(Bytes::new());
        let buf_bytes = buf.concat().freeze();
        // Skip the tag (first byte) and just pass the value
        let mut value_bytes = buf_bytes.slice(1..);
        decoded_u32
            .merge_field(1, WireType::Varint, &mut value_bytes, &mut ctx, true)
            .unwrap();
        assert_eq!(original_u32, decoded_u32);

        // Test string
        let original_string = String::from("test message");
        let mut buf = create_test_buffer();
        original_string.encode_raw(&mut buf);

        let mut decoded_string = String::new();
        let mut ctx = DecodeContext::new(Bytes::new());
        let buf_bytes = buf.concat().freeze();
        // Skip the tag (first byte) and just pass the length + data
        let mut value_bytes = buf_bytes.slice(1..);
        decoded_string
            .merge_field(
                1,
                WireType::LengthDelimited,
                &mut value_bytes,
                &mut ctx,
                true,
            )
            .unwrap();
        assert_eq!(original_string, decoded_string);
    }
}
