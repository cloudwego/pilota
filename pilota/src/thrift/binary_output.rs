//! Binary output with a buffered write cursor and optional zero-copy fields.
//!
//! Unlike the unchecked protocol, this writer accepts arbitrary safe `Message`
//! implementations: an underestimated size grows the buffer, and `buf_mut()`
//! cannot invalidate the cursor. Only initialized bytes are committed on drop.

use std::ptr;

use bytes::{BufMut, Bytes, BytesMut, buf::UninitSlice};
use faststr::FastStr;
use linkedbytes::LinkedBytes;

use super::{
    TLengthProtocol, TListIdentifier, TMapIdentifier, TMessageIdentifier, TOutputProtocol,
    TSetIdentifier, TStructIdentifier, TType, ThriftException, ZERO_COPY_THRESHOLD,
};

const VERSION_1: u32 = 0x80010000;

/// A binary writer that batches updates of the output buffer's initialized
/// length.
///
/// Reserve the estimated size before constructing the writer for best
/// performance. The size is only a hint: all writes remain checked and can grow
/// the buffer. Pending writes are committed when the writer is flushed or
/// dropped, including when message encoding returns an error or unwinds.
pub struct TBinaryOutputProtocol<'a> {
    buffer: BinaryOutputBuffer<'a>,
    zero_copy: bool,
    zero_copy_len: usize,
}

impl<'a> TBinaryOutputProtocol<'a> {
    /// Append to a contiguous buffer, preserving any existing prefix.
    #[inline]
    pub fn new(output: &'a mut BytesMut) -> Self {
        Self {
            buffer: BinaryOutputBuffer::new(Storage::Contiguous(output)),
            zero_copy: false,
            zero_copy_len: 0,
        }
    }

    /// Append to a segmented buffer without copying large owned binary/string
    /// fields.
    #[inline]
    pub fn new_linked(output: &'a mut LinkedBytes) -> Self {
        Self {
            buffer: BinaryOutputBuffer::new(Storage::Linked(output)),
            zero_copy: true,
            zero_copy_len: 0,
        }
    }
}

enum Storage<'a> {
    Contiguous(&'a mut BytesMut),
    Linked(&'a mut LinkedBytes),
}

impl Storage<'_> {
    #[inline]
    fn bytes_mut(&mut self) -> &mut BytesMut {
        match self {
            Self::Contiguous(output) => output,
            Self::Linked(output) => output.bytes_mut(),
        }
    }
}

/// The checked `BufMut` view exposed by [`TBinaryOutputProtocol`].
///
/// It owns the cursor so custom `Message::encode` implementations can use
/// `TOutputProtocol::buf_mut` without invalidating cached allocation pointers.
pub struct BinaryOutputBuffer<'a> {
    storage: Storage<'a>,
    ptr: *mut u8,
    capacity: usize,
    initialized: usize,
    written: usize,
}

impl<'a> BinaryOutputBuffer<'a> {
    #[inline]
    fn new(storage: Storage<'a>) -> Self {
        let mut buffer = Self {
            storage,
            ptr: ptr::NonNull::<u8>::dangling().as_ptr(),
            capacity: 0,
            initialized: 0,
            written: 0,
        };
        buffer.refresh();
        buffer
    }

    #[inline]
    fn refresh(&mut self) {
        debug_assert_eq!(self.written, 0);
        let output = self.storage.bytes_mut();
        self.initialized = output.len();
        let spare = output.spare_capacity_mut();
        self.ptr = spare.as_mut_ptr().cast();
        self.capacity = spare.len();
    }

    #[inline]
    fn flush(&mut self) {
        if self.written != 0 {
            // SAFETY: put_slice initializes every byte, and BufMut::advance_mut
            // requires its caller to do so. Both paths check remaining capacity.
            unsafe { self.storage.bytes_mut().advance_mut(self.written) };
            self.written = 0;
            self.refresh();
        }
    }

    #[cold]
    #[inline(never)]
    fn grow(&mut self, additional: usize) {
        self.flush();
        self.storage.bytes_mut().reserve(additional);
        self.refresh();
    }
}

impl Drop for BinaryOutputBuffer<'_> {
    fn drop(&mut self) {
        self.flush();
    }
}

// SAFETY: chunk_mut exposes only spare capacity, and advance_mut checks its
// bounds. Growth commits initialized bytes and refreshes the allocation
// pointer.
unsafe impl BufMut for BinaryOutputBuffer<'_> {
    #[inline]
    fn remaining_mut(&self) -> usize {
        (isize::MAX as usize).saturating_sub(self.initialized + self.written)
    }

    #[inline]
    unsafe fn advance_mut(&mut self, count: usize) {
        assert!(count <= self.capacity - self.written);
        self.written += count;
    }

    #[inline]
    fn chunk_mut(&mut self) -> &mut UninitSlice {
        if self.written == self.capacity {
            self.grow(64);
        }
        // SAFETY: the allocation remains exclusively borrowed by storage;
        // written is in bounds and the exposed tail is writable spare capacity.
        unsafe {
            UninitSlice::from_raw_parts_mut(
                self.ptr.add(self.written),
                self.capacity - self.written,
            )
        }
    }

    #[inline]
    fn put_slice(&mut self, bytes: &[u8]) {
        if bytes.len() > self.capacity - self.written {
            self.grow(bytes.len());
        }
        // SAFETY: capacity was checked above. A safe input slice cannot overlap
        // the exclusively borrowed spare capacity. No uninitialized u8 slice is
        // constructed, and the output length is advanced only after this copy.
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.add(self.written), bytes.len());
        }
        self.written += bytes.len();
    }
}

