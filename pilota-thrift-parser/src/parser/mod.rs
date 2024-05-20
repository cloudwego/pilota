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
mod thrift;
mod ty;
mod typedef;

use std::sync::Arc;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{multispace1, one_of, satisfy},
    combinator::{map, opt},
    multi::{many0, many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use super::descriptor::{Ident, Path};

/// combinator for parsing thrift idl
pub trait Parser: Sized {
    /// parse from input idl
    fn parse(input: &str) -> IResult<&str, Self>;
}

impl Parser for Path {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(tuple((opt(blank), tag("."), opt(blank))), Ident::parse),
            |idents| Path {
                segments: Arc::from(idents),
            },
        )(input)
    }
}

pub(crate) fn list_separator(input: &str) -> IResult<&str, char> {
    one_of(",;")(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("//"), take_till(|c| c == '\n')),
        preceded(tag("/*"), terminated(take_until("*/"), tag("*/"))),
        preceded(tag("#"), take_till(|c| c == '\n')),
    ))(input)
}

pub(crate) fn blank(input: &str) -> IResult<&str, ()> {
    map(many1(alt((comment, multispace1))), |_| ())(input)
}

pub(crate) fn alphanumeric_or_underscore(input: &str) -> IResult<&str, char> {
    satisfy(|c: char| c.is_alphanumeric() || c == '_')(input)
}
