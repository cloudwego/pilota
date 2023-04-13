use super::{Annotations, ConstValue, Ident, Type};

#[derive(Debug, Clone, Default)]
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
}
