use std::sync::Arc;

pub use super::{Annotations, Ident, IntConstant};

#[derive(Debug)]
pub struct EnumValue {
    pub comments: Arc<String>,
    pub name: Ident,
    pub value: Option<IntConstant>,
    pub annotations: Annotations,
}

#[derive(Debug)]
pub struct Enum {
    pub comments: Arc<String>,
    pub name: Ident,
    pub values: Vec<EnumValue>,
    pub annotations: Annotations,
}
