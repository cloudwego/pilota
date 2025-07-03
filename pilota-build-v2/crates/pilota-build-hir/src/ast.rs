//! HIR AST node definitions.

use pilota_build_common::{Span, Symbol};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// An attribute like `#[deprecated]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: Path,
    pub args: Option<AttrArgs>,
    pub span: Span,
}

/// Attribute arguments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttrArgs {
    /// `#[attr = "value"]`
    Eq(Span, Expr),
    /// `#[attr(args)]`
    Paren(Vec<Expr>),
}

/// A path like `std::vec::Vec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

impl Path {
    /// Create a path from a single identifier.
    pub fn from_ident(ident: Symbol, span: Span) -> Self {
        Path {
            segments: vec![PathSegment {
                ident,
                args: None,
                span,
            }],
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

/// A segment of a path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathSegment {
    pub ident: Symbol,
    pub args: Option<GenericArgs>,
    pub span: Span,
}

/// Generic arguments like `<T, U>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericArgs {
    pub args: Vec<Type>,
    pub span: Span,
}

/// A type in HIR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    /// A path type like `std::vec::Vec`.
    Path(Path),
    /// A primitive type.
    Primitive(PrimitiveType),
    /// A vector type `vec<T>`.
    Vec(Box<Type>),
    /// A set type `set<T>`.
    Set(Box<Type>),
    /// A map type `map<K, V>`.
    Map {
        key: Box<Type>,
        value: Box<Type>,
    },
    /// An optional type `T?`.
    Optional(Box<Type>),
    /// A reference type (for recursive types).
    Reference {
        mutable: bool,
        ty: Box<Type>,
    },
}

/// Primitive types.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

/// An expression (for constants and defaults).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    /// A literal value.
    Literal(Literal),
    /// A path reference.
    Path(Path),
    /// A list literal `[1, 2, 3]`.
    List(Vec<Expr>),
    /// A map literal `{"key": value}`.
    Map(Vec<(Expr, Expr)>),
    /// A struct literal.
    Struct {
        path: Path,
        fields: Vec<FieldExpr>,
    },
    /// Unary operation.
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    /// Binary operation.
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

/// A field in a struct expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldExpr {
    pub name: Symbol,
    pub value: Expr,
    pub span: Span,
}

/// Literal values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Symbol),
    Bytes(Vec<u8>),
}

/// Unary operators.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    /// `-`
    Neg,
    /// `!`
    Not,
}

/// Binary operators.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Rem,
    /// `&&`
    And,
    /// `||`
    Or,
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
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

/// A message/struct definition.
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

/// Field requirement level.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FieldRequired {
    /// Required field.
    Required,
    /// Optional field.
    Optional,
    /// Default (depends on protocol).
    Default,
}

/// An enum definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: Symbol,
    pub variants: Vec<EnumVariant>,
}

/// A variant in an enum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: Symbol,
    pub value: Option<i32>,
    pub attrs: Vec<Attribute>,
    pub span: Span,
}

/// A constant definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Const {
    pub name: Symbol,
    pub ty: Type,
    pub value: Expr,
}

/// A type alias.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: Symbol,
    pub ty: Type,
}

/// A module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: Symbol,
    pub items: Vec<super::Item>,
}

/// A use/import statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Use {
    pub path: Path,
    pub alias: Option<Symbol>,
}