use chumsky::prelude::*;
use faststr::FastStr;

use super::super::{
    descriptor::{Attribute, Function},
    parser::*,
};
use crate::{Annotation, Field, Type};

impl Function {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Function, extra::Err<Rich<'a, char>>> {
        let fields = Field::get_parser()
            .padded_by(Components::blank().or_not())
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .boxed();

        let throws = Components::blank()
            .or_not()
            .ignore_then(just("throws"))
            .ignore_then(Components::blank().or_not())
            .ignore_then(just("("))
            .ignore_then(fields.clone())
            .then_ignore(Components::blank().or_not())
            .then_ignore(just(")"));

        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then(just("oneway").then_ignore(Components::blank()).or_not())
            .then(Type::get_parser())
            .then_ignore(Components::blank())
            .then(Ident::get_parser())
            .then_ignore(just("(").padded_by(Components::blank().or_not()))
            .then(fields.clone().or_not())
            .then_ignore(Components::blank().or_not())
            .then_ignore(just(")"))
            .then(throws.or_not())
            .then(Annotation::get_parser().or_not())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(
                |(
                    ((((((comments, oneway), r#type), name), arguments), throws), annotations),
                    trailing_comments,
                )| {
                    let ow = oneway.is_some();
                    let mut args = arguments.unwrap_or_default();
                    args.iter_mut().for_each(|f| {
                        if f.attribute == Attribute::Default {
                            f.attribute = Attribute::Required
                        }
                    });
                    Function {
                        leading_comments: FastStr::from(comments.join("\n\n")),
                        name: Ident(name.into()),
                        oneway: ow,
                        result_type: r#type,
                        arguments: args,
                        throws: throws.unwrap_or_default(),
                        annotations: annotations.unwrap_or_default(),
                        trailing_comments: trailing_comments.unwrap_or_default(),
                    }
                },
            )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_func() {
        let _f = Function::get_parser()
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
        let _f = Function::get_parser()
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
        let _f = Function::get_parser()
            .parse(r#"Err test_enum_var_type_name_conflict (1: Request req);"#)
            .unwrap();
    }
}
