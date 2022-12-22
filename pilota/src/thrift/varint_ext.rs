use std::{io::Error, mem::size_of};

use integer_encoding::VarInt;

/// Most-significant byte, == 0x80
pub const MSB: u8 = 0b1000_0000;

pub trait VarIntExt {
    fn varint_max_size() -> usize;
}

impl<VI: VarInt> VarIntExt for VI {
    fn varint_max_size() -> usize {
        (size_of::<VI>() * 8 + 7) / 7
    }
}

#[derive(Default)]
pub struct VarIntProcessor {
    buf: [u8; 10],
    maxsize: usize,
    pub i: usize,
}

impl VarIntProcessor {
    pub fn new<VI: VarIntExt>() -> VarIntProcessor {
        VarIntProcessor {
            maxsize: VI::varint_max_size(),
            ..VarIntProcessor::default()
        }
    }
    pub fn push(&mut self, b: u8) -> Result<(), Error> {
        if self.i >= self.maxsize {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Unterminated varint",
            ));
        }
        self.buf[self.i] = b;
        self.i += 1;
        Ok(())
    }
    pub fn finished(&self) -> bool {
        self.i > 0 && (self.buf[self.i - 1] & MSB == 0)
    }
    pub fn decode<VI: VarInt>(&self) -> Option<VI> {
        Some(VI::decode_var(&self.buf[0..self.i])?.0)
    }
}
