use super::{Annotations, ConstValue, Ident, Type};

#[derive(Debug, Clone)]
pub enum Attribute {
    Optional,
    Required,
    Default,
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute::Default
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub id: i32,
    pub name: Ident,
    pub attribute: Attribute,
    pub ty: Type,
    pub default: Option<ConstValue>,
    pub annotations: Annotations,
}
