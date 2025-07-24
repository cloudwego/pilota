use nom::{
    IResult,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
};

use super::super::{
    descriptor::{Annotations, Enum, EnumValue, Ident, IntConstant},
    parser::*,
};

impl Parser for EnumValue {
    fn parse(input: &str) -> IResult<&str, EnumValue> {
        map(
            tuple((
                // Collect comments before the enum value
                opt(collect_comments),
                Ident::parse,
                opt(blank),
                opt(map(
                    tuple((tag("="), opt(blank), IntConstant::parse)),
                    |(_, _, value)| value,
                )),
                opt(blank),
                opt(Annotations::parse),
                opt(list_separator),
                opt(blank),
            )),
            |(leading_comments, name, _, value, _, annotations, _, _)| EnumValue {
                name,
                value,
                annotations: annotations.unwrap_or_default(),
                comments: leading_comments
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            },
        )(input)
    }
}

impl Parser for Enum {
    fn parse(input: &str) -> IResult<&str, Enum> {
        map(
            tuple((
                // Collect comments before the enum
                opt(collect_comments),
                tag("enum"),
                blank,
                Ident::parse,
                opt(blank),
                tag("{"),
                opt(blank),
                many0(EnumValue::parse),
                opt(blank),
                tag("}"),
                opt(blank),
                opt(Annotations::parse),
            )),
            |(leading_comments, _, _, name, _, _, _, values, _, _, _, annotations)| Enum {
                name,
                values,
                annotations: annotations.unwrap_or_default(),
                comments: leading_comments
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enum() {
        let (_remain, _e) = Enum::parse(
            r#"enum Sex {
                UNKNOWN = 0,
                MALE = 1 (pilota.key="male") // male
                FEMALE = 2,
            }"#,
        )
        .unwrap();
    }
}
