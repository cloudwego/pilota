pub mod serde_preserve_idl_names {
    #![allow(warnings, clippy::all)]

    pub mod serde_preserve_idl_names {

        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct A {
            #[serde(rename = "type")]
            pub r#type: ::pilota::FastStr,

            #[serde(rename = "SomeField")]
            pub some_field: ::pilota::FastStr,

            #[serde(rename = "CC")]
            pub c: ::pilota::FastStr,

            #[serde(alias = "renamed_value")]
            #[serde(rename = "d")]
            pub d: ::pilota::FastStr,
        }
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.r#type).clone())?;
                __protocol.write_faststr_field(2, (&self.some_field).clone())?;
                __protocol.write_faststr_field(3, (&self.c).clone())?;
                __protocol.write_faststr_field(4, (&self.d).clone())?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;

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
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_1 = Some(__protocol.read_faststr()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_2 = Some(__protocol.read_faststr()?);
                            }
                            Some(3)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_3 = Some(__protocol.read_faststr()?);
                            }
                            Some(4)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_4 = Some(__protocol.read_faststr()?);
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
                            "decode struct `A` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field r#type is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field some_field is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field c is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field d is required".to_string(),
                    ));
                };

                let data = Self {
                    r#type: var_1,
                    some_field: var_2,
                    c: var_3,
                    d: var_4,
                };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut var_1 = None;
                    let mut var_2 = None;
                    let mut var_3 = None;
                    let mut var_4 = None;

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
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_1 = Some(__protocol.read_faststr().await?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_2 = Some(__protocol.read_faststr().await?);
                                }
                                Some(3)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_3 = Some(__protocol.read_faststr().await?);
                                }
                                Some(4)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_4 = Some(__protocol.read_faststr().await?);
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
                            err.prepend_msg(&format!(
                                "decode struct `A` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field r#type is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field some_field is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field c is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field d is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        r#type: var_1,
                        some_field: var_2,
                        c: var_3,
                        d: var_4,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + __protocol.faststr_field_len(Some(1), &self.r#type)
                    + __protocol.faststr_field_len(Some(2), &self.some_field)
                    + __protocol.faststr_field_len(Some(3), &self.c)
                    + __protocol.faststr_field_len(Some(4), &self.d)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
