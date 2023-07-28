use std::{ops::Deref, sync::Arc};

use super::ty::Ty;
use crate::{
    symbol::{DefId, EnumRepr, FileId, Ident, Symbol},
    tags::TagId,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal {
    Path(Path),
    Bool(bool),
    String(Arc<str>),
    Int(i64),
    Float(Arc<str>),
    List(Vec<Literal>),
    Map(Vec<(Literal, Literal)>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Arg {
    pub ty: Ty,
    pub def_id: DefId,
    pub name: Ident,
    pub id: i32,
    pub tags_id: TagId,
}

#[derive(Clone, Debug)]
pub struct ExceptionVariant {
    pub id: i32,
    pub ty: Ty,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MethodSource {
    Extend(/* Service DefId */ DefId),
    Own,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Method {
    pub def_id: DefId,
    pub name: Ident,
    pub args: Vec<Arc<Arg>>,
    pub ret: Ty,
    pub oneway: bool,
    pub exceptions: Option<Path>,
    pub source: MethodSource,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Service {
    pub name: Ident,
    pub methods: Vec<Arc<Method>>,
    pub extend: Vec<Path>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Const {
    pub name: Ident,
    pub ty: Ty,
    pub lit: Literal,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum FieldKind {
    Required,
    Optional,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Field {
    pub did: DefId,
    pub name: Ident,
    pub id: i32,
    pub ty: Ty,
    pub kind: FieldKind,
    pub tags_id: TagId,
    pub default: Option<Literal>,
}

impl Field {
    pub fn is_optional(&self) -> bool {
        matches!(self.kind, FieldKind::Optional)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Message {
    pub name: Ident,
    pub fields: Vec<Arc<Field>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnumVariant {
    pub id: Option<i32>,
    pub did: DefId,
    pub name: Ident,
    pub discr: Option<i64>,
    pub fields: Vec<Ty>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Enum {
    pub name: Ident,
    pub variants: Vec<Arc<EnumVariant>>,
    pub repr: Option<EnumRepr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NewType {
    pub name: Ident,
    pub ty: Ty,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mod {
    pub name: Ident,
    pub items: Vec<DefId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Item {
    Message(Message),
    Enum(Enum),
    Service(Service),
    NewType(NewType),
    Const(Const),
    Mod(Mod),
}

impl Item {
    pub fn symbol_name(&self) -> Symbol {
        match self {
            Item::Message(s) => (*s.name).clone(),
            Item::Enum(e) => (*e.name).clone(),
            Item::Service(s) => (*s.name).clone(),
            Item::NewType(t) => (*t.name).clone(),
            Item::Const(c) => (*c.name).clone(),
            Item::Mod(m) => (*m.name).clone(),
        }
    }

    pub fn is_ty(&self) -> bool {
        matches!(
            self,
            Item::Message(_) | Item::Enum(_) | Item::Service(_) | Item::NewType(_)
        )
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum DefKind {
    Type,
    Value,
    Mod,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Path {
    pub kind: DefKind,
    pub did: DefId,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash, PartialOrd, Ord)]
pub struct ItemPath(Arc<[Symbol]>);

impl Deref for ItemPath {
    type Target = [Symbol];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for ItemPath
where
    T: Into<Arc<[Symbol]>>,
{
    fn from(t: T) -> Self {
        ItemPath(t.into())
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct File {
    pub package: ItemPath,
    pub items: Vec<DefId>,
    pub file_id: FileId,
    pub uses: Vec<FileId>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NodeKind {
    Item(Arc<Item>),
    Variant(Arc<EnumVariant>),
    Field(Arc<Field>),
    Method(Arc<Method>),
    Arg(Arc<Arg>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub file_id: FileId,
    pub kind: NodeKind,
    pub parent: Option<DefId>,
    pub tags: TagId,
    pub related_nodes: Vec<DefId>,
}

impl Node {
    pub(crate) fn expect_item(&self) -> &Item {
        match &self.kind {
            NodeKind::Item(item) => item,
            _ => panic!(),
        }
    }

    pub(crate) fn name(&self) -> Symbol {
        match &self.kind {
            NodeKind::Item(item) => item.symbol_name(),
            NodeKind::Variant(v) => v.name.sym.clone(),
            NodeKind::Field(f) => f.name.sym.clone(),
            NodeKind::Method(m) => m.name.sym.clone(),
            NodeKind::Arg(a) => a.name.sym.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pkg {
    pub path: ItemPath,
    pub items: Vec<DefId>,
}
