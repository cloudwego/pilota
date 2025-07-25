pub mod r#gen {
    #![allow(warnings, clippy::all)]

    pub mod article {

        pub mod image {

            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct ImageServiceGetImageArgsRecv {
                pub req: GetImageRequest,
            }
            impl ::pilota::thrift::Message for ImageServiceGetImageArgsRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    __protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "ImageServiceGetImageArgsRecv",
                    };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    __protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                {
                    #[allow(unused_imports)]
                    use ::pilota::{Buf, thrift::TLengthProtocolExt};

                    let mut var_1 = None;

                    let mut __pilota_decoding_field_id = None;

                    __protocol.read_struct_begin()?;
                    if let ::std::result::Result::Err(mut err) = (|| {
                        loop {
                            let field_ident = __protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                __protocol.field_stop_len();
                                break;
                            } else {
                                __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type)?;
                                }
                            }

                            __protocol.read_field_end()?;
                            __protocol.field_end_len();
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ImageServiceGetImageArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end()?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field req is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { req: var_1 };
                    ::std::result::Result::Ok(data)
                }

                fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                    __protocol: &'a mut T,
                ) -> ::std::pin::Pin<
                    ::std::boxed::Box<
                        dyn ::std::future::Future<
                                Output = ::std::result::Result<
                                    Self,
                                    ::pilota::thrift::ThriftException,
                                >,
                            > + Send
                            + 'a,
                    >,
                > {
                    ::std::boxed::Box::pin(async move {
                        let mut var_1 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin().await?;
                        if let ::std::result::Result::Err(mut err) = async {
                    loop {


                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = Some(<GetImageRequest as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `ImageServiceGetImageArgsRecv` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                        __protocol.read_struct_end().await?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field req is required".to_string(),
                                ),
                            );
                        };

                        let data = Self { req: var_1 };
                        ::std::result::Result::Ok(data)
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "ImageServiceGetImageArgsRecv",
                    }) + __protocol.struct_field_len(Some(1), &self.req)
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
            impl ::std::default::Default for ImageServiceGetImageResultSend {
                fn default() -> Self {
                    ImageServiceGetImageResultSend::Ok(::std::default::Default::default())
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
            pub enum ImageServiceGetImageResultSend {
                Ok(GetImageResponse),
            }

            impl ::pilota::thrift::Message for ImageServiceGetImageResultSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    __protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "ImageServiceGetImageResultSend",
                    })?;
                    match self {
                        ImageServiceGetImageResultSend::Ok(value) => {
                            __protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    __protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                {
                    #[allow(unused_imports)]
                    use ::pilota::{Buf, thrift::TLengthProtocolExt};
                    let mut ret = None;
                    __protocol.read_struct_begin()?;
                    loop {
                        let field_ident = __protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __protocol.field_stop_len();
                            break;
                        } else {
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode(__protocol)?;
                                    __protocol.struct_len(&field_ident);
                                    ret = Some(ImageServiceGetImageResultSend::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            _ => {
                                __protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    __protocol.read_field_end()?;
                    __protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        ::std::result::Result::Ok(ret)
                    } else {
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                    __protocol: &'a mut T,
                ) -> ::std::pin::Pin<
                    ::std::boxed::Box<
                        dyn ::std::future::Future<
                                Output = ::std::result::Result<
                                    Self,
                                    ::pilota::thrift::ThriftException,
                                >,
                            > + Send
                            + 'a,
                    >,
                > {
                    ::std::boxed::Box::pin(async move {
                        let mut ret = None;
                        __protocol.read_struct_begin().await?;
                        loop {
                            let field_ident = __protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            match field_ident.id {
                                Some(0) => {
                                    if ret.is_none() {
                                        let field_ident = <GetImageResponse as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                        ret = Some(ImageServiceGetImageResultSend::Ok(field_ident));
                                    } else {
                                        return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message"
                                        ));
                                    }
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }
                        }
                        __protocol.read_field_end().await?;
                        __protocol.read_struct_end().await?;
                        if let Some(ret) = ret {
                            ::std::result::Result::Ok(ret)
                        } else {
                            ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received empty union from remote Message",
                            ))
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "ImageServiceGetImageResultSend",
                    }) + match self {
                        ImageServiceGetImageResultSend::Ok(value) => {
                            __protocol.struct_field_len(Some(0), value)
                        }
                    } + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct GetImageResponse {
                pub image: ::common::article::image::Image,
            }
            impl ::pilota::thrift::Message for GetImageResponse {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    __protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "GetImageResponse",
                    };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_struct_field(
                        1,
                        &self.image,
                        ::pilota::thrift::TType::Struct,
                    )?;
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    __protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                {
                    #[allow(unused_imports)]
                    use ::pilota::{Buf, thrift::TLengthProtocolExt};

                    let mut var_1 = None;

                    let mut __pilota_decoding_field_id = None;

                    __protocol.read_struct_begin()?;
                    if let ::std::result::Result::Err(mut err) = (|| {
                        loop {
                            let field_ident = __protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                __protocol.field_stop_len();
                                break;
                            } else {
                                __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type)?;
                                }
                            }

                            __protocol.read_field_end()?;
                            __protocol.field_end_len();
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `GetImageResponse` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end()?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field image is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { image: var_1 };
                    ::std::result::Result::Ok(data)
                }

                fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                    __protocol: &'a mut T,
                ) -> ::std::pin::Pin<
                    ::std::boxed::Box<
                        dyn ::std::future::Future<
                                Output = ::std::result::Result<
                                    Self,
                                    ::pilota::thrift::ThriftException,
                                >,
                            > + Send
                            + 'a,
                    >,
                > {
                    ::std::boxed::Box::pin(async move {
                        let mut var_1 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin().await?;
                        if let ::std::result::Result::Err(mut err) = async {
                    loop {


                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = Some(<::common::article::image::Image as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `GetImageResponse` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                        __protocol.read_struct_end().await?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field image is required".to_string(),
                                ),
                            );
                        };

                        let data = Self { image: var_1 };
                        ::std::result::Result::Ok(data)
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "GetImageResponse",
                    }) + __protocol.struct_field_len(Some(1), &self.image)
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct ImageServiceGetImageArgsSend {
                pub req: GetImageRequest,
            }
            impl ::pilota::thrift::Message for ImageServiceGetImageArgsSend {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    __protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "ImageServiceGetImageArgsSend",
                    };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    __protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                {
                    #[allow(unused_imports)]
                    use ::pilota::{Buf, thrift::TLengthProtocolExt};

                    let mut var_1 = None;

                    let mut __pilota_decoding_field_id = None;

                    __protocol.read_struct_begin()?;
                    if let ::std::result::Result::Err(mut err) = (|| {
                        loop {
                            let field_ident = __protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                __protocol.field_stop_len();
                                break;
                            } else {
                                __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type)?;
                                }
                            }

                            __protocol.read_field_end()?;
                            __protocol.field_end_len();
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ImageServiceGetImageArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end()?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field req is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { req: var_1 };
                    ::std::result::Result::Ok(data)
                }

                fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                    __protocol: &'a mut T,
                ) -> ::std::pin::Pin<
                    ::std::boxed::Box<
                        dyn ::std::future::Future<
                                Output = ::std::result::Result<
                                    Self,
                                    ::pilota::thrift::ThriftException,
                                >,
                            > + Send
                            + 'a,
                    >,
                > {
                    ::std::boxed::Box::pin(async move {
                        let mut var_1 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin().await?;
                        if let ::std::result::Result::Err(mut err) = async {
                    loop {


                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = Some(<GetImageRequest as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `ImageServiceGetImageArgsSend` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                        __protocol.read_struct_end().await?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field req is required".to_string(),
                                ),
                            );
                        };

                        let data = Self { req: var_1 };
                        ::std::result::Result::Ok(data)
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "ImageServiceGetImageArgsSend",
                    }) + __protocol.struct_field_len(Some(1), &self.req)
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct GetImageRequest {
                pub id: i64,
            }
            impl ::pilota::thrift::Message for GetImageRequest {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    __protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "GetImageRequest",
                    };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_i64_field(1, *&self.id)?;
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    __protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                {
                    #[allow(unused_imports)]
                    use ::pilota::{Buf, thrift::TLengthProtocolExt};

                    let mut var_1 = None;

                    let mut __pilota_decoding_field_id = None;

                    __protocol.read_struct_begin()?;
                    if let ::std::result::Result::Err(mut err) = (|| {
                        loop {
                            let field_ident = __protocol.read_field_begin()?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                __protocol.field_stop_len();
                                break;
                            } else {
                                __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                                {
                                    var_1 = Some(__protocol.read_i64()?);
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type)?;
                                }
                            }

                            __protocol.read_field_end()?;
                            __protocol.field_end_len();
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    })() {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `GetImageRequest` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end()?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field id is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { id: var_1 };
                    ::std::result::Result::Ok(data)
                }

                fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                    __protocol: &'a mut T,
                ) -> ::std::pin::Pin<
                    ::std::boxed::Box<
                        dyn ::std::future::Future<
                                Output = ::std::result::Result<
                                    Self,
                                    ::pilota::thrift::ThriftException,
                                >,
                            > + Send
                            + 'a,
                    >,
                > {
                    ::std::boxed::Box::pin(async move {
                        let mut var_1 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin().await?;
                        if let ::std::result::Result::Err(mut err) = async {
                            loop {
                                let field_ident = __protocol.read_field_begin().await?;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    break;
                                } else {
                                }
                                __pilota_decoding_field_id = field_ident.id;
                                match field_ident.id {
                                    Some(1)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::I64 =>
                                    {
                                        var_1 = Some(__protocol.read_i64().await?);
                                    }
                                    _ => {
                                        __protocol.skip(field_ident.field_type).await?;
                                    }
                                }

                                __protocol.read_field_end().await?;
                            }
                            ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                        }
                        .await
                        {
                            if let Some(field_id) = __pilota_decoding_field_id {
                                err.prepend_msg(&format!("decode struct `GetImageRequest` field(#{}) failed, caused by: ", field_id));
                            }
                            return ::std::result::Result::Err(err);
                        };
                        __protocol.read_struct_end().await?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field id is required".to_string(),
                                ),
                            );
                        };

                        let data = Self { id: var_1 };
                        ::std::result::Result::Ok(data)
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "GetImageRequest",
                    }) + __protocol.i64_field_len(Some(1), *&self.id)
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
            pub trait ImageService {}

            impl ::std::default::Default for ImageServiceGetImageResultRecv {
                fn default() -> Self {
                    ImageServiceGetImageResultRecv::Ok(::std::default::Default::default())
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
            pub enum ImageServiceGetImageResultRecv {
                Ok(GetImageResponse),
            }

            impl ::pilota::thrift::Message for ImageServiceGetImageResultRecv {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    __protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                        name: "ImageServiceGetImageResultRecv",
                    })?;
                    match self {
                        ImageServiceGetImageResultRecv::Ok(value) => {
                            __protocol.write_struct_field(
                                0,
                                value,
                                ::pilota::thrift::TType::Struct,
                            )?;
                        }
                    }
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    __protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                {
                    #[allow(unused_imports)]
                    use ::pilota::{Buf, thrift::TLengthProtocolExt};
                    let mut ret = None;
                    __protocol.read_struct_begin()?;
                    loop {
                        let field_ident = __protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __protocol.field_stop_len();
                            break;
                        } else {
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        ::pilota::thrift::Message::decode(__protocol)?;
                                    __protocol.struct_len(&field_ident);
                                    ret = Some(ImageServiceGetImageResultRecv::Ok(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            _ => {
                                __protocol.skip(field_ident.field_type)?;
                            }
                        }
                    }
                    __protocol.read_field_end()?;
                    __protocol.read_struct_end()?;
                    if let Some(ret) = ret {
                        ::std::result::Result::Ok(ret)
                    } else {
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                }

                fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                    __protocol: &'a mut T,
                ) -> ::std::pin::Pin<
                    ::std::boxed::Box<
                        dyn ::std::future::Future<
                                Output = ::std::result::Result<
                                    Self,
                                    ::pilota::thrift::ThriftException,
                                >,
                            > + Send
                            + 'a,
                    >,
                > {
                    ::std::boxed::Box::pin(async move {
                        let mut ret = None;
                        __protocol.read_struct_begin().await?;
                        loop {
                            let field_ident = __protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            match field_ident.id {
                                Some(0) => {
                                    if ret.is_none() {
                                        let field_ident = <GetImageResponse as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                        ret = Some(ImageServiceGetImageResultRecv::Ok(field_ident));
                                    } else {
                                        return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message"
                                        ));
                                    }
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }
                        }
                        __protocol.read_field_end().await?;
                        __protocol.read_struct_end().await?;
                        if let Some(ret) = ret {
                            ::std::result::Result::Ok(ret)
                        } else {
                            ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received empty union from remote Message",
                            ))
                        }
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "ImageServiceGetImageResultRecv",
                    }) + match self {
                        ImageServiceGetImageResultRecv::Ok(value) => {
                            __protocol.struct_field_len(Some(0), value)
                        }
                    } + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
            pub use ::common::article::image::Image;
            pub mod cdn {

                pub use ::common::article::image::cdn::Cdn;
            }
        }
    }

    pub mod common {

        pub use ::common::common::CommonData;
    }
    pub use article::image::*;
}
