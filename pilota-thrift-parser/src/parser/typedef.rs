use chumsky::prelude::*;

use super::super::{descriptor::Typedef, parser::*};
use crate::{Annotation, Type, descriptor::Ident};

pub fn type_def<'a>() -> impl Parser<'a, &'a str, Typedef, extra::Err<Rich<'a, char>>> {
    just("typedef")
        .ignore_then(Type::parse().padded_by(blank()))
        .then(Ident::parse())
        .then_ignore(blank().or_not())
        .then(Annotation::parse().or_not())
        .then_ignore(list_separator().or_not())
        .map(|((r#type, alias), annotations)| Typedef {
            r#type,
            alias: Ident(Arc::from(alias)),
            annotations: annotations.unwrap_or_default(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_def() {
        let _td = type_def().parse("typedef i32 Int32,").unwrap();
    }
}
