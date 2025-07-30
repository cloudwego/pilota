pub mod auto_name {
    #![allow(warnings, clippy::all)]

    pub mod auto_name {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TEST {
            pub id: ::pilota::FastStr,
        }
        impl ::pilota::thrift::Message for TEST {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "TEST" };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.id).clone())?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_1 = Some(__protocol.read_faststr()?);
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
                            "decode struct `TEST` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field id is required".to_string(),
                    ));
                };

                let data = Self { id: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_1 = Some(__protocol.read_faststr().await?);
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `TEST` field(#{}) failed, caused by: ",
                                field_id
                            ));
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "TEST" })
                    + __protocol.faststr_field_len(Some(1), &self.id)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub trait service {}

        impl ::std::default::Default for serviceTest2ResultRecv {
            fn default() -> Self {
                serviceTest2ResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum serviceTest2ResultRecv {
            Ok(Test),
        }

        impl ::pilota::thrift::Message for serviceTest2ResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTest2ResultRecv",
                })?;
                match self {
                    serviceTest2ResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(serviceTest2ResultRecv::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident =
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(serviceTest2ResultRecv::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTest2ResultRecv",
                }) + match self {
                    serviceTest2ResultRecv::Ok(value) => {
                        __protocol.struct_field_len(Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct servicetestArgsSend {
            pub req: TEST,
        }
        impl ::pilota::thrift::Message for servicetestArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "servicetestArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `servicetestArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `servicetestArgsSend` field(#{}) failed, caused by: ", field_id));
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "servicetestArgsSend",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TestException {}
        impl ::pilota::thrift::Message for TestException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestException",
                };

                __protocol.write_struct_begin(&struct_ident)?;

                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut __pilota_decoding_field_id = None;

                __protocol.read_struct_begin()?;
                if let ::std::result::Result::Err(mut err) = (|| {
                    loop {
                        let field_ident = __protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __protocol.field_stop_len();
                            break;
                        } else {
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
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
                            "decode struct `TestException` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self {};
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
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
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `TestException` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self {};
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestException",
                }) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub const ip: &'static str = "ip";
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct servicetestArgsRecv {
            pub req: TEST,
        }
        impl ::pilota::thrift::Message for servicetestArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "servicetestArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `servicetestArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `servicetestArgsRecv` field(#{}) failed, caused by: ", field_id));
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "servicetestArgsRecv",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for servicetestResultSend {
            fn default() -> Self {
                servicetestResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum servicetestResultSend {
            Ok(Test),

            E(TestException),
        }

        impl ::pilota::thrift::Message for servicetestResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "servicetestResultSend",
                })?;
                match self {
                    servicetestResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    servicetestResultSend::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(servicetestResultSend::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(servicetestResultSend::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident =
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(servicetestResultSend::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(servicetestResultSend::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "servicetestResultSend",
                }) + match self {
                    servicetestResultSend::Ok(value) => __protocol.struct_field_len(Some(0), value),
                    servicetestResultSend::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ServiceTest2ArgsSend {
            pub r#type: ::std::sync::Arc<TEST>,
        }
        impl ::pilota::thrift::Message for ServiceTest2ArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ServiceTest2ArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.r#type, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `ServiceTest2ArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field r#type is required".to_string(),
                    ));
                };

                let data = Self { r#type: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(::std::sync::Arc::new(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ServiceTest2ArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field r#type is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { r#type: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTest2ArgsSend",
                }) + __protocol.struct_field_len(Some(1), &self.r#type)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ServiceTestArgsSend {
            pub req: ::std::sync::Arc<TEST>,
        }
        impl ::pilota::thrift::Message for ServiceTestArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `ServiceTestArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(::std::sync::Arc::new(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ServiceTestArgsSend` field(#{}) failed, caused by: ", field_id));
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestArgsSend",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ServicetestException {
            fn default() -> Self {
                ServicetestException::E(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ServicetestException {
            E(TestException),
        }

        impl ::pilota::thrift::Message for ServicetestException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ServicetestException",
                })?;
                match self {
                    ServicetestException::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServicetestException::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(ServicetestException::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServicetestException",
                }) + match self {
                    ServicetestException::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ServicetestResultRecv {
            fn default() -> Self {
                ServicetestResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ServicetestResultRecv {
            Ok(::std::sync::Arc<Test>),

            E(TestException),
        }

        impl ::pilota::thrift::Message for ServicetestResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ServicetestResultRecv",
                })?;
                match self {
                    ServicetestResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    ServicetestResultRecv::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServicetestResultRecv::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServicetestResultRecv::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );

                                    ret = Some(ServicetestResultRecv::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(ServicetestResultRecv::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServicetestResultRecv",
                }) + match self {
                    ServicetestResultRecv::Ok(value) => __protocol.struct_field_len(Some(0), value),
                    ServicetestResultRecv::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub const IP: &'static str = "IP";
        impl ::std::default::Default for serviceTest2ResultSend {
            fn default() -> Self {
                serviceTest2ResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum serviceTest2ResultSend {
            Ok(Test),
        }

        impl ::pilota::thrift::Message for serviceTest2ResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTest2ResultSend",
                })?;
                match self {
                    serviceTest2ResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(serviceTest2ResultSend::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident =
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(serviceTest2ResultSend::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTest2ResultSend",
                }) + match self {
                    serviceTest2ResultSend::Ok(value) => {
                        __protocol.struct_field_len(Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for serviceTestException {
            fn default() -> Self {
                serviceTestException::E(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum serviceTestException {
            E(TestException),
        }

        impl ::pilota::thrift::Message for serviceTestException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTestException",
                })?;
                match self {
                    serviceTestException::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(serviceTestException::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(serviceTestException::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTestException",
                }) + match self {
                    serviceTestException::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for serviceTestResultRecv {
            fn default() -> Self {
                serviceTestResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum serviceTestResultRecv {
            Ok(Test),

            E(TestException),
        }

        impl ::pilota::thrift::Message for serviceTestResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTestResultRecv",
                })?;
                match self {
                    serviceTestResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    serviceTestResultRecv::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(serviceTestResultRecv::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(serviceTestResultRecv::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident =
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(serviceTestResultRecv::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(serviceTestResultRecv::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTestResultRecv",
                }) + match self {
                    serviceTestResultRecv::Ok(value) => __protocol.struct_field_len(Some(0), value),
                    serviceTestResultRecv::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ServiceTest2ArgsRecv {
            pub r#type: ::std::sync::Arc<TEST>,
        }
        impl ::pilota::thrift::Message for ServiceTest2ArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ServiceTest2ArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.r#type, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `ServiceTest2ArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field r#type is required".to_string(),
                    ));
                };

                let data = Self { r#type: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(::std::sync::Arc::new(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ServiceTest2ArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field r#type is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { r#type: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTest2ArgsRecv",
                }) + __protocol.struct_field_len(Some(1), &self.r#type)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ServiceTestArgsRecv {
            pub req: ::std::sync::Arc<TEST>,
        }
        impl ::pilota::thrift::Message for ServiceTestArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `ServiceTestArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(::std::sync::Arc::new(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ServiceTestArgsRecv` field(#{}) failed, caused by: ", field_id));
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestArgsRecv",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ServiceTestResultSend {
            fn default() -> Self {
                ServiceTestResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ServiceTestResultSend {
            Ok(::std::sync::Arc<Test>),

            E(TestException),
        }

        impl ::pilota::thrift::Message for ServiceTestResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestResultSend",
                })?;
                match self {
                    ServiceTestResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    ServiceTestResultSend::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServiceTestResultSend::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServiceTestResultSend::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );

                                    ret = Some(ServiceTestResultSend::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(ServiceTestResultSend::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestResultSend",
                }) + match self {
                    ServiceTestResultSend::Ok(value) => __protocol.struct_field_len(Some(0), value),
                    ServiceTestResultSend::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Test {
            pub ID: ::pilota::FastStr,

            pub Id: ::pilota::FastStr,
        }
        impl ::pilota::thrift::Message for Test {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Test" };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.ID).clone())?;
                __protocol.write_faststr_field(2, (&self.Id).clone())?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_1 = Some(__protocol.read_faststr()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_2 = Some(__protocol.read_faststr()?);
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
                            "decode struct `Test` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field ID is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field Id is required".to_string(),
                    ));
                };

                let data = Self {
                    ID: var_1,
                    Id: var_2,
                };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_1 = Some(__protocol.read_faststr().await?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_2 = Some(__protocol.read_faststr().await?);
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `Test` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field ID is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field Id is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        ID: var_1,
                        Id: var_2,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Test" })
                    + __protocol.faststr_field_len(Some(1), &self.ID)
                    + __protocol.faststr_field_len(Some(2), &self.Id)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub trait Service {}

        impl ::std::default::Default for ServiceTest2ResultRecv {
            fn default() -> Self {
                ServiceTest2ResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ServiceTest2ResultRecv {
            Ok(::std::sync::Arc<Test>),
        }

        impl ::pilota::thrift::Message for ServiceTest2ResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTest2ResultRecv",
                })?;
                match self {
                    ServiceTest2ResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServiceTest2ResultRecv::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );

                                    ret = Some(ServiceTest2ResultRecv::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTest2ResultRecv",
                }) + match self {
                    ServiceTest2ResultRecv::Ok(value) => {
                        __protocol.struct_field_len(Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ServicetestArgsSend {
            pub req: ::std::sync::Arc<TEST>,

            pub Req: ::std::sync::Arc<TEST>,
        }
        impl ::pilota::thrift::Message for ServicetestArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ServicetestArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_struct_field(2, &self.Req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_2 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `ServicetestArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field Req is required".to_string(),
                    ));
                };

                let data = Self {
                    req: var_1,
                    Req: var_2,
                };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(::std::sync::Arc::new(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_2 = Some(::std::sync::Arc::new(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ServicetestArgsSend` field(#{}) failed, caused by: ", field_id));
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
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field Req is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        req: var_1,
                        Req: var_2,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServicetestArgsSend",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.struct_field_len(Some(2), &self.Req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Index(i32);

        impl Index {
            pub const A: Self = Self(0);
            pub const a: Self = Self(1);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("A"),
                    Self(1) => ::std::string::String::from("a"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from(value: i32) -> Option<Self> {
                match value {
                    0 => Some(Self::A),
                    1 => Some(Self::a),
                    _ => None,
                }
            }
        }

        impl ::std::convert::From<i32> for Index {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Index> for i32 {
            fn from(value: Index) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for Index {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_i32(self.inner())?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                let value = __protocol.read_i32()?;
                ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(
                    |err| {
                        ::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            format!("invalid enum value for Index, value: {}", value),
                        )
                    },
                )?)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let value = __protocol.read_i32().await?;
                    ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(
                        |err| {
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                format!("invalid enum value for Index, value: {}", value),
                            )
                        },
                    )?)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.i32_len(self.inner())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct serviceTest2ArgsSend {
            pub self_: TEST,
        }
        impl ::pilota::thrift::Message for serviceTest2ArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "serviceTest2ArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.self_, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `serviceTest2ArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field self_ is required".to_string(),
                    ));
                };

                let data = Self { self_: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `serviceTest2ArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field self_ is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { self_: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTest2ArgsSend",
                }) + __protocol.struct_field_len(Some(1), &self.self_)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct serviceTestArgsSend {
            pub req: TEST,
        }
        impl ::pilota::thrift::Message for serviceTestArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "serviceTestArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `serviceTestArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `serviceTestArgsSend` field(#{}) failed, caused by: ", field_id));
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTestArgsSend",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for servicetestException {
            fn default() -> Self {
                servicetestException::E(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum servicetestException {
            E(TestException),
        }

        impl ::pilota::thrift::Message for servicetestException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "servicetestException",
                })?;
                match self {
                    servicetestException::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(servicetestException::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(servicetestException::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "servicetestException",
                }) + match self {
                    servicetestException::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for servicetestResultRecv {
            fn default() -> Self {
                servicetestResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum servicetestResultRecv {
            Ok(Test),

            E(TestException),
        }

        impl ::pilota::thrift::Message for servicetestResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "servicetestResultRecv",
                })?;
                match self {
                    servicetestResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    servicetestResultRecv::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(servicetestResultRecv::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(servicetestResultRecv::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident =
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(servicetestResultRecv::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(servicetestResultRecv::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "servicetestResultRecv",
                }) + match self {
                    servicetestResultRecv::Ok(value) => __protocol.struct_field_len(Some(0), value),
                    servicetestResultRecv::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ServicetestArgsRecv {
            pub req: ::std::sync::Arc<TEST>,

            pub Req: ::std::sync::Arc<TEST>,
        }
        impl ::pilota::thrift::Message for ServicetestArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ServicetestArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_struct_field(2, &self.Req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_2 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `ServicetestArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field Req is required".to_string(),
                    ));
                };

                let data = Self {
                    req: var_1,
                    Req: var_2,
                };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(::std::sync::Arc::new(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_2 = Some(::std::sync::Arc::new(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ServicetestArgsRecv` field(#{}) failed, caused by: ", field_id));
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
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field Req is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        req: var_1,
                        Req: var_2,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServicetestArgsRecv",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.struct_field_len(Some(2), &self.Req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ServicetestResultSend {
            fn default() -> Self {
                ServicetestResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ServicetestResultSend {
            Ok(::std::sync::Arc<Test>),

            E(TestException),
        }

        impl ::pilota::thrift::Message for ServicetestResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ServicetestResultSend",
                })?;
                match self {
                    ServicetestResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    ServicetestResultSend::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServicetestResultSend::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServicetestResultSend::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );

                                    ret = Some(ServicetestResultSend::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(ServicetestResultSend::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServicetestResultSend",
                }) + match self {
                    ServicetestResultSend::Ok(value) => __protocol.struct_field_len(Some(0), value),
                    ServicetestResultSend::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct serviceTest2ArgsRecv {
            pub self_: TEST,
        }
        impl ::pilota::thrift::Message for serviceTest2ArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "serviceTest2ArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.self_, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `serviceTest2ArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field self_ is required".to_string(),
                    ));
                };

                let data = Self { self_: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `serviceTest2ArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field self_ is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { self_: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTest2ArgsRecv",
                }) + __protocol.struct_field_len(Some(1), &self.self_)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct serviceTestArgsRecv {
            pub req: TEST,
        }
        impl ::pilota::thrift::Message for serviceTestArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "serviceTestArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `serviceTestArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(
                                        <TEST as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `serviceTestArgsRecv` field(#{}) failed, caused by: ", field_id));
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTestArgsRecv",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for serviceTestResultSend {
            fn default() -> Self {
                serviceTestResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum serviceTestResultSend {
            Ok(Test),

            E(TestException),
        }

        impl ::pilota::thrift::Message for serviceTestResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTestResultSend",
                })?;
                match self {
                    serviceTestResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    serviceTestResultSend::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(serviceTestResultSend::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(serviceTestResultSend::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident =
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(serviceTestResultSend::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(serviceTestResultSend::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "serviceTestResultSend",
                }) + match self {
                    serviceTestResultSend::Ok(value) => __protocol.struct_field_len(Some(0), value),
                    serviceTestResultSend::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ServiceTest2ResultSend {
            fn default() -> Self {
                ServiceTest2ResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ServiceTest2ResultSend {
            Ok(::std::sync::Arc<Test>),
        }

        impl ::pilota::thrift::Message for ServiceTest2ResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTest2ResultSend",
                })?;
                match self {
                    ServiceTest2ResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServiceTest2ResultSend::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );

                                    ret = Some(ServiceTest2ResultSend::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTest2ResultSend",
                }) + match self {
                    ServiceTest2ResultSend::Ok(value) => {
                        __protocol.struct_field_len(Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ServiceTestException {
            fn default() -> Self {
                ServiceTestException::E(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ServiceTestException {
            E(TestException),
        }

        impl ::pilota::thrift::Message for ServiceTestException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestException",
                })?;
                match self {
                    ServiceTestException::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServiceTestException::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(ServiceTestException::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestException",
                }) + match self {
                    ServiceTestException::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ServiceTestResultRecv {
            fn default() -> Self {
                ServiceTestResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ServiceTestResultRecv {
            Ok(::std::sync::Arc<Test>),

            E(TestException),
        }

        impl ::pilota::thrift::Message for ServiceTestResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestResultRecv",
                })?;
                match self {
                    ServiceTestResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    ServiceTestResultRecv::E(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServiceTestResultRecv::Ok(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = Some(ServiceTestResultRecv::E(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <Test as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );

                                    ret = Some(ServiceTestResultRecv::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <TestException as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(ServiceTestResultRecv::E(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
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
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceTestResultRecv",
                }) + match self {
                    ServiceTestResultRecv::Ok(value) => __protocol.struct_field_len(Some(0), value),
                    ServiceTestResultRecv::E(value) => __protocol.struct_field_len(Some(1), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
