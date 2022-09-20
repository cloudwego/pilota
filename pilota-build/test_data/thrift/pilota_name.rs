pub mod pilota_name {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::unused_unit,
        clippy::needless_borrow,
        unused_mut
    )]
    pub mod pilota_name {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Test {
            pub id: ::std::string::String,
            pub hello: ::std::string::String,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for Test {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Test" };
                protocol.write_struct_begin(&struct_ident)?;
                {
                    let value = &self.id;
                    let field = ::pilota::thrift::TFieldIdentifier {
                        name: Some("ID"),
                        field_type: ::pilota::thrift::TType::String,
                        id: Some(1i16),
                    };
                    protocol.write_field_begin(&field)?;
                    protocol.write_string(value)?;
                    protocol.write_field_end()?;
                }
                {
                    let value = &self.hello;
                    let field = ::pilota::thrift::TFieldIdentifier {
                        name: Some("Id"),
                        field_type: ::pilota::thrift::TType::String,
                        id: Some(2i16),
                    };
                    protocol.write_field_begin(&field)?;
                    protocol.write_string(value)?;
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
                let mut hello = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::String => {
                            id = Some(protocol.read_string()?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::String => {
                            hello = Some(protocol.read_string()?);
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
                let hello = if let Some(hello) = hello {
                    hello
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field hello is required".to_string(),
                        ),
                    ));
                };
                let data = Self { id, hello };
                Ok(data)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut id = None;
                let mut hello = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::String => {
                            id = Some(protocol.read_string().await?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::String => {
                            hello = Some(protocol.read_string().await?);
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
                let hello = if let Some(hello) = hello {
                    hello
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field hello is required".to_string(),
                        ),
                    ));
                };
                let data = Self { id, hello };
                Ok(data)
            }
        }
        impl ::pilota::thrift::Size for Test {
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &T) -> usize {
                protocol
                    .write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Test" })
                    + {
                        let value = &self.id;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("ID"),
                            field_type: ::pilota::thrift::TType::String,
                            id: Some(1i16),
                        }) + protocol.write_string_len(&value)
                            + protocol.write_field_end_len()
                    }
                    + {
                        let value = &self.hello;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("Id"),
                            field_type: ::pilota::thrift::TType::String,
                            id: Some(2i16),
                        }) + protocol.write_string_len(&value)
                            + protocol.write_field_end_len()
                    }
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
    }
}
