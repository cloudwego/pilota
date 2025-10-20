use chumsky::prelude::*;
use faststr::FastStr;

use super::super::parser::*;
use crate::{Annotation, Namespace, Scope};

impl Namespace {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Namespace, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("namespace"))
            .then_ignore(Components::blank())
            .then(Scope::parse().padded_by(Components::blank()))
            .then(Path::parse())
            .then(Annotation::get_parser().or_not())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(
                |((((comments, scope), name), annotations), trailing_comments)| Namespace {
                    leading_comments: FastStr::from(comments.join("\n\n")),
                    scope,
                    name,
                    annotations,
                    trailing_comments: trailing_comments.unwrap_or_default(),
                },
            )
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
