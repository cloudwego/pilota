use chumsky::prelude::*;

use super::super::{
    descriptor::{Exception, Struct, StructLike, Union},
    parser::*,
};
use crate::{Annotation, Field, Ident};

impl Struct {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, Struct, extra::Err<Rich<'a, char>>> {
        just("struct")
            .ignore_then(blank())
            .ignore_then(StructLike::parse())
            .map(Struct)
    }
}

impl Union {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, Union, extra::Err<Rich<'a, char>>> {
        just("union")
            .ignore_then(blank())
            .ignore_then(StructLike::parse())
            .map(Union)
    }
}

impl Exception {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, Exception, extra::Err<Rich<'a, char>>> {
        just("exception")
            .ignore_then(blank())
            .ignore_then(StructLike::parse())
            .map(Exception)
    }
}

impl StructLike {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, StructLike, extra::Err<Rich<'a, char>>> {
        Ident::parse()
            .then_ignore(blank().or_not())
            .then_ignore(just("{"))
            .then(
                blank()
                    .ignore_then(Field::parse())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just("}").padded_by(blank().or_not()))
            .then(Annotation::parse().or_not())
            .then_ignore(list_separator().or_not())
            .map(|((name, fields), annotations)| StructLike {
                name: Ident(name.into()),
                fields,
                annotations: annotations.unwrap_or_default(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        let str = r#"struct MGetRequest {
            1: set<i64> Ids (go.tag = "json:\"Ids\" split:\"type=tenant\""),
            2: optional set<ExtendField> extendFields,


            3: optional set<ExtendField>extendFields2,
        
            255: base.Base Base,
        }
        "#;

        Struct::parse().parse(str).unwrap();
    }

    #[test]
    fn test_struct3() {
        let str = r#"struct TestComment {
            // 1
        }
        "#;
        Struct::parse().parse(str).unwrap();
    }

    #[test]
    fn test_tag() {
        let str = r#"struct ImMsgContent {
            1: string user_id (go.tag = 'json:\"user_id,omitempty\"'),
            2: string __files (go.tag = 'json:\"__files,omitempty\"'),
        }"#;
        Struct::parse().parse(str).unwrap();
    }

    #[test]
    fn test_ty() {
        let str = r#"struct Test {
            1: required string(pilota.annotation="test") Service,      // required service
            2: required bytet_i.Injection Injection,
        }"#;
        Struct::parse().parse(str).unwrap();
    }
}
