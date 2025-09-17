use std::path::PathBuf;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use faststr::FastStr;

use super::super::{descriptor::File, parser::*};
use crate::{
    Constant, CppInclude, Enum, Exception, Include, Item, Namespace, Service, Struct, Typedef,
    Union,
};

impl Item {
    pub fn parse<'a>() -> impl Parser<'a, &'a str, Item, extra::Err<Rich<'a, char>>> {
        choice((
            Include::get_parser().map(Item::Include),
            CppInclude::parse().map(Item::CppInclude),
            Namespace::get_parser().map(Item::Namespace),
            Typedef::get_parser().map(Item::Typedef),
            Constant::get_parser().map(Item::Constant),
            Enum::get_parser().map(Item::Enum),
            Struct::get_parser().map(Item::Struct),
            Union::parse().map(Item::Union),
            Exception::parse().map(Item::Exception),
            Service::get_parser().map(Item::Service),
        ))
    }
}

pub struct FileSource<'a> {
    path: Option<PathBuf>,
    content: &'a str,
}

impl<'a> FileSource<'a> {
    pub fn new(inline: &'a str) -> Self {
        Self {
            path: None,
            content: inline,
        }
    }

    pub fn new_with_path(path: PathBuf, content: &'a str) -> Result<Self, error::Error> {
        if !path.exists() {
            return Err(error::Error::FileNotFound(path));
        }

        Ok(Self {
            path: Some(path),
            content,
        })
    }
}

pub struct FileParser<'a> {
    pub source: FileSource<'a>,
}

impl<'a> FileParser<'a> {
    pub fn new(source: FileSource<'a>) -> Self {
        Self { source }
    }

    pub fn parse(&self) -> Result<File, error::Error> {
        let (ast, errs) = File::get_parser()
            .parse(self.source.content)
            .into_output_errors();

        let path_str = match &self.source.path {
            Some(path) => &path.display().to_string(),
            None => "inline",
        };

        if !errs.is_empty() {
            let mut report_strings = Vec::with_capacity(errs.len() + 1);

            let title = if errs.len() == 1 {
                format!("Failed to parse thrift file: {}", path_str)
            } else {
                format!(
                    "Failed to parse thrift file: {} ({} errors found)",
                    path_str,
                    errs.len()
                )
            };
            report_strings.push(title);
            report_strings.push(String::new());

            for (i, e) in errs.iter().enumerate() {
                if errs.len() > 1 {
                    let error_header = format!("Error {}:", i + 1);
                    report_strings.push(error_header.clone());
                }

                let mut buffer = Vec::new();
                Report::build(ReportKind::Error, (path_str, e.span().into_range()))
                    .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                    .with_message(e.to_string())
                    .with_label(
                        Label::new((path_str, e.span().into_range()))
                            .with_message(e.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .write((path_str, Source::from(self.source.content)), &mut buffer)
                    .unwrap();
                report_strings.push(String::from_utf8_lossy(&buffer).to_string());

                if i < errs.len() - 1 {
                    report_strings.push(String::new());
                }
            }

            let report = report_strings.join("\n").into();
            let summary = create_error_summary(&errs, path_str, self.source.content).into();
            let custom_error = CustomSyntaxError { report };

            return Err(error::Error::Syntax {
                summary,
                source: anyhow::anyhow!(custom_error),
            });
        }

        Ok(ast.unwrap())
    }
}

fn create_error_summary(errs: &[chumsky::error::Rich<char>], path_str: &str, text: &str) -> String {
    if errs.is_empty() {
        return String::new();
    }

    let mut summary = format!("Failed to parse thrift file: {}", path_str);

    if errs.len() == 1 {
        let err = &errs[0];
        // 计算行号和列号
        let (line, col) = calculate_line_col(err.span().start, text);
        summary.push_str(&format!(" at line {}:{} - {}", line, col, err.reason()));
    } else {
        summary.push_str(&format!(" ({} errors found):", errs.len()));
        for (i, err) in errs.iter().enumerate() {
            let (line, col) = calculate_line_col(err.span().start, text);
            summary.push_str(&format!(
                "\n  {}. Line {}:{} - {}",
                i + 1,
                line,
                col,
                err.reason()
            ));
        }
    }

    summary
}

fn calculate_line_col(pos: usize, text: &str) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in text.char_indices() {
        if i >= pos {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

#[derive(Debug)]
pub struct CustomSyntaxError {
    pub report: FastStr,
}

impl std::fmt::Display for CustomSyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.report)
    }
}

impl std::error::Error for CustomSyntaxError {}

impl File {
    pub(crate) fn get_parser<'a>() -> impl Parser<'a, &'a str, File, extra::Err<Rich<'a, char>>> {
        let item_or_none = blank()
            .or_not()
            .ignore_then(Item::parse())
            .then_ignore(blank().or_not());

        item_or_none
            .repeated()
            .collect()
            .then_ignore(blank().or_not())
            .then_ignore(end())
            .map(|items: Vec<Item>| {
                let mut file = File {
                    items,
                    ..Default::default()
                };

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
        let (file, errs) = File::get_parser().parse(body).into_output_errors();
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
        let (file, errs) = File::get_parser().parse(body).into_output_errors();
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
        let (file, errs) = File::get_parser().parse(body).into_output_errors();
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
