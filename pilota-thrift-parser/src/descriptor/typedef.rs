use faststr::FastStr;

use super::{Annotations, Ident, Type};

#[derive(Debug)]
pub struct Typedef {
    pub r#type: Type,
    pub alias: Ident,
    pub annotations: Annotations,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}
