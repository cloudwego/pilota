use faststr::FastStr;

use crate::{Annotations, Path};

#[derive(Debug, Clone)]
pub struct Scope(pub String);

#[derive(Debug, Clone)]
pub struct Namespace {
    pub scope: Scope,
    pub name: Path,
    pub annotations: Option<Annotations>,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}
