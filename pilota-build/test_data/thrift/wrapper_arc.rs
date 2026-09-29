pub mod wrapper_arc {
    #![allow(warnings, clippy::all)]

    pub mod wrapper_arc {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {}
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

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
                            "decode struct `A` field(#{}) failed, caused by: ",
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
                                "decode struct `A` field(#{}) failed, caused by: ",
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
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTest2ArgsSend {
            pub req: ::std::sync::Arc<Test>,
        }
        impl ::pilota::thrift::Message for TestServiceTest2ArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTest2ArgsSend",
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
                        err.prepend_msg(&format!("decode struct `TestServiceTest2ArgsSend` field(#{}) failed, caused by: ", field_id));
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
                                        <Test as ::pilota::thrift::Message>::decode_async(
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
                            err.prepend_msg(&format!("decode struct `TestServiceTest2ArgsSend` field(#{}) failed, caused by: ", field_id));
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
                    name: "TestServiceTest2ArgsSend",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTestArgsSend {
            pub req: ::std::sync::Arc<Test>,
        }
        impl ::pilota::thrift::Message for TestServiceTestArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsSend",
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
                        err.prepend_msg(&format!("decode struct `TestServiceTestArgsSend` field(#{}) failed, caused by: ", field_id));
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
                                        <Test as ::pilota::thrift::Message>::decode_async(
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
                            err.prepend_msg(&format!("decode struct `TestServiceTestArgsSend` field(#{}) failed, caused by: ", field_id));
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
                    name: "TestServiceTestArgsSend",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub static DEFAULT_C: ::std::sync::LazyLock<C> = ::std::sync::LazyLock::new(|| C {
            values: Some(::std::vec![1i32, 2i32]),
        });

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct B {
            pub id: ::std::option::Option<i32>,
        }
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "B" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.id.as_ref() {
                    __protocol.write_i32_field(1, *value)?;
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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                var_1 = Some(__protocol.read_i32()?);
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
                            "decode struct `B` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

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
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_1 = Some(__protocol.read_i32().await?);
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
                                "decode struct `B` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self { id: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "B" })
                    + self
                        .id
                        .as_ref()
                        .map_or(0, |value| __protocol.i32_field_len(Some(1), *value))
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTest2ArgsRecv {
            pub req: ::std::sync::Arc<Test>,
        }
        impl ::pilota::thrift::Message for TestServiceTest2ArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTest2ArgsRecv",
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
                        err.prepend_msg(&format!("decode struct `TestServiceTest2ArgsRecv` field(#{}) failed, caused by: ", field_id));
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
                                        <Test as ::pilota::thrift::Message>::decode_async(
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
                            err.prepend_msg(&format!("decode struct `TestServiceTest2ArgsRecv` field(#{}) failed, caused by: ", field_id));
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
                    name: "TestServiceTest2ArgsRecv",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TestServiceTest2ResultSend {
            fn default() -> Self {
                TestServiceTest2ResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum TestServiceTest2ResultSend {
            Ok(::std::sync::Arc<Test>),
        }

        impl ::pilota::thrift::Message for TestServiceTest2ResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTest2ResultSend",
                })?;
                match self {
                    TestServiceTest2ResultSend::Ok(value) => {
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
                                ret = Some(TestServiceTest2ResultSend::Ok(field_ident));
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

                                    ret = Some(TestServiceTest2ResultSend::Ok(field_ident));
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
                    name: "TestServiceTest2ResultSend",
                }) + match self {
                    TestServiceTest2ResultSend::Ok(value) => {
                        __protocol.struct_field_len(Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTestArgsRecv {
            pub req: ::std::sync::Arc<Test>,
        }
        impl ::pilota::thrift::Message for TestServiceTestArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsRecv",
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
                        err.prepend_msg(&format!("decode struct `TestServiceTestArgsRecv` field(#{}) failed, caused by: ", field_id));
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
                                        <Test as ::pilota::thrift::Message>::decode_async(
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
                            err.prepend_msg(&format!("decode struct `TestServiceTestArgsRecv` field(#{}) failed, caused by: ", field_id));
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
                    name: "TestServiceTestArgsRecv",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TestServiceTestResultSend {
            fn default() -> Self {
                TestServiceTestResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum TestServiceTestResultSend {
            Ok(::std::sync::Arc<Test>),
        }

        impl ::pilota::thrift::Message for TestServiceTestResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResultSend",
                })?;
                match self {
                    TestServiceTestResultSend::Ok(value) => {
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
                                ret = Some(TestServiceTestResultSend::Ok(field_ident));
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

                                    ret = Some(TestServiceTestResultSend::Ok(field_ident));
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
                    name: "TestServiceTestResultSend",
                }) + match self {
                    TestServiceTestResultSend::Ok(value) => {
                        __protocol.struct_field_len(Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }

        impl ::std::default::Default for Test {
            fn default() -> Self {
                Test {
                    id: ::std::default::Default::default(),
                    name2: ::std::default::Default::default(),
                    name3: ::std::default::Default::default(),
                    name4: Some(::std::sync::Arc::new(A {})),
                    name5: Some(::std::sync::Arc::new(DEFAULT_B)),
                    name6: Some(::std::sync::Arc::new(DEFAULT_C.clone())),
                    name7: ::std::sync::Arc::new(DEFAULT_C.clone()),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Test {
            pub id: ::pilota::FastStr,

            pub name2: ::std::vec::Vec<::std::vec::Vec<::std::sync::Arc<A>>>,

            pub name3: ::pilota::AHashMap<i32, ::std::vec::Vec<::std::sync::Arc<A>>>,

            pub name4: ::std::option::Option<::std::sync::Arc<A>>,

            pub name5: ::std::option::Option<::std::sync::Arc<B>>,

            pub name6: ::std::option::Option<::std::sync::Arc<C>>,

            pub name7: ::std::sync::Arc<C>,
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
                __protocol.write_faststr_field(1, (&self.id).clone())?;
                __protocol.write_list_field(
                    2,
                    ::pilota::thrift::TType::List,
                    &&self.name2,
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Struct,
                            &val,
                            |__protocol, val| {
                                __protocol.write_struct(val)?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_map_field(
                    3,
                    ::pilota::thrift::TType::I32,
                    ::pilota::thrift::TType::List,
                    &&self.name3,
                    |__protocol, key| {
                        __protocol.write_i32(*key)?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Struct,
                            &val,
                            |__protocol, val| {
                                __protocol.write_struct(val)?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                if let Some(value) = self.name4.as_ref() {
                    __protocol.write_struct_field(4, value, ::pilota::thrift::TType::Struct)?;
                }
                if let Some(value) = self.name5.as_ref() {
                    __protocol.write_struct_field(5, value, ::pilota::thrift::TType::Struct)?;
                }
                if let Some(value) = self.name6.as_ref() {
                    __protocol.write_struct_field(6, value, ::pilota::thrift::TType::Struct)?;
                }
                __protocol.write_struct_field(7, &self.name7, ::pilota::thrift::TType::Struct)?;
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
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;
                let mut var_7 = None;

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
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_2 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<
                                        ::std::vec::Vec<::std::sync::Arc<A>>,
                                    > = ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr().offset(i as isize).write(unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::std::sync::Arc<A>> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr().offset(i as isize).write(
                                                    ::std::sync::Arc::new(
                                                        ::pilota::thrift::Message::decode(
                                                            __protocol,
                                                        )?,
                                                    ),
                                                );
                                            }
                                            val.set_len(list_ident.size);
                                            __protocol.read_list_end()?;
                                            val
                                        });
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                });
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_3 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_i32()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::std::sync::Arc<A>> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr().offset(i as isize).write(
                                                    ::std::sync::Arc::new(
                                                        ::pilota::thrift::Message::decode(
                                                            __protocol,
                                                        )?,
                                                    ),
                                                );
                                            }
                                            val.set_len(list_ident.size);
                                            __protocol.read_list_end()?;
                                            val
                                        });
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(4)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_4 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
                            }
                            Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_5 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
                            }
                            Some(6)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_6 = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
                            }
                            Some(7)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_7 = Some(::std::sync::Arc::new(
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
                        "field id is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name2 is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name3 is required".to_string(),
                    ));
                };

                if var_4.is_none() {
                    var_4 = Some(::std::sync::Arc::new(A {}));
                }
                if var_5.is_none() {
                    var_5 = Some(::std::sync::Arc::new(DEFAULT_B));
                }
                if var_6.is_none() {
                    var_6 = Some(::std::sync::Arc::new(DEFAULT_C.clone()));
                }
                let var_7 = var_7.unwrap_or_else(|| ::std::sync::Arc::new(DEFAULT_C.clone()));

                let data = Self {
                    id: var_1,
                    name2: var_2,
                    name3: var_3,
                    name4: var_4,
                    name5: var_5,
                    name6: var_6,
                    name7: var_7,
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
                    let mut var_3 = None;
                    let mut var_4 = None;
                    let mut var_5 = None;
                    let mut var_6 = None;
                    let mut var_7 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_2 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(::std::sync::Arc::new(<A as ::pilota::thrift::Message>::decode_async(__protocol).await?));
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_3 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_i32().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(::std::sync::Arc::new(<A as ::pilota::thrift::Message>::decode_async(__protocol).await?));
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_4 = Some(::std::sync::Arc::new(<A as ::pilota::thrift::Message>::decode_async(__protocol).await?));

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_5 = Some(::std::sync::Arc::new(<B as ::pilota::thrift::Message>::decode_async(__protocol).await?));

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_6 = Some(::std::sync::Arc::new(<C as ::pilota::thrift::Message>::decode_async(__protocol).await?));

                },Some(7) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_7 = Some(::std::sync::Arc::new(<C as ::pilota::thrift::Message>::decode_async(__protocol).await?));

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
                    err.prepend_msg(&format!("decode struct `Test` field(#{}) failed, caused by: ", field_id));
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
                                "field name2 is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name3 is required".to_string(),
                            ),
                        );
                    };

                    if var_4.is_none() {
                        var_4 = Some(::std::sync::Arc::new(A {}));
                    }
                    if var_5.is_none() {
                        var_5 = Some(::std::sync::Arc::new(DEFAULT_B));
                    }
                    if var_6.is_none() {
                        var_6 = Some(::std::sync::Arc::new(DEFAULT_C.clone()));
                    }
                    let var_7 = var_7.unwrap_or_else(|| ::std::sync::Arc::new(DEFAULT_C.clone()));

                    let data = Self {
                        id: var_1,
                        name2: var_2,
                        name3: var_3,
                        name4: var_4,
                        name5: var_5,
                        name6: var_6,
                        name7: var_7,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Test" })
                    + __protocol.faststr_field_len(Some(1), &self.id)
                    + __protocol.list_field_len(
                        Some(2),
                        ::pilota::thrift::TType::List,
                        &self.name2,
                        |__protocol, el| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Struct,
                                el,
                                |__protocol, el| __protocol.struct_len(el),
                            )
                        },
                    )
                    + __protocol.map_field_len(
                        Some(3),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::List,
                        &self.name3,
                        |__protocol, key| __protocol.i32_len(*key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Struct,
                                val,
                                |__protocol, el| __protocol.struct_len(el),
                            )
                        },
                    )
                    + self
                        .name4
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(4), value))
                    + self
                        .name5
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(5), value))
                    + self
                        .name6
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(6), value))
                    + __protocol.struct_field_len(Some(7), &self.name7)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub const DEFAULT_B: B = B { id: Some(1i32) };

        pub trait TestService {}

        impl ::std::default::Default for TestServiceTest2ResultRecv {
            fn default() -> Self {
                TestServiceTest2ResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum TestServiceTest2ResultRecv {
            Ok(::std::sync::Arc<Test>),
        }

        impl ::pilota::thrift::Message for TestServiceTest2ResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTest2ResultRecv",
                })?;
                match self {
                    TestServiceTest2ResultRecv::Ok(value) => {
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
                                ret = Some(TestServiceTest2ResultRecv::Ok(field_ident));
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

                                    ret = Some(TestServiceTest2ResultRecv::Ok(field_ident));
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
                    name: "TestServiceTest2ResultRecv",
                }) + match self {
                    TestServiceTest2ResultRecv::Ok(value) => {
                        __protocol.struct_field_len(Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TestServiceTestResultRecv {
            fn default() -> Self {
                TestServiceTestResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum TestServiceTestResultRecv {
            Ok(::std::sync::Arc<Test>),
        }

        impl ::pilota::thrift::Message for TestServiceTestResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResultRecv",
                })?;
                match self {
                    TestServiceTestResultRecv::Ok(value) => {
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
                                ret = Some(TestServiceTestResultRecv::Ok(field_ident));
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

                                    ret = Some(TestServiceTestResultRecv::Ok(field_ident));
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
                    name: "TestServiceTestResultRecv",
                }) + match self {
                    TestServiceTestResultRecv::Ok(value) => {
                        __protocol.struct_field_len(Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct C {
            pub values: ::std::option::Option<::std::vec::Vec<i32>>,
        }
        impl ::pilota::thrift::Message for C {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "C" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.values.as_ref() {
                    __protocol.write_list_field(
                        1,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_1 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<i32> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(__protocol.read_i32()?);
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
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
                            "decode struct `C` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self { values: var_1 };
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
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_1 = Some({
                                        let list_ident = __protocol.read_list_begin().await?;
                                        let mut val =
                                            ::std::vec::Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(__protocol.read_i32().await?);
                                        }
                                        __protocol.read_list_end().await?;
                                        val
                                    });
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
                                "decode struct `C` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self { values: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "C" })
                    + self.values.as_ref().map_or(0, |value| {
                        __protocol.list_field_len(
                            Some(1),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
