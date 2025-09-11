use chumsky::prelude::*;

use crate::{
    descriptor::{Annotation, Annotations},
    parser::*,
};

pub fn parse<'a>() -> impl Parser<'a, &'a str, Annotations, extra::Err<Rich<'a, char>>> {
    let key = identifier::ident_with_dot();

    let value = literal::parse();

    just("(")
        .ignore_then(
            key.padded_by(blank().or_not())
                .then_ignore(just("=").padded_by(blank().or_not()))
                .then(value)
                .then_ignore(list_separator().padded_by(blank().or_not()).or_not())
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
            python.type ="DenseFoo",
            java.final="",
            )"#;
        let res = parse().parse(input).unwrap();
        assert_eq!(res.len(), 3);
    }
}
