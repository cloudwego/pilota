use chumsky::prelude::*;

use super::super::{
    descriptor::{CppInclude, Include},
    parser::*,
};
use crate::Literal;

impl Include {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Include, extra::Err<Rich<'a, char>>> {
        just("include")
            .ignore_then(blank())
            .ignore_then(Literal::parse())
            .then_ignore(blank().or_not())
            .then_ignore(list_separator().or_not())
            .map(|path| Include { path })
    }
}

impl CppInclude {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, CppInclude, extra::Err<Rich<'a, char>>> {
        just("cpp_include")
            .ignore_then(blank())
            .ignore_then(Literal::parse())
            .then_ignore(blank().or_not())
            .then_ignore(list_separator().or_not())
            .map(CppInclude)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_include() {
        let _f = Include::get_parser()
            .parse(r#"include "shared.thrift""#)
            .unwrap();
    }

    #[test]
    fn test_cpp_include() {
        let _f = CppInclude::parse()
            .parse(r#"cpp_include "shared.thrift""#)
            .unwrap();
    }
}
