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
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
        )]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq)]
        pub struct A {
            #[serde(rename = "AA")]
            pub a: ::pilota::FastStr,

            pub b: i32,
        }
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_faststr_field(1, (&self.a).clone())?;
                protocol.write_i32_field(2, *&self.b)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut a = None;
                let mut b = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                a = Some(protocol.read_faststr()?);
                            }
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                b = Some(protocol.read_i32()?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `A` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let Some(a) = a else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field a is required".to_string(),
                    ));
                };
                let Some(b) = b else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field b is required".to_string(),
                    ));
                };

                let data = Self { a, b };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut a = None;
                    let mut b = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    a = Some(protocol.read_faststr().await?);
                                }
                                Some(2)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    b = Some(protocol.read_i32().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `A` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let Some(a) = a else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field a is required".to_string(),
                        ));
                    };
                    let Some(b) = b else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field b is required".to_string(),
                        ));
                    };

                    let data = Self { a, b };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + protocol.faststr_field_len(Some(1), &self.a)
                    + protocol.i32_field_len(Some(2), *&self.b)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize)]
        #[serde(untagged)]
        #[serde(transparent)]
        #[derive(Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct C(i32);

        impl C {
            pub const D: Self = Self(0);
            pub const E: Self = Self(1);

            pub fn inner(&self) -> i32 {
                self.0
            }
        }

        impl ::std::convert::From<i32> for C {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<C> for i32 {
            fn from(value: C) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for C {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(self.inner())?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let value = protocol.read_i32()?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        format!("invalid enum value for C, value: {}", value),
                    )
                })?)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let value = protocol.read_i32().await?;
                    Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                        ::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            format!("invalid enum value for C, value: {}", value),
                        )
                    })?)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.i32_len(self.inner())
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
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

        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(*(&**self))?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                Ok(B(protocol.read_i32()?))
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move { Ok(B(protocol.read_i32().await?)) })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.i32_len(*&**self)
            }
        }
    }
}
