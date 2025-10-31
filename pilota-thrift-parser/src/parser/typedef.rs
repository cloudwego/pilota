use chumsky::prelude::*;
use faststr::FastStr;

use super::super::{descriptor::Typedef, parser::*};
use crate::{Annotation, Type, descriptor::Ident};

impl Typedef {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Typedef, extra::Err<Rich<'a, char>>> {
        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("typedef"))
            .then_ignore(Components::blank_with_comments())
            .then(Type::get_parser().padded_by(Components::blank()))
            .then(Ident::get_parser())
            .then(Annotation::get_parser().or_not())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(
                |((((comments, r#type), alias), annotations), trailing_comments)| Typedef {
                    leading_comments: FastStr::from(comments.join("\n\n")),
                    r#type,
                    alias: Ident(alias.into()),
                    annotations: annotations.unwrap_or_default(),
                    trailing_comments: trailing_comments.unwrap_or_default(),
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_def() {
        let _td = Typedef::get_parser().parse("typedef i32 Int32,").unwrap();
    }
}
