use std::collections::VecDeque;

use bytes::Bytes;

const DEFAULT_DEQUE_SIZE: usize = 16;

#[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]

pub struct LinkedBytes {
    pub list: VecDeque<Bytes>,
    pub size: usize,
}

impl LinkedBytes {
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_DEQUE_SIZE)
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            list: VecDeque::with_capacity(cap),
            size: 0,
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push_back(&mut self, bytes: Bytes) {
        self.size += bytes.len();
        self.list.push_back(bytes);
    }
}

impl Default for LinkedBytes {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
