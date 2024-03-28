pub mod void {
    #![allow(warnings, clippy::all)]

    pub mod void {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        pub enum TestTest123ResultRecv {
            #[derivative(Default)]
            Ok(()),
        }

        impl ::pilota::thrift::Message for TestTest123ResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultRecv",
                })?;
                match self {
                    TestTest123ResultRecv::Ok(ref value) => {}
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
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
                    Ok(TestTest123ResultRecv::Ok(()))
                }
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
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
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
                        Ok(TestTest123ResultRecv::Ok(()))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultRecv",
                }) + match self {
                    TestTest123ResultRecv::Ok(ref value) => 0,
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TestTest123ArgsSend {}
        impl ::pilota::thrift::Message for TestTest123ArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsSend",
                };

                protocol.write_struct_begin(&struct_ident)?;

                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

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
                            "decode struct `TestTest123ArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self {};
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
                            err.prepend_msg(&format!("decode struct `TestTest123ArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {};
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsSend",
                }) + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TestTest123ArgsRecv {}
        impl ::pilota::thrift::Message for TestTest123ArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsRecv",
                };

                protocol.write_struct_begin(&struct_ident)?;

                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

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
                            "decode struct `TestTest123ArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self {};
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
                            err.prepend_msg(&format!("decode struct `TestTest123ArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {};
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsRecv",
                }) + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        pub enum TestTest123ResultSend {
            #[derivative(Default)]
            Ok(()),
        }

        impl ::pilota::thrift::Message for TestTest123ResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultSend",
                })?;
                match self {
                    TestTest123ResultSend::Ok(ref value) => {}
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
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
                    Ok(TestTest123ResultSend::Ok(()))
                }
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
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
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
                        Ok(TestTest123ResultSend::Ok(()))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultSend",
                }) + match self {
                    TestTest123ResultSend::Ok(ref value) => 0,
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        pub trait Test {}
    }
}
