pub mod string {
    #![allow(warnings, clippy::all)]
    pub mod string {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub faststr: ::pilota::FastStr,
            pub string: ::std::string::String,
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
                    let value = &self.faststr;
                    protocol.write_field_begin(::pilota::thrift::TType::Binary, 1i16)?;
                    protocol.write_faststr(value.clone())?;
                    protocol.write_field_end()?;
                }
                {
                    let value = &self.string;
                    protocol.write_field_begin(::pilota::thrift::TType::Binary, 2i16)?;
                    protocol.write_string(&value)?;
                    protocol.write_field_end()?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut faststr = None;
                let mut string = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            faststr = Some(protocol.read_faststr()?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            string = Some(protocol.read_string()?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let faststr = if let Some(faststr) = faststr {
                    faststr
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field faststr is required".to_string(),
                        ),
                    ));
                };
                let string = if let Some(string) = string {
                    string
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field string is required".to_string(),
                        ),
                    ));
                };
                let data = Self { faststr, string };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut faststr = None;
                let mut string = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            faststr = Some(protocol.read_faststr().await?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            string = Some(protocol.read_string().await?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let faststr = if let Some(faststr) = faststr {
                    faststr
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field faststr is required".to_string(),
                        ),
                    ));
                };
                let string = if let Some(string) = string {
                    string
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field string is required".to_string(),
                        ),
                    ));
                };
                let data = Self { faststr, string };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + {
                        let value = &self.faststr;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("faststr"),
                            field_type: ::pilota::thrift::TType::Binary,
                            id: Some(1i16),
                        }) + protocol.write_faststr_len(value)
                            + protocol.write_field_end_len()
                    }
                    + {
                        let value = &self.string;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("string"),
                            field_type: ::pilota::thrift::TType::Binary,
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
