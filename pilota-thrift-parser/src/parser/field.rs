use chumsky::prelude::*;
use faststr::FastStr;

use super::super::{
    descriptor::{Attribute, Field},
    parser::*,
};
use crate::{Annotation, ConstValue, IntConstant, Type};

impl Attribute {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Attribute, extra::Err<Rich<'a, char>>> {
        choice((
            Components::keyword("required").to(Attribute::Required),
            Components::keyword("optional").to(Attribute::Optional),
        ))
    }
}

impl Field {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Field, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then(IntConstant::parse().try_map(|id, span| {
                i16::try_from(id.0)
                    .map(i32::from)
                    .map_err(|_| Rich::custom(span, "field id does not fit in an i16"))
            }))
            .then_ignore(just(":").padded_by(Components::blank_with_comments().or_not()))
            .then(Attribute::get_parser().or_not())
            .then(Type::get_parser().padded_by(Components::blank_with_comments().or_not()))
            .then(Ident::get_parser().padded_by(Components::blank_with_comments().or_not()))
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
                    id,
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

    #[test]
    fn test_field_comment() {
        let input = r#"
        /* comment */ 1: /* comment */ required /* comment */ string /* comment */ LogID = /* comment */ "xxx" // comment
        "#;
        let _f = Field::get_parser().parse(input).unwrap();
    }

    /// Field ids are an `i16` on the wire; anything outside that range would be
    /// silently truncated by codegen, so it must be a parse error, not a panic.
    #[test]
    fn test_field_id_range() {
        assert_eq!(Field::get_parser().parse("-1: i32 a").unwrap().id, -1);
        assert_eq!(
            Field::get_parser().parse("32767: i32 a").unwrap().id,
            i16::MAX as i32
        );
        assert!(Field::get_parser().parse("32768: i32 a").has_errors());
        assert!(Field::get_parser().parse("-32769: i32 a").has_errors());
        assert!(Field::get_parser().parse("99999999999: i32 a").has_errors());
    }

    /// A type merely starting with `required`/`optional` is a plain type, so
    /// the field keeps the default requiredness.
    #[test]
    fn test_requiredness_keyword_boundary() {
        let f = Field::get_parser().parse("1: requiredThing a").unwrap();
        assert_eq!(f.attribute, crate::Attribute::Default);
        assert!(matches!(&f.ty.0, crate::Ty::Path(p) if p.segments[0].as_str() == "requiredThing"));

        let f = Field::get_parser().parse("1: optionalThing a").unwrap();
        assert_eq!(f.attribute, crate::Attribute::Default);
    }
}
