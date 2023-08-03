pub mod wrapper_arc {
    #![allow(warnings, clippy::all)]

    pub mod wrapper_arc {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {}
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                protocol.write_struct_begin(&struct_ident)?;

                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            offset += protocol.field_stop_len();
                            break;
                        } else {
                            offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            _ => {
                                offset += protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `A` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let data = Self {};
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `A` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let data = Self {};
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[::async_trait::async_trait]
        pub trait TestService {}
        #[derive(Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]

        pub enum TestServiceTestResultRecv {
            #[derivative(Default)]
            Ok(Test),
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResultRecv",
                })?;
                match self {
                    TestServiceTestResultRecv::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let mut offset = 0;

                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        offset += protocol.field_stop_len();
                        break;
                    } else {
                        offset += protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                offset += protocol.struct_len(&field_ident);
                                ret = Some(TestServiceTestResultRecv::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            offset += protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut ret = None;
                protocol.read_struct_begin().await?;
                loop {
                    let mut offset = 0;

                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    } else {
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident =
                                    ::pilota::thrift::Message::decode_async(protocol).await?;

                                ret = Some(TestServiceTestResultRecv::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                }
                protocol.read_field_end().await?;
                protocol.read_struct_end().await?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResultRecv",
                }) + match self {
                    TestServiceTestResultRecv::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTestArgsRecv {
            pub req: Test,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsRecv",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            offset += protocol.field_stop_len();
                            break;
                        } else {
                            offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                offset += protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `TestServiceTestArgsRecv` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field req is required".to_string()
                    )
                )
            };

                let data = Self { req };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `TestServiceTestArgsRecv` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(req) = req else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field req is required".to_string()
                    )
                )
            };

                let data = Self { req };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsRecv",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]

        pub enum TestServiceTestResultSend {
            #[derivative(Default)]
            Ok(::std::sync::Arc<Test>),
        }

        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResultSend",
                })?;
                match self {
                    TestServiceTestResultSend::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let mut offset = 0;

                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        offset += protocol.field_stop_len();
                        break;
                    } else {
                        offset += protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(protocol)?,
                                );
                                offset += protocol.struct_len(&field_ident);
                                ret = Some(TestServiceTestResultSend::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            offset += protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut ret = None;
                protocol.read_struct_begin().await?;
                loop {
                    let mut offset = 0;

                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    } else {
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode_async(protocol).await?,
                                );

                                ret = Some(TestServiceTestResultSend::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                }
                protocol.read_field_end().await?;
                protocol.read_struct_end().await?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResultSend",
                }) + match self {
                    TestServiceTestResultSend::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct Test {
            pub id: ::pilota::FastStr,

            pub name2: ::std::vec::Vec<::std::vec::Vec<::std::sync::Arc<A>>>,

            pub name3: ::std::collections::HashMap<i32, ::std::vec::Vec<::std::sync::Arc<A>>>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for Test {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "TEST" };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_faststr_field(1, (&self.id).clone())?;
                protocol.write_list_field(
                    2,
                    ::pilota::thrift::TType::List,
                    &&self.name2,
                    |protocol, val| {
                        protocol.write_list(
                            ::pilota::thrift::TType::Struct,
                            &val,
                            |protocol, val| {
                                protocol.write_struct(val)?;
                                Ok(())
                            },
                        )?;
                        Ok(())
                    },
                )?;
                protocol.write_map_field(
                    3,
                    ::pilota::thrift::TType::I32,
                    ::pilota::thrift::TType::List,
                    &&self.name3,
                    |protocol, key| {
                        protocol.write_i32(*key)?;
                        Ok(())
                    },
                    |protocol, val| {
                        protocol.write_list(
                            ::pilota::thrift::TType::Struct,
                            &val,
                            |protocol, val| {
                                protocol.write_struct(val)?;
                                Ok(())
                            },
                        )?;
                        Ok(())
                    },
                )?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut fields_num = 0;
                let mut id = None;
                fields_num += 1;
                let mut name2 = None;
                fields_num += 1;
                let mut name3 = None;
                fields_num += 1;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        if fields_num == 0 {
                            break;
                        }

                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            offset += protocol.field_stop_len();
                            break;
                        } else {
                            offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                id = Some(protocol.read_faststr()?);
                                fields_num -= 1;
                            }
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                name2 = Some(unsafe {
                                    let list_ident = protocol.read_list_begin()?;
                                    let mut val: Vec<::std::vec::Vec<::std::sync::Arc<A>>> =
                                        Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr().offset(i as isize).write(unsafe {
                                            let list_ident = protocol.read_list_begin()?;
                                            let mut val: Vec<::std::sync::Arc<A>> =
                                                Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr().offset(i as isize).write(
                                                    ::std::sync::Arc::new(
                                                        ::pilota::thrift::Message::decode(
                                                            protocol,
                                                        )?,
                                                    ),
                                                );
                                            }
                                            val.set_len(list_ident.size);
                                            protocol.read_list_end()?;
                                            val
                                        });
                                    }
                                    val.set_len(list_ident.size);
                                    protocol.read_list_end()?;
                                    val
                                });
                                fields_num -= 1;
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                name3 = Some({
                                    let map_ident = protocol.read_map_begin()?;
                                    let mut val =
                                        ::std::collections::HashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(protocol.read_i32()?, unsafe {
                                            let list_ident = protocol.read_list_begin()?;
                                            let mut val: Vec<::std::sync::Arc<A>> =
                                                Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr().offset(i as isize).write(
                                                    ::std::sync::Arc::new(
                                                        ::pilota::thrift::Message::decode(
                                                            protocol,
                                                        )?,
                                                    ),
                                                );
                                            }
                                            val.set_len(list_ident.size);
                                            protocol.read_list_end()?;
                                            val
                                        });
                                    }
                                    protocol.read_map_end()?;
                                    val
                                });
                                fields_num -= 1;
                            }
                            _ => {
                                offset += protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `TEST` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(id) = id else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field id is required".to_string()
                    )
                )
            };
                let Some(name2) = name2 else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field name2 is required".to_string()
                    )
                )
            };
                let Some(name3) = name3 else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field name3 is required".to_string()
                    )
                )
            };

                let data = Self { id, name2, name3 };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut id = None;
                let mut name2 = None;
                let mut name3 = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                id = Some(protocol.read_faststr().await?);
                            }
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                name2 = Some({
                                    let list_ident = protocol.read_list_begin().await?;
                                    let mut val = Vec::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.push({
                                            let list_ident = protocol.read_list_begin().await?;
                                            let mut val = Vec::with_capacity(list_ident.size);
                                            for _ in 0..list_ident.size {
                                                val.push(::std::sync::Arc::new(
                                                    ::pilota::thrift::Message::decode_async(
                                                        protocol,
                                                    )
                                                    .await?,
                                                ));
                                            }
                                            protocol.read_list_end().await?;
                                            val
                                        });
                                    }
                                    protocol.read_list_end().await?;
                                    val
                                });
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                name3 = Some({
                                    let map_ident = protocol.read_map_begin().await?;
                                    let mut val =
                                        ::std::collections::HashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(protocol.read_i32().await?, {
                                            let list_ident = protocol.read_list_begin().await?;
                                            let mut val = Vec::with_capacity(list_ident.size);
                                            for _ in 0..list_ident.size {
                                                val.push(::std::sync::Arc::new(
                                                    ::pilota::thrift::Message::decode_async(
                                                        protocol,
                                                    )
                                                    .await?,
                                                ));
                                            }
                                            protocol.read_list_end().await?;
                                            val
                                        });
                                    }
                                    protocol.read_map_end().await?;
                                    val
                                });
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `TEST` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(id) = id else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field id is required".to_string()
                    )
                )
            };
                let Some(name2) = name2 else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field name2 is required".to_string()
                    )
                )
            };
                let Some(name3) = name3 else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field name3 is required".to_string()
                    )
                )
            };

                let data = Self { id, name2, name3 };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "TEST" })
                    + protocol.faststr_field_len(Some(1), &self.id)
                    + protocol.list_field_len(
                        Some(2),
                        ::pilota::thrift::TType::List,
                        &self.name2,
                        |protocol, el| {
                            protocol.list_len(
                                ::pilota::thrift::TType::Struct,
                                el,
                                |protocol, el| protocol.struct_len(el),
                            )
                        },
                    )
                    + protocol.map_field_len(
                        Some(3),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::List,
                        &self.name3,
                        |protocol, key| protocol.i32_len(*key),
                        |protocol, val| {
                            protocol.list_len(
                                ::pilota::thrift::TType::Struct,
                                val,
                                |protocol, el| protocol.struct_len(el),
                            )
                        },
                    )
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTestArgsSend {
            pub req: ::std::sync::Arc<Test>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsSend",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            offset += protocol.field_stop_len();
                            break;
                        } else {
                            offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(protocol)?,
                                ));
                            }
                            _ => {
                                offset += protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `TestServiceTestArgsSend` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field req is required".to_string()
                    )
                )
            };

                let data = Self { req };
                Ok(data)
            }

            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let mut offset = 0;

                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode_async(protocol).await?,
                                ));
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }

                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `TestServiceTestArgsSend` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;

                let Some(req) = req else {
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                            "field req is required".to_string()
                    )
                )
            };

                let data = Self { req };
                Ok(data)
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsSend",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
    }
}
