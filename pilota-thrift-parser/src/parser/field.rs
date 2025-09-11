use chumsky::prelude::*;

use super::super::{
    descriptor::{Attribute, Field},
    parser::*,
};

pub fn attribute<'a>() -> impl Parser<'a, &'a str, Attribute, extra::Err<Rich<'a, char>>> {
    choice((
        just("required").to(Attribute::Required),
        just("optional").to(Attribute::Optional),
    ))
}

pub fn parse<'a>() -> impl Parser<'a, &'a str, Field, extra::Err<Rich<'a, char>>> {
    // 1: required i32 name = 123;
    text::int(10)
        .then_ignore(just(":").padded_by(blank().or_not()))
        .then(attribute().or_not())
        .then(ty::r#type().padded_by(blank().or_not()))
        .then(identifier::parse())
        .then(
            just("=")
                .padded_by(blank().or_not())
                .ignore_then(constant::const_value())
                .or_not(),
        )
        .then(annotation::parse().or_not().padded_by(blank().or_not()))
        .then_ignore(list_separator().or_not())
        .map(
            |(((((id, attribute), r#type), name), value), annotations)| Field {
                id: id.parse().unwrap(),
                attribute: attribute.unwrap_or_default(),
                ty: r#type,
                name: Ident(Arc::from(name)),
                default: value,
                annotations: annotations.unwrap_or_default(),
            },
        )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_field() {
        let _f = field::parse()
            .parse(r#"1: required string(foo="1", bar='2') LogID = "xxx" (foo = '1', bar="2"),"#)
            .unwrap();
    }

    #[test]
    fn test_field2() {
        let _f = field::parse()
            .parse(r#"1: set<i64> Ids (go.tag = "json:\"Ids\" split:\"type=tenant\""),"#)
            .unwrap();
    }

    #[test]
    fn test_field3() {
        let _f = field::parse()
            .parse(r#"2: required bytet_i.Injection Injection,"#)
            .unwrap();
    }
}
