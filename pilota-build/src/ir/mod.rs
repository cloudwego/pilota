use std::{fmt::Display, sync::Arc};

use itertools::Itertools;
use pilota::Bytes;

use crate::{
    symbol::{EnumRepr, FileId, Ident, Symbol},
    tags::Tags,
};

pub mod visit;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PbOptionsExtendee {
    File,
    Message,
    Field,
    Enum,
    EnumValue,
    Service,
    Method,
    Oneof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PbFieldType {
    Bool,
    Int32,
    Int64,
    UInt32,
    UInt64,
    Float,
    Double,
    String,
    Bytes,
    Message,
}

#[derive(Clone, Debug)]
pub struct Extension {
    pub name: Ident,
    pub number: u32,
    pub field_ty: PbFieldType,
    pub extendee: PbOptionsExtendee,
    pub value_ty: Ty,
}

#[derive(Clone, Debug)]
pub enum Literal {
    Bool(bool),
    Path(Path),
    String(Arc<str>),
    Int(i64),
    Float(Arc<str>),
    List(Vec<Literal>),
    Map(Vec<(Literal, Literal)>),
}

#[derive(Clone, Debug)]
pub enum TyKind {
    String,
    Void,
    U8,
    Bool,
    Bytes,
    I8,
    I16,
    I32,
    I64,
    UInt64,
    UInt32,
    F32,
    F64,
    Uuid,
    Vec(Arc<Ty>),
    Set(Arc<Ty>),
    Map(Arc<Ty>, Arc<Ty>),
    Path(Path),
}

#[derive(Clone, Debug)]
pub struct Ty {
    pub tags: Arc<Tags>,
    pub kind: TyKind,
}

#[derive(Clone, Debug)]
pub struct Arg {
    pub ty: Ty,
    pub name: Ident,
    pub id: i32,
    pub tags: Arc<Tags>,
    pub attribute: FieldKind,
}

#[derive(Clone, Debug)]
pub struct ExceptionVariant {
    pub id: i32,
    pub ty: Ty,
}

#[derive(Clone, Debug)]
pub struct Method {
    pub name: Ident,
    pub args: Vec<Arg>,
    pub ret: Ty,
    pub oneway: bool,
    pub exceptions: Option<Path>,
    pub tags: Arc<Tags>,
}

#[derive(Clone, Debug)]
pub struct Service {
    pub name: Ident,
    pub methods: Vec<Method>,
    pub extend: Vec<Path>,
}

#[derive(Clone, Debug)]
pub struct Const {
    pub name: Ident,
    pub ty: Ty,
    pub lit: Literal,
}

#[derive(Clone, Debug)]
pub enum FieldKind {
    Required,
    Optional,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: Ident,
    pub id: i32,
    pub ty: Ty,
    pub kind: FieldKind,
    pub tags: Arc<Tags>,
    pub default: Option<Literal>,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub name: Ident,
    pub fields: Vec<Field>,
    pub is_wrapper: bool,
    pub extensions: Vec<Extension>,
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub id: Option<i32>,
    pub name: Ident,
    pub discr: Option<i64>,
    pub fields: Vec<Ty>,
    pub tags: Arc<Tags>,
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
    pub repr: Option<EnumRepr>,
}

#[derive(Clone, Debug)]
pub struct NewType {
    pub name: Ident,
    pub ty: Ty,
}

#[derive(Clone, Debug)]
pub struct Mod {
    pub name: Ident,
    pub items: Vec<Arc<Item>>,
    pub extensions: Vec<Extension>,
}

#[derive(Clone, Debug)]
pub enum ItemKind {
    Message(Message),
    Enum(Enum),
    Service(Service),
    NewType(NewType),
    Const(Const),
    Mod(Mod),
    Use(Use),
}

#[derive(Clone, Debug)]
pub struct Item {
    pub kind: ItemKind,
    pub related_items: Vec<Ident>,
    pub tags: Arc<Tags>,
}

impl Item {
    pub fn name(&self) -> Symbol {
        match &self.kind {
            ItemKind::Message(s) => (*s.name).clone(),
            ItemKind::Enum(e) => (*e.name).clone(),
            ItemKind::Service(s) => (*s.name).clone(),
            ItemKind::NewType(t) => (*t.name).clone(),
            ItemKind::Const(c) => (*c.name).clone(),
            ItemKind::Use(_) => panic!("there is no name for `Use`"),
            ItemKind::Mod(m) => (*m.name).clone(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub struct Use {
    pub file: FileId,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct Path {
    pub segments: Arc<[Ident]>,
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.segments.iter().map(|s| &**s).join("."))
    }
}

#[derive(Clone, Debug)]
pub struct File {
    pub package: Path,
    pub items: Vec<Arc<Item>>,
    pub id: FileId,
    pub uses: Vec<(Path, FileId)>,
    pub descriptor: Bytes,
    pub extensions: Vec<Extension>,
}
