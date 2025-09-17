use chumsky::prelude::*;

use super::super::{
    descriptor::{ConstValue, Constant, DoubleConstant, IntConstant},
    parser::*,
};
use crate::{Annotation, Literal, Type};

impl ConstValue {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, ConstValue, extra::Err<Rich<'a, char>>> {
        recursive(|const_value| {
            let list_value = just("[")
                .ignore_then(
                    const_value
                        .clone()
                        .padded_by(blank().or_not())
                        .then_ignore(list_separator().or_not())
                        .repeated()
                        .collect(),
                )
                .then_ignore(blank().or_not())
                .then_ignore(just("]"))
                .map(ConstValue::List);

            let map_value = just("{")
                .ignore_then(
                    const_value
                        .clone()
                        .padded_by(blank().or_not())
                        .then_ignore(just(":"))
                        .then(const_value.clone().padded_by(blank().or_not()))
                        .then_ignore(list_separator().or_not())
                        .repeated()
                        .collect(),
                )
                .then_ignore(blank().or_not())
                .then_ignore(just("}"))
                .map(ConstValue::Map);

            choice((
                Literal::parse().map(ConstValue::String),
                just("true").to(ConstValue::Bool(true)),
                just("false").to(ConstValue::Bool(false)),
                Path::parse().map(ConstValue::Path),
                DoubleConstant::parse().map(ConstValue::Double),
                IntConstant::parse().map(ConstValue::Int),
                list_value,
                map_value,
            ))
            .boxed()
        })
    }
}

impl Constant {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Constant, extra::Err<Rich<'a, char>>> {
        just("const")
            .ignore_then(Type::get_parser().padded_by(blank()))
            .then(Ident::get_parser())
            .then_ignore(just("=").padded_by(blank().or_not()))
            .then(ConstValue::get_parser())
            .then_ignore(blank().or_not())
            .then(Annotation::get_parser().or_not())
            .then_ignore(list_separator().padded_by(blank().or_not()).or_not())
            .map(|(((r#type, name), value), annotations)| Constant {
                name: Ident(name.into()),
                r#type,
                value,
                annotations: annotations.unwrap_or_default(),
            })
    }
}

impl IntConstant {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, IntConstant, extra::Err<Rich<'a, char>>> {
        recursive(|int_constant| {
            choice((
                just("-")
                    .ignore_then(int_constant)
                    .map(|d: IntConstant| IntConstant(-d.0)),
                just("0x")
                    .ignore_then(
                        any()
                            .filter(|c: &char| c.is_ascii_hexdigit())
                            .repeated()
                            .at_least(1)
                            .collect::<String>(),
                    )
                    .map(|d| IntConstant(i64::from_str_radix(d.as_str(), 16).unwrap())),
                any()
                    .filter(|c: &char| c.is_ascii_digit())
                    .repeated()
                    .at_least(1)
                    .collect::<String>()
                    .map(|d| IntConstant(d.as_str().parse::<i64>().unwrap())),
            ))
        })
    }
}

impl DoubleConstant {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, DoubleConstant, extra::Err<Rich<'a, char>>> {
        let digits = any()
            .filter(|c: &char| c.is_ascii_digit())
            .repeated()
            .at_least(1);
        let sign = one_of("+-").or_not();
        let integer_part = digits;
        let fractional_part = just('.').then(digits);
        let exponent_part = one_of("eE").then(one_of("+-").or_not()).then(digits);
        let with_fraction = sign
            .then(integer_part)
            .then(fractional_part)
            .then(exponent_part.or_not())
            .to_slice()
            .map(|s: &str| DoubleConstant(s.into()));
        let with_exponent_only = sign
            .then(integer_part)
            .then(exponent_part)
            .to_slice()
            .map(|s: &str| DoubleConstant(s.into()));
        choice((with_fraction, with_exponent_only))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_int_constant() {
        let _i = IntConstant::parse().parse("0x01").unwrap();
        let _i = IntConstant::parse().parse("1").unwrap();
    }

    #[test]
    fn test_list_constant() {
        let _i = ConstValue::get_parser().parse("[1, 2]").unwrap();
        let _i = ConstValue::get_parser().parse("[1, 0xBC]").unwrap();
    }

    #[test]
    fn test_map() {
        let input = r#"const map<i32,set<i32>> aXa1 = {1:[1,1], 2:[2,2]}"#;
        let _c = Constant::get_parser().parse(input).unwrap();
    }

    #[test]
    fn test_list() {
        let input = r#"const list<i32> aXa1 = [1,2]"#;
        let _c = Constant::get_parser().parse(input).unwrap();
    }

    #[test]
    fn test_set() {
        let input = r#"const set<i32> aXa1 = [1,2]"#;
        let _c = Constant::get_parser().parse(input).unwrap();
    }

    #[test]
    fn test_bool() {
        let input = r#"const bool aXa1 = true"#;
        let _c = Constant::get_parser().parse(input).unwrap();
    }

    #[test]
    fn test_i64() {
        let input = r#"const i64 aXa1 = 0x1"#;
        let _c = Constant::get_parser().parse(input).unwrap();
    }

    #[test]
    fn test_f64() {
        let input = r#"const double aXa1 = 1.01e10"#;
        let _c = Constant::get_parser().parse(input).unwrap();
    }

    #[test]
    fn test_str() {
        let input = r#"const string aXa1 = "hello""#;
        let _c = Constant::get_parser().parse(input).unwrap();
    }
}
