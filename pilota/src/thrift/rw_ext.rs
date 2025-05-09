use std::mem;

use bytes::{Buf as _, BufMut, BytesMut};

use super::{ThriftException, new_protocol_exception};

#[derive(thiserror::Error, Debug)]
pub enum IOError {
    #[error("no remaining {0}")]
    NoRemaining(String),
}

impl From<IOError> for ThriftException {
    fn from(e: IOError) -> Self {
        new_protocol_exception(super::ProtocolExceptionKind::InvalidData, format!("{e}"))
    }
}

macro_rules! io_read_impl {
    ($this:ident, $typ:tt::$conv:tt) => {{
        const SIZE: usize = mem::size_of::<$typ>();
        if $this.remaining() < SIZE {
            return Err(IOError::NoRemaining(format!("{}", stringify!($typ))));
        }
        // try to convert directly from the bytes
        // this Option<ret> trick is to avoid keeping a borrow on self
        // when advance() is called (mut borrow) and to call bytes() only once
        let ret = $this
            .chunk()
            .get(..SIZE)
            .map(|src| unsafe { $typ::$conv(*(src as *const _ as *const [_; SIZE])) });

        if let Some(ret) = ret {
            // if the direct conversion was possible, advance and return
            $this.advance(SIZE);
            return Ok(ret);
        } else {
            // if not we copy the bytes in a temp buffer then convert
            let mut buf = [0; SIZE];
            $this.copy_to_slice(&mut buf); // (do the advance)
            return Ok($typ::$conv(buf));
        }
    }};
    (le => $this:ident, $typ:tt, $len_to_read:expr_2021) => {{
        debug_assert!(mem::size_of::<$typ>() >= $len_to_read);

        // The same trick as above does not improve the best case speed.
        // It seems to be linked to the way the method is optimised by the compiler
        let mut buf = [0; (mem::size_of::<$typ>())];
        $this.copy_to_slice(&mut buf[..($len_to_read)]);
        return Ok($typ::from_le_bytes(buf));
    }};
    (be => $this:ident, $typ:tt, $len_to_read:expr_2021) => {{
        debug_assert!(mem::size_of::<$typ>() >= $len_to_read);

        let mut buf = [0; (mem::size_of::<$typ>())];
        $this.copy_to_slice(&mut buf[mem::size_of::<$typ>() - ($len_to_read)..]);
        return Ok($typ::from_be_bytes(buf));
    }};
}

#[macro_export]
macro_rules! assert_remaining {
    ($cond: expr_2021, $($arg:tt)+) => {
        #[cfg(not(feature = "unstable"))]
        if !$cond {
            return Err(IOError::NoRemaining(format!($($arg)+)))?;
        }
        #[cfg(feature = "unstable")]
        if !std::intrinsics::unlikely($cond) {
            return Err(IOError::NoRemaining(format!($($arg)+)))?;
        }
    };
    ($cond: expr_2021) => {
        #[cfg(not(feature = "unstable"))]
        if !$cond {
            return Err(IOError::NoRemaining(String::new()))?;
        }
        #[cfg(feature = "unstable")]
        if !std::intrinsics::unlikely($cond) {
            return Err(IOError::NoRemaining(String::new()))?;
        }
    };
}

pub trait WriteExt {
    fn write_slice(&mut self, src: &[u8]);
    fn write_u8(&mut self, n: u8);
    fn write_i8(&mut self, n: i8);
    fn write_u16(&mut self, n: u16);
    fn write_u16_le(&mut self, n: u16);
    fn write_i16(&mut self, n: i16);
    fn write_i16_le(&mut self, n: i16);
    fn write_u32(&mut self, n: u32);
    fn write_u32_le(&mut self, n: u32);
    fn write_i32(&mut self, n: i32);
    fn write_i32_le(&mut self, n: i32);
    fn write_u64(&mut self, n: u64);

    fn write_u64_le(&mut self, n: u64);

