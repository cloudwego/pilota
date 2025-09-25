use std::sync::Arc;

use chumsky::prelude::*;

use super::super::parser::*;
use crate::{Annotation, Namespace, Scope};

impl Namespace {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Namespace, extra::Err<Rich<'a, char>>> {
        comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(blank().or_not())
            .then_ignore(just("namespace"))
            .then_ignore(blank())
            .then(Scope::parse().padded_by(blank()))
            .then(Path::parse())
            .then(
                Annotation::get_parser()
                    .or_not()
                    .padded_by(blank().or_not()),
            )
            .then_ignore(list_separator().or_not())
            .map(|(((comments, scope), name), annotations)| Namespace {
                comments: Arc::new(comments.join("\n\n")),
                scope,
                name,
                annotations,
            })
    }
}

fn is_white_space(c: &char) -> bool {
    *c == ' ' || *c == '\t' || *c == '\n' || *c == '\r'
}

impl Scope {
    fn parse<'a>() -> impl Parser<'a, &'a str, Scope, extra::Err<Rich<'a, char>>> {
        any()
            .filter(|c: &char| !is_white_space(c))
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(|s: String| Scope(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace() {
        let _ = Namespace::get_parser()
            .parse("namespace * foo.bar")
            .unwrap();
        let _ = Namespace::get_parser()
            .parse("namespace py.twisted ThriftTest")
            .unwrap();
    }
}
