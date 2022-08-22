use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use super::super::{
    descriptor::{CppInclude, Include, Literal},
    parser::*,
};

impl Parser for Include {
    fn parse(input: &str) -> IResult<&str, Include> {
        map(
            tuple((tag("include"), blank, Literal::parse, opt(list_separator))),
            |(_, _, path, _)| Include { path },
        )(input)
    }
}

impl Parser for CppInclude {
    fn parse(input: &str) -> IResult<&str, CppInclude> {
        map(
            tuple((
                tag("cpp_include"),
                blank,
                Literal::parse,
                opt(list_separator),
            )),
            |(_, _, path, _)| CppInclude(path),
        )(input)
    }
}
