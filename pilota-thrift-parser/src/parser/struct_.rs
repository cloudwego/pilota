use chumsky::prelude::*;
use faststr::FastStr;

use super::super::{
    descriptor::{Exception, Struct, StructLike, Union},
    parser::*,
};
use crate::{Annotation, Field, Ident};

impl Struct {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Struct, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("struct"))
            .then_ignore(Components::blank())
            .then(StructLike::parse())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(|((comments, struct_like), trailing_comments)| {
                let leading_comments = FastStr::from(format!(
                    "{}\n\n{}",
                    comments.join("\n\n"),
                    struct_like.comments
                ));
                Struct {
                    leading_comments,
                    struct_like,
                    trailing_comments: FastStr::from(trailing_comments.unwrap_or_default()),
                }
            })
    }
}

impl Union {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, Union, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("union"))
            .then_ignore(Components::blank())
            .then(StructLike::parse())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(|((comments, struct_like), trailing_comments)| Union {
                leading_comments: FastStr::from(comments.join("\n\n")),
                struct_like,
                trailing_comments: FastStr::from(trailing_comments.unwrap_or_default()),
            })
    }
}

impl Exception {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, Exception, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("exception"))
            .then_ignore(Components::blank())
            .then(StructLike::parse())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(|((comments, struct_like), trailing_comments)| Exception {
                leading_comments: FastStr::from(comments.join("\n\n")),
                struct_like,
                trailing_comments: FastStr::from(trailing_comments.unwrap_or_default()),
            })
    }
}

impl StructLike {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, StructLike, extra::Err<Rich<'a, char>>> {
        Ident::get_parser()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("{"))
            .then(
                Components::blank()
                    .ignore_then(Field::get_parser())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .then(Components::comment().repeated().collect::<Vec<_>>())
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("}"))
            .then(Annotation::get_parser().or_not())
            .then_ignore(Components::list_separator().or_not())
            .map(|(((name, fields), comments), annotations)| StructLike {
                name: Ident(name.into()),
                fields,
                annotations: annotations.unwrap_or_default(),
                comments: FastStr::from(comments.join("\n\n")),
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

        Struct::get_parser().parse(str).unwrap();
    }

    #[test]
    fn test_struct3() {
        let str = r#"struct TestComment {
            // 1
        }
        "#;
        Struct::get_parser().parse(str).unwrap();
    }

    #[test]
    fn test_tag() {
        let str = r#"
        struct ImMsgContent {
            1: string user_id (go.tag = 'json:\"user_id,omitempty\"'),
            2: string __files (go.tag = 'json:\"__files,omitempty\"'),
        }"#;
        Struct::get_parser().parse(str).unwrap();
    }

    #[test]
    fn test_ty() {
        let str = r#"struct Test {
            1: required string(pilota.annotation="test") Service,      // required service
            2: required bytet_i.Injection Injection,
        }"#;
        Struct::get_parser().parse(str).unwrap();
    }
}
