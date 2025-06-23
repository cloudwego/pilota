use nom::{
    IResult,
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
            nom::bytes::complete::take_while1(|c: char| !c.is_whitespace()),
            |s: &str| Scope(s.into()),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope() {
        let input = "cocoa ";
        let (remaining, scope) = Scope::parse(input).unwrap();
        assert_eq!(scope.0, "cocoa");
        assert_eq!(remaining, " ");

        let input = "*";
        let (_, scope) = Scope::parse(input).unwrap();
        assert_eq!(scope.0, "*");

        let input = "py.twisted";
        let (_, scope) = Scope::parse(input).unwrap();
        assert_eq!(scope.0, "py.twisted");

        let input = "  ";
        let res = Scope::parse(input);
        assert!(res.is_err());
    }

    #[test]
    fn test_namespace() {
        let input = "namespace cocoa java.lang.Object";
        let (_, namespace) = Namespace::parse(input).unwrap();
        assert_eq!(namespace.scope.0, "cocoa");
        assert_eq!(
            namespace
                .name
                .segments
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            vec!["java", "lang", "Object"]
        );
    }
}
