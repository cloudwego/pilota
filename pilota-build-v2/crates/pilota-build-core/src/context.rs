//! Compiler context for managing compilation state.

use dashmap::DashMap;
use pilota_build_common::{DefId, FastHashMap, Symbol};
use pilota_build_hir::HirCrate;
use std::sync::Arc;

/// The main compilation context.
pub struct CompilerContext {
    /// HIR representation of all crates.
    pub hir_crates: DashMap<Symbol, Arc<HirCrate>>,
    /// Symbol definitions.
    pub definitions: DashMap<DefId, Definition>,
    /// Type information.
    pub types: DashMap<DefId, TypeInfo>,
    /// Module hierarchy.
    pub modules: DashMap<DefId, ModuleInfo>,
}

/// A definition in the compiler.
#[derive(Debug, Clone)]
pub struct Definition {
    pub def_id: DefId,
    pub name: Symbol,
    pub kind: DefKind,
    pub parent: Option<DefId>,
    pub visibility: Visibility,
}

/// The kind of definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefKind {
    Service,
    Message,
    Enum,
    Const,
    TypeAlias,
    Module,
    Method,
    Field,
    EnumVariant,
}

/// Visibility of a definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

/// Type information for a definition.
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub def_id: DefId,
    pub ty: Type,
}

/// A resolved type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Primitive types.
    Primitive(PrimitiveType),
    /// User-defined type.
    Adt(DefId),
    /// Vector type.
    Vec(Box<Type>),
    /// Set type.
    Set(Box<Type>),
    /// Map type.
    Map(Box<Type>, Box<Type>),
    /// Optional type.
    Optional(Box<Type>),
    /// Function type (for methods).
    Function {
        params: Vec<Type>,
        result: Box<Type>,
    },
    /// Error type (for error recovery).
    Error,
}

/// Primitive types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Bytes,
    Void,
}

/// Module information.
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub def_id: DefId,
    pub name: Symbol,
    pub parent: Option<DefId>,
    pub children: Vec<DefId>,
    pub items: FastHashMap<Symbol, DefId>,
}

impl CompilerContext {
    /// Create a new compiler context.
    pub fn new() -> Self {
        CompilerContext {
            hir_crates: DashMap::new(),
            definitions: DashMap::new(),
            types: DashMap::new(),
            modules: DashMap::new(),
        }
    }

    /// Register a definition.
    pub fn register_def(&self, def: Definition) {
        let def_id = def.def_id;
        self.definitions.insert(def_id, def);
    }

    /// Get a definition by ID.
    pub fn get_def(&self, def_id: DefId) -> Option<Definition> {
        self.definitions.get(&def_id).map(|d| d.clone())
    }

    /// Register type information.
    pub fn register_type(&self, def_id: DefId, ty: Type) {
        self.types.insert(def_id, TypeInfo { def_id, ty });
    }

    /// Get type information.
    pub fn get_type(&self, def_id: DefId) -> Option<Type> {
        self.types.get(&def_id).map(|t| t.ty.clone())
    }
}

impl Default for CompilerContext {
    fn default() -> Self {
        Self::new()
    }
}