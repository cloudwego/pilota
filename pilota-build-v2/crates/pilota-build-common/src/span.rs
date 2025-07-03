//! Source location tracking.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A byte position in a source file.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct BytePos(pub u32);

impl BytePos {
    pub fn from(n: usize) -> Self {
        BytePos(n as u32)
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for BytePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A character position in a source file.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct CharPos(pub u32);

impl CharPos {
    pub fn from(n: usize) -> Self {
        CharPos(n as u32)
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for CharPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A span representing a range of bytes in a source file.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Span {
    /// The starting byte position.
    pub start: BytePos,
    /// The ending byte position (exclusive).
    pub end: BytePos,
    /// The file this span is from.
    pub file_id: super::FileId,
}

impl Span {
    /// Create a new span.
    pub fn new(start: BytePos, end: BytePos, file_id: super::FileId) -> Self {
        Span { start, end, file_id }
    }

    /// Create a span covering two spans.
    pub fn with_lo(&self, start: BytePos) -> Span {
        Span { start, ..*self }
    }

    /// Create a span covering two spans.
    pub fn with_hi(&self, end: BytePos) -> Span {
        Span { end, ..*self }
    }

    /// Return the length of this span in bytes.
    pub fn len(&self) -> u32 {
        self.end.0 - self.start.0
    }

    /// Check if this span is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Check if this span contains a byte position.
    pub fn contains(&self, pos: BytePos) -> bool {
        self.start <= pos && pos < self.end
    }

    /// Combine two spans into one that covers both.
    pub fn merge(self, other: Span) -> Span {
        use std::cmp::{max, min};
        Span {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
            file_id: self.file_id,
        }
    }
}

/// A dummy span for when we don't have location information.
pub const DUMMY_SPAN: Span = Span {
    start: BytePos(0),
    end: BytePos(0),
    file_id: 0,
};

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}