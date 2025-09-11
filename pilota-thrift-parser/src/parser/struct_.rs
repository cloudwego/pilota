use chumsky::prelude::*;

use crate::Ident;

use super::super::{
    descriptor::{Exception, Struct, StructLike, Union},
    parser::*,
};

pub fn r#struct<'a>() -> impl Parser<'a, &'a str, Struct, extra::Err<Rich<'a, char>>> {
    just("struct")
        .ignore_then(blank())
        .ignore_then(struct_like())
        .map(Struct)
}

pub fn union<'a>() -> impl Parser<'a, &'a str, Union, extra::Err<Rich<'a, char>>> {
    just("union")
        .ignore_then(blank())
        .ignore_then(struct_like())
        .map(Union)
}

pub fn exception<'a>() -> impl Parser<'a, &'a str, Exception, extra::Err<Rich<'a, char>>> {
    just("exception")
        .ignore_then(blank())
        .ignore_then(struct_like())
        .map(Exception)
}

pub fn struct_like<'a>() -> impl Parser<'a, &'a str, StructLike, extra::Err<Rich<'a, char>>> {
    identifier::parse()
        .then_ignore(blank().or_not())
        .then_ignore(just("{"))
        .then(
            blank()
                .ignore_then(field::parse())
                .repeated()
                .collect::<Vec<_>>(),
        )
        .then_ignore(just("}").padded_by(blank().or_not()))
        .then(annotation::parse().or_not())
        .then_ignore(list_separator().or_not())
        .map(|((name, fields), annotations)| StructLike {
            name: Ident(Arc::from(name)),
            fields,
            annotations: annotations.unwrap_or_default(),
        })
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

        r#struct().parse(str).unwrap();
    }

    #[test]
    fn test_struct3() {
        let str = r#"struct TestComment {
            // 1
        }
        "#;
        r#struct().parse(str).unwrap();
    }

    #[test]
    fn test_tag() {
        let str = r#"struct ImMsgContent {
            1: string user_id (go.tag = 'json:\"user_id,omitempty\"'),
            2: string __files (go.tag = 'json:\"__files,omitempty\"'),
        }"#;
        r#struct().parse(str).unwrap();
    }

    #[test]
    fn test_ty() {
        let str = r#"struct Test {
            1: required string(pilota.annotation="test") Service,      // required service
            2: required bytet_i.Injection Injection,
        }"#;
        r#struct().parse(str).unwrap();
    }
}
