use chumsky::prelude::*;
use faststr::FastStr;

use super::super::{
    descriptor::{Enum, EnumValue},
    parser::*,
};
use crate::{Annotation, IntConstant};

impl EnumValue {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, EnumValue, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then(Ident::get_parser())
            .then(
                Components::blank()
                    .or_not()
                    .ignore_then(just("="))
                    .ignore_then(Components::blank().or_not())
                    .ignore_then(IntConstant::parse())
                    .or_not(),
            )
            .then(Annotation::get_parser().or_not())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(
                |((((comments, name), value), annotations), trailing_comments)| EnumValue {
                    leading_comments: FastStr::from(comments.join("\n\n")),
                    name: Ident(name.into()),
                    value,
                    annotations: annotations.unwrap_or_default(),
                    trailing_comments: trailing_comments.unwrap_or_default(),
                },
            )
    }
}

impl Enum {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Enum, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("enum"))
            .then_ignore(Components::blank_with_comments())
            .then(Ident::get_parser())
            .then(Components::comment().repeated().collect::<Vec<_>>())
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("{"))
            .then(EnumValue::get_parser().repeated().collect())
            .then(Components::comment().repeated().collect::<Vec<_>>())
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("}"))
            .then(Annotation::get_parser().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(
                |(
                    (((((leading, name), name_comments), values), comments), annotations),
                    trailing,
                )| Enum {
                    leading_comments: FastStr::from(format!(
                        "{}\n\n{}\n\n{}",
                        leading.join("\n\n"),
                        comments.join("\n\n"),
                        name_comments.join("\n\n"),
                    )),
                    name: Ident(name.into()),
                    values,
                    annotations: annotations.unwrap_or_default(),
                    trailing_comments: trailing.unwrap_or_default(),
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enum() {
        let _ = Enum::get_parser()
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
        let _ = Enum::get_parser()
            .parse(
                r#"enum Index {
                            A = 0x01,
                            B = 0x10,
                        }"#,
            )
            .unwrap();
    }

    #[test]
    fn test_enum3() {
        let _ = Enum::get_parser()
            .parse(
                r#"enum ERR {
    # From 133120 ~ 134143
}"#,
            )
            .unwrap();
    }
}
