use chumsky::prelude::*;
use faststr::FastStr;

use super::super::{descriptor::Service, parser::*};
use crate::{Annotation, Function, Ident};

impl Service {
    pub fn get_parser<'a>() -> impl Parser<'a, &'a str, Service, extra::Err<Rich<'a, char>>> {
        let extends = just("extends")
            .padded_by(Components::blank())
            .ignore_then(Path::parse());
        let functions = Components::blank()
            .or_not()
            .ignore_then(Function::get_parser())
            .repeated()
            .collect::<Vec<_>>();

        Components::comment()
            .repeated()
            .collect::<Vec<_>>()
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("service"))
            .then_ignore(Components::blank())
            .then(Ident::get_parser())
            .then(extends.or_not())
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("{"))
            .then(functions)
            .then_ignore(Components::blank().or_not())
            .then_ignore(just("}"))
            .then(Annotation::get_parser().or_not())
            .then_ignore(Components::list_separator().or_not())
            .then(Components::trailing_comment().or_not())
            .then_ignore(Components::blank().or_not())
            .map(
                |(((((comments, name), extends), functions), annotations), trailing_comments)| {
                    Service {
                        leading_comments: FastStr::from(comments.join("\n\n")),
                        name: Ident(name.into()),
                        extends,
                        functions,
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
    fn test_service() {
        let _ = Service::get_parser().parse(
            r#"
            service ComplexService {

                        /**
                         * 函数1: processUserData
                         * 这是一个复杂的 RPC 调用，用于处理用户数据。
                         * 它接收一个用户 Profile 列表，返回一个处理结果的映射。
                         * 可能会抛出自定义异常。
                         */
                        map<i64, shared.ProcessingStatus> processUserData(
                            1: required list<UserProfile> profiles,
                            2: optional map<string, string(go.tag='json:"config_value"')> config = {"timeout": "10s", "retries": "3"},
                            3: i32(some.annotation = "for_i32_type") executionPriority = 1
                        ) throws (1: ServiceException ex),

                        /**
                         * 函数2: pingServer
                         * 这是一个 oneway 函数，客户端发送后不等待服务器响应。
                         * 用于发送心跳或日志，参数中包含了复杂的嵌套结构和注解。
                         * Oneway 函数不能有返回值（除了 void），也不能抛出异常。
                         */
                        oneway void pingServer(
                            1: required string(go.tag = 'json:"source_service"') source,
                            2: optional list<map<i64, set<double>>> nestedDataPoints
                        ) (api.version = "2.5", deprecated = "false")

                        }"#,
        );
    }

    #[test]
    fn test_service2() {
        let _ = Service::get_parser()
            .parse(
                r#"service Test {
                            Err test_enum(1: Ok req);
                            Err test_enum_var_type_name_conflict (1: Request req);
                        }"#,
            )
            .unwrap();
    }
}
