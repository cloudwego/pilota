//! AST definitions for HIR.

use crate::{HirId, LocalId};
use pilota_build_common::{Span, Symbol};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// A HIR node with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirNode<T> {
    pub id: HirId,
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub kind: T,
}

/// A top-level item.
pub type Item = HirNode<ItemKind>;

/// The kind of item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemKind {
    /// Message (struct/exception in Thrift, message in Protobuf)
    Message(Message),
    /// Service definition
    Service(Service),
    /// Enum definition
    Enum(Enum),
    /// Type alias
    TypeAlias(TypeAlias),
    /// Constant
    Const(Const),
    /// Module (for organizing items)
    Module(Module),
}

/// A message definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub name: Symbol,
    pub fields: Vec<Field>,
    pub is_exception: bool,
}

/// A field in a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub id: Option<i32>,
    pub name: Symbol,
    pub ty: Type,
    pub required: FieldRequired,
    pub default: Option<Expr>,
    pub attrs: Vec<Attribute>,
    pub span: Span,
}

/// Field requiredness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldRequired {
    Required,
    Optional,
    Default,
}

/// A service definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: Symbol,
    pub extends: Option<Path>,
    pub methods: Vec<Method>,
}

/// A method in a service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: Symbol,
    pub oneway: bool,
    pub params: Vec<Field>,
    pub result: Option<Type>,
    pub exceptions: Vec<Field>,
    pub attrs: Vec<Attribute>,
    pub span: Span,
}

/// An enum definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: Symbol,
    pub variants: Vec<EnumVariant>,
}

/// An enum variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: Symbol,
    pub value: Option<i32>,
    pub attrs: Vec<Attribute>,
    pub span: Span,
}

/// A type alias.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: Symbol,
    pub ty: Type,
}

/// A constant definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Const {
    pub name: Symbol,
    pub ty: Type,
    pub value: Expr,
}

/// A module for organizing items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: Symbol,
    pub items: Vec<Item>,
}

/// Type representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    /// Primitive type
    Primitive(PrimitiveType),
    /// Path to a type
    Path(Path),
    /// List/vector type
    Vec(Box<Type>),
    /// Set type
    Set(Box<Type>),
    /// Map type
    Map { key: Box<Type>, value: Box<Type> },
    /// Optional type
    Optional(Box<Type>),
}

/// Primitive types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    Bool,
    Byte,
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

/// Path to an item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

impl Path {
    /// Create a path from a single identifier.
    pub fn from_ident(ident: Symbol, span: Span) -> Self {
        Path {
            segments: vec![PathSegment { ident, args: None }],
            span,
        }
    }

    /// Check if this is a single-segment path.
    pub fn is_ident(&self) -> bool {
        self.segments.len() == 1 && self.segments[0].args.is_none()
    }

    /// Get the identifier if this is a single-segment path.
    pub fn as_ident(&self) -> Option<Symbol> {
        if self.is_ident() {
            Some(self.segments[0].ident)
        } else {
            None
        }
    }
}

/// A segment in a path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathSegment {
    pub ident: Symbol,
    pub args: Option<Vec<Type>>,
}

/// Expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    /// Literal value
    Literal(Literal),
    /// Path expression
    Path(Path),
    /// List expression
    List(Vec<Expr>),
    /// Map expression
    Map(Vec<(Expr, Expr)>),
}

/// Literal values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Symbol),
}

/// Attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: Symbol,
    pub args: Vec<AttributeArg>,
    pub span: Span,
}

/// Attribute argument.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeArg {
    /// Named argument
    Named { name: Symbol, value: Expr },
    /// Positional argument
    Positional(Expr),
}