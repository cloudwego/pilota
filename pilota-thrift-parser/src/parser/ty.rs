use std::sync::Arc;

use nom::{
    self, IResult,
    branch::{alt, permutation},
    bytes::complete::tag,
    combinator::{map, not, opt, peek},
    sequence::{preceded, tuple},
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
            map(
                tuple((tag("string"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::String,
            ),
            map(
                tuple((tag("void"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::Void,
            ),
            map(
                tuple((tag("byte"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::Byte,
            ),
            map(
                tuple((tag("bool"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::Bool,
            ),
            map(
                tuple((tag("binary"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::Binary,
            ),
            map(
                tuple((tag("i8"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::I8,
            ),
            map(
                tuple((tag("i16"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::I16,
            ),
            map(
                tuple((tag("i32"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::I32,
            ),
            map(
                tuple((tag("i64"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::I64,
            ),
            map(
                tuple((tag("double"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::Double,
            ),
            map(
                tuple((tag("uuid"), peek(not(alphanumeric_or_underscore)))),
                |_| Ty::Uuid,
            ),
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
