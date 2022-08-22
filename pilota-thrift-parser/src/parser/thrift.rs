use std::cell::RefCell;

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

use super::{
    super::{
        descriptor::{
            Constant, CppInclude, Enum, Exception, File, Include, Service, Struct, Typedef, Union,
        },
        parser::*,
    },
    namespace::Namespace,
};

impl Parser for File {
    fn parse(input: &str) -> IResult<&str, File> {
        let mut t: File = Default::default();
        let includes = &mut t.includes;
        let cpp_includes = &mut t.cpp_includes;
        let items = RefCell::new(&mut t.items);

        let mut namespaces = Vec::new();

        let (remain, _) = many0(tuple((
            opt(blank),
            alt((
                map(Include::parse, |item| includes.push(item)),
                map(CppInclude::parse, |item| cpp_includes.push(item)),
                map(Namespace::parse, |item| namespaces.push(item)),
                map(Typedef::parse, |item| items.borrow_mut().push(item.into())),
                map(Constant::parse, |item| items.borrow_mut().push(item.into())),
                map(Enum::parse, |item| items.borrow_mut().push(item.into())),
                map(Struct::parse, |item| items.borrow_mut().push(item.into())),
                map(Union::parse, |item| items.borrow_mut().push(item.into())),
                map(Exception::parse, |item| {
                    items.borrow_mut().push(item.into())
                }),
                map(Service::parse, |item| items.borrow_mut().push(item.into())),
            )),
        )))(input)?;

        t.package = namespaces.into_iter().find_map(|n| {
            if n.scope.0 == "rs" {
                Some(n.name)
            } else {
                None
            }
        });

        Ok((remain, t))
    }
}

#[cfg(test)]
mod tests {
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
        let (_remain, res) = File::parse(body).unwrap();
        assert_eq!(res.includes.len(), 1);
    }
}
