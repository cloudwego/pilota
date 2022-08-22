use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};

use super::super::{
    descriptor::{Annotation, Annotations, Literal},
    parser::*,
};

impl Parser for Annotations {
    // (foo = 'bar', x = "1")
    fn parse(input: &str) -> IResult<&str, Annotations> {
        map(
            tuple((
                tag("("),
                many1(map(
                    tuple((
                        opt(blank),
                        Path::parse,
                        opt(blank),
                        tag("="),
                        opt(blank),
                        Literal::parse,
                        opt(blank),
                        opt(list_separator),
                    )),
                    |(_, p, _, _, _, lit, _, _)| Annotation { key: p, value: lit },
                )),
                tag(")"),
            )),
            |(_, annotations, _)| Annotations(annotations),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::super::parser::Parser, Annotations};

    #[test]
    fn test_annotations() {
        let _a = Annotations::parse(r#"(go.tag = "json:\"Ids\" split:\"type=tenant\"")"#).unwrap();
    }
}
