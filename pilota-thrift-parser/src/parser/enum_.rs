use chumsky::prelude::*;

use super::super::{
    descriptor::{Enum, EnumValue},
    parser::*,
};
use crate::{Annotation, IntConstant};

impl EnumValue {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, EnumValue, extra::Err<Rich<'a, char>>> {
        Ident::parse()
            .padded_by(blank().or_not())
            .then(
                just("=")
                    .ignore_then(blank().or_not())
                    .ignore_then(IntConstant::parse())
                    .or_not(),
            )
            .then_ignore(blank().or_not())
            .then(Annotation::parse().or_not())
            .then_ignore(list_separator().or_not())
            .map(|((name, value), annotations)| EnumValue {
                name: Ident(name.into()),
                value,
                annotations: annotations.unwrap_or_default(),
            })
    }
}

impl Enum {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, Enum, extra::Err<Rich<'a, char>>> {
        just("enum")
            .ignore_then(blank())
            .ignore_then(Ident::parse())
            .then_ignore(blank().or_not())
            .then_ignore(just("{"))
            .then(EnumValue::parse().repeated().collect())
            .then_ignore(blank().or_not())
            .then_ignore(just("}"))
            .then_ignore(blank().or_not())
            .then(Annotation::parse().or_not())
            .map(|((name, values), annotations)| Enum {
                name: Ident(name.into()),
                values,
                annotations: annotations.unwrap_or_default(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enum() {
        let _ = Enum::parse()
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
        let _ = Enum::parse()
            .parse(
                r#"enum Index {
                            A = 0x01,
                            B = 0x10,
                        }"#,
            )
            .unwrap();
    }
}