impl TLengthProtocol for TBinaryOutputProtocol<'_> {
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
        if self.zero_copy && b.len() >= ZERO_COPY_THRESHOLD {
            self.zero_copy_len += b.len();
        }
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
        if self.zero_copy && s.len() >= ZERO_COPY_THRESHOLD {
            self.zero_copy_len += s.len();
        }
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

    #[inline]
    fn zero_copy_len(&mut self) -> usize {
        self.zero_copy_len
    }

    #[inline]
    fn reset(&mut self) {
        self.zero_copy_len = 0;
    }
}

impl<'a> TOutputProtocol for TBinaryOutputProtocol<'a> {
    type BufMut = BinaryOutputBuffer<'a>;

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
        self.buffer.put_slice(&data);
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
        if self.zero_copy && b.len() >= ZERO_COPY_THRESHOLD {
            self.buffer.flush();
            if let Storage::Linked(output) = &mut self.buffer.storage {
                output.insert(b);
                self.buffer.refresh();
                return Ok(());
            }
        }
        self.buffer.put_slice(&b);
        Ok(())
    }

    #[inline]
    fn write_byte(&mut self, b: u8) -> Result<(), ThriftException> {
        self.buffer.put_slice(&b.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn write_uuid(&mut self, u: [u8; 16]) -> Result<(), ThriftException> {
        self.buffer.put_slice(&u);
        Ok(())
    }

    #[inline]
    fn write_i8(&mut self, i: i8) -> Result<(), ThriftException> {
        self.buffer.put_slice(&i.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn write_i16(&mut self, i: i16) -> Result<(), ThriftException> {
        self.buffer.put_slice(&i.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), ThriftException> {
        self.buffer.put_slice(&i.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn write_i64(&mut self, i: i64) -> Result<(), ThriftException> {
        self.buffer.put_slice(&i.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn write_double(&mut self, d: f64) -> Result<(), ThriftException> {
        self.buffer.put_slice(&d.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn write_string(&mut self, s: &str) -> Result<(), ThriftException> {
        self.write_i32(s.len() as i32)?;
        self.buffer.put_slice(s.as_bytes());
        Ok(())
    }
    #[inline]
    fn write_faststr(&mut self, s: FastStr) -> Result<(), ThriftException> {
        self.write_i32(s.len() as i32)?;
        if self.zero_copy && s.len() >= ZERO_COPY_THRESHOLD {
            self.buffer.flush();
            if let Storage::Linked(output) = &mut self.buffer.storage {
                output.insert_faststr(s);
                self.buffer.refresh();
                return Ok(());
            }
        }
        self.buffer.put_slice(s.as_ref());
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
        self.buffer.flush();
        Ok(())
    }
    #[inline]
    fn write_bytes_vec(&mut self, b: &[u8]) -> Result<(), ThriftException> {
        self.write_i32(b.len() as i32)?;
        self.buffer.put_slice(b);
        Ok(())
    }

    #[inline]
    fn buf_mut(&mut self) -> &mut Self::BufMut {
        &mut self.buffer
    }
}

#[cfg(test)]
mod tests {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use super::*;
    use crate::thrift::{TMessageType, binary::TBinaryProtocol};

    fn write_fixture<P: TOutputProtocol>(p: &mut P) -> Result<(), ThriftException> {
        p.write_message_begin(&TMessageIdentifier::new(
            "fixture".into(),
            TMessageType::Call,
            -7,
        ))?;
        p.write_struct_begin(&TStructIdentifier::new("fixture"))?;
        p.write_field_begin(TType::I16, -1)?;
        p.write_i16(i16::MIN)?;
        p.write_field_end()?;
        p.write_i32(i32::MAX)?;
        p.write_i64(i64::MIN)?;
        p.write_double(-1.25)?;
        p.write_uuid([0xab; 16])?;
        p.write_bool(true)?;
        p.write_i8(-128)?;
        p.write_list_begin(TListIdentifier::new(TType::I32, 1))?;
        p.write_i32(42)?;
        p.write_list_end()?;
        p.write_set_begin(TSetIdentifier::new(TType::I8, 1))?;
        p.write_i8(-7)?;
        p.write_set_end()?;
        p.write_map_begin(TMapIdentifier::new(TType::I16, TType::Binary, 1))?;
        p.write_i16(3)?;
        p.write_string("中文")?;
        p.write_map_end()?;
        p.write_bytes_vec(b"bytes")?;
        p.write_bytes(Bytes::from_static(b"owned bytes"))?;
        p.write_faststr("faststr".into())?;
        p.flush()?;
        p.write_field_stop()?;
        p.write_struct_end()?;
        p.write_message_end()
    }

    #[test]
    fn matches_reference_with_prefix_and_underestimated_capacity() {
        for prefix in [0, 5, 32] {
            let mut expected = BytesMut::from(vec![0xab; prefix].as_slice());
            write_fixture(&mut TBinaryProtocol::new(&mut expected, false)).unwrap();
            for spare in [0, 1, 16, 512] {
                let mut output = BytesMut::with_capacity(prefix + spare);
                output.extend_from_slice(&vec![0xab; prefix]);
                write_fixture(&mut TBinaryOutputProtocol::new(&mut output)).unwrap();
                assert_eq!(output, expected);
                let mut output = LinkedBytes::with_capacity(prefix + spare);
                output.put_slice(&vec![0xab; prefix]);
                write_fixture(&mut TBinaryOutputProtocol::new_linked(&mut output)).unwrap();
                assert_eq!(output.concat(), expected);
            }
        }
    }

    #[test]
    fn linked_fields_keep_backing_allocations_and_following_bytes() {
        let bytes = Bytes::from(vec![0xff; ZERO_COPY_THRESHOLD + 1]);
        let text = FastStr::new("x".repeat(ZERO_COPY_THRESHOLD + 1));
        let mut expected = LinkedBytes::new();
        let mut output = LinkedBytes::with_capacity(8);
        for optimized in [false, true] {
            fn fields<P: TOutputProtocol>(p: &mut P, bytes: Bytes, text: FastStr) {
                p.write_i64(11).unwrap();
                p.write_bytes(bytes).unwrap();
                p.write_i16(-3).unwrap();
                p.write_faststr(text).unwrap();
                p.write_i32(17).unwrap();
            }
            if optimized {
                fields(
                    &mut TBinaryOutputProtocol::new_linked(&mut output),
                    bytes.clone(),
                    text.clone(),
                );
            } else {
                fields(
                    &mut TBinaryProtocol::new(&mut expected, true),
                    bytes.clone(),
                    text.clone(),
                );
            }
        }
        assert_eq!(output.concat(), expected.concat());
        let slices = output.io_slice();
        assert!(
            slices
                .iter()
                .any(|s| s.as_ptr() == bytes.as_ptr() && s.len() == bytes.len())
        );
        assert!(
            slices
                .iter()
                .any(|s| s.as_ptr() == text.as_ptr() && s.len() == text.len())
        );
    }

    #[test]
    fn custom_bufmut_writes_cannot_invalidate_the_cursor() {
        let mut output = BytesMut::with_capacity(1);
        {
            let mut p = TBinaryOutputProtocol::new(&mut output);
            p.write_byte(9).unwrap();
            p.buf_mut().put_slice(&[42; 100]);
            let raw = p.buf_mut();
            raw.chunk_mut()[..2].copy_from_slice(&[7, 8]);
            // SAFETY: the two bytes were initialized immediately above.
            unsafe { raw.advance_mut(2) };
            p.write_i16(-1).unwrap();
        }
        let mut expected = vec![9];
        expected.extend_from_slice(&[42; 100]);
        expected.extend_from_slice(&[7, 8, 255, 255]);
        assert_eq!(output.as_ref(), expected);
    }

    #[test]
    fn exchanged_bufmut_views_keep_large_fields() {
        let bytes = Bytes::from(vec![42; ZERO_COPY_THRESHOLD + 1]);
        let text = FastStr::new("x".repeat(ZERO_COPY_THRESHOLD + 1));
        let mut contiguous = BytesMut::new();
        let mut linked = LinkedBytes::new();
        {
            let mut first = TBinaryOutputProtocol::new(&mut contiguous);
            let mut second = TBinaryOutputProtocol::new_linked(&mut linked);
            first.write_byte(1).unwrap();
            second.write_byte(2).unwrap();
            std::mem::swap(first.buf_mut(), second.buf_mut());
            for p in [&mut first, &mut second] {
                p.write_bytes(bytes.clone()).unwrap();
                p.write_faststr(text.clone()).unwrap();
                p.write_i16(7).unwrap();
            }
        }
        let mut expected = BytesMut::new();
        let mut p = TBinaryProtocol::new(&mut expected, false);
        p.write_bytes(bytes).unwrap();
        p.write_faststr(text).unwrap();
        p.write_i16(7).unwrap();
        assert_eq!(contiguous[0], 1);
        assert_eq!(&contiguous[1..], expected.as_ref());
        let linked = linked.concat();
        assert_eq!(linked[0], 2);
        assert_eq!(&linked[1..], expected.as_ref());
    }

    #[test]
    fn unwinding_commits_only_initialized_bytes() {
        let mut output = BytesMut::with_capacity(64);
        output.extend_from_slice(b"prefix");
        assert!(
            catch_unwind(AssertUnwindSafe(|| {
                let mut p = TBinaryOutputProtocol::new(&mut output);
                p.write_i16(0x1234).unwrap();
                panic!("message encoder failed");
            }))
            .is_err()
        );
        assert_eq!(output.as_ref(), b"prefix\x12\x34");
    }
}
