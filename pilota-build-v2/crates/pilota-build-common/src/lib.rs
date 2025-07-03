//! Common types and utilities used across pilota-build crates.

pub mod span;
pub mod symbol;
pub mod def_id;
pub mod source_map;

pub use span::{Span, BytePos, CharPos, DUMMY_SPAN};
pub use symbol::{Symbol, SymbolInterner};
pub use def_id::{DefId, LocalDefId, DefIdGenerator, LocalId, CrateId, LOCAL_CRATE};
pub use source_map::{SourceMap, FileId};

use rustc_hash::FxHashMap;
pub type FastHashMap<K, V> = FxHashMap<K, V>;