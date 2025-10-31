pub mod comments {
    #![allow(warnings, clippy::all)]

    pub mod volo {

        pub mod rpc {

            pub mod example {

                // namespace declaration

                // File comments test

                // Another file comment line

                /*
                 * Item struct represents an item with id, title, content, and extra metadata
                 */

                // This is a comment for the Item struct

                #[derive(Debug, Default, Clone, PartialEq)]
                pub struct Item {
                    // id of the item
                    pub id: i64, // id of the item

                    /*
                     * title of the item
                     */
                    pub title: ::pilota::FastStr, // trailing comment test

                    // content of the item
                    pub content: ::pilota::FastStr, // trailing comment

                    // extra metadata of the item
                    pub extra: ::std::option::Option<
                        ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
                    >, // trailing comment
                }
                impl ::pilota::thrift::Message for Item {
                    fn encode<T: ::pilota::thrift::TOutputProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TOutputProtocolExt;
                        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Item" };

                        __protocol.write_struct_begin(&struct_ident)?;
                        __protocol.write_i64_field(1, *&self.id)?;
                        __protocol.write_faststr_field(2, (&self.title).clone())?;
                        __protocol.write_faststr_field(3, (&self.content).clone())?;
                        if let Some(value) = self.extra.as_ref() {
                            __protocol.write_map_field(
                                10,
                                ::pilota::thrift::TType::Binary,
                                ::pilota::thrift::TType::Binary,
                                &value,
                                |__protocol, key| {
                                    __protocol.write_faststr((key).clone())?;
                                    ::std::result::Result::Ok(())
                                },
                                |__protocol, val| {
                                    __protocol.write_faststr((val).clone())?;
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                        }
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }

                    fn decode<T: ::pilota::thrift::TInputProtocol>(
                        __protocol: &mut T,
                    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::{Buf, thrift::TLengthProtocolExt};

                        let mut var_1 = None;
                        let mut var_2 = None;
                        let mut var_3 = None;
                        let mut var_10 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin()?;
                        if let ::std::result::Result::Err(mut err) = (|| {
                            loop {
                                let field_ident = __protocol.read_field_begin()?;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    __protocol.field_stop_len();
                                    break;
                                } else {
                                    __protocol
                                        .field_begin_len(field_ident.field_type, field_ident.id);
                                }
                                __pilota_decoding_field_id = field_ident.id;
                                match field_ident.id {
                                    Some(1)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::I64 =>
                                    {
                                        var_1 = Some(__protocol.read_i64()?);
                                    }
                                    Some(2)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::Binary =>
                                    {
                                        var_2 = Some(__protocol.read_faststr()?);
                                    }
                                    Some(3)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::Binary =>
                                    {
                                        var_3 = Some(__protocol.read_faststr()?);
                                    }
                                    Some(10)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::Map =>
                                    {
                                        var_10 = Some({
                                            let map_ident = __protocol.read_map_begin()?;
                                            let mut val =
                                                ::pilota::AHashMap::with_capacity(map_ident.size);
                                            for _ in 0..map_ident.size {
                                                val.insert(
                                                    __protocol.read_faststr()?,
                                                    __protocol.read_faststr()?,
                                                );
                                            }
                                            __protocol.read_map_end()?;
                                            val
                                        });
                                    }
                                    _ => {
                                        __protocol.skip(field_ident.field_type)?;
                                    }
                                }

                                __protocol.read_field_end()?;
                                __protocol.field_end_len();
                            }
                            ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                        })() {
                            if let Some(field_id) = __pilota_decoding_field_id {
                                err.prepend_msg(&format!(
                                    "decode struct `Item` field(#{}) failed, caused by: ",
                                    field_id
                                ));
                            }
                            return ::std::result::Result::Err(err);
                        };
                        __protocol.read_struct_end()?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field id is required".to_string(),
                                ),
                            );
                        };
                        let Some(var_2) = var_2 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field title is required".to_string(),
                                ),
                            );
                        };
                        let Some(var_3) = var_3 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field content is required".to_string(),
                                ),
                            );
                        };

                        let data = Self {
                            id: var_1,
                            title: var_2,
                            content: var_3,
                            extra: var_10,
                        };
                        ::std::result::Result::Ok(data)
                    }

                    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                        __protocol: &'a mut T,
                    ) -> ::std::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::std::future::Future<
                                    Output = ::std::result::Result<
                                        Self,
                                        ::pilota::thrift::ThriftException,
                                    >,
                                > + Send
                                + 'a,
                        >,
                    > {
                        ::std::boxed::Box::pin(async move {
                            let mut var_1 = None;
                            let mut var_2 = None;
                            let mut var_3 = None;
                            let mut var_10 = None;

                            let mut __pilota_decoding_field_id = None;

                            __protocol.read_struct_begin().await?;
                            if let ::std::result::Result::Err(mut err) = async {
                    loop {


                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64  => {
                    var_1 = Some(__protocol.read_i64().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_3 = Some(__protocol.read_faststr().await?);

                },Some(10) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_10 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `Item` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                            __protocol.read_struct_end().await?;

                            let Some(var_1) = var_1 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field id is required".to_string(),
                                    ),
                                );
                            };
                            let Some(var_2) = var_2 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field title is required".to_string(),
                                    ),
                                );
                            };
                            let Some(var_3) = var_3 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field content is required".to_string(),
                                    ),
                                );
                            };

                            let data = Self {
                                id: var_1,
                                title: var_2,
                                content: var_3,
                                extra: var_10,
                            };
                            ::std::result::Result::Ok(data)
                        })
                    }

                    fn size<T: ::pilota::thrift::TLengthProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> usize {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TLengthProtocolExt;
                        __protocol
                            .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Item" })
                            + __protocol.i64_field_len(Some(1), *&self.id)
                            + __protocol.faststr_field_len(Some(2), &self.title)
                            + __protocol.faststr_field_len(Some(3), &self.content)
                            + self.extra.as_ref().map_or(0, |value| {
                                __protocol.map_field_len(
                                    Some(10),
                                    ::pilota::thrift::TType::Binary,
                                    ::pilota::thrift::TType::Binary,
                                    value,
                                    |__protocol, key| __protocol.faststr_len(key),
                                    |__protocol, val| __protocol.faststr_len(val),
                                )
                            })
                            + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                }
                // method to get an item
                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
                pub struct TestServiceGetItemArgsRecv {
                    pub req: GetItemRequest,
                }
                impl ::pilota::thrift::Message for TestServiceGetItemArgsRecv {
                    fn encode<T: ::pilota::thrift::TOutputProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TOutputProtocolExt;
                        let struct_ident = ::pilota::thrift::TStructIdentifier {
                            name: "TestServiceGetItemArgsRecv",
                        };

                        __protocol.write_struct_begin(&struct_ident)?;
                        __protocol.write_struct_field(
                            1,
                            &self.req,
                            ::pilota::thrift::TType::Struct,
                        )?;
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }

                    fn decode<T: ::pilota::thrift::TInputProtocol>(
                        __protocol: &mut T,
                    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::{Buf, thrift::TLengthProtocolExt};

                        let mut var_1 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin()?;
                        if let ::std::result::Result::Err(mut err) = (|| {
                            loop {
                                let field_ident = __protocol.read_field_begin()?;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    __protocol.field_stop_len();
                                    break;
                                } else {
                                    __protocol
                                        .field_begin_len(field_ident.field_type, field_ident.id);
                                }
                                __pilota_decoding_field_id = field_ident.id;
                                match field_ident.id {
                                    Some(1)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::Struct =>
                                    {
                                        var_1 =
                                            Some(::pilota::thrift::Message::decode(__protocol)?);
                                    }
                                    _ => {
                                        __protocol.skip(field_ident.field_type)?;
                                    }
                                }

                                __protocol.read_field_end()?;
                                __protocol.field_end_len();
                            }
                            ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                        })() {
                            if let Some(field_id) = __pilota_decoding_field_id {
                                err.prepend_msg(&format!("decode struct `TestServiceGetItemArgsRecv` field(#{}) failed, caused by: ", field_id));
                            }
                            return ::std::result::Result::Err(err);
                        };
                        __protocol.read_struct_end()?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field req is required".to_string(),
                                ),
                            );
                        };

                        let data = Self { req: var_1 };
                        ::std::result::Result::Ok(data)
                    }

                    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                        __protocol: &'a mut T,
                    ) -> ::std::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::std::future::Future<
                                    Output = ::std::result::Result<
                                        Self,
                                        ::pilota::thrift::ThriftException,
                                    >,
                                > + Send
                                + 'a,
                        >,
                    > {
                        ::std::boxed::Box::pin(async move {
                            let mut var_1 = None;

                            let mut __pilota_decoding_field_id = None;

                            __protocol.read_struct_begin().await?;
                            if let ::std::result::Result::Err(mut err) = async {
                    loop {


                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = Some(<GetItemRequest as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `TestServiceGetItemArgsRecv` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                            __protocol.read_struct_end().await?;

                            let Some(var_1) = var_1 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field req is required".to_string(),
                                    ),
                                );
                            };

                            let data = Self { req: var_1 };
                            ::std::result::Result::Ok(data)
                        })
                    }

                    fn size<T: ::pilota::thrift::TLengthProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> usize {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TLengthProtocolExt;
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "TestServiceGetItemArgsRecv",
                        }) + __protocol.struct_field_len(Some(1), &self.req)
                            + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                }
                // method to get an item

                impl ::std::default::Default for TestServiceGetItemResultSend {
                    fn default() -> Self {
                        TestServiceGetItemResultSend::Ok(::std::default::Default::default())
                    }
                }
                #[derive(Debug, Clone, PartialEq)]
                pub enum TestServiceGetItemResultSend {
                    // method to get an item
                    Ok(GetItemResponse),
                }

                impl ::pilota::thrift::Message for TestServiceGetItemResultSend {
                    fn encode<T: ::pilota::thrift::TOutputProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TOutputProtocolExt;
                        __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                            name: "TestServiceGetItemResultSend",
                        })?;
                        match self {
                            TestServiceGetItemResultSend::Ok(value) => {
                                __protocol.write_struct_field(
                                    0,
                                    value,
                                    ::pilota::thrift::TType::Struct,
                                )?;
                            }
                        }
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }

                    fn decode<T: ::pilota::thrift::TInputProtocol>(
                        __protocol: &mut T,
                    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::{Buf, thrift::TLengthProtocolExt};
                        let mut ret = None;
                        __protocol.read_struct_begin()?;
                        loop {
                            let field_ident = __protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                __protocol.field_stop_len();
                                break;
                            } else {
                                __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            match field_ident.id {
                                Some(0) => {
                                    if ret.is_none() {
                                        let field_ident =
                                            ::pilota::thrift::Message::decode(__protocol)?;
                                        __protocol.struct_len(&field_ident);
                                        ret = Some(TestServiceGetItemResultSend::Ok(field_ident));
                                    } else {
                                        return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message"
                                        ));
                                    }
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type)?;
                                }
                            }
                        }
                        __protocol.read_field_end()?;
                        __protocol.read_struct_end()?;
                        if let Some(ret) = ret {
                            ::std::result::Result::Ok(ret)
                        } else {
                            ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received empty union from remote Message",
                            ))
                        }
                    }

                    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                        __protocol: &'a mut T,
                    ) -> ::std::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::std::future::Future<
                                    Output = ::std::result::Result<
                                        Self,
                                        ::pilota::thrift::ThriftException,
                                    >,
                                > + Send
                                + 'a,
                        >,
                    > {
                        ::std::boxed::Box::pin(async move {
                            let mut ret = None;
                            __protocol.read_struct_begin().await?;
                            loop {
                                let field_ident = __protocol.read_field_begin().await?;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    break;
                                } else {
                                }
                                match field_ident.id {
                                    Some(0) => {
                                        if ret.is_none() {
                                            let field_ident = <GetItemResponse as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                            ret =
                                                Some(TestServiceGetItemResultSend::Ok(field_ident));
                                        } else {
                                            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message"
                                        ));
                                        }
                                    }
                                    _ => {
                                        __protocol.skip(field_ident.field_type).await?;
                                    }
                                }
                            }
                            __protocol.read_field_end().await?;
                            __protocol.read_struct_end().await?;
                            if let Some(ret) = ret {
                                ::std::result::Result::Ok(ret)
                            } else {
                                ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received empty union from remote Message",
                                    ),
                                )
                            }
                        })
                    }

                    fn size<T: ::pilota::thrift::TLengthProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> usize {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TLengthProtocolExt;
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "TestServiceGetItemResultSend",
                        }) + match self {
                            TestServiceGetItemResultSend::Ok(value) => {
                                __protocol.struct_field_len(Some(0), value)
                            }
                        } + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                }
                // GetItemResponse struct represents the response for getting an item

                #[derive(Debug, Default, Clone, PartialEq)]
                pub struct GetItemResponse {
                    pub item: Item,

                    pub status: Status,
                }
                impl ::pilota::thrift::Message for GetItemResponse {
                    fn encode<T: ::pilota::thrift::TOutputProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TOutputProtocolExt;
                        let struct_ident = ::pilota::thrift::TStructIdentifier {
                            name: "GetItemResponse",
                        };

                        __protocol.write_struct_begin(&struct_ident)?;
                        __protocol.write_struct_field(
                            1,
                            &self.item,
                            ::pilota::thrift::TType::Struct,
                        )?;
                        __protocol.write_i32_field(2, (&self.status).inner())?;
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }

                    fn decode<T: ::pilota::thrift::TInputProtocol>(
                        __protocol: &mut T,
                    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::{Buf, thrift::TLengthProtocolExt};

                        let mut var_1 = None;
                        let mut var_2 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin()?;
                        if let ::std::result::Result::Err(mut err) = (|| {
                            loop {
                                let field_ident = __protocol.read_field_begin()?;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    __protocol.field_stop_len();
                                    break;
                                } else {
                                    __protocol
                                        .field_begin_len(field_ident.field_type, field_ident.id);
                                }
                                __pilota_decoding_field_id = field_ident.id;
                                match field_ident.id {
                                    Some(1)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::Struct =>
                                    {
                                        var_1 =
                                            Some(::pilota::thrift::Message::decode(__protocol)?);
                                    }
                                    Some(2)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::I32 =>
                                    {
                                        var_2 =
                                            Some(::pilota::thrift::Message::decode(__protocol)?);
                                    }
                                    _ => {
                                        __protocol.skip(field_ident.field_type)?;
                                    }
                                }

                                __protocol.read_field_end()?;
                                __protocol.field_end_len();
                            }
                            ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                        })() {
                            if let Some(field_id) = __pilota_decoding_field_id {
                                err.prepend_msg(&format!("decode struct `GetItemResponse` field(#{}) failed, caused by: ", field_id));
                            }
                            return ::std::result::Result::Err(err);
                        };
                        __protocol.read_struct_end()?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field item is required".to_string(),
                                ),
                            );
                        };
                        let Some(var_2) = var_2 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field status is required".to_string(),
                                ),
                            );
                        };

                        let data = Self {
                            item: var_1,
                            status: var_2,
                        };
                        ::std::result::Result::Ok(data)
                    }

                    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                        __protocol: &'a mut T,
                    ) -> ::std::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::std::future::Future<
                                    Output = ::std::result::Result<
                                        Self,
                                        ::pilota::thrift::ThriftException,
                                    >,
                                > + Send
                                + 'a,
                        >,
                    > {
                        ::std::boxed::Box::pin(async move {
                            let mut var_1 = None;
                            let mut var_2 = None;

                            let mut __pilota_decoding_field_id = None;

                            __protocol.read_struct_begin().await?;
                            if let ::std::result::Result::Err(mut err) = async {
                    loop {


                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = Some(<Item as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::I32  => {
                    var_2 = Some(<Status as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `GetItemResponse` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                            __protocol.read_struct_end().await?;

                            let Some(var_1) = var_1 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field item is required".to_string(),
                                    ),
                                );
                            };
                            let Some(var_2) = var_2 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field status is required".to_string(),
                                    ),
                                );
                            };

                            let data = Self {
                                item: var_1,
                                status: var_2,
                            };
                            ::std::result::Result::Ok(data)
                        })
                    }

                    fn size<T: ::pilota::thrift::TLengthProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> usize {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TLengthProtocolExt;
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "GetItemResponse",
                        }) + __protocol.struct_field_len(Some(1), &self.item)
                            + __protocol.i32_field_len(Some(2), (&self.status).inner())
                            + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                }
                // method to get an item
                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
                pub struct TestServiceGetItemArgsSend {
                    pub req: GetItemRequest,
                }
                impl ::pilota::thrift::Message for TestServiceGetItemArgsSend {
                    fn encode<T: ::pilota::thrift::TOutputProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TOutputProtocolExt;
                        let struct_ident = ::pilota::thrift::TStructIdentifier {
                            name: "TestServiceGetItemArgsSend",
                        };

                        __protocol.write_struct_begin(&struct_ident)?;
                        __protocol.write_struct_field(
                            1,
                            &self.req,
                            ::pilota::thrift::TType::Struct,
                        )?;
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }

                    fn decode<T: ::pilota::thrift::TInputProtocol>(
                        __protocol: &mut T,
                    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::{Buf, thrift::TLengthProtocolExt};

                        let mut var_1 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin()?;
                        if let ::std::result::Result::Err(mut err) = (|| {
                            loop {
                                let field_ident = __protocol.read_field_begin()?;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    __protocol.field_stop_len();
                                    break;
                                } else {
                                    __protocol
                                        .field_begin_len(field_ident.field_type, field_ident.id);
                                }
                                __pilota_decoding_field_id = field_ident.id;
                                match field_ident.id {
                                    Some(1)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::Struct =>
                                    {
                                        var_1 =
                                            Some(::pilota::thrift::Message::decode(__protocol)?);
                                    }
                                    _ => {
                                        __protocol.skip(field_ident.field_type)?;
                                    }
                                }

                                __protocol.read_field_end()?;
                                __protocol.field_end_len();
                            }
                            ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                        })() {
                            if let Some(field_id) = __pilota_decoding_field_id {
                                err.prepend_msg(&format!("decode struct `TestServiceGetItemArgsSend` field(#{}) failed, caused by: ", field_id));
                            }
                            return ::std::result::Result::Err(err);
                        };
                        __protocol.read_struct_end()?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field req is required".to_string(),
                                ),
                            );
                        };

                        let data = Self { req: var_1 };
                        ::std::result::Result::Ok(data)
                    }

                    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                        __protocol: &'a mut T,
                    ) -> ::std::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::std::future::Future<
                                    Output = ::std::result::Result<
                                        Self,
                                        ::pilota::thrift::ThriftException,
                                    >,
                                > + Send
                                + 'a,
                        >,
                    > {
                        ::std::boxed::Box::pin(async move {
                            let mut var_1 = None;

                            let mut __pilota_decoding_field_id = None;

                            __protocol.read_struct_begin().await?;
                            if let ::std::result::Result::Err(mut err) = async {
                    loop {


                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = Some(<GetItemRequest as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `TestServiceGetItemArgsSend` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                            __protocol.read_struct_end().await?;

                            let Some(var_1) = var_1 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field req is required".to_string(),
                                    ),
                                );
                            };

                            let data = Self { req: var_1 };
                            ::std::result::Result::Ok(data)
                        })
                    }

                    fn size<T: ::pilota::thrift::TLengthProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> usize {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TLengthProtocolExt;
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "TestServiceGetItemArgsSend",
                        }) + __protocol.struct_field_len(Some(1), &self.req)
                            + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                }
                // GetItemRequest struct represents the request for getting an item

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
                pub struct GetItemRequest {
                    pub id: i64,
                }
                impl ::pilota::thrift::Message for GetItemRequest {
                    fn encode<T: ::pilota::thrift::TOutputProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TOutputProtocolExt;
                        let struct_ident = ::pilota::thrift::TStructIdentifier {
                            name: "GetItemRequest",
                        };

                        __protocol.write_struct_begin(&struct_ident)?;
                        __protocol.write_i64_field(1, *&self.id)?;
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }

                    fn decode<T: ::pilota::thrift::TInputProtocol>(
                        __protocol: &mut T,
                    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::{Buf, thrift::TLengthProtocolExt};

                        let mut var_1 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin()?;
                        if let ::std::result::Result::Err(mut err) = (|| {
                            loop {
                                let field_ident = __protocol.read_field_begin()?;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    __protocol.field_stop_len();
                                    break;
                                } else {
                                    __protocol
                                        .field_begin_len(field_ident.field_type, field_ident.id);
                                }
                                __pilota_decoding_field_id = field_ident.id;
                                match field_ident.id {
                                    Some(1)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::I64 =>
                                    {
                                        var_1 = Some(__protocol.read_i64()?);
                                    }
                                    _ => {
                                        __protocol.skip(field_ident.field_type)?;
                                    }
                                }

                                __protocol.read_field_end()?;
                                __protocol.field_end_len();
                            }
                            ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                        })() {
                            if let Some(field_id) = __pilota_decoding_field_id {
                                err.prepend_msg(&format!(
                                    "decode struct `GetItemRequest` field(#{}) failed, caused by: ",
                                    field_id
                                ));
                            }
                            return ::std::result::Result::Err(err);
                        };
                        __protocol.read_struct_end()?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field id is required".to_string(),
                                ),
                            );
                        };

                        let data = Self { id: var_1 };
                        ::std::result::Result::Ok(data)
                    }

                    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                        __protocol: &'a mut T,
                    ) -> ::std::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::std::future::Future<
                                    Output = ::std::result::Result<
                                        Self,
                                        ::pilota::thrift::ThriftException,
                                    >,
                                > + Send
                                + 'a,
                        >,
                    > {
                        ::std::boxed::Box::pin(async move {
                            let mut var_1 = None;

                            let mut __pilota_decoding_field_id = None;

                            __protocol.read_struct_begin().await?;
                            if let ::std::result::Result::Err(mut err) = async {
                    loop {


                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64  => {
                    var_1 = Some(__protocol.read_i64().await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `GetItemRequest` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                            __protocol.read_struct_end().await?;

                            let Some(var_1) = var_1 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field id is required".to_string(),
                                    ),
                                );
                            };

                            let data = Self { id: var_1 };
                            ::std::result::Result::Ok(data)
                        })
                    }

                    fn size<T: ::pilota::thrift::TLengthProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> usize {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TLengthProtocolExt;
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "GetItemRequest",
                        }) + __protocol.i64_field_len(Some(1), *&self.id)
                            + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                }
                // Status enum represents the status of an operation

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct Status(i32);

                impl Status {
                    pub const SUCCESS: Self = Self(0);
                    pub const ERROR: Self = Self(1);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(0) => ::std::string::String::from("SUCCESS"),
                            Self(1) => ::std::string::String::from("ERROR"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            0 => Some(Self::SUCCESS),
                            1 => Some(Self::ERROR),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for Status {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<Status> for i32 {
                    fn from(value: Status) -> i32 {
                        value.0
                    }
                }

                impl ::pilota::thrift::Message for Status {
                    fn encode<T: ::pilota::thrift::TOutputProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TOutputProtocolExt;
                        __protocol.write_i32(self.inner())?;
                        ::std::result::Result::Ok(())
                    }

                    fn decode<T: ::pilota::thrift::TInputProtocol>(
                        __protocol: &mut T,
                    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::{Buf, thrift::TLengthProtocolExt};
                        let value = __protocol.read_i32()?;
                        ::std::result::Result::Ok(
                            ::std::convert::TryFrom::try_from(value).map_err(|err| {
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    format!("invalid enum value for Status, value: {}", value),
                                )
                            })?,
                        )
                    }

                    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                        __protocol: &'a mut T,
                    ) -> ::std::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::std::future::Future<
                                    Output = ::std::result::Result<
                                        Self,
                                        ::pilota::thrift::ThriftException,
                                    >,
                                > + Send
                                + 'a,
                        >,
                    > {
                        ::std::boxed::Box::pin(async move {
                            let value = __protocol.read_i32().await?;
                            ::std::result::Result::Ok(
                                ::std::convert::TryFrom::try_from(value).map_err(|err| {
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        format!("invalid enum value for Status, value: {}", value),
                                    )
                                })?,
                            )
                        })
                    }

                    fn size<T: ::pilota::thrift::TLengthProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> usize {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TLengthProtocolExt;
                        __protocol.i32_len(self.inner())
                    }
                }
                // Test Service

                // This is a comment for the TestService

                pub trait TestService {}

                // method to get an item

                impl ::std::default::Default for TestServiceGetItemResultRecv {
                    fn default() -> Self {
                        TestServiceGetItemResultRecv::Ok(::std::default::Default::default())
                    }
                }
                #[derive(Debug, Clone, PartialEq)]
                pub enum TestServiceGetItemResultRecv {
                    // method to get an item
                    Ok(GetItemResponse),
                }

                impl ::pilota::thrift::Message for TestServiceGetItemResultRecv {
                    fn encode<T: ::pilota::thrift::TOutputProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TOutputProtocolExt;
                        __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                            name: "TestServiceGetItemResultRecv",
                        })?;
                        match self {
                            TestServiceGetItemResultRecv::Ok(value) => {
                                __protocol.write_struct_field(
                                    0,
                                    value,
                                    ::pilota::thrift::TType::Struct,
                                )?;
                            }
                        }
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }

                    fn decode<T: ::pilota::thrift::TInputProtocol>(
                        __protocol: &mut T,
                    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::{Buf, thrift::TLengthProtocolExt};
                        let mut ret = None;
                        __protocol.read_struct_begin()?;
                        loop {
                            let field_ident = __protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                __protocol.field_stop_len();
                                break;
                            } else {
                                __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            match field_ident.id {
                                Some(0) => {
                                    if ret.is_none() {
                                        let field_ident =
                                            ::pilota::thrift::Message::decode(__protocol)?;
                                        __protocol.struct_len(&field_ident);
                                        ret = Some(TestServiceGetItemResultRecv::Ok(field_ident));
                                    } else {
                                        return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message"
                                        ));
                                    }
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type)?;
                                }
                            }
                        }
                        __protocol.read_field_end()?;
                        __protocol.read_struct_end()?;
                        if let Some(ret) = ret {
                            ::std::result::Result::Ok(ret)
                        } else {
                            ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received empty union from remote Message",
                            ))
                        }
                    }

                    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                        __protocol: &'a mut T,
                    ) -> ::std::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::std::future::Future<
                                    Output = ::std::result::Result<
                                        Self,
                                        ::pilota::thrift::ThriftException,
                                    >,
                                > + Send
                                + 'a,
                        >,
                    > {
                        ::std::boxed::Box::pin(async move {
                            let mut ret = None;
                            __protocol.read_struct_begin().await?;
                            loop {
                                let field_ident = __protocol.read_field_begin().await?;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    break;
                                } else {
                                }
                                match field_ident.id {
                                    Some(0) => {
                                        if ret.is_none() {
                                            let field_ident = <GetItemResponse as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                            ret =
                                                Some(TestServiceGetItemResultRecv::Ok(field_ident));
                                        } else {
                                            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message"
                                        ));
                                        }
                                    }
                                    _ => {
                                        __protocol.skip(field_ident.field_type).await?;
                                    }
                                }
                            }
                            __protocol.read_field_end().await?;
                            __protocol.read_struct_end().await?;
                            if let Some(ret) = ret {
                                ::std::result::Result::Ok(ret)
                            } else {
                                ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received empty union from remote Message",
                                    ),
                                )
                            }
                        })
                    }

                    fn size<T: ::pilota::thrift::TLengthProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> usize {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TLengthProtocolExt;
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "TestServiceGetItemResultRecv",
                        }) + match self {
                            TestServiceGetItemResultRecv::Ok(value) => {
                                __protocol.struct_field_len(Some(0), value)
                            }
                        } + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                }
            }
        }
    }
}
