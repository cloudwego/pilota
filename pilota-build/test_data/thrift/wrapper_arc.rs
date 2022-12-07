pub mod wrapper_arc {
    #![allow(warnings, clippy::all)]
    pub mod wrapper_arc {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Test {
            pub id: ::std::sync::Arc<::pilota::FastStr>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for Test {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "TEST" };
                protocol.write_struct_begin(&struct_ident)?;
                {
                    let value = &self.id;
                    protocol.write_field_begin(::pilota::thrift::TType::Binary, 1i16)?;
                    protocol.write_faststr(value.clone())?;
                    protocol.write_field_end()?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut id = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            id = Some(protocol.read_faststr()?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let id = if let Some(id) = id {
                    id
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field id is required".to_string(),
                        ),
                    ));
                };
                let data = Self { id: id.into() };
                Ok(data)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut id = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            id = Some(protocol.read_faststr().await?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let id = if let Some(id) = id {
                    id
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field id is required".to_string(),
                        ),
                    ));
                };
                let data = Self { id: id.into() };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                protocol
                    .write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "TEST" })
                    + {
                        let value = &self.id;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("ID"),
                            field_type: ::pilota::thrift::TType::Binary,
                            id: Some(1i16),
                        }) + protocol.write_faststr_len(value)
                            + protocol.write_field_end_len()
                    }
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, :: pilota :: derivative :: Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        pub enum TestServiceTestResult {
            #[derivative(Default)]
            Ok(::pilota::FastStr),
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestResult {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResult",
                })?;
                match self {
                    TestServiceTestResult::Ok(ref value) => {
                        protocol.write_field_begin(::pilota::thrift::TType::Binary, 0i16)?;
                        protocol.write_faststr(value.clone())?;
                        protocol.write_field_end()?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(0i16) => {
                            if ret.is_none() {
                                ret = Some(TestServiceTestResult::Ok(protocol.read_faststr()?));
                            } else {
                                return Err(::pilota::thrift::new_protocol_error(
                                    ::pilota::thrift::ProtocolErrorKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::new_protocol_error(
                        ::pilota::thrift::ProtocolErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut ret = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(0i16) => {
                            if ret.is_none() {
                                ret =
                                    Some(TestServiceTestResult::Ok(protocol.read_faststr().await?));
                            } else {
                                return Err(::pilota::thrift::new_protocol_error(
                                    ::pilota::thrift::ProtocolErrorKind::InvalidData,
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
                    Err(::pilota::thrift::new_protocol_error(
                        ::pilota::thrift::ProtocolErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResult",
                }) + match self {
                    TestServiceTestResult::Ok(ref value) => {
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("Ok"),
                            field_type: ::pilota::thrift::TType::Binary,
                            id: Some(0i16),
                        }) + protocol.write_faststr_len(value)
                            + protocol.write_field_end_len()
                    }
                } + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTestArgsSend {
            pub req: ::std::sync::Arc<Test>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsSend",
                };
                protocol.write_struct_begin(&struct_ident)?;
                {
                    let value = &self.req;
                    protocol.write_field_begin(::pilota::thrift::TType::Struct, 1i16)?;
                    ::pilota::thrift::Message::encode(value, protocol)?;
                    protocol.write_field_end()?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut req = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            req = Some(::pilota::thrift::Message::decode(protocol)?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let req = if let Some(req) = req {
                    req
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field req is required".to_string(),
                        ),
                    ));
                };
                let data = Self { req: req.into() };
                Ok(data)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut req = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            req = Some(::pilota::thrift::Message::decode_async(protocol).await?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let req = if let Some(req) = req {
                    req
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field req is required".to_string(),
                        ),
                    ));
                };
                let data = Self { req: req.into() };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsSend",
                }) + {
                    let value = &self.req;
                    protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                        name: Some("req"),
                        field_type: ::pilota::thrift::TType::Struct,
                        id: Some(1i16),
                    }) + ::pilota::thrift::Message::size(value, protocol)
                        + protocol.write_field_end_len()
                } + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTestArgsRecv {
            pub req: Test,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsRecv",
                };
                protocol.write_struct_begin(&struct_ident)?;
                {
                    let value = &self.req;
                    protocol.write_field_begin(::pilota::thrift::TType::Struct, 1i16)?;
                    ::pilota::thrift::Message::encode(value, protocol)?;
                    protocol.write_field_end()?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut req = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            req = Some(::pilota::thrift::Message::decode(protocol)?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let req = if let Some(req) = req {
                    req
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field req is required".to_string(),
                        ),
                    ));
                };
                let data = Self { req: req };
                Ok(data)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut req = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            req = Some(::pilota::thrift::Message::decode_async(protocol).await?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let req = if let Some(req) = req {
                    req
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field req is required".to_string(),
                        ),
                    ));
                };
                let data = Self { req: req };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsRecv",
                }) + {
                    let value = &self.req;
                    protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                        name: Some("req"),
                        field_type: ::pilota::thrift::TType::Struct,
                        id: Some(1i16),
                    }) + ::pilota::thrift::Message::size(value, protocol)
                        + protocol.write_field_end_len()
                } + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[::async_trait::async_trait]
        pub trait TestService {}
    }
}
