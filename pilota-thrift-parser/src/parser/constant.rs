use chumsky::prelude::*;
use faststr::FastStr;

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
                        .padded_by(Components::blank_with_comments().or_not())
                        .then_ignore(Components::list_separator().or_not())
                        .repeated()
                        .collect(),
                )
                .then_ignore(Components::blank_with_comments().or_not())
                .then_ignore(just("]"))
                .map(ConstValue::List);

            let map_value = just("{")
                .ignore_then(
                    const_value
                        .clone()
                        .padded_by(Components::blank_with_comments().or_not())
                        .then_ignore(just(":"))
                        .then(
                            const_value
                                .clone()
                                .padded_by(Components::blank_with_comments().or_not()),
                        )
                        .then_ignore(Components::list_separator().or_not())
                        .repeated()
                        .collect(),
                )
                .then_ignore(Components::blank_with_comments().or_not())
                .then_ignore(just("}"))
                .map(ConstValue::Map);

            choice((
                Literal::parse().map(ConstValue::String),
                Components::keyword("true").to(ConstValue::Bool(true)),
                Components::keyword("false").to(ConstValue::Bool(false)),
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
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(Components::keyword("const"))
            .then(Type::get_parser().padded_by(Components::blank_with_comments()))
            .then(Ident::get_parser())
            .then_ignore(just("=").padded_by(Components::blank_with_comments().or_not()))
            .then(ConstValue::get_parser())
            .then(Annotation::get_parser().or_not())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(
                |(((((comments, r#type), name), value), annotations), trailing_comments)| {
                    Constant {
                        leading_comments: FastStr::from(comments.join("\n\n")),
                        name: Ident(name.into()),
                        r#type,
                        value,
                        annotations: annotations.unwrap_or_default(),
                        trailing_comments: trailing_comments.unwrap_or_default(),
                    }
                },
            )
    }
}

impl IntConstant {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, IntConstant, extra::Err<Rich<'a, char>>> {
        // A single optional sign, per `IntConstant ::= ('+' | '-')? Digit+`
        let sign = one_of("+-").or_not();

        let hex_digits = just("0x")
            .ignore_then(
                any()
                    .filter(char::is_ascii_hexdigit)
                    .repeated()
                    .at_least(1)
                    .to_slice(),
            )
            .map(|digits: &str| (16u32, digits));

        let decimal_digits = any()
            .filter(char::is_ascii_digit)
            .repeated()
            .at_least(1)
            .to_slice()
            .and_is(just("0x").not())
            .map(|digits: &str| (10u32, digits));

        sign.then(choice((hex_digits, decimal_digits))).validate(
            |(sign, (radix, digits)), e, emitter| {
                let result = match sign {
                    Some('-') => {
                        let signed = format!("-{digits}");
                        i64::from_str_radix(&signed, radix)
                    }
                    _ => i64::from_str_radix(digits, radix),
                };

                match result {
                    Ok(value) => IntConstant(value),
                    Err(_) => {
                        emitter.emit(Rich::custom(
                            e.span(),
                            "integer constant does not fit in an i64",
                        ));
                        IntConstant(0)
                    }
                }
            },
        )
    }
}

impl DoubleConstant {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, DoubleConstant, extra::Err<Rich<'a, char>>> {
        let digits = any().filter(char::is_ascii_digit).repeated().at_least(1);
        let sign = one_of("+-").or_not();
        // `Digit*` — the integer part is optional so that `.5` parses.
        let integer_part = any().filter(char::is_ascii_digit).repeated();
        let fractional_part = just('.').then(digits);
        let exponent_part = one_of("eE").then(one_of("+-").or_not()).then(digits);
        let with_fraction = sign
            .then(integer_part)
            .then(fractional_part)
            .then(exponent_part.or_not())
            .to_slice()
            .map(|s: &str| DoubleConstant(s.into()));
        let with_exponent_only = sign
            .then(digits)
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

    #[test]
    fn test_constant_comment() {
        let input = r#"
        /* comment */ const /* comment */ string /* comment */ aXa1 = /* comment */ "hello" // comment
        "#;
        let _c = Constant::get_parser().parse(input).unwrap();
    }

    /// `IntConstant ::= ('+' | '-')? Digit+` — exactly one optional sign.
    #[test]
    fn test_int_constant_sign() {
        assert_eq!(IntConstant::parse().parse("+1").unwrap().0, 1);
        assert_eq!(IntConstant::parse().parse("-1").unwrap().0, -1);
        assert_eq!(IntConstant::parse().parse("-0x10").unwrap().0, -16);
        // The sign is preserved separately, allowing `i64::MIN` to round-trip.
        assert_eq!(
            IntConstant::parse()
                .parse("-9223372036854775808")
                .unwrap()
                .0,
            i64::MIN
        );
        // A doubled or contradictory sign is not a valid integer constant.
        assert!(IntConstant::parse().parse("--1").has_errors());
        assert!(IntConstant::parse().parse("+-1").has_errors());
    }

    /// A hex literal that does not fit in an i64 must report the overflow over
    /// the span of the whole literal.
    #[test]
    fn test_hex_constant_out_of_range() {
        for input in [
            "0x8000000000000000",
            "-0x8000000000000001",
            "0xFFFFFFFFFFFFFFFF",
            "0x10000000000000000",
        ] {
            let result = IntConstant::parse().parse(input);
            assert!(
                result.has_errors(),
                "{input} is out of range and must not parse"
            );

            let errors: Vec<_> = result.errors().collect();
            assert_eq!(errors.len(), 1, "{input} should report exactly one error");
            assert!(
                errors[0].to_string().contains("does not fit in an i64"),
                "{input} should report an overflow, got {}",
                errors[0]
            );
            assert_eq!(
                errors[0].span().into_range(),
                0..input.len(),
                "{input} should be reported over its full span"
            );
        }

        // The i64 boundaries themselves still parse.
        assert_eq!(
            IntConstant::parse().parse("0x7fffffffffffffff").unwrap().0,
            i64::MAX
        );
        assert_eq!(
            IntConstant::parse().parse("-0x8000000000000000").unwrap().0,
            i64::MIN
        );
    }

    #[test]
    fn test_hex_constant_requires_digits() {
        assert!(IntConstant::parse().parse("0x").has_errors());
        assert!(IntConstant::parse().parse("-0x").has_errors());
        // A plain `0` is unaffected.
        assert_eq!(IntConstant::parse().parse("0").unwrap().0, 0);
    }

    /// Well-formed literals, in both radices and with every sign, across zero,
    /// the i64 boundaries and the digit casings hex allows.
    #[test]
    fn test_int_constant_accepted_forms() {
        for (input, expected) in [
            ("0", 0),
            ("-0", 0),
            ("+0", 0),
            ("0x0", 0),
            ("-0x0", 0),
            ("1", 1),
            ("+1", 1),
            ("-1", -1),
            ("0x10", 16),
            ("+0x10", 16),
            ("-0x10", -16),
            ("0xABCDEF", 11_259_375),
            ("0xabcdef", 11_259_375),
            ("0xAbCdEf", 11_259_375),
            ("007", 7),
            ("-007", -7),
            ("0x007", 7),
            ("9223372036854775807", i64::MAX),
            ("+9223372036854775807", i64::MAX),
            ("-9223372036854775808", i64::MIN),
            ("0x7fffffffffffffff", i64::MAX),
            ("-0x8000000000000000", i64::MIN),
        ] {
            assert_eq!(
                IntConstant::parse().parse(input).unwrap().0,
                expected,
                "{input} should parse as {expected}"
            );
        }
    }

    #[test]
    fn test_decimal_constant_out_of_range() {
        for input in [
            "9223372036854775808",
            "+9223372036854775808",
            "-9223372036854775809",
            "99999999999999999999999999",
            "-99999999999999999999999999",
        ] {
            let result = IntConstant::parse().parse(input);
            let errors: Vec<_> = result.errors().collect();
            assert_eq!(errors.len(), 1, "{input} should report exactly one error");
            assert!(
                errors[0].to_string().contains("does not fit in an i64"),
                "{input} should report an overflow, got {}",
                errors[0]
            );
            assert_eq!(
                errors[0].span().into_range(),
                0..input.len(),
                "{input} should be reported over its full span"
            );
        }
    }

    #[test]
    fn test_out_of_range_reported_in_context() {
        for input in [
            "0x8000000000000000",
            "9223372036854775808",
            "[1, 0x8000000000000000]",
            "{1: 9223372036854775808}",
        ] {
            let result = ConstValue::get_parser().parse(input);
            assert!(
                result
                    .errors()
                    .any(|e| e.to_string().contains("does not fit in an i64")),
                "{input} should report an overflow"
            );
        }

        for input in [
            "const i64 A = 0x8000000000000000",
            "const i64 A = 9223372036854775808",
            "const i64 A = -9223372036854775809",
        ] {
            let result = Constant::get_parser().parse(input);
            assert!(
                result
                    .errors()
                    .any(|e| e.to_string().contains("does not fit in an i64")),
                "{input} should report an overflow"
            );
            assert_eq!(result.output().expect("recovered").name.0.to_string(), "A");
        }
    }

    /// `DoubleConstant ::= ('+'|'-')? Digit* ('.' Digit+)? (('E'|'e')
    /// IntConstant)?` — the integer part is optional, so `.5` is a valid
    /// double.
    #[test]
    fn test_double_constant_forms() {
        for input in ["-.5", ".5", "1.5", "+1.5", "1e10", "1.5e-10"] {
            assert!(
                DoubleConstant::parse().parse(input).into_result().is_ok(),
                "{input} should parse as a double"
            );
        }
    }

    #[test]
    fn test_const_keyword_boundary() {
        // `const` glued to an identifier is not the `const` keyword.
        assert!(
            Constant::get_parser()
                .parse("constbool A = true")
                .has_errors()
        );

        // Identifiers that begin with `true`/`false` are not bool literals.
        let v = ConstValue::get_parser().parse("trueValue").unwrap();
        assert!(matches!(v, ConstValue::Path(p) if p.segments[0].as_str() == "trueValue"));
        let v = ConstValue::get_parser().parse("true").unwrap();
        assert!(matches!(v, ConstValue::Bool(true)));
        let v = ConstValue::get_parser().parse("falseValue").unwrap();
        assert!(matches!(v, ConstValue::Path(p) if p.segments[0].as_str() == "falseValue"));
        let v = ConstValue::get_parser().parse("false").unwrap();
        assert!(matches!(v, ConstValue::Bool(false)));
    }
}
