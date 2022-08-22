use super::{Annotations, Function, Ident, Path};

#[derive(Debug)]
pub struct Service {
    pub name: Ident,
    pub extends: Option<Path>,
    pub functions: Vec<Function>,
    pub annotations: Annotations,
}
