//! High-level Intermediate Representation (HIR) for pilota-build.
//!
//! This module defines the AST-like structure that represents parsed IDL files.

pub mod ast;
pub mod visitor;
pub mod lower;

pub use ast::{HirNode, Item, ItemKind};

use pilota_build_common::{DefId, Span};
use serde::{Deserialize, Serialize};

/// HIR ID type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HirId {
    pub owner: DefId,
    pub local: LocalId,
}

/// Local ID within a definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalId(pub u32);

/// HIR crate represents a parsed file.
#[derive(Debug)]
pub struct HirCrate {
    pub items: Vec<Item>,
    pub span: Span,
}