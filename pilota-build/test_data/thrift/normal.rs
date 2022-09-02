pub mod normal {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::unused_unit,
        clippy::needless_borrow,
        unused_mut
    )]
    pub mod normal {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub a: ::std::option::Option<i32>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };
                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.a.as_ref() {
                    let field = ::pilota::thrift::TFieldIdentifier {
                        name: Some("a"),
                        field_type: ::pilota::thrift::TType::I32,
                        id: Some(1i16),
                    };
                    protocol.write_field_begin(&field)?;
                    protocol.write_i32(*value)?;
                    protocol.write_field_end()?;
                };
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut a = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) => {
                            if field_ident.field_type == ::pilota::thrift::TType::I32 {
                                a = Some(protocol.read_i32()?);
                            } else {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let data = Self { a };
                Ok(data)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut a = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) => {
                            if field_ident.field_type == ::pilota::thrift::TType::I32 {
                                a = Some(protocol.read_i32().await?);
                            } else {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let data = Self { a };
                Ok(data)
            }
        }
        impl ::pilota::thrift::Size for A {
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &T) -> usize {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + if let Some(value) = self.a.as_ref() {
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("a"),
                            field_type: ::pilota::thrift::TType::I32,
                            id: Some(1i16),
                        }) + protocol.write_i32_len(*value)
                            + protocol.write_field_end_len()
                    } else {
                        0
                    }
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct B {
            pub a: ::std::option::Option<A>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "b" };
                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.a.as_ref() {
                    let field = ::pilota::thrift::TFieldIdentifier {
                        name: Some("a"),
                        field_type: ::pilota::thrift::TType::Struct,
                        id: Some(2i16),
                    };
                    protocol.write_field_begin(&field)?;
                    ::pilota::thrift::Message::encode(value, protocol)?;
                    protocol.write_field_end()?;
                };
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut a = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(2i16) => {
                            if field_ident.field_type == ::pilota::thrift::TType::Struct {
                                a = Some(::pilota::thrift::Message::decode(protocol)?);
                            } else {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let data = Self { a };
                Ok(data)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut a = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(2i16) => {
                            if field_ident.field_type == ::pilota::thrift::TType::Struct {
                                a = Some(::pilota::thrift::Message::decode_async(protocol).await?);
                            } else {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let data = Self { a };
                Ok(data)
            }
        }
        impl ::pilota::thrift::Size for B {
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &T) -> usize {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "b" })
                    + if let Some(value) = self.a.as_ref() {
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("a"),
                            field_type: ::pilota::thrift::TType::Struct,
                            id: Some(2i16),
                        }) + ::pilota::thrift::Size::size(value, protocol)
                            + protocol.write_field_end_len()
                    } else {
                        0
                    }
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
    }
}
