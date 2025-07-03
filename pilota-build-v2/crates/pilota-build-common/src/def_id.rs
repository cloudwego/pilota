//! Definition IDs for uniquely identifying items.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The ID of a crate.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct CrateId(pub u32);

/// The local crate.
pub const LOCAL_CRATE: CrateId = CrateId(0);

impl fmt::Display for CrateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "crate{}", self.0)
    }
}

/// A definition ID that uniquely identifies an item.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct DefId {
    pub krate: CrateId,
    pub index: DefIndex,
}

impl DefId {
    pub fn local(index: DefIndex) -> Self {
        DefId {
            krate: LOCAL_CRATE,
            index,
        }
    }

    pub fn is_local(self) -> bool {
        self.krate == LOCAL_CRATE
    }

    pub fn as_local(self) -> Option<LocalDefId> {
        if self.is_local() {
            Some(LocalDefId { index: self.index })
        } else {
            None
        }
    }
}

impl fmt::Debug for DefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DefId({}:{})", self.krate.0, self.index.as_u32())
    }
}

impl fmt::Display for DefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.krate, self.index.as_u32())
    }
}

/// A local definition ID (within the current crate).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct LocalDefId {
    pub index: DefIndex,
}

impl LocalDefId {
    pub fn to_def_id(self) -> DefId {
        DefId {
            krate: LOCAL_CRATE,
            index: self.index,
        }
    }
}

/// An index into the definition table.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct DefIndex(u32);

impl DefIndex {
    pub const INVALID: DefIndex = DefIndex(u32::MAX);

    pub fn from_u32(index: u32) -> Self {
        assert_ne!(index, u32::MAX);
        DefIndex(index)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

/// A generator for definition IDs.
pub struct DefIdGenerator {
    next_index: u32,
}

impl DefIdGenerator {
    pub fn new() -> Self {
        DefIdGenerator { next_index: 0 }
    }

    pub fn next_def_id(&mut self) -> LocalDefId {
        let index = DefIndex::from_u32(self.next_index);
        self.next_index += 1;
        LocalDefId { index }
    }
}

impl Default for DefIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// A local ID within an item (re-exported for HIR).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct LocalId(pub u32);