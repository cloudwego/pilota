use faststr::FastStr;

use super::{Annotations, ConstValue, Ident, Type};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Attribute {
    Optional,
    Required,
    #[default]
    Default,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub id: i32,
    pub name: Ident,
    pub attribute: Attribute,
    pub ty: Type,
    pub default: Option<ConstValue>,
    pub annotations: Annotations,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}
