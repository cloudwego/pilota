use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    sequence::tuple,
};

use super::super::{
    descriptor::{Annotations, Attribute, ConstValue, Field, Ident, Type},
    parser::*,
};

impl Parser for Attribute {
    fn parse(input: &str) -> IResult<&str, Attribute> {
        alt((
            map(tag("required"), |_| Attribute::Required),
            map(tag("optional"), |_| Attribute::Optional),
        ))(input)
    }
}

impl Parser for Field {
    fn parse(input: &str) -> IResult<&str, Field> {
        // 1: required i32 name = 123;
        map(
            tuple((
                collect_comments,
                map(tuple((digit1, opt(blank), tag(":"))), |(id, _, _)| {
                    id.parse::<i32>().unwrap()
                }),
                opt(blank),
                opt(Attribute::parse),
                opt(blank),
                Type::parse,
                opt(blank),
                Ident::parse,
                opt(blank),
                opt(map(
                    tuple((tag("="), opt(blank), ConstValue::parse)),
                    |(_, _, default)| default,
                )),
                opt(blank),
                opt(Annotations::parse),
                opt(blank),
                opt(list_separator),
            )),
            |(comments, id, _, attribute, _, r#type, _, name, _, default, _, annotations, _, _)| Field {
                id,
                attribute: attribute.unwrap_or_default(),
                ty: r#type,
                name,
                default,
                annotations: annotations.unwrap_or_default(),
                comments,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_field() {
        let _f = Field::parse(
            r#"1: required string(foo="1", bar='2') LogID = "xxx" (foo = '1', bar="2"),"#,
        )
        .unwrap()
        .1;
    }

    #[test]
    fn test_field2() {
        let _f =
            Field::parse(r#"1: set<i64> Ids (go.tag = "json:\"Ids\" split:\"type=tenant\""),"#)
                .unwrap();
    }
}
