//! rust language descriptor and parser for thrift
//! powered by chumsky
//! idl descriptor: https://thrift.apache.org/docs/idl

mod annotation;
mod constant;
mod enum_;
pub mod error;
mod field;
mod function;
mod identifier;
mod include;
mod literal;
mod namespace;
mod service;
mod struct_;
pub mod thrift;
mod ty;
mod typedef;

use chumsky::prelude::*;
use faststr::FastStr;

use super::descriptor::{Components, Path};
use crate::Ident;

impl Path {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, Path, extra::Err<Rich<'a, char>>> {
        Components::blank()
            .ignore_then(Ident::get_parser())
            .separated_by(just('.'))
            .at_least(1)
            .collect()
            .then_ignore(Components::blank_without_newline())
            .map(|s: Vec<String>| {
                let idents: Vec<Ident> = s.into_iter().map(Ident::from).collect();
                Path {
                    segments: idents.into(),
                }
            })
    }
}

impl Components {
    pub fn list_separator<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
        Components::blank()
            .or_not()
            .ignore_then(one_of(",;"))
            .ignored()
    }

    pub fn blank<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
        one_of(" \t\r\n").repeated().ignored()
    }

    pub fn comment<'a>() -> impl Parser<'a, &'a str, FastStr, extra::Err<Rich<'a, char>>> {
        choice((
            just("//")
                .then(
                    any()
                        .and_is(just('\n').not())
                        .repeated()
                        .collect::<String>(),
                )
                .padded_by(Components::blank().or_not())
                .map(|(start, content)| FastStr::from(format!("{}{}", start, content))),
            just("#")
                .then(
                    any()
                        .and_is(just('\n').not())
                        .repeated()
                        .collect::<String>(),
                )
                .padded_by(Components::blank().or_not())
                .map(|(_, content)| FastStr::from(format!("//{}", content))),
            just("/*")
                .then(
                    any()
                        .and_is(just("*/").not())
                        .repeated()
                        .collect::<String>(),
                )
                .then(just("*/"))
                .padded_by(Components::blank().or_not())
                .map(|((start, content), end)| {
                    FastStr::from(format!("{}{}{}", start, content, end))
                }),
        ))
    }

    pub fn trailing_comment<'a>() -> impl Parser<'a, &'a str, FastStr, extra::Err<Rich<'a, char>>> {
        just(" ")
            .repeated()
            .ignored()
            .then(choice((
                just("//")
                    .then(
                        any()
                            .and_is(just('\n').not())
                            .repeated()
                            .collect::<String>(),
                    )
                    .then_ignore(Components::blank().or_not())
                    .map(|(start, content)| FastStr::from(format!("{}{}", start, content))),
                just("#")
                    .then(
                        any()
                            .and_is(just('\n').not())
                            .repeated()
                            .collect::<String>(),
                    )
                    .then_ignore(Components::blank().or_not())
                    .map(|(_, content)| FastStr::from(format!("//{}", content))),
                just("/*")
                    .then(
                        any()
                            .and_is(just("*/").not())
                            .repeated()
                            .collect::<String>(),
                    )
                    .then(just("*/"))
                    .then_ignore(Components::blank().or_not())
                    .map(|((start, content), end)| {
                        FastStr::from(format!("{}{}{}", start, content, end))
                    }),
            )))
            .map(|(_, c)| c)
    }

    pub fn blank_with_comments<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
        choice((
            just("//")
                .then(any().and_is(just('\n').not()).repeated())
                .ignored(),
            just("#")
                .then(any().and_is(just('\n').not()).repeated())
                .ignored(),
            just("/*")
                .then(any().and_is(just("*/").not()).repeated())
                .then(just("*/"))
                .ignored(),
            one_of(" \t\r\n").ignored(),
        ))
        .repeated()
        .ignored()
    }

    pub fn blank_without_newline<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
        one_of(" \t\r").repeated().ignored()
    }

    pub fn not_alphanumeric_or_underscore<'a>()
    -> impl Parser<'a, &'a str, char, extra::Err<Rich<'a, char>>> {
        any()
            .rewind()
            .filter(|c: &char| !c.is_alphanumeric() && *c != '_')
    }

    /// Succeeds without consuming input when the next character cannot continue
    /// an identifier, i.e. at an identifier boundary or at the end of input.
    pub fn word_boundary<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
        Components::not_alphanumeric_or_underscore()
            .ignored()
            .or(end())
    }

    /// A reserved word that must not be immediately followed by more identifier
    /// characters, so that `structFoo` is not read as `struct Foo`.
    pub fn keyword<'a>(kw: &'a str) -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
        just(kw).ignore_then(Components::word_boundary())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blank() {
        let _ = Components::blank().parse(" \t\r\n").unwrap();
    }

    #[test]
    fn test_path() {
        let p = Path::parse().parse("foo.bar.baz").unwrap();
        assert_eq!(p.segments.len(), 3);
        assert_eq!(p.segments[0].as_str(), "foo");
        assert_eq!(p.segments[1].as_str(), "bar");
        assert_eq!(p.segments[2].as_str(), "baz");

        let p = Path::parse().parse("foo").unwrap();
        assert_eq!(p.segments.len(), 1);
        assert_eq!(p.segments[0].as_str(), "foo");
    }

    #[test]
    fn test_comment() {
        let _ = Components::comment().parse("// foo").unwrap();
        let _ = Components::comment()
            .parse("# From 133120 ~ 134143\n")
            .unwrap();
        let _ = Components::comment().parse("/* foo */").unwrap();
    }

    #[test]
    fn test_trailing_comment() {
        let _ = Components::trailing_comment().parse(" // foo").unwrap();
        let _ = Components::trailing_comment().parse(" # foo").unwrap();
        let _ = Components::trailing_comment().parse(" /* foo */").unwrap();
    }

    #[test]
    fn test_blank_with_comments() {
        let _ = Components::blank_with_comments().parse(" // foo").unwrap();
        let _ = Components::blank_with_comments().parse(" # foo").unwrap();
        let _ = Components::blank_with_comments()
            .parse(" /* foo */")
            .unwrap();
    }

    #[test]
    fn test_blank_without_newline() {
        let _ = Components::blank_without_newline().parse(" \t\r").unwrap();
    }

    #[test]
    fn test_keyword() {
        // A keyword matches at an identifier boundary, including end of input,
        // but not when more identifier characters follow (`structFoo`).
        assert!(
            Components::keyword("struct")
                .parse("struct")
                .into_result()
                .is_ok()
        );
        assert!(
            Components::keyword("struct")
                .then_ignore(just("{"))
                .parse("struct{")
                .into_result()
                .is_ok()
        );
        assert!(
            Components::keyword("struct")
                .parse("structFoo")
                .has_errors()
        );
        assert!(Components::keyword("enum").parse("enumValue").has_errors());
    }
}
