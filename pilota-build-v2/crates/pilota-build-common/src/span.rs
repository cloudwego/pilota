//! Source code spans and positions.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A byte position in a source file.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct BytePos(pub u32);

impl BytePos {
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for BytePos {
    fn from(n: u32) -> BytePos {
        BytePos(n)
    }
}

impl From<usize> for BytePos {
    fn from(n: usize) -> BytePos {
        BytePos(n as u32)
    }
}

/// A character position in a source file.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CharPos(pub u32);

impl CharPos {
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

/// A span in the source code.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
    pub file_id: super::FileId,
}

impl Span {
    /// Create a new span.
    pub fn new(lo: BytePos, hi: BytePos, file_id: super::FileId) -> Self {
        assert!(lo <= hi, "lo ({:?}) must be <= hi ({:?})", lo, hi);
        Span { lo, hi, file_id }
    }

    /// Create a span that covers `self` and `other`.
    pub fn to(self, other: Span) -> Span {
        assert_eq!(self.file_id, other.file_id, "spans must be in the same file");
        Span::new(
            std::cmp::min(self.lo, other.lo),
            std::cmp::max(self.hi, other.hi),
            self.file_id,
        )
    }

    /// Create a span that covers `self` and the point `pos`.
    pub fn with_lo(self, lo: BytePos) -> Span {
        Span::new(lo, self.hi, self.file_id)
    }

    /// Create a span that covers the point `pos` and `self`.
    pub fn with_hi(self, hi: BytePos) -> Span {
        Span::new(self.lo, hi, self.file_id)
    }

    /// Returns true if this span contains the given position.
    pub fn contains(self, pos: BytePos) -> bool {
        self.lo <= pos && pos < self.hi
    }

    /// Returns true if this span overlaps with another span.
    pub fn overlaps(self, other: Span) -> bool {
        self.file_id == other.file_id && self.lo < other.hi && other.lo < self.hi
    }

    /// Returns the length of this span in bytes.
    pub fn len(self) -> u32 {
        self.hi.0 - self.lo.0
    }

    /// Returns true if this span is empty.
    pub fn is_empty(self) -> bool {
        self.lo == self.hi
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.lo.0, self.hi.0)
    }
}

/// A dummy span for when we don't have location information.
pub const DUMMY_SPAN: Span = Span {
    lo: BytePos(0),
    hi: BytePos(0),
    file_id: super::FileId::DUMMY,
};