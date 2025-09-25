use std::sync::Arc;

use crate::{Annotations, Path};

#[derive(Debug, Clone)]
pub struct Scope(pub String);

#[derive(Debug, Clone)]
pub struct Namespace {
    pub comments: Arc<String>,
    pub scope: Scope,
    pub name: Path,
    pub annotations: Option<Annotations>,
}