    fn write_i64(&mut self, n: i64);

    fn write_i64_le(&mut self, n: i64);

    fn write_u128(&mut self, n: u128);

    fn write_u128_le(&mut self, n: u128);

    fn write_i128(&mut self, n: i128);

    fn write_i128_le(&mut self, n: i128);

    fn write_uint(&mut self, n: u64, nbytes: usize);

    fn write_uint_le(&mut self, n: u64, nbytes: usize);

    fn write_int(&mut self, n: i64, nbytes: usize);

    fn write_int_le(&mut self, n: i64, nbytes: usize);

    fn write_f32(&mut self, n: f32);

    fn write_f32_le(&mut self, n: f32);

    fn write_f64(&mut self, n: f64);

    fn write_f64_le(&mut self, n: f64);
}

impl WriteExt for BytesMut {
    #[inline]
    fn write_slice(&mut self, src: &[u8]) {
        self.put_slice(src);
    }

    #[inline]
    fn write_u8(&mut self, n: u8) {
        let src = [n];
        self.write_slice(&src)
    }

    #[inline]
    fn write_i8(&mut self, n: i8) {
        let src = [n as u8];
        self.write_slice(&src)
    }

    #[inline]
    fn write_u16(&mut self, n: u16) {
        self.write_slice(&n.to_be_bytes())
    }

    #[inline]
    fn write_u16_le(&mut self, n: u16) {
        self.write_slice(&n.to_le_bytes())
    }

    #[inline]
    fn write_i16(&mut self, n: i16) {
        self.write_slice(&n.to_be_bytes())
    }

    #[inline]
    fn write_i16_le(&mut self, n: i16) {
        self.write_slice(&n.to_le_bytes())
    }

    #[inline]
    fn write_u32(&mut self, n: u32) {
        self.write_slice(&n.to_be_bytes())
    }

    #[inline]
    fn write_u32_le(&mut self, n: u32) {
        self.write_slice(&n.to_le_bytes())
    }

    #[inline]
    fn write_i32(&mut self, n: i32) {
        self.write_slice(&n.to_be_bytes())
    }

    #[inline]
    fn write_i32_le(&mut self, n: i32) {
        self.write_slice(&n.to_le_bytes())
    }

    #[inline]
    fn write_u64(&mut self, n: u64) {
        self.write_slice(&n.to_be_bytes())
    }

    #[inline]
    fn write_u64_le(&mut self, n: u64) {
        self.write_slice(&n.to_le_bytes())
    }

    #[inline]
    fn write_i64(&mut self, n: i64) {
        self.write_slice(&n.to_be_bytes())
    }

    #[inline]
    fn write_i64_le(&mut self, n: i64) {
        self.write_slice(&n.to_le_bytes())
    }

    #[inline]
    fn write_u128(&mut self, n: u128) {
        self.write_slice(&n.to_be_bytes())
    }

    #[inline]
    fn write_u128_le(&mut self, n: u128) {
        self.write_slice(&n.to_le_bytes())
    }

    #[inline]
    fn write_i128(&mut self, n: i128) {
        self.write_slice(&n.to_be_bytes())
    }

    #[inline]
    fn write_i128_le(&mut self, n: i128) {
        self.write_slice(&n.to_le_bytes())
    }

    #[inline]
    fn write_uint(&mut self, n: u64, nbytes: usize) {
        self.write_slice(&n.to_be_bytes()[mem::size_of_val(&n) - nbytes..])
    }

    #[inline]
    fn write_uint_le(&mut self, n: u64, nbytes: usize) {
        self.write_slice(&n.to_le_bytes()[0..nbytes])
    }

    #[inline]
    fn write_int(&mut self, n: i64, nbytes: usize) {
        self.write_slice(&n.to_be_bytes()[mem::size_of_val(&n) - nbytes..])
    }

    #[inline]
    fn write_int_le(&mut self, n: i64, nbytes: usize) {
        self.write_slice(&n.to_le_bytes()[0..nbytes])
    }

