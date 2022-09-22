use std::ops::Deref;

use super::Literal;

#[derive(Debug, Clone)]
pub struct Annotation {
    pub key: String,
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
