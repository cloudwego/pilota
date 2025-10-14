use chumsky::prelude::*;
use faststr::FastStr;

use super::super::{
    descriptor::{CppInclude, Include},
    parser::*,
};
use crate::Literal;

impl Include {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Include, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("include"))
            .then_ignore(Components::blank())
            .then(Literal::parse())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(|((comments, path), trailing_comments)| Include {
                leading_comments: FastStr::from(comments.join("\n\n")),
                path,
                trailing_comments: FastStr::from(trailing_comments.unwrap_or_default()),
            })
    }
}

impl CppInclude {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, CppInclude, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("cpp_include"))
            .then_ignore(Components::blank())
            .then(Literal::parse())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(|((comments, path), trailing_comments)| CppInclude {
                leading_comments: FastStr::from(comments.join("\n\n")),
                path,
                trailing_comments: FastStr::from(trailing_comments.unwrap_or_default()),
            })
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
