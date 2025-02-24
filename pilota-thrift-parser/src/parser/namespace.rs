use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

use super::super::{descriptor::Annotations, parser::*};
use crate::{Namespace, Scope};

impl Parser for Namespace {
    fn parse(input: &str) -> IResult<&str, Namespace> {
        map(
            tuple((
                tag("namespace"),
                preceded(blank, Scope::parse),
                preceded(blank, Path::parse),
                opt(blank),
                opt(Annotations::parse),
                opt(blank),
                opt(list_separator),
            )),
            |(_, scope, name, _, annotations, _, _)| Namespace {
                scope,
                name,
                annotations,
            },
        )(input)
    }
}

impl Parser for Scope {
    fn parse(input: &str) -> IResult<&str, Scope> {
        map(
            alt((
                tag("*"),
                tag("c_glib"),
                tag("cpp"),
                tag("delphi"),
                tag("haxe"),
                tag("go"),
                tag("java"),
                tag("js"),
                tag("lua"),
                tag("netstd"),
                tag("perl"),
                tag("php"),
                tag("py.twisted"),
                tag("py"),
                tag("rb"),
                tag("st"),
                tag("xsd"),
                tag("rs"),
            )),
            |s: &str| Scope(s.into()),
        )(input)
    }
}
