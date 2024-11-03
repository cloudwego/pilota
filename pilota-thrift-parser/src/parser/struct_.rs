use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    IResult,
};

use super::super::{
    descriptor::{Annotations, Exception, Field, Ident, Struct, StructLike, Union},
    parser::*,
};

impl Parser for Struct {
    fn parse(input: &str) -> IResult<&str, Struct> {
        map(
            tuple((tag("struct"), blank, StructLike::parse)),
            |(_, _, st)| Struct(st),
        )(input)
    }
}

impl Parser for Union {
    fn parse(input: &str) -> IResult<&str, Union> {
        let u: IResult<&str, Union> = map(
            tuple((tag("union"), blank, StructLike::parse)),
            |(_, _, st)| Union(st),
        )(input);

        u
    }
}

impl Parser for Exception {
    fn parse(input: &str) -> IResult<&str, Exception> {
        map(
            tuple((tag("exception"), blank, StructLike::parse)),
            |(_, _, st)| Exception(st),
        )(input)
    }
}

impl Parser for StructLike {
    fn parse(input: &str) -> IResult<&str, StructLike> {
        let (r, a) = map(
            tuple((
                Ident::parse,
                opt(blank),
                tag("{"),
                many0(map(tuple((opt(blank), Field::parse)), |(_, field)| field)),
                opt(blank),
                tag("}"),
                opt(blank),
                opt(Annotations::parse),
            )),
            |(name, _, _, fields, _, _, _, annotations)| StructLike {
                name,
                fields,
                annotations: annotations.unwrap_or_default(),
            },
        )(input)?;
        Ok((r, a))
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Parser, Struct};

    #[test]
    fn test_struct() {
        let str = r#"struct MGetRequest {
            1: set<i64> Ids (go.tag = "json:\"Ids\" split:\"type=tenant\""),
            2: optional set<ExtendField> extendFields,


            3: optional set<ExtendField>extendFields2,
        
            255: base.Base Base,
        }
        "#;

        Struct::parse(str).unwrap();
    }

    #[test]
    fn test_struct3() {
        let str = r#"struct TestComment {
            // 1
        }
        "#;
        Struct::parse(str).unwrap();
    }

    #[test]
    fn test_tag() {
        let str = r#"struct ImMsgContent {
            1: string user_id (go.tag = 'json:\"user_id,omitempty\"'),
            2: string __files (go.tag = 'json:\"__files,omitempty\"'),
        }
        (
        cpp.type = "DenseFoo",
        python.type = "DenseFoo",
        java.final = "",
        )"#;
        let (remain, _) = Struct::parse(str).unwrap();
        assert!(remain.is_empty());
    }

    #[test]
    fn test_ty() {
        let str = r#"struct Test {
            1: required string(pilota.annotation="test") Service,      // required service
            2: required bytet_i.Injection Injection,
        }"#;
        Struct::parse(str).unwrap();
    }
}
