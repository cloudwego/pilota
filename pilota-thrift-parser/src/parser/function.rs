use chumsky::prelude::*;

use super::super::{
    Attribute,
    descriptor::Function,
    parser::{Parser, blank, list_separator},
};
use crate::parser::*;

pub fn parse<'a>() -> impl Parser<'a, &'a str, Function, extra::Err<Rich<'a, char>>> {
    let fields = field::parse()
        .padded_by(blank().or_not())
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .boxed();

    let throws = just("throws")
        .ignore_then(blank().or_not())
        .ignore_then(just("("))
        .ignore_then(fields.clone())
        .then_ignore(blank().or_not())
        .then_ignore(just(")"));

    just("oneway")
        .then_ignore(blank())
        .or_not()
        .then(ty::r#type())
        .then_ignore(blank())
        .then(identifier::parse())
        .then_ignore(just("(").padded_by(blank().or_not()))
        .then(fields.clone().or_not())
        .then_ignore(just(")"))
        .padded_by(blank().or_not())
        .then(throws.or_not())
        .then_ignore(blank().or_not())
        .then(annotation::parse().or_not())
        .then_ignore(list_separator().or_not())
        .map(
            |(((((oneway, r#type), name), arguments), throws), annotations)| {
                let ow = oneway.is_some();
                let mut args = arguments.unwrap_or_default();
                args.iter_mut().for_each(|f| {
                    if f.attribute == Attribute::Default {
                        f.attribute = Attribute::Required
                    }
                });
                Function {
                    name: Ident(Arc::from(name)),
                    oneway: ow,
                    result_type: r#type,
                    arguments: args,
                    throws: throws.unwrap_or_default(),
                    annotations: annotations.unwrap_or_default(),
                }
            },
        )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_func() {
        let _f = function::parse()
            .parse(
                r#"map<i64, shared.ProcessingStatus> processUserData(
                            1: required list<UserProfile> profiles,
                            2: optional map<string, string(go.tag='json:"config_value"')> config = {"timeout": "10s", "retries": "3"},
                            3: i32(some.annotation = "for_i32_type") executionPriority = 1
                        ) throws (1: ServiceException ex),"#
            )
            .unwrap();
    }

    #[test]
    fn test_func2() {
        let _f = function::parse()
            .parse(
                r#"oneway void pingServer(
                            1: required string(go.tag = 'json:"source_service"') source,
                            2: optional list<map<i64, set<double>>> nestedDataPoints
                        ) (api.version = "2.5", deprecated = "false")"#,
            )
            .unwrap();
    }

    #[test]
    fn test_func3() {
        let _f = function::parse()
            .parse(r#"Err test_enum_var_type_name_conflict (1: Request req);"#)
            .unwrap();
    }
}
