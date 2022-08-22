use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};

use super::super::{
    descriptor::{Annotations, Field, Function, Ident, Type},
    parser::{blank, list_separator, Parser},
    Attribute,
};

impl Parser for Function {
    fn parse(input: &str) -> IResult<&str, Function> {
        map(
            tuple((
                map(opt(tuple((tag("oneway"), blank))), |x| x.is_some()),
                Type::parse,
                blank,
                Ident::parse,
                opt(blank),
                tag("("),
                opt(many1(map(
                    tuple((opt(blank), Field::parse)),
                    |(_, field)| field,
                ))),
                opt(blank),
                tag(")"),
                opt(blank),
                opt(map(
                    tuple((
                        tag("throws"),
                        opt(blank),
                        tag("("),
                        many1(map(tuple((opt(blank), Field::parse)), |(_, field)| field)),
                        opt(blank),
                        tag(")"),
                    )),
                    |(_, _, _, fields, _, _)| fields,
                )),
                opt(blank),
                opt(Annotations::parse),
                opt(list_separator),
            )),
            |(oneway, r#type, _, name, _, _, arguments, _, _, _, throws, _, annotations, _)| {
                let mut args = arguments.unwrap_or_default();
                args.iter_mut().for_each(|f| {
                    f.attribute = Attribute::Required;
                });
                Function {
                    name,
                    oneway,
                    result_type: r#type,
                    arguments: args,
                    throws: throws.unwrap_or_default(),
                    annotations: annotations.unwrap_or_default(),
                }
            },
        )(input)
    }
}
