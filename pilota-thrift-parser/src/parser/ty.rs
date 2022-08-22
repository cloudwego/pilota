use std::sync::Arc;

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use super::super::{
    descriptor::{Annotations, CppType, Literal, Path, Ty, Type},
    parser::*,
};

impl Parser for Type {
    fn parse(input: &str) -> IResult<&str, Type> {
        map(
            tuple((
                Ty::parse,
                opt(map(
                    permutation((opt(blank), Annotations::parse)),
                    |(_, an)| an,
                )),
            )),
            |(ty, an)| Type(ty, an.unwrap_or_default()),
        )(input)
    }
}

impl Parser for CppType {
    fn parse(input: &str) -> IResult<&str, CppType> {
        map(
            tuple((tag("cpp_type"), blank, Literal::parse)),
            |(_, _, cpp_type)| CppType(cpp_type),
        )(input)
    }
}

impl Parser for Ty {
    fn parse(input: &str) -> IResult<&str, Ty> {
        alt((
            map(tag("string"), |_| Ty::String),
            map(tag("void"), |_| Ty::Void),
            map(tag("byte"), |_| Ty::Byte),
            map(tag("bool"), |_| Ty::Bool),
            map(tag("binary"), |_| Ty::Binary),
            map(tag("i8"), |_| Ty::I8),
            map(tag("i16"), |_| Ty::I16),
            map(tag("i32"), |_| Ty::I32),
            map(tag("i64"), |_| Ty::I64),
            map(tag("double"), |_| Ty::Double),
            map(
                tuple((
                    tag("list"),
                    opt(blank),
                    tag("<"),
                    opt(blank),
                    Type::parse,
                    opt(blank),
                    tag(">"),
                    opt(preceded(blank, CppType::parse)),
                )),
                |(_, _, _, _, inner_type, _, _, cpp_type)| Ty::List {
                    value: Arc::new(inner_type),
                    cpp_type,
                },
            ),
            map(
                tuple((
                    tag("set"),
                    opt(preceded(blank, CppType::parse)),
                    opt(blank),
                    tag("<"),
                    opt(blank),
                    Type::parse,
                    opt(blank),
                    tag(">"),
                )),
                |(_, cpp_type, _, _, _, inner_type, _, _)| Ty::Set {
                    value: Arc::new(inner_type),
                    cpp_type,
                },
            ),
            map(
                tuple((
                    tag("map"),
                    opt(preceded(blank, CppType::parse)),
                    opt(blank),
                    tag("<"),
                    opt(blank),
                    Type::parse,
                    opt(blank),
                    list_separator,
                    opt(blank),
                    Type::parse,
                    opt(blank),
                    tag(">"),
                )),
                |(_, cpp_type, _, _, _, key_type, _, _, _, value_type, _, _)| Ty::Map {
                    key: Arc::new(key_type),
                    value: Arc::new(value_type),
                    cpp_type,
                },
            ),
            map(Path::parse, Ty::Path),
        ))(input)
    }
}
