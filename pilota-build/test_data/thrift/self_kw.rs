pub mod self_kw {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::unused_unit,
        clippy::needless_borrow,
        unused_mut
    )]
    pub mod self_kw {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, :: pilota :: derivative :: Derivative)]
        #[derivative(Default)]
        #[derive(
            :: pilota :: num_enum :: IntoPrimitive,
            :: pilota :: num_enum :: TryFromPrimitive,
            Clone,
            PartialEq,
        )]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum Index {
            #[derivative(Default)]
            A = 0i32,
            Self_ = 1i32,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for Index {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                protocol.write_i32(*self as i32)?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let value = protocol.read_i32()?;
                Ok(Self::try_from(value).map_err(|err| {
                    ::pilota::thrift::new_protocol_error(
                        ::pilota::thrift::ProtocolErrorKind::InvalidData,
                        format!("invalid enum value for Index, value: {}", value),
                    )
                })?)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let value = protocol.read_i32().await?;
                Ok(Self::try_from(value).map_err(|err| {
                    ::pilota::thrift::new_protocol_error(
                        ::pilota::thrift::ProtocolErrorKind::InvalidData,
                        format!("invalid enum value for Index, value: {}", value),
                    )
                })?)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &T) -> usize {
                protocol.write_i32_len(*self as i32)
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub r#type: ::std::string::String,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };
                protocol.write_struct_begin(&struct_ident)?;
                {
                    let value = &self.r#type;
                    protocol.write_field_begin(::pilota::thrift::TType::String, 1i16)?;
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
                let mut r#type = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::String => {
                            r#type = Some(protocol.read_string()?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let r#type = if let Some(r#type) = r#type {
                    r#type
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field r#type is required".to_string(),
                        ),
                    ));
                };
                let data = Self { r#type };
                Ok(data)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut r#type = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::String => {
                            r#type = Some(protocol.read_string().await?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let r#type = if let Some(r#type) = r#type {
                    r#type
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field r#type is required".to_string(),
                        ),
                    ));
                };
                let data = Self { r#type };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &T) -> usize {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + {
                        let value = &self.r#type;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("type"),
                            field_type: ::pilota::thrift::TType::String,
                            id: Some(1i16),
                        }) + protocol.write_string_len(&value)
                            + protocol.write_field_end_len()
                    }
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
    }
}
