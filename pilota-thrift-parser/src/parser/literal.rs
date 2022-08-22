use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::none_of,
    combinator::map,
    sequence::delimited,
    IResult,
};

use super::super::{descriptor::Literal, parser::*};

macro_rules! gen_parse_quote {
    ($func_name: ident, $char: tt) => {
        fn $func_name(input: &str) -> IResult<&str, &str> {
            let esc = escaped(none_of(concat!("\\", $char)), '\\', one_of(r#"'"n\"#));
            let esc_or_empty = alt((esc, tag("")));
            let res = delimited(tag($char), esc_or_empty, tag($char))(input)?;

            Ok(res)
        }
    };
}

gen_parse_quote!(single_quote, "\'");
gen_parse_quote!(double_quote, "\"");

impl Parser for Literal {
    fn parse(input: &str) -> IResult<&str, Literal> {
        alt((
            map(single_quote, |x| Literal(x.into())),
            map(double_quote, |x| Literal(x.into())),
        ))(input)
    }
}
