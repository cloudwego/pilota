use std::{num::ParseIntError, str::FromStr};

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{digit1, hex_digit1},
    combinator::{map, map_res, opt, recognize},
    multi::many0,
    sequence::{delimited, preceded, tuple},
};

use super::super::{
    descriptor::{
        Annotations, ConstValue, Constant, DoubleConstant, Ident, IntConstant, Literal, Type,
    },
    parser::*,
};

impl Parser for ConstValue {
    fn parse(input: &str) -> IResult<&str, ConstValue> {
        alt((
            map(Literal::parse, ConstValue::String),
            map(tag("true"), |_| ConstValue::Bool(true)),
            map(tag("false"), |_| ConstValue::Bool(false)),
            map(Path::parse, ConstValue::Path),
            map(DoubleConstant::parse, ConstValue::Double),
            map(IntConstant::parse, ConstValue::Int),
            map(
                tuple((
                    tag("["),
                    many0(map(
                        tuple((
                            opt(blank),
                            ConstValue::parse,
                            opt(blank),
                            opt(list_separator),
                        )),
                        |(_, elements, _, _)| elements,
                    )),
                    opt(blank),
                    tag("]"),
                )),
                |(_, elements, _, _)| ConstValue::List(elements),
            ),
            map(
                tuple((
                    tag("{"),
                    many0(map(
                        tuple((
                            opt(blank),
                            ConstValue::parse,
                            opt(blank),
                            tag(":"),
                            opt(blank),
                            ConstValue::parse,
                            opt(blank),
                            opt(list_separator),
                        )),
                        |(_, key, _, _, _, value, _, _)| (key, value),
                    )),
                    opt(blank),
                    tag("}"),
                )),
                |(_, key_value_pairs, _, _)| ConstValue::Map(key_value_pairs),
            ),
        ))(input)
    }
}

impl Parser for Constant {
    fn parse(input: &str) -> IResult<&str, Constant> {
        map(
            tuple((
                tag("const"),
                preceded(blank, Type::parse),
                preceded(blank, Ident::parse),
                preceded(opt(blank), tag("=")),
                preceded(opt(blank), ConstValue::parse),
                opt(blank),
                opt(Annotations::parse),
                opt(list_separator),
            )),
            |(_, r#type, name, _, value, _, annotations, _)| Constant {
                name,
                r#type,
                value,
                annotations: annotations.unwrap_or_default(),
            },
        )(input)
    }
}

impl Parser for IntConstant {
    fn parse(input: &str) -> IResult<&str, IntConstant> {
        alt((
            preceded(tag("-"), map(IntConstant::parse, |d| IntConstant(-d.0))),
            preceded(
                tag("0x"),
                map_res(hex_digit1, |d| i64::from_str_radix(d, 16).map(IntConstant)),
            ),
            map_res(digit1, |d| {
                let d = FromStr::from_str(d)?;
                Ok::<_, ParseIntError>(IntConstant(d))
            }),
        ))(input)
    }
}

impl Parser for DoubleConstant {
    fn parse(input: &str) -> IResult<&str, DoubleConstant> {
        map_res(
            recognize(tuple((
                opt(tag("-")),
                opt(tag("+")),
                alt((
                    delimited(
                        digit1,
                        tag("."),
                        tuple((
                            opt(digit1),
                            opt(tuple((tag_no_case("e"), IntConstant::parse))),
                        )),
                    ),
                    delimited(
                        opt(digit1),
                        tag("."),
                        tuple((digit1, opt(tuple((tag_no_case("e"), IntConstant::parse))))),
                    ),
                    delimited(digit1, tag_no_case("e"), IntConstant::parse),
                )),
            ))),
            |d_str| -> Result<DoubleConstant, std::num::ParseFloatError> {
                Ok(DoubleConstant(d_str.to_owned().into()))
            },
        )(input)
        // map(double, DoubleConstant)(input)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_int_constant() {
        let _i = IntConstant::parse("0x").unwrap().1;
        let _i = IntConstant::parse("1.01e10").unwrap().1;
    }
    #[test]
    fn test_map() {
        let input = r#"const map<i32,set<i32>> aXa1 = {1:[1,1], 2:[2,2]}"#;
        let _c = Constant::parse(input).unwrap().1;
    }

    #[test]
    fn test_list() {
        let input = r#"const list<i32> aXa1 = [1,2]"#;
        let _c = Constant::parse(input).unwrap().1;
    }

    #[test]
    fn test_set() {
        let input = r#"const set<i32> aXa1 = [1,2]"#;
        let _c = Constant::parse(input).unwrap().1;
    }

    #[test]
    fn test_bool() {
        let input = r#"const bool aXa1 = true"#;
        let _c = Constant::parse(input).unwrap().1;
    }

    #[test]
    fn test_i64() {
        let input = r#"const i64 aXa1 = 1"#;
        let _c = Constant::parse(input).unwrap().1;
    }

    #[test]
    fn test_f64() {
        let input = r#"const double aXa1 = 1.01e10"#;
        let _c = Constant::parse(input).unwrap().1;
    }

    #[test]
    fn test_str() {
        let input = r#"const string aXa1 = "hello""#;
        let _c = Constant::parse(input).unwrap().1;
    }
}
