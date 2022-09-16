use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{delimited, tuple},
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
                annotations,
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
    fn test_struct2() {
        let str = r#"struct AgwCommonParam {
            1: agw_common_param.Session                                 Session
            2: agw_common_param.UnifyArgs                               UnifyArgs
            3: agw_common_param.CommonArgs                              CommonArgs
            4: string RealIP   (agw.source="header", agw.key = 'X-REAL-IP')
            5: string Protocol (agw.source="header", agw.key = 'X-Forwarded-Protocol')
            6: string UserAgent (agw.source="header", agw.key = 'User-Agent')
            7: string Forwarded (agw.source="header", agw.key = 'X-Forwarded-For')
            8: string SessionDict (agw.source="header", agw.key="Tt-Agw-Loader-Session-rsp")
            9: abtest_version.VersionRsp                                AbtestVersionRsp (go.tag = 'json:\"-\"')
        }"#;
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
}
