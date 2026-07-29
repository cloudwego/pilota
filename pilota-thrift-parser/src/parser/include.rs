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
            .then_ignore(Components::keyword("include").padded_by(Components::blank().or_not()))
            .then(Literal::parse())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(|((comments, path), trailing_comments)| Include {
                leading_comments: FastStr::from(comments.join("\n\n")),
                path,
                trailing_comments: trailing_comments.unwrap_or_default(),
            })
    }
}

impl CppInclude {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, CppInclude, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(Components::keyword("cpp_include").padded_by(Components::blank().or_not()))
            .then(Literal::parse())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(|((comments, path), trailing_comments)| CppInclude {
                leading_comments: FastStr::from(comments.join("\n\n")),
                path,
                trailing_comments: trailing_comments.unwrap_or_default(),
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

    #[test]
    fn test_include_comment() {
        let _f = Include::get_parser()
            .parse(
                r#"// comment
                        include "shared.thrift" // comment"#,
            )
            .unwrap();
    }

    #[test]
    fn test_cpp_include_comment() {
        let _f = CppInclude::parse()
            .parse(
                r#"// comment
                        cpp_include "shared.thrift" // comment"#,
            )
            .unwrap();
    }

    /// `include`/`cpp_include` must end at an identifier boundary, so
    /// `includes` is not read as `include` followed by `s`. A quote is a
    /// boundary, so the separating blank stays optional.
    #[test]
    fn test_include_keyword_boundary() {
        assert!(
            Include::get_parser()
                .parse(r#"includes "shared.thrift""#)
                .has_errors()
        );
        assert!(
            CppInclude::parse()
                .parse(r#"cpp_includes "shared.thrift""#)
                .has_errors()
        );

        let f = Include::get_parser()
            .parse(r#"include"shared.thrift""#)
            .unwrap();
        assert_eq!(f.path.0.as_str(), "shared.thrift");
        let f = CppInclude::parse()
            .parse(r#"cpp_include"shared.thrift""#)
            .unwrap();
        assert_eq!(f.path.0.as_str(), "shared.thrift");
    }
}
