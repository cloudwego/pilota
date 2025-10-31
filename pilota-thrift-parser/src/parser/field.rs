use chumsky::prelude::*;
use faststr::FastStr;

use super::super::{
    descriptor::{Attribute, Field},
    parser::*,
};
use crate::{Annotation, ConstValue, Type};

impl Attribute {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Attribute, extra::Err<Rich<'a, char>>> {
        choice((
            just("required").to(Attribute::Required),
            just("optional").to(Attribute::Optional),
        ))
    }
}

impl Field {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Field, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then(text::int(10))
            .then_ignore(just(":").padded_by(Components::blank_with_comments().or_not()))
            .then(Attribute::get_parser().or_not())
            .then(Type::get_parser().padded_by(Components::blank_with_comments().or_not()))
            .then(Ident::get_parser())
            .then(
                just("=")
                    .padded_by(Components::blank_with_comments().or_not())
                    .ignore_then(ConstValue::get_parser())
                    .or_not(),
            )
            .then(Annotation::get_parser().or_not())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(
                |(
                    ((((((comments, id), attribute), r#type), name), value), annotations),
                    trailing_comments,
                )| Field {
                    leading_comments: FastStr::from(comments.join("\n\n")),
                    id: id.parse().unwrap(),
                    attribute: attribute.unwrap_or_default(),
                    ty: r#type,
                    name: Ident(name.into()),
                    default: value,
                    annotations: annotations.unwrap_or_default(),
                    trailing_comments: trailing_comments.unwrap_or_default(),
                },
            )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_field() {
        let _f = Field::get_parser()
            .parse(r#"1: required string(foo="1", bar='2') LogID = "xxx" (foo = '1', bar="2"),"#)
            .unwrap();
    }

    #[test]
    fn test_field2() {
        let _f = Field::get_parser()
            .parse(r#"1: set<i64> Ids (go.tag = "json:\"Ids\" split:\"type=tenant\""),"#)
            .unwrap();
    }

    #[test]
    fn test_field3() {
        let _f = Field::get_parser()
            .parse(r#"2: required bytet_i.Injection Injection,"#)
            .unwrap();
    }
}
