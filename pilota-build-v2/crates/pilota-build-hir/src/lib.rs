//! High-level Intermediate Representation (HIR) for pilota-build.
//!
//! HIR is the first intermediate representation after parsing. It preserves
//! all source-level information and uses unresolved paths for references.

pub mod ast;
pub mod lower;
pub mod visit;

pub use ast::*;
pub use visit::{walk_item, Visitor};

use pilota_build_common::{DefId, LocalDefId, LocalId, Span, Symbol};
use serde::{Deserialize, Serialize};

/// A HIR crate containing all items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirCrate {
    pub items: Vec<Item>,
    pub span: Span,
}

/// A HIR node with location and attribute information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirNode<T> {
    pub id: HirId,
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub kind: T,
}

/// HIR node ID.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct HirId {
    pub owner: LocalDefId,
    pub local_id: LocalId,
}

/// A top-level item.
pub type Item = HirNode<ItemKind>;

/// The kind of item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemKind {
    /// A service definition.
    Service(Service),
    /// A message/struct definition.
    Message(Message),
    /// An enum definition.
    Enum(Enum),
    /// A constant definition.
    Const(Const),
    /// A type alias.
    TypeAlias(TypeAlias),
    /// A module.
    Module(Module),
    /// A use/import statement.
    Use(Use),
}

impl Item {
    /// Get the name of this item.
    pub fn name(&self) -> Option<Symbol> {
        match &self.kind {
            ItemKind::Service(s) => Some(s.name),
            ItemKind::Message(m) => Some(m.name),
            ItemKind::Enum(e) => Some(e.name),
            ItemKind::Const(c) => Some(c.name),
            ItemKind::TypeAlias(t) => Some(t.name),
            ItemKind::Module(m) => Some(m.name),
            ItemKind::Use(_) => None,
        }
    }
}