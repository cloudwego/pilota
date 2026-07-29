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
            .then_ignore(Components::keyword("namespace"))
            .then_ignore(Components::blank())
            .then(Scope::parse())
            .then_ignore(Components::blank())
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

impl Scope {
    /// `NamespaceScope ::= '*' | Identifier` — dotted scopes such as
    /// `py.twisted` are also in use, so the identifier form allows dots.
    fn parse<'a>() -> impl Parser<'a, &'a str, Scope, extra::Err<Rich<'a, char>>> {
        just("*")
            .to_slice()
            .map(str::to_string)
            .or(Ident::ident_with_dot())
            .map(Scope)
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

    #[test]
    fn test_namespace_comment() {
        let input = r#"
        /* comment */ namespace * foo.bar // comment
        "#;
        let _ = Namespace::get_parser().parse(input).unwrap();
    }

    /// `NamespaceScope ::= '*' | Identifier` — a wildcard or a (dotted)
    /// identifier, not an arbitrary run of non-whitespace.
    #[test]
    fn test_namespace_scope() {
        assert!(
            Namespace::get_parser()
                .parse("namespace * foo.bar")
                .into_result()
                .is_ok()
        );
        assert!(
            Namespace::get_parser()
                .parse("namespace py.twisted ThriftTest")
                .into_result()
                .is_ok()
        );
        assert!(
            Namespace::get_parser()
                .parse("namespace @@!! foo.bar")
                .has_errors()
        );
        // A scope is mandatory.
        assert!(
            Namespace::get_parser()
                .parse("namespace foo.bar")
                .has_errors()
        );
        // `namespace` glued to the scope is not the keyword.
        assert!(
            Namespace::get_parser()
                .parse("namespacego foo")
                .has_errors()
        );
    }
}
