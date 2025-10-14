use chumsky::prelude::*;

use crate::{
    Literal,
    descriptor::{Annotation, Annotations},
    parser::*,
};

impl Annotation {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Annotations, extra::Err<Rich<'a, char>>> {
        let key = Ident::ident_with_dot();

        let value = Literal::parse();

        Components::blank()
            .or_not()
            .ignore_then(just("("))
            .ignore_then(
                key.padded_by(Components::blank().or_not())
                    .then_ignore(just("=").padded_by(Components::blank().or_not()))
                    .then(value)
                    .then_ignore(
                        Components::list_separator()
                            .padded_by(Components::blank().or_not())
                            .or_not(),
                    )
                    .map(|(key, value)| Annotation { key, value })
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<Annotation>>(),
            )
            .then_ignore(just(")"))
            .map(Annotations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_annotations() {
        let _a = Annotation::get_parser()
            .parse(r#"(go.tag = "json:\"Ids\" split:\"type=tenant\"")"#)
            .unwrap();

        let input = r#"(
            cpp.type = "DenseFoo",
            python.type ="DenseFoo",
            java.final="",
            )"#;
        let res = Annotation::get_parser().parse(input).unwrap();
        assert_eq!(res.len(), 3);
    }
}
