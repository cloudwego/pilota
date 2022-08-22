use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use super::super::{
    descriptor::{Annotations, Ident, Type, Typedef},
    parser::*,
};

impl Parser for Typedef {
    fn parse(input: &str) -> IResult<&str, Typedef> {
        map(
            tuple((
                tag("typedef"),
                blank,
                Type::parse,
                blank,
                Ident::parse,
                opt(blank),
                opt(Annotations::parse),
            )),
            |(_, _, r#type, _, alias, _, annotations)| Typedef {
                r#type,
                alias,
                annotations,
            },
        )(input)
    }
}
