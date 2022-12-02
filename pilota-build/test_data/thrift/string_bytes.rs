pub mod string_bytes {
    #![allow(warnings, clippy::all)]
    pub mod string_bytes {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub smol: ::pilota::SmolStr,
            pub bytes: ::pilota::Bytes,
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
                    let value = &self.smol;
                    protocol.write_field_begin(::pilota::thrift::TType::Binary, 1i16)?;
                    protocol.write_smolstr(value.clone())?;
                    protocol.write_field_end()?;
                }
                {
                    let value = &self.bytes;
                    protocol.write_field_begin(::pilota::thrift::TType::Binary, 2i16)?;
                    protocol.write_bytes(value.clone())?;
                    protocol.write_field_end()?;
                }
                {
                    let value = &self.string;
                    protocol.write_field_begin(::pilota::thrift::TType::Binary, 3i16)?;
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
                let mut smol = None;
                let mut bytes = None;
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
                            smol = Some(protocol.read_smolstr()?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            bytes = Some(protocol.read_bytes()?);
                        }
                        Some(3i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            string = Some(protocol.read_string()?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let smol = if let Some(smol) = smol {
                    smol
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field smol is required".to_string(),
                        ),
                    ));
                };
                let bytes = if let Some(bytes) = bytes {
                    bytes
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field bytes is required".to_string(),
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
                let data = Self {
                    smol: smol,
                    bytes: bytes,
                    string: string,
                };
                Ok(data)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut smol = None;
                let mut bytes = None;
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
                            smol = Some(protocol.read_smolstr().await?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            bytes = Some(protocol.read_bytes().await?);
                        }
                        Some(3i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            string = Some(protocol.read_string().await?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let smol = if let Some(smol) = smol {
                    smol
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field smol is required".to_string(),
                        ),
                    ));
                };
                let bytes = if let Some(bytes) = bytes {
                    bytes
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field bytes is required".to_string(),
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
                let data = Self {
                    smol: smol,
                    bytes: bytes,
                    string: string,
                };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + {
                        let value = &self.smol;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("smol"),
                            field_type: ::pilota::thrift::TType::Binary,
                            id: Some(1i16),
                        }) + protocol.write_smolstr_len(value)
                            + protocol.write_field_end_len()
                    }
                    + {
                        let value = &self.bytes;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("bytes"),
                            field_type: ::pilota::thrift::TType::Binary,
                            id: Some(2i16),
                        }) + protocol.write_bytes_len(value)
                            + protocol.write_field_end_len()
                    }
                    + {
                        let value = &self.string;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("string"),
                            field_type: ::pilota::thrift::TType::Binary,
                            id: Some(3i16),
                        }) + protocol.write_string_len(&value)
                            + protocol.write_field_end_len()
                    }
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
    }
}
