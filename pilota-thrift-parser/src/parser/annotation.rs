use chumsky::prelude::*;

use crate::{
    descriptor::{Annotation, Annotations},
    parser::*,
};

pub fn parse<'a>() -> impl Parser<'a, &'a str, Annotations, extra::Err<Rich<'a, char>>> {
    just("(")
        .ignore_then(
            blank()
                .or_not()
                .ignore_then(any().filter(|c: &char| c.is_ascii_alphabetic() || *c == '_'))
                .then(
                    any()
                        .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_' || *c == '.')
                        .repeated(),
                )
                .to_slice()
                .map(|s| s.to_string())
                .then_ignore(blank().or_not())
                .then_ignore(just("="))
                .then_ignore(blank().or_not())
                .then(literal::parse())
                .padded_by(blank())
                .then_ignore(list_separator().or_not())
                .map(|(key, value)| Annotation { key, value })
                .repeated()
                .at_least(1)
                .collect::<Vec<Annotation>>(),
        )
        .then_ignore(just(")"))
        .map(Annotations)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_annotations() {
        let _a = parse()
            .parse(r#"(go.tag = "json:\"Ids\" split:\"type=tenant\"")"#)
            .unwrap();

        let input = r#"(
            cpp.type = "DenseFoo",
            python.type = "DenseFoo",
            java.final = "",
            )"#;
        let res = parse().parse(input).unwrap();
        assert_eq!(res.len(), 3);
    }
}
