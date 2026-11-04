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
            .ignore_then(Components::keyword("throws"))
            .ignore_then(Components::blank().or_not())
            .ignore_then(just("("))
            .ignore_then(fields.clone())
            .then_ignore(Components::blank().or_not())
            .then_ignore(just(")"));

        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then(
                Components::keyword("oneway")
                    .then_ignore(Components::blank())
                    .or_not(),
            )
            .then(Type::get_parser())
            .then_ignore(Components::blank())
            .then(Ident::get_parser())
            .then_ignore(just("(").padded_by(Components::blank().or_not()))
            .then(fields.clone().or_not())
            .then_ignore(Components::blank_with_comments().or_not())
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
                            2: optional list<map<i64 /* comment */ , set<double>>> nestedDataPoints /* comment */
                        ) throws (1: ServiceException ex /* comment */) (api.version = "2.5", deprecated = "false")"#,
            )
            .unwrap();
    }

    #[test]
    fn test_func3() {
        let _f = Function::get_parser()
            .parse(r#"Err test_enum_var_type_name_conflict (1: Request req);"#)
            .unwrap();
    }

    #[test]
    fn test_func_comment() {
        let _f = Function::get_parser()
            .parse(r#"// comment
                        void pingServer( /* comment */
                            1: required /* comment */ string /* comment */ (go.tag = 'json:"source_service"') source /* comment */,
                            2: optional list<map<i64 /* comment */ , set<double>>> nestedDataPoints /* comment */
                        ) /* comment */ (api.version = "2.5", deprecated = "false")"#)
            .unwrap();
    }

    /// `onewayvoid` is a return type named `onewayvoid`, not `oneway void`, so
    /// the `oneway` keyword must end at an identifier boundary.
    #[test]
    fn test_oneway_keyword_boundary() {
        let f = Function::get_parser().parse("onewayvoid f()").unwrap();
        assert!(!f.oneway);
        assert!(
            matches!(&f.result_type.0, crate::Ty::Path(p) if p.segments[0].as_str() == "onewayvoid")
        );
    }
}

#[cfg(test)]
mod streaming_tests {
    use chumsky::Parser;

    use crate::{FileParser, FileSource, Function};

    #[test]
    fn duplicate_annotations_remain_available_for_semantic_diagnostics() {
        let function = Function::get_parser()
            .parse(
                r#"Resp call(1: Req request) (streaming.mode="client", streaming.mode="server")"#,
            )
            .unwrap();
        let values: Vec<_> = function
            .annotations
            .iter()
            .filter(|annotation| annotation.key == "streaming.mode")
            .map(|annotation| annotation.value.to_string())
            .collect();
        assert_eq!(values, ["client", "server"]);
    }

    #[test]
    fn locations_distinguish_methods_with_the_same_name_in_different_services() {
        let source = r#"// Unicode 注释 and a fake name: service B { void call() }
struct Req { 1: optional string text = "call( streaming.mode" }
service A { Req call(1:Req req) (streaming.mode="client") }
service B { Req call(1:Req req) (streaming.mode="server") }
"#;
        let parser = FileParser::new(FileSource::new(source));
        parser.parse().unwrap();
        let locations = parser.function_locations();
        assert_eq!(locations.len(), 2);
        for (location, service, mode) in [
            (&locations[0], "A", "client"),
            (&locations[1], "B", "server"),
        ] {
            assert_eq!(location.service, service);
            assert_eq!(location.function, "call");
            assert!(source[location.span.clone()].contains(&format!("streaming.mode=\"{mode}\"")));
        }
    }
}
