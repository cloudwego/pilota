use super::{Annotations, Field, Ident, Type};

#[derive(Debug)]
pub struct Function {
    pub name: Ident,
    pub oneway: bool,
    pub result_type: Type,
    pub arguments: Vec<Field>,
    pub throws: Vec<Field>, // throws as an exception
    pub annotations: Annotations,
}
