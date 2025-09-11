use chumsky::prelude::*;
use std::sync::Arc;

use super::super::{
    descriptor::{CppType, Ty, Type},
    parser::*,
};

pub fn cpp_type<'a>() -> impl Parser<'a, &'a str, CppType, extra::Err<Rich<'a, char>>> {
    just("cpp_type")
        .ignore_then(blank())
        .ignore_then(literal::parse())
        .map(CppType)
}

pub fn r#type<'a>() -> impl Parser<'a, &'a str, Type, extra::Err<Rich<'a, char>>> {
    recursive(|self_parser| {
        let base_ty = choice((
            just("string").to(Ty::String),
            just("void").to(Ty::Void),
            just("byte").to(Ty::Byte),
            just("bool").to(Ty::Bool),
            just("binary").to(Ty::Binary),
            just("i8").to(Ty::I8),
            just("i16").to(Ty::I16),
            just("i32").to(Ty::I32),
            just("i64").to(Ty::I64),
            just("double").to(Ty::Double),
            just("uuid").to(Ty::Uuid),
        ))
        .then_ignore(any().and_is(not_alphanumeric_or_underscore()).rewind());

        let list = just("list")
            .ignore_then(just("<").padded_by(blank().or_not()))
            .ignore_then(self_parser.clone())
            .then_ignore(blank().or_not())
            .then_ignore(just(">"))
            .then(blank().ignore_then(cpp_type()).or_not())
            .map(|(inner_type, cpp_type)| Ty::List {
                value: Arc::new(inner_type),
                cpp_type,
            })
            .boxed();

        let set = just("set")
            .ignore_then(blank().ignore_then(cpp_type()).or_not())
            .then_ignore(just("<"))
            .padded_by(blank().or_not())
            .then(self_parser.clone())
            .then_ignore(blank().or_not())
            .then_ignore(just(">"))
            .map(|(cpp_type, inner_type)| Ty::Set {
                value: Arc::new(inner_type),
                cpp_type,
            })
            .boxed();

        let map_parser = just("map")
            .ignore_then(blank().ignore_then(cpp_type()).or_not())
            .then_ignore(just("<").padded_by(blank().or_not()))
            .then(self_parser.clone())
            .then_ignore(list_separator().padded_by(blank().or_not()))
            .then(self_parser.clone())
            .then_ignore(blank().or_not())
            .then_ignore(just(">"))
            .map(|((cpp_type, key_type), value_type)| Ty::Map {
                key: Arc::new(key_type),
                value: Arc::new(value_type),
                cpp_type,
            })
            .boxed();

        let ty_parser = choice((base_ty, list, set, map_parser, path().map(Ty::Path)));

        ty_parser
            .then(annotation::parse().or_not().padded_by(blank().or_not()))
            .map(|(ty, an)| {
                // println!("type: {:?}, an: {:?}", ty, an);
                Type(ty, an.unwrap_or_default())
            })
            .boxed()
    })
    .boxed()
}

//test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        let parser = r#type();
        let input = "map<i32, string>";
        let _res = parser.parse(input).unwrap();
    }

    #[test]
    fn test_type_path() {
        let parser = r#type();
        let input = "bytet_i.Injection";
        let _res = parser.parse(input).unwrap();
    }
}
