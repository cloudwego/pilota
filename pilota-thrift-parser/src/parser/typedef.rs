use std::sync::Arc;

use chumsky::prelude::*;

use super::super::{descriptor::Typedef, parser::*};
use crate::{Annotation, Type, descriptor::Ident};

impl Typedef {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Typedef, extra::Err<Rich<'a, char>>> {
        comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(blank().or_not())
            .then_ignore(just("typedef"))
            .then_ignore(blank())
            .then(Type::get_parser().padded_by(blank()))
            .then(Ident::get_parser())
            .then_ignore(blank().or_not())
            .then(Annotation::get_parser().or_not())
            .then_ignore(list_separator().or_not())
            .map(|(((comments, r#type), alias), annotations)| Typedef {
                comments: Arc::new(comments.join("\n\n")),
                r#type,
                alias: Ident(alias.into()),
                annotations: annotations.unwrap_or_default(),
            })
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
