use std::ops::Deref;

use super::{Literal, Path};

#[derive(Debug, Clone)]
pub struct Annotation {
    pub key: Path,
    pub value: Literal,
}

impl Deref for Annotations {
    type Target = Vec<Annotation>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Annotations(pub Vec<Annotation>);
