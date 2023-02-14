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
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let data = Self {};
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let data = Self {};
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
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
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "TEST" };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_faststr_field(1i16, (&self.id).clone())?;
                protocol.write_list_field(
                    2i16,
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
                    3i16,
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
                            id = Some(protocol.read_faststr()?);
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
                                            val.push(::std::sync::Arc::new(
                                                ::pilota::thrift::Message::decode(protocol)?,
                                            ));
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
                                            val.push(::std::sync::Arc::new(
                                                ::pilota::thrift::Message::decode(protocol)?,
                                            ));
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
                let Some (id) = id else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field id is required" . to_string ()))) } ;
                let Some (name2) = name2 else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field name2 is required" . to_string ()))) } ;
                let Some (name3) = name3 else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field name3 is required" . to_string ()))) } ;
                let data = Self { id, name2, name3 };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
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
                            id = Some(protocol.read_faststr().await?);
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
                                                ::pilota::thrift::Message::decode_async(protocol)
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
                                                ::pilota::thrift::Message::decode_async(protocol)
                                                    .await?,
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
                let Some (id) = id else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field id is required" . to_string ()))) } ;
                let Some (name2) = name2 else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field name2 is required" . to_string ()))) } ;
                let Some (name3) = name3 else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field name3 is required" . to_string ()))) } ;
                let data = Self { id, name2, name3 };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol
                    .write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "TEST" })
                    + protocol.write_faststr_field_len(Some(1i16), &self.id)
                    + protocol.write_list_field_len(
                        Some(2i16),
                        ::pilota::thrift::TType::List,
                        &self.name2,
                        |protocol, el| {
                            protocol.write_list_len(
                                ::pilota::thrift::TType::Struct,
                                el,
                                |protocol, el| protocol.write_struct_len(el),
                            )
                        },
                    )
                    + protocol.write_map_field_len(
                        Some(3i16),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::List,
                        &self.name3,
                        |protocol, key| protocol.write_i32_len(*key),
                        |protocol, val| {
                            protocol.write_list_len(
                                ::pilota::thrift::TType::Struct,
                                val,
                                |protocol, el| protocol.write_struct_len(el),
                            )
                        },
                    )
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
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResult",
                })?;
                match self {
                    TestServiceTestResult::Ok(ref value) => {
                        protocol.write_faststr_field(0i16, (value).clone())?;
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
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
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
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResult",
                }) + match self {
                    TestServiceTestResult::Ok(ref value) => {
                        protocol.write_faststr_field_len(Some(0i16), value)
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
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsSend",
                };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1i16, &self.req)?;
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
                let Some (req) = req else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field req is required" . to_string ()))) } ;
                let data = Self { req };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
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
                let Some (req) = req else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field req is required" . to_string ()))) } ;
                let data = Self { req };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsSend",
                }) + protocol.write_struct_field_len(Some(1i16), &self.req)
                    + protocol.write_field_stop_len()
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
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsRecv",
                };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1i16, &self.req)?;
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
                let Some (req) = req else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field req is required" . to_string ()))) } ;
                let data = Self { req };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
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
                let Some (req) = req else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field req is required" . to_string ()))) } ;
                let data = Self { req };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsRecv",
                }) + protocol.write_struct_field_len(Some(1i16), &self.req)
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[::async_trait::async_trait]
        pub trait TestService {}
    }
}
