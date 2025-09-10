use super::super::{
    descriptor::{CppInclude, Include},
    parser::*,
};
use chumsky::prelude::*;

pub fn include<'a>() -> impl Parser<'a, &'a str, Include, extra::Err<Rich<'a, char>>> {
    just("include")
        .ignore_then(blank())
        .ignore_then(literal::parse())
        .then_ignore(blank().or_not())
        .then_ignore(list_separator().or_not())
        .map(|path| Include { path })
}

pub fn cpp_include<'a>() -> impl Parser<'a, &'a str, CppInclude, extra::Err<Rich<'a, char>>> {
    just("cpp_include")
        .ignore_then(blank())
        .ignore_then(literal::parse())
        .then_ignore(blank().or_not())
        .then_ignore(list_separator().or_not())
        .map(|path| CppInclude(path))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_include() {
        let _f = include().parse(r#"include "shared.thrift""#).unwrap();
    }

    #[test]
    fn test_cpp_include() {
        let _f = cpp_include().parse(r#"cpp_include "shared.thrift""#).unwrap();
    }
}
