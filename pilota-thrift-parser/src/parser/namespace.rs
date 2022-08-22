use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use super::super::{descriptor::Annotations, parser::*};

#[derive(Debug, Clone)]
pub struct Namespace {
    pub scope: Scope,
    pub name: Path,
    pub annotations: Option<Annotations>,
}

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

#[derive(Debug, Clone)]
pub struct Scope(pub String);

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
                tag("py"),
                tag("py.twisted"),
                tag("rb"),
                tag("st"),
                tag("xsd"),
                tag("rs"),
            )),
            |s: &str| Scope(s.into()),
        )(input)
    }
}
