use chumsky::prelude::*;

use super::super::{descriptor::File, parser::*};
use crate::Item;

pub fn item<'a>() -> impl Parser<'a, &'a str, Item, extra::Err<Rich<'a, char>>> {
    choice((
        include::include().map(Item::Include),
        include::cpp_include().map(Item::CppInclude),
        namespace::parse().map(Item::Namespace),
        typedef::type_def().map(Item::Typedef),
        constant::constant().map(Item::Constant),
        enum_::parse().map(Item::Enum),
        struct_::r#struct().map(Item::Struct),
        struct_::union().map(Item::Union),
        struct_::exception().map(Item::Exception),
        service::parse().map(Item::Service),
    ))
}

pub fn file<'a>() -> impl Parser<'a, &'a str, File, extra::Err<Rich<'a, char>>> {
    let item_or_none = blank()
        .or_not()
        .ignore_then(item())
        .then_ignore(blank().or_not());

    item_or_none
        .repeated()
        .collect()
        .then_ignore(blank().or_not())
        .then_ignore(end())
        .map(|items: Vec<Item>| {
            let mut file = File::default();

            file.items = items;

            let mut namespaces = file.items.iter().filter_map(|i| match i {
                Item::Namespace(ns) => Some(ns),
                _ => None,
            });

            file.package = namespaces
                .clone()
                .find_map(|n| {
                    if n.scope.0 == "rs" {
                        Some(n.name.clone())
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    namespaces.find_map(|n| {
                        if n.scope.0 == "*" {
                            Some(n.name.clone())
                        } else {
                            None
                        }
                    })
                });

            file
        })
}

#[cfg(test)]
mod tests {
    use ariadne::{Color, Label, Report, ReportKind, Source};

    use super::*;

    #[test]
    fn test_thrift() {
        let body = r#"
        namespace go http

        include "base.thrift"

        enum Sex {
            UNKNOWN = 0,
            MALE = 1,
            FEMALE = 2,
        }
        
        struct ReqItem{
            1: optional i64 id(api.js_conv = '', go.tag = 'json:"MyID" tagexpr:"$<0||$>=100"')
            2: optional string text='hello world'
            3: required string x
        }
        
        struct BizCommonParam {
            1: optional i64 api_version (api.query = 'api_version')
            2: optional i32 token(api.header = 'token')
        }
        
        struct BizRequest {
            1: optional i64 v_int64(api.query = 'v_int64', api.vd = "$>0&&$<200")
            2: optional string text(api.body = 'text')
            3: optional i32 token(api.header = 'token')
            4: optional map<i64, ReqItem> req_items_map (api.body='req_items_map')
            5: optional ReqItem some(api.body = 'some')
            6: optional list<string> req_items(api.query = 'req_items')
            7: optional i32 api_version(api.path = 'action')
            8: optional i64 uid(api.path = 'biz')
            9: optional list<i64> cids(api.query = 'cids')
            10: optional list<string> vids(api.query = 'vids')
            255: base.Base base
            256: optional BizCommonParam biz_common_param (agw.source='not_body_struct')
        }
        
        struct RspItem{
            1: optional i64 item_id
            2: optional string text
        }
        
        struct BizResponse {
            1: optional string T                             (api.header= 'T') 
            2: optional map<i64, RspItem> rsp_items           (api.body='rsp_items')
            3: optional i32 v_enum                       (api.none = '')
            4: optional list<RspItem> rsp_item_list            (api.body = 'rsp_item_list')
            5: optional i32 http_code                         (api.http_code = '') 
            6: optional list<i64> item_count (api.header = 'item_count')
        }
        
        exception Exception{
            1: i32 code (api.http_code = '') 
            2: string msg 
        }
        
        service BizService {
            BizResponse BizMethod1(1: BizRequest req)(api.get = '/life/client/:action/:biz', api.baseurl = 'ib.snssdk.com', api.param = 'true')
            BizResponse BizMethod2(1: BizRequest req)throws(1: Exception err)(api.post = '/life/client/:action/:biz', api.baseurl = 'ib.snssdk.com', api.param = 'true', api.serializer = 'form')
            BizResponse BizMethod3(1: BizRequest req)(api.post = '/life/client/:action/:biz/other', api.baseurl = 'ib.snssdk.com', api.param = 'true', api.serializer = 'json')
        }
        "#;
        let (file, errs) = file().parse(body).into_output_errors();
        println!("{file:#?}");
        errs.into_iter().for_each(|e| {
            Report::build(ReportKind::Error, ("test.thrift", e.span().into_range()))
                .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                .with_message(e.to_string())
                .with_label(
                    Label::new(("test.thrift", e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .print(("test.thrift", Source::from(body)))
                .unwrap()
        });
    }

    #[test]
    fn test_separator() {
        let body = r#"typedef i32 MyInt32
typedef string MyString;

struct TypedefTestStruct {
  1: MyInt32 field_MyInt32;
  2: MyString field_MyString;
  3: i32 field_Int32;
  4: string field_String;
};

typedef TypedefTestStruct MyStruct,

const list<string> TEST_LIST = [
    "hello",
    "world",
];

service Service {
  MyStruct testEpisode(1:MyStruct arg)
},"#;
        let (file, errs) = file().parse(body).into_output_errors();
        println!("{file:#?}");
        errs.into_iter().for_each(|e| {
            Report::build(ReportKind::Error, ("test.thrift", e.span().into_range()))
                .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                .with_message(e.to_string())
                .with_label(
                    Label::new(("test.thrift", e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .print(("test.thrift", Source::from(body)))
                .unwrap()
        });
    }

    #[test]
    fn test_only_comment() {
        let body = r#"
        /*** comment test ***/
        // comment 1

        # comment 2
        "#;
        let (file, errs) = file().parse(body).into_output_errors();
        println!("{file:#?}");
        errs.into_iter().for_each(|e| {
            Report::build(ReportKind::Error, ("test.thrift", e.span().into_range()))
                .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                .with_message(e.to_string())
                .with_label(
                    Label::new(("test.thrift", e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .print(("test.thrift", Source::from(body)))
                .unwrap()
        });
    }
}
