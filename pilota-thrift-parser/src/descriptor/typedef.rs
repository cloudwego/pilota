use std::sync::Arc;

use super::{Annotations, Ident, Type};

#[derive(Debug)]
pub struct Typedef {
    pub comments: Arc<String>,
    pub r#type: Type,
    pub alias: Ident,
    pub annotations: Annotations,
}
