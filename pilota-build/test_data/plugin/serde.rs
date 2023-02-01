pub mod serde {
    #![allow(warnings, clippy::all)]
    pub mod serde {
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            :: serde :: Serialize,
            :: serde :: Deserialize,
        )]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq)]
        pub struct A {
            #[serde(rename = "AA")]
            pub a: ::pilota::FastStr,
            pub b: i32,
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
                    let value = &self.a;
                    protocol.write_field_begin(::pilota::thrift::TType::Binary, 1i16)?;
                    protocol.write_faststr(value.clone())?;
                    protocol.write_field_end()?;
                }
                {
                    let value = &self.b;
                    protocol.write_field_begin(::pilota::thrift::TType::I32, 2i16)?;
                    protocol.write_i32(*value)?;
                    protocol.write_field_end()?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut a = None;
                let mut b = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            a = Some(protocol.read_faststr()?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            b = Some(protocol.read_i32()?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let a = if let Some(a) = a {
                    a
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field a is required".to_string(),
                        ),
                    ));
                };
                let b = if let Some(b) = b {
                    b
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field b is required".to_string(),
                        ),
                    ));
                };
                let data = Self { a, b };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut a = None;
                let mut b = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            a = Some(protocol.read_faststr().await?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            b = Some(protocol.read_i32().await?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let a = if let Some(a) = a {
                    a
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field a is required".to_string(),
                        ),
                    ));
                };
                let b = if let Some(b) = b {
                    b
                } else {
                    return Err(::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                            "field b is required".to_string(),
                        ),
                    ));
                };
                let data = Self { a, b };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + {
                        let value = &self.a;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("a"),
                            field_type: ::pilota::thrift::TType::Binary,
                            id: Some(1i16),
                        }) + protocol.write_faststr_len(value)
                            + protocol.write_field_end_len()
                    }
                    + {
                        let value = &self.b;
                        protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                            name: Some("b"),
                            field_type: ::pilota::thrift::TType::I32,
                            id: Some(2i16),
                        }) + protocol.write_i32_len(*value)
                            + protocol.write_field_end_len()
                    }
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            :: serde :: Serialize,
            :: serde :: Deserialize,
        )]
        #[serde(rename = "BB")]
        #[derive(Clone, PartialEq)]
        pub struct B(pub i32);
        impl ::std::ops::Deref for B {
            type Target = i32;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl From<i32> for B {
            fn from(v: i32) -> Self {
                Self(v)
            }
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                let value = &**self;
                protocol.write_i32(*value)?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                Ok(B(protocol.read_i32()?))
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                Ok(B(protocol.read_i32().await?))
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                {
                    let value = &**self;
                    protocol.write_i32_len(*value)
                }
            }
        }
        impl ::std::convert::From<C> for i32 {
            fn from(e: C) -> Self {
                e as _
            }
        }
        impl ::std::convert::TryFrom<i32> for C {
            type Error = ::pilota::EnumConvertError<i32>;
            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> Result<Self, ::pilota::EnumConvertError<i32>> {
                const D: i32 = C::D as i32;
                const E: i32 = C::E as i32;
                match v {
                    D => ::std::result::Result::Ok(C::D),
                    E => ::std::result::Result::Ok(C::E),
                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(v, "C")),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, :: pilota :: derivative :: Derivative)]
        #[derivative(Default)]
        #[derive(:: serde :: Serialize, :: serde :: Deserialize)]
        #[serde(untagged)]
        #[derive(Clone, PartialEq)]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum C {
            #[derivative(Default)]
            #[serde(rename = "DD")]
            D,
            E,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for C {
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
                        format!("invalid enum value for C, value: {}", value),
                    )
                })?)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let value = protocol.read_i32().await?;
                Ok(Self::try_from(value).map_err(|err| {
                    ::pilota::thrift::new_protocol_error(
                        ::pilota::thrift::ProtocolErrorKind::InvalidData,
                        format!("invalid enum value for C, value: {}", value),
                    )
                })?)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                protocol.write_i32_len(*self as i32)
            }
        }
    }
}
