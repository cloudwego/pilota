use super::Literal;

#[derive(Debug)]
pub struct Include {
    pub path: Literal,
}

#[derive(Debug)]
pub struct CppInclude(pub Literal);