    #[inline]
    fn write_f32(&mut self, n: f32) {
        self.write_u32(n.to_bits())
    }

    #[inline]
    fn write_f32_le(&mut self, n: f32) {
        self.write_u32_le(n.to_bits())
    }

    #[inline]
    fn write_f64(&mut self, n: f64) {
        self.write_u64(n.to_bits())
    }

    #[inline]
    fn write_f64_le(&mut self, n: f64) {
        self.write_u64_le(n.to_bits())
    }
}

pub trait ReadExt {
    fn read_to_bytes(&mut self, size: usize) -> Result<bytes::Bytes, IOError>;
    fn read_to_string(&mut self, len: usize) -> Result<String, IOError>;

    fn read_to_slice(&mut self, dst: &mut [u8]) -> Result<(), IOError>;

    fn read_u8(&mut self) -> Result<u8, IOError>;

    fn read_i8(&mut self) -> Result<i8, IOError>;

    fn read_u16(&mut self) -> Result<u16, IOError>;

    fn read_u16_le(&mut self) -> Result<u16, IOError>;

    fn read_i16(&mut self) -> Result<i16, IOError>;

    fn read_i16_le(&mut self) -> Result<i16, IOError>;

    fn read_u32(&mut self) -> Result<u32, IOError>;

    fn read_u32_le(&mut self) -> Result<u32, IOError>;

    fn read_i32(&mut self) -> Result<i32, IOError>;

    fn read_i32_le(&mut self) -> Result<i32, IOError>;

    fn read_u64(&mut self) -> Result<u64, IOError>;

    fn read_u64_le(&mut self) -> Result<u64, IOError>;

    fn read_i64(&mut self) -> Result<i64, IOError>;

    fn read_i64_le(&mut self) -> Result<i64, IOError>;

    fn read_u128(&mut self) -> Result<u128, IOError>;

    fn read_u128_le(&mut self) -> Result<u128, IOError>;

    fn read_i128(&mut self) -> Result<i128, IOError>;

    fn read_i128_le(&mut self) -> Result<i128, IOError>;

    fn read_uint(&mut self, nbytes: usize) -> Result<u64, IOError>;

    fn read_uint_le(&mut self, nbytes: usize) -> Result<u64, IOError>;

    fn read_int(&mut self, nbytes: usize) -> Result<i64, IOError>;

    fn read_int_le(&mut self, nbytes: usize) -> Result<i64, IOError>;

    fn read_f32(&mut self) -> Result<f32, IOError>;

    fn read_f32_le(&mut self) -> Result<f32, IOError>;

    fn read_f64(&mut self) -> Result<f64, IOError>;

    fn read_f64_le(&mut self) -> Result<f64, IOError>;
}

