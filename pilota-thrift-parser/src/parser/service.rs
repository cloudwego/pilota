use nom::{
    IResult,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
};

use super::super::{
    descriptor::{Annotations, Function, Ident, Service},
    parser::*,
};

impl Parser for Service {
    fn parse(input: &str) -> IResult<&str, Service> {
        map(
            tuple((
                tag("service"),
                blank,
                Ident::parse,
                opt(map(
                    tuple((blank, tag("extends"), blank, Path::parse)),
                    |(_, _, _, p)| p,
                )),
                opt(blank),
                tag("{"),
                many0(map(tuple((opt(blank), Function::parse)), |(_, f)| f)),
                opt(blank),
                tag("}"),
                opt(blank),
                opt(Annotations::parse),
                opt(list_separator),
            )),
            |(_, _, name, extends, _, _, functions, _, _, _, annotations, _)| Service {
                name,
                extends,
                functions,
                annotations: annotations.unwrap_or_default(),
            },
        )(input)
    }
}

#[test]
fn test_gen() {}
