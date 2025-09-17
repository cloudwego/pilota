use chumsky::prelude::*;

use crate::Ident;

impl Ident {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
        text::ascii::ident().map(|ident: &str| ident.to_string())
    }

    pub fn ident_with_dot<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
        any()
            .filter(|c: &char| c.is_ascii_alphabetic() || *c == '_')
            .then(
                any()
                    .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_' || *c == '.')
                    .repeated(),
            )
            .to_slice()
            .map(|s: &str| s.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Path;

    #[test]
    fn test_identifier() {
        assert_eq!(Ident::get_parser().parse("abc").unwrap(), "abc");
        assert_eq!(Ident::get_parser().parse("a1d").unwrap(), "a1d");
        assert_eq!(Ident::get_parser().parse("foo_bar").unwrap(), "foo_bar");

        assert_eq!(Ident::get_parser().parse("_123").unwrap(), "_123");
        assert_eq!(Ident::get_parser().parse("_").unwrap(), "_");
    }

    #[test]
    fn test_path() {
        assert_eq!(
            &*Path::parse().parse("prefix.foo_bar").unwrap().segments,
            ["prefix", "foo_bar"]
        );
    }
}
