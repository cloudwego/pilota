//! Common types and utilities for pilota-build.

mod def_id;
mod source_map;
mod span;
mod symbol;

pub use def_id::{DefId, DefIndex, LocalDefId};
pub use source_map::{SourceFile, SourceMap};
pub use span::{BytePos, CharPos, Span, DUMMY_SPAN};
pub use symbol::Symbol;

/// File ID type.
pub type FileId = u32;

use rustc_hash::FxHashMap;
pub type FastHashMap<K, V> = FxHashMap<K, V>;