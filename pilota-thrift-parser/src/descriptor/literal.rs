use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Literal(pub String);

impl Deref for Literal {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}
