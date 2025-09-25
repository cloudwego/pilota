use std::sync::Arc;

use super::{Annotations, Field, Ident, Type};

#[derive(Debug)]
pub struct Function {
    pub comments: Arc<String>,
    pub name: Ident,
    pub oneway: bool,
    pub result_type: Type,
    pub arguments: Vec<Field>,
    pub throws: Vec<Field>, // throws as an exception
    pub annotations: Annotations,
}
