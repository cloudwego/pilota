pub mod wrapper_arc {
    #![allow(warnings, clippy::all)]
    pub mod wrapper_arc {
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct Test {
            pub id: ::std::sync::Arc<::pilota::FastStr>,
            pub name2: ::std::vec::Vec<::std::vec::Vec<::std::sync::Arc<i32>>>,
            pub name3: ::std::collections::HashMap<i32, ::std::vec::Vec<::std::sync::Arc<i32>>>,
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
                {
                    let value = &self.name2;
                    protocol.write_field_begin(::pilota::thrift::TType::List, 2i16)?;
                    let list_ident = ::pilota::thrift::TListIdentifier {
                        element_type: ::pilota::thrift::TType::List,
                        size: value.len(),
                    };
                    protocol.write_list_begin(&list_ident)?;
                    for val in value {
                        let list_ident = ::pilota::thrift::TListIdentifier {
                            element_type: ::pilota::thrift::TType::I32,
                            size: val.len(),
                        };
                        protocol.write_list_begin(&list_ident)?;
                        for val in val {
                            protocol.write_i32(*val)?;
                        }
                        protocol.write_list_end()?;
                    }
                    protocol.write_list_end()?;
                    protocol.write_field_end()?;
                }
                {
                    let value = &self.name3;
                    protocol.write_field_begin(::pilota::thrift::TType::Map, 3i16)?;
                    let map_ident = ::pilota::thrift::TMapIdentifier {
                        key_type: ::pilota::thrift::TType::I32,
                        value_type: ::pilota::thrift::TType::List,
                        size: value.len(),
                    };
                    protocol.write_map_begin(&map_ident)?;
                    for (key, val) in value.iter() {
                        protocol.write_i32(*key)?;
                        let list_ident = ::pilota::thrift::TListIdentifier {
                            element_type: ::pilota::thrift::TType::I32,
                            size: val.len(),
                        };
                        protocol.write_list_begin(&list_ident)?;
                        for val in val {
                            protocol.write_i32(*val)?;
                        }
                        protocol.write_list_end()?;
                    }
                    protocol.write_map_end()?;
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
                let mut name2 = None;
                let mut name3 = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            id = Some(::std::sync::Arc::new(protocol.read_faststr()?));
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::List => {
                            name2 = Some({
                                let list_ident = protocol.read_list_begin()?;
                                let mut val = Vec::with_capacity(list_ident.size);
                                for _ in 0..list_ident.size {
                                    val.push({
                                        let list_ident = protocol.read_list_begin()?;
                                        let mut val = Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(::std::sync::Arc::new(protocol.read_i32()?));
                                        }
                                        protocol.read_list_end()?;
                                        val
                                    });
                                }
                                protocol.read_list_end()?;
                                val
                            });
                        }
                        Some(3i16) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                            name3 = Some({
                                let map_ident = protocol.read_map_begin()?;
                                let mut val =
                                    ::std::collections::HashMap::with_capacity(map_ident.size);
                                for _ in 0..map_ident.size {
                                    let el_key = protocol.read_i32()?;
                                    let el_val = {
                                        let list_ident = protocol.read_list_begin()?;
                                        let mut val = Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(::std::sync::Arc::new(protocol.read_i32()?));
                                        }
                                        protocol.read_list_end()?;
                                        val
                                    };
                                    val.insert(el_key, el_val);
                                }
                                protocol.read_map_end()?;
                                val
                            });
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
                let name2 = if let Some(name2) = name2 {
                    name2
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field name2 is required".to_string(),
                        ),
                    ));
                };
                let name3 = if let Some(name3) = name3 {
                    name3
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field name3 is required".to_string(),
                        ),
                    ));
                };
                let data = Self {
                    id: id,
                    name2: name2,
                    name3: name3,
                };
                Ok(data)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut id = None;
                let mut name2 = None;
                let mut name3 = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            id = Some(::std::sync::Arc::new(protocol.read_faststr().await?));
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::List => {
                            name2 = Some({
                                let list_ident = protocol.read_list_begin().await?;
                                let mut val = Vec::with_capacity(list_ident.size);
                                for _ in 0..list_ident.size {
                                    val.push({
                                        let list_ident = protocol.read_list_begin().await?;
                                        let mut val = Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(::std::sync::Arc::new(
                                                protocol.read_i32().await?,
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
                        Some(3i16) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                            name3 = Some({
                                let map_ident = protocol.read_map_begin().await?;
                                let mut val =
                                    ::std::collections::HashMap::with_capacity(map_ident.size);
                                for _ in 0..map_ident.size {
                                    let el_key = protocol.read_i32().await?;
                                    let el_val = {
                                        let list_ident = protocol.read_list_begin().await?;
                                        let mut val = Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(::std::sync::Arc::new(
                                                protocol.read_i32().await?,
                                            ));
                                        }
                                        protocol.read_list_end().await?;
                                        val
                                    };
                                    val.insert(el_key, el_val);
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
                let name2 = if let Some(name2) = name2 {
                    name2
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field name2 is required".to_string(),
                        ),
                    ));
                };
                let name3 = if let Some(name3) = name3 {
                    name3
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field name3 is required".to_string(),
                        ),
                    ));
                };
                let data = Self {
                    id: id,
                    name2: name2,
                    name3: name3,
                };
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
                    + {
                        let value = &self.name2;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("Name2"),
                            field_type: ::pilota::thrift::TType::List,
                            id: Some(2i16),
                        }) + {
                            let list_ident = ::pilota::thrift::TListIdentifier {
                                element_type: ::pilota::thrift::TType::List,
                                size: value.len(),
                            };
                            protocol.write_list_begin_len(&list_ident)
                                + {
                                    let mut size = 0;
                                    for el in value {
                                        size += {
                                            let list_ident = ::pilota::thrift::TListIdentifier {
                                                element_type: ::pilota::thrift::TType::I32,
                                                size: el.len(),
                                            };
                                            protocol.write_list_begin_len(&list_ident)
                                                + {
                                                    let mut size = 0;
                                                    for el in el {
                                                        size += protocol.write_i32_len(*el);
                                                    }
                                                    size
                                                }
                                                + protocol.write_list_end_len()
                                        };
                                    }
                                    size
                                }
                                + protocol.write_list_end_len()
                        } + protocol.write_field_end_len()
                    }
                    + {
                        let value = &self.name3;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("Name3"),
                            field_type: ::pilota::thrift::TType::Map,
                            id: Some(3i16),
                        }) + {
                            let map_id = ::pilota::thrift::TMapIdentifier {
                                key_type: ::pilota::thrift::TType::I32,
                                value_type: ::pilota::thrift::TType::List,
                                size: value.len(),
                            };
                            protocol.write_map_begin_len(&map_id)
                                + {
                                    let mut size = 0;
                                    for (key, val) in value {
                                        size += protocol.write_i32_len(*key);
                                        size += {
                                            let list_ident = ::pilota::thrift::TListIdentifier {
                                                element_type: ::pilota::thrift::TType::I32,
                                                size: val.len(),
                                            };
                                            protocol.write_list_begin_len(&list_ident)
                                                + {
                                                    let mut size = 0;
                                                    for el in val {
                                                        size += protocol.write_i32_len(*el);
                                                    }
                                                    size
                                                }
                                                + protocol.write_list_end_len()
                                        };
                                    }
                                    size
                                }
                                + protocol.write_map_end_len()
                        } + protocol.write_field_end_len()
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
                            req = Some(::std::sync::Arc::new(::pilota::thrift::Message::decode(
                                protocol,
                            )?));
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
        #[derive(Debug, Default, Clone, PartialEq)]
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
