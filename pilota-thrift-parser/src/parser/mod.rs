//! rust language descriptor and parser for thrift
//! powered by nom
//! idl descriptor: https://thrift.apache.org/docs/idl

mod annotation;
mod constant;
mod enum_;
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

use std::sync::Arc;

use chumsky::prelude::*;

use super::descriptor::Path;
use crate::Ident;

impl Path {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, Path, extra::Err<Rich<'a, char>>> {
        Ident::parse()
            .separated_by(just('.').padded_by(blank()))
            .at_least(1)
            .collect()
            .map(|s: Vec<String>| {
                let idents: Vec<Ident> = s.into_iter().map(Ident::from).collect();
                Path {
                    segments: idents.into(),
                }
            })
            .padded_by(blank())
    }
}

pub fn list_separator<'a>() -> impl Parser<'a, &'a str, char, extra::Err<Rich<'a, char>>> {
    one_of(",;").then(blank().or_not()).map(|(sep, _)| sep)
}

pub fn blank<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
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

pub fn not_alphanumeric_or_underscore<'a>()
-> impl Parser<'a, &'a str, char, extra::Err<Rich<'a, char>>> {
    any()
        .rewind()
        .filter(|c: &char| !c.is_alphanumeric() && *c != '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blank() {
        let _ = blank().parse(" \t\r\n").unwrap();
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
}
