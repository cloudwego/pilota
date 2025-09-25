use std::sync::Arc;

use super::Literal;

#[derive(Debug)]
pub struct Include {
    pub comments: Arc<String>,
    pub path: Literal,
}

#[derive(Debug)]
pub struct CppInclude {
    pub comments: Arc<String>,
    pub path: Literal,
}
