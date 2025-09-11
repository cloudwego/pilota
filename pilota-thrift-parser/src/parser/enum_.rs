use chumsky::prelude::*;

use crate::parser::constant::int_constant;

use super::super::{
    descriptor::{Enum, EnumValue},
    parser::*,
};

pub fn enum_value<'a>() -> impl Parser<'a, &'a str, EnumValue, extra::Err<Rich<'a, char>>> {
    identifier::parse()
        .padded_by(blank().or_not())
        .then(
            just("=")
                .ignore_then(blank().or_not())
                .ignore_then(int_constant())
                .or_not(),
        )
        .then_ignore(blank().or_not())
        .then(annotation::parse().or_not())
        .then_ignore(list_separator().or_not())
        .map(|((name, value), annotations)| EnumValue {
            name: Ident(Arc::from(name)),
            value,
            annotations: annotations.unwrap_or_default(),
        })
}

pub fn parse<'a>() -> impl Parser<'a, &'a str, Enum, extra::Err<Rich<'a, char>>> {
    just("enum")
        .ignore_then(blank())
        .ignore_then(identifier::parse())
        .then_ignore(blank().or_not())
        .then_ignore(just("{"))
        .then(enum_value().repeated().collect())
        .then_ignore(blank().or_not())
        .then_ignore(just("}"))
        .then_ignore(blank().or_not())
        .then(annotation::parse().or_not())
        .map(|((name, values), annotations)| Enum {
            name: Ident(Arc::from(name)),
            values,
            annotations: annotations.unwrap_or_default(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enum() {
        let _ = enum_::parse()
            .parse(
                r#"enum Sex {
                            UNKNOWN = 0,
                            MALE = 1 (pilota.key="male") // male
                            FEMALE = 2,
                        }"#,
            )
            .unwrap();
    }

    #[test]
    fn test_enum2() {
        let _ = enum_::parse()
            .parse(
                r#"enum Index {
                            A = 0x01,
                            B = 0x10,
                        }"#,
            )
            .unwrap();
    }
}
