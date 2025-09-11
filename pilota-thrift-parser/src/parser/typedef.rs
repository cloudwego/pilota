use chumsky::prelude::*;

use super::super::{descriptor::Typedef, parser::*};
use crate::descriptor::Ident;

pub fn type_def<'a>() -> impl Parser<'a, &'a str, Typedef, extra::Err<Rich<'a, char>>> {
    just("typedef")
        .ignore_then(ty::r#type().padded_by(blank()))
        .then(identifier::parse())
        .then_ignore(blank().or_not())
        .then(annotation::parse().or_not())
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
