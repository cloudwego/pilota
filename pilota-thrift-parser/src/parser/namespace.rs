use chumsky::prelude::*;

use super::super::parser::*;
use crate::{Namespace, Scope};

pub fn parse<'a>() -> impl Parser<'a, &'a str, Namespace, extra::Err<Rich<'a, char>>> {
    just("namespace")
        .ignore_then(scope().padded_by(blank()))
        .then(path())
        .then(annotation::parse().or_not().padded_by(blank().or_not()))
        .then_ignore(list_separator().or_not())
        .map(|((scope, name), annotations)| Namespace {
            scope,
            name,
            annotations,
        })
}

fn is_white_space(c: &char) -> bool {
    *c == ' ' || *c == '\t' || *c == '\n' || *c == '\r'
}

fn scope<'a>() -> impl Parser<'a, &'a str, Scope, extra::Err<Rich<'a, char>>> {
    any()
        .filter(|c: &char| !is_white_space(c))
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|s: String| Scope(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace() {
        let _ = parse().parse("namespace * foo.bar").unwrap();
        let _ = parse().parse("namespace py.twisted ThriftTest").unwrap();
    }
}
