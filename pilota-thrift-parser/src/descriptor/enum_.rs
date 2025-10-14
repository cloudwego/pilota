use faststr::FastStr;

pub use super::{Annotations, Ident, IntConstant};

#[derive(Debug)]
pub struct EnumValue {
    pub name: Ident,
    pub value: Option<IntConstant>,
    pub annotations: Annotations,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}

#[derive(Debug)]
pub struct Enum {
    pub name: Ident,
    pub values: Vec<EnumValue>,
    pub annotations: Annotations,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}
