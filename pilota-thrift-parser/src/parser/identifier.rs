use nom::{
    IResult, bytes::complete::take_while, character::complete::satisfy, combinator::recognize,
    sequence::tuple,
};

use super::super::{descriptor::Ident, parser::*};

/// Identifier is not strictly following the BNF: ( Letter | '_' ) ( Letter |
/// Digit | '.' | '_' )* Instead, "_" and "_123" are not allowed since in rust
/// they are invalid parameter names.
impl Parser for Ident {
    fn parse(input: &str) -> IResult<&str, Ident> {
        map(
            recognize(tuple((
                satisfy(|c| c.is_ascii_alphabetic() || c == '_'),
                take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
            ))),
            |ident: &str| -> Ident { Ident(ident.into()) },
        )(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identifier() {
        assert_eq!(Ident::parse("abc").unwrap().1, "abc");
        assert_eq!(Ident::parse("a1d").unwrap().1, "a1d");
        assert_eq!(Ident::parse("foo_bar").unwrap().1, "foo_bar");
        assert_eq!(Ident::parse("foo_bar =").unwrap().1, "foo_bar");
        assert_eq!(Ident::parse("foo_bar=").unwrap().1, "foo_bar");
        assert_eq!(Ident::parse("foo_bar{").unwrap().1, "foo_bar");
        assert_eq!(Ident::parse("foo_bar;").unwrap().1, "foo_bar");
        assert!(Ident::parse("1foo_bar").is_err());
        assert!(Ident::parse("").is_err());

        assert_eq!(Ident::parse("_ihciah,").unwrap().1, "_ihciah");
        assert_eq!(Ident::parse("ihciah,").unwrap().1, "ihciah");
        assert_eq!(Ident::parse("_123").unwrap().1, "_123");
        assert_eq!(Ident::parse("_").unwrap().1, "_");
        assert!(Ident::parse("123").is_err());
    }

    #[test]
    fn test_path() {
        assert_eq!(
            &*Path::parse("prefix.foo_bar").unwrap().1.segments,
            ["prefix", "foo_bar"]
        );
    }
}
