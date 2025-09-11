use chumsky::prelude::*;

use super::super::{
    descriptor::{ConstValue, Constant, DoubleConstant, IntConstant},
    parser::*,
};

pub fn const_value<'a>() -> impl Parser<'a, &'a str, ConstValue, extra::Err<Rich<'a, char>>> {
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
            .map(|elements| ConstValue::List(elements));

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
            .map(|elements| ConstValue::Map(elements));

        choice((
            literal::parse().map(ConstValue::String),
            just("true").to(ConstValue::Bool(true)),
            just("false").to(ConstValue::Bool(false)),
            path().map(ConstValue::Path),
            double_constant().map(ConstValue::Double),
            int_constant().map(ConstValue::Int),
            list_value,
            map_value,
        ))
        .boxed()
    })
}

pub fn constant<'a>() -> impl Parser<'a, &'a str, Constant, extra::Err<Rich<'a, char>>> {
    just("const")
        .ignore_then(ty::r#type().padded_by(blank()))
        .then(identifier::parse())
        .then_ignore(just("=").padded_by(blank().or_not()))
        .then(const_value())
        .then_ignore(blank().or_not())
        .then(annotation::parse().or_not())
        .then_ignore(list_separator().padded_by(blank().or_not()).or_not())
        .map(|(((r#type, name), value), annotations)| Constant {
            name: Ident(Arc::from(name)),
            r#type,
            value,
            annotations: annotations.unwrap_or_default(),
        })
}

pub fn int_constant<'a>() -> impl Parser<'a, &'a str, IntConstant, extra::Err<Rich<'a, char>>> {
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
                .map(|d| IntConstant(i64::from_str_radix(d.as_str(), 10).unwrap())),
        ))
    })
}

pub fn double_constant<'a>() -> impl Parser<'a, &'a str, DoubleConstant, extra::Err<Rich<'a, char>>>
{
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
        .map(|s: &str| DoubleConstant(Arc::from(s.to_string())));
    let with_exponent_only = sign
        .then(integer_part)
        .then(exponent_part)
        .to_slice()
        .map(|s: &str| DoubleConstant(Arc::from(s.to_string())));
    choice((with_fraction, with_exponent_only))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_int_constant() {
        let _i = int_constant().parse("0x01").unwrap();
        let _i = int_constant().parse("1").unwrap();
    }

    #[test]
    fn test_list_constant() {
        let _i = const_value().parse("[1, 2]").unwrap();
        let _i = const_value().parse("[1, 0xBC]").unwrap();
    }

    #[test]
    fn test_map() {
        let input = r#"const map<i32,set<i32>> aXa1 = {1:[1,1], 2:[2,2]}"#;
        let _c = constant().parse(input).unwrap();
    }

    #[test]
    fn test_list() {
        let input = r#"const list<i32> aXa1 = [1,2]"#;
        let _c = constant().parse(input).unwrap();
    }

    #[test]
    fn test_set() {
        let input = r#"const set<i32> aXa1 = [1,2]"#;
        let _c = constant().parse(input).unwrap();
    }

    #[test]
    fn test_bool() {
        let input = r#"const bool aXa1 = true"#;
        let _c = constant().parse(input).unwrap();
    }

    #[test]
    fn test_i64() {
        let input = r#"const i64 aXa1 = 0x1"#;
        let _c = constant().parse(input).unwrap();
    }

    #[test]
    fn test_f64() {
        let input = r#"const double aXa1 = 1.01e10"#;
        let _c = constant().parse(input).unwrap();
    }

    #[test]
    fn test_str() {
        let input = r#"const string aXa1 = "hello""#;
        let _c = constant().parse(input).unwrap();
    }
}
