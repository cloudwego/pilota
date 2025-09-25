use std::{ops::Deref, sync::Arc};

use super::{Annotations, Ident, Literal, Path, Type};

#[derive(Debug, Clone)]
pub enum ConstValue {
    Bool(bool),
    Path(Path),
    String(Literal),
    Int(IntConstant),
    Double(DoubleConstant),
    List(Vec<ConstValue>),
    Map(Vec<(ConstValue, ConstValue)>),
}

#[derive(Debug)]
pub struct Constant {
    pub comments: Arc<String>,
    pub name: Ident,
    pub r#type: Type,
    pub value: ConstValue,
    pub annotations: Annotations,
}

#[derive(Debug, Clone, Copy)]
pub struct IntConstant(pub i64);

impl Deref for IntConstant {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct DoubleConstant(pub Arc<str>);