impl<B> ReadExt for B
where
    B: bytes::Buf,
{
    #[inline]
    fn read_to_bytes(&mut self, len: usize) -> Result<bytes::Bytes, IOError> {
        assert_remaining!(len <= self.remaining(), "`len` greater than remaining");

        let mut ret = bytes::BytesMut::with_capacity(len);
        ret.put(self.take(len));
        Ok(ret.freeze())
    }

    #[inline]
    fn read_to_string(&mut self, len: usize) -> Result<String, IOError> {
        assert_remaining!(len <= self.remaining(), "`len` greater than remaining");
        // FIXME: use maybe_uninit?
        let mut vec = vec![0; len];
        self.read_to_slice(vec.as_mut_slice())?;
        unsafe { Ok(String::from_utf8_unchecked(vec)) }
    }

    #[inline]
    fn read_to_slice(&mut self, dst: &mut [u8]) -> Result<(), IOError> {
        assert_remaining!(self.remaining() >= dst.len());

        self.copy_to_slice(dst);

        Ok(())
    }

    #[inline]
    fn read_u8(&mut self) -> Result<u8, IOError> {
        assert_remaining!(self.remaining() >= 1);
        let ret = self.chunk()[0];
        self.advance(1);
        Ok(ret)
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8, IOError> {
        assert_remaining!(self.remaining() >= 1);
        let ret = self.chunk()[0] as i8;
        self.advance(1);
        Ok(ret)
    }

    #[inline]
    fn read_u16(&mut self) -> Result<u16, IOError> {
        io_read_impl!(self, u16::from_be_bytes);
    }

    #[inline]
    fn read_u16_le(&mut self) -> Result<u16, IOError> {
        io_read_impl!(self, u16::from_le_bytes);
    }

    #[inline]
    fn read_i16(&mut self) -> Result<i16, IOError> {
        io_read_impl!(self, i16::from_be_bytes);
    }

    #[inline]
    fn read_i16_le(&mut self) -> Result<i16, IOError> {
        io_read_impl!(self, i16::from_le_bytes);
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32, IOError> {
        io_read_impl!(self, u32::from_be_bytes);
    }

    #[inline]
    fn read_u32_le(&mut self) -> Result<u32, IOError> {
        io_read_impl!(self, u32::from_le_bytes);
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32, IOError> {
        io_read_impl!(self, i32::from_be_bytes);
    }

    #[inline]
    fn read_i32_le(&mut self) -> Result<i32, IOError> {
        io_read_impl!(self, i32::from_le_bytes);
    }

    #[inline]
    fn read_u64(&mut self) -> Result<u64, IOError> {
        io_read_impl!(self, u64::from_be_bytes);
    }

    #[inline]
    fn read_u64_le(&mut self) -> Result<u64, IOError> {
        io_read_impl!(self, u64::from_le_bytes);
    }

    #[inline]
    fn read_i64(&mut self) -> Result<i64, IOError> {
        io_read_impl!(self, i64::from_be_bytes);
    }

    #[inline]
    fn read_i64_le(&mut self) -> Result<i64, IOError> {
        io_read_impl!(self, i64::from_le_bytes);
    }

    #[inline]
    fn read_u128(&mut self) -> Result<u128, IOError> {
        io_read_impl!(self, u128::from_be_bytes);
    }

    #[inline]
    fn read_u128_le(&mut self) -> Result<u128, IOError> {
        io_read_impl!(self, u128::from_le_bytes);
    }

    #[inline]
    fn read_i128(&mut self) -> Result<i128, IOError> {
        io_read_impl!(self, i128::from_be_bytes);
    }

    #[inline]
    fn read_i128_le(&mut self) -> Result<i128, IOError> {
        io_read_impl!(self, i128::from_le_bytes);
    }

    #[inline]
    fn read_uint(&mut self, nbytes: usize) -> Result<u64, IOError> {
        io_read_impl!(be => self, u64, nbytes);
    }

    #[inline]
    fn read_uint_le(&mut self, nbytes: usize) -> Result<u64, IOError> {
        io_read_impl!(le => self, u64, nbytes);
    }

    #[inline]
    fn read_int(&mut self, nbytes: usize) -> Result<i64, IOError> {
        io_read_impl!(be => self, i64, nbytes);
    }

    #[inline]
    fn read_int_le(&mut self, nbytes: usize) -> Result<i64, IOError> {
        io_read_impl!(le => self, i64, nbytes);
    }

    #[inline]
    fn read_f32(&mut self) -> Result<f32, IOError> {
        Ok(f32::from_bits(Self::read_u32(self)?))
    }

    #[inline]
    fn read_f32_le(&mut self) -> Result<f32, IOError> {
        Ok(f32::from_bits(Self::read_u32_le(self)?))
    }

    #[inline]
    fn read_f64(&mut self) -> Result<f64, IOError> {
        Ok(f64::from_bits(Self::read_u64(self)?))
    }

    #[inline]
    fn read_f64_le(&mut self) -> Result<f64, IOError> {
        Ok(f64::from_bits(Self::read_u64_le(self)?))
    }
}
