use faststr::FastStr;

use super::Literal;

#[derive(Debug)]
pub struct Include {
    pub path: Literal,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}

#[derive(Debug)]
pub struct CppInclude {
    pub path: Literal,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}
