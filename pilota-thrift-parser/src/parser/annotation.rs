use chumsky::prelude::*;

use crate::{
    Literal,
    descriptor::{Annotation, Annotations},
    parser::*,
};

impl Annotation {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Annotations, extra::Err<Rich<'a, char>>> {
        let leading_blank = Components::blank().or_not();

        let key = Ident::ident_with_dot();
        let value = Literal::parse();

        let annotation = key
            .then_ignore(just("=").padded_by(Components::blank().or_not()))
            .then(value)
            .map(|(key, value)| Annotation { key, value })
            .then_ignore(Components::blank_with_comments().or_not());

        let separator =
            Components::list_separator().padded_by(Components::blank_with_comments().or_not());

        let annotation_list = annotation
            .separated_by(separator)
            .allow_trailing()
            .collect::<Vec<Annotation>>()
            .padded_by(Components::blank_with_comments().or_not());

        leading_blank
            .ignore_then(just("("))
            .ignore_then(annotation_list)
            .then_ignore(just(")"))
            .map(Annotations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_annotations() {
        let input = r#"  ()"#;
        let res = Annotation::get_parser().parse(input).unwrap();
        assert_eq!(res.len(), 0);

        let input = r#" (go.tag = "json:\"Ids\" split:\"type=tenant\"")"#;
        let res = Annotation::get_parser().parse(input).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].key, "go.tag");
        assert_eq!(
            res[0].value.to_string(),
            "json:\"Ids\" split:\"type=tenant\""
        );

        let input = r#"(go.tag = "json:\"Ids\" split:\"type=tenant\"";
        )"#;
        let res = Annotation::get_parser().parse(input).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].key, "go.tag");
        assert_eq!(
            res[0].value.to_string(),
            "json:\"Ids\" split:\"type=tenant\""
        );

        let input = r#"(
            cpp.type = "DenseFoo",
            python.type ="DenseFoo", 
            go.type ="DenseFoo";
            java.final=""
        )"#;
        let res = Annotation::get_parser().parse(input).unwrap();
        assert_eq!(res.len(), 4);
        assert_eq!(res[0].key, "cpp.type");
        assert_eq!(res[0].value.to_string(), "DenseFoo");
        assert_eq!(res[1].key, "python.type");
        assert_eq!(res[1].value.to_string(), "DenseFoo");
        assert_eq!(res[2].key, "go.type");
        assert_eq!(res[2].value.to_string(), "DenseFoo");
        assert_eq!(res[3].key, "java.final");
        assert_eq!(res[3].value.to_string(), "");

        let input = r#"(
            // comment before first annotation
            cpp.type = "DenseFoo"; // cpp.type
            python.type ="DenseFoo", go.type ="DenseFoo";
            java.final="")"#;
        let res = Annotation::get_parser().parse(input).unwrap();
        assert_eq!(res.len(), 4);
        assert_eq!(res[0].key, "cpp.type");
        assert_eq!(res[0].value.to_string(), "DenseFoo");
        assert_eq!(res[1].key, "python.type");
        assert_eq!(res[1].value.to_string(), "DenseFoo");
        assert_eq!(res[2].key, "go.type");
        assert_eq!(res[2].value.to_string(), "DenseFoo");
        assert_eq!(res[3].key, "java.final");
        assert_eq!(res[3].value.to_string(), "");

        let input = r#"(
            /* separated comment */
            cpp.type = "DenseFoo"
        )"#;
        let res = Annotation::get_parser().parse(input).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].key, "cpp.type");
        assert_eq!(res[0].value.to_string(), "DenseFoo");

        let input = r#"(
            cpp.type = "DenseFoo";
            python.type ="DenseFoo", go.type ="DenseFoo"; /* go.type */
            java.final="",)"#;
        let res = Annotation::get_parser().parse(input).unwrap();
        assert_eq!(res.len(), 4);
        assert_eq!(res[0].key, "cpp.type");
        assert_eq!(res[0].value.to_string(), "DenseFoo");
        assert_eq!(res[1].key, "python.type");
        assert_eq!(res[1].value.to_string(), "DenseFoo");
        assert_eq!(res[2].key, "go.type");
        assert_eq!(res[2].value.to_string(), "DenseFoo");
        assert_eq!(res[3].key, "java.final");
        assert_eq!(res[3].value.to_string(), "");
    }
}
