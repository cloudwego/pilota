pub mod ttstream_codec {
    #![allow(warnings, clippy::all)]

    pub mod codec {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Nested {
            pub value: i32,

            pub label: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::thrift::Message for Nested {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Nested" };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_i32_field(1, *&self.value)?;
                if let ::std::option::Option::Some(value) = self.label.as_ref() {
                    __protocol.write_faststr_field(2, (value).clone())?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;
                let mut var_2 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                            {
                                var_1 = ::std::option::Option::Some(__protocol.read_i32()?);
                            }
                            ::std::option::Option::Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_2 = ::std::option::Option::Some(__protocol.read_faststr()?);
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `Nested` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };

                let data = Self {
                    value: var_1,
                    label: var_2,
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
                    let mut var_1 = ::std::option::Option::None;
                    let mut var_2 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_1 =
                                        ::std::option::Option::Some(__protocol.read_i32().await?);
                                }
                                ::std::option::Option::Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_2 = ::std::option::Option::Some(
                                        __protocol.read_faststr().await?,
                                    );
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `Nested` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        value: var_1,
                        label: var_2,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Nested" })
                    + __protocol.i32_field_len(::std::option::Option::Some(1), *&self.value)
                    + self.label.as_ref().map_or(0, |value| {
                        __protocol.faststr_field_len(::std::option::Option::Some(2), value)
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct CodecServiceUnaryArgsSend {
            pub request: CodecMessage,
        }
        impl ::pilota::thrift::Message for CodecServiceUnaryArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "CodecServiceUnaryArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!("decode struct `CodecServiceUnaryArgsSend` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        <CodecMessage as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `CodecServiceUnaryArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "CodecServiceUnaryArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct CodecServicePlainArgsSend {
            pub request: CodecMessage,
        }
        impl ::pilota::thrift::Message for CodecServicePlainArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "CodecServicePlainArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!("decode struct `CodecServicePlainArgsSend` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        <CodecMessage as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `CodecServicePlainArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "CodecServicePlainArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for CodecMessage {
            fn default() -> Self {
                CodecMessage {
                    integer: ::std::default::Default::default(),
                    flag: ::std::default::Default::default(),
                    real: ::std::default::Default::default(),
                    text: ::std::default::Default::default(),
                    blob: ::std::default::Default::default(),
                    numbers: ::std::default::Default::default(),
                    nested: ::std::default::Default::default(),
                    note: ::std::default::Default::default(),
                    ids: ::std::default::Default::default(),
                    child: ::std::default::Default::default(),
                    children: ::std::default::Default::default(),
                    default_text: ::std::option::Option::Some(::pilota::FastStr::from_static_str(
                        "default",
                    )),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CodecMessage {
            pub integer: ::std::option::Option<i64>,

            pub flag: ::std::option::Option<bool>,

            pub real: ::std::option::Option<f64>,

            pub text: ::std::option::Option<::pilota::FastStr>,

            pub blob: ::std::option::Option<::pilota::Bytes>,

            pub numbers: ::std::option::Option<::std::vec::Vec<i32>>,

            pub nested:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<i64>>>,

            pub note: ::std::option::Option<::pilota::FastStr>,

            pub ids: ::std::option::Option<::pilota::AHashSet<i16>>,

            pub child: ::std::option::Option<Nested>,

            pub children: ::std::option::Option<::std::vec::Vec<Nested>>,

            pub default_text: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::thrift::Message for CodecMessage {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "CodecMessage",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                if let ::std::option::Option::Some(value) = self.integer.as_ref() {
                    __protocol.write_i64_field(1, *value)?;
                }
                if let ::std::option::Option::Some(value) = self.flag.as_ref() {
                    __protocol.write_bool_field(2, *value)?;
                }
                if let ::std::option::Option::Some(value) = self.real.as_ref() {
                    __protocol.write_double_field(3, *value)?;
                }
                if let ::std::option::Option::Some(value) = self.text.as_ref() {
                    __protocol.write_faststr_field(4, (value).clone())?;
                }
                if let ::std::option::Option::Some(value) = self.blob.as_ref() {
                    __protocol.write_bytes_field(5, (value).clone())?;
                }
                if let ::std::option::Option::Some(value) = self.numbers.as_ref() {
                    __protocol.write_list_field(
                        6,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let ::std::option::Option::Some(value) = self.nested.as_ref() {
                    __protocol.write_map_field(
                        7,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_list(
                                ::pilota::thrift::TType::I64,
                                &val,
                                |__protocol, val| {
                                    __protocol.write_i64(*val)?;
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let ::std::option::Option::Some(value) = self.note.as_ref() {
                    __protocol.write_faststr_field(8, (value).clone())?;
                }
                if let ::std::option::Option::Some(value) = self.ids.as_ref() {
                    __protocol.write_set_field(
                        9,
                        ::pilota::thrift::TType::I16,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i16(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let ::std::option::Option::Some(value) = self.child.as_ref() {
                    __protocol.write_struct_field(10, value, ::pilota::thrift::TType::Struct)?;
                }
                if let ::std::option::Option::Some(value) = self.children.as_ref() {
                    __protocol.write_list_field(
                        11,
                        ::pilota::thrift::TType::Struct,
                        &value,
                        |__protocol, val| {
                            __protocol.write_struct(val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let ::std::option::Option::Some(value) = self.default_text.as_ref() {
                    __protocol.write_faststr_field(12, (value).clone())?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;
                let mut var_2 = ::std::option::Option::None;
                let mut var_3 = ::std::option::Option::None;
                let mut var_4 = ::std::option::Option::None;
                let mut var_5 = ::std::option::Option::None;
                let mut var_6 = ::std::option::Option::None;
                let mut var_7 = ::std::option::Option::None;
                let mut var_8 = ::std::option::Option::None;
                let mut var_9 = ::std::option::Option::None;
                let mut var_10 = ::std::option::Option::None;
                let mut var_11 = ::std::option::Option::None;
                let mut var_12 =
                    ::std::option::Option::Some(::pilota::FastStr::from_static_str("default"));

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                            {
                                var_1 = ::std::option::Option::Some(__protocol.read_i64()?);
                            }
                            ::std::option::Option::Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Bool =>
                            {
                                var_2 = ::std::option::Option::Some(__protocol.read_bool()?);
                            }
                            ::std::option::Option::Some(3)
                                if field_ident.field_type == ::pilota::thrift::TType::Double =>
                            {
                                var_3 = ::std::option::Option::Some(__protocol.read_double()?);
                            }
                            ::std::option::Option::Some(4)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_4 = ::std::option::Option::Some(__protocol.read_faststr()?);
                            }
                            ::std::option::Option::Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_5 = ::std::option::Option::Some(__protocol.read_bytes()?);
                            }
                            ::std::option::Option::Some(6)
                                if field_ident.field_type == ::pilota::thrift::TType::List =>
                            {
                                var_6 = ::std::option::Option::Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<i32> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(__protocol.read_i32()?);
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                });
                            }
                            ::std::option::Option::Some(7)
                                if field_ident.field_type == ::pilota::thrift::TType::Map =>
                            {
                                var_7 = ::std::option::Option::Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<i64> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(__protocol.read_i64()?);
                                            }
                                            val.set_len(list_ident.size);
                                            __protocol.read_list_end()?;
                                            val
                                        });
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            ::std::option::Option::Some(8)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_8 = ::std::option::Option::Some(__protocol.read_faststr()?);
                            }
                            ::std::option::Option::Some(9)
                                if field_ident.field_type == ::pilota::thrift::TType::Set =>
                            {
                                var_9 = ::std::option::Option::Some({
                                    let list_ident = __protocol.read_set_begin()?;
                                    let mut val =
                                        ::pilota::AHashSet::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.insert(__protocol.read_i16()?);
                                    }
                                    __protocol.read_set_end()?;
                                    val
                                });
                            }
                            ::std::option::Option::Some(10)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_10 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                            }
                            ::std::option::Option::Some(11)
                                if field_ident.field_type == ::pilota::thrift::TType::List =>
                            {
                                var_11 = ::std::option::Option::Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<Nested> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(::pilota::thrift::Message::decode(__protocol)?);
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                });
                            }
                            ::std::option::Option::Some(12)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_12 = ::std::option::Option::Some(__protocol.read_faststr()?);
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `CodecMessage` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self {
                    integer: var_1,
                    flag: var_2,
                    real: var_3,
                    text: var_4,
                    blob: var_5,
                    numbers: var_6,
                    nested: var_7,
                    note: var_8,
                    ids: var_9,
                    child: var_10,
                    children: var_11,
                    default_text: var_12,
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
                    let mut var_1 = ::std::option::Option::None;
                    let mut var_2 = ::std::option::Option::None;
                    let mut var_3 = ::std::option::Option::None;
                    let mut var_4 = ::std::option::Option::None;
                    let mut var_5 = ::std::option::Option::None;
                    let mut var_6 = ::std::option::Option::None;
                    let mut var_7 = ::std::option::Option::None;
                    let mut var_8 = ::std::option::Option::None;
                    let mut var_9 = ::std::option::Option::None;
                    let mut var_10 = ::std::option::Option::None;
                    let mut var_11 = ::std::option::Option::None;
                    let mut var_12 =
                        ::std::option::Option::Some(::pilota::FastStr::from_static_str("default"));

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                    ::std::option::Option::Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64  => {
                    var_1 = ::std::option::Option::Some(__protocol.read_i64().await?);

                },::std::option::Option::Some(2) if field_ident.field_type == ::pilota::thrift::TType::Bool  => {
                    var_2 = ::std::option::Option::Some(__protocol.read_bool().await?);

                },::std::option::Option::Some(3) if field_ident.field_type == ::pilota::thrift::TType::Double  => {
                    var_3 = ::std::option::Option::Some(__protocol.read_double().await?);

                },::std::option::Option::Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_4 = ::std::option::Option::Some(__protocol.read_faststr().await?);

                },::std::option::Option::Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_5 = ::std::option::Option::Some(__protocol.read_bytes().await?);

                },::std::option::Option::Some(6) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_6 = ::std::option::Option::Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_i32().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },::std::option::Option::Some(7) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_7 = ::std::option::Option::Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_i64().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },::std::option::Option::Some(8) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_8 = ::std::option::Option::Some(__protocol.read_faststr().await?);

                },::std::option::Option::Some(9) if field_ident.field_type == ::pilota::thrift::TType::Set  => {
                    var_9 = ::std::option::Option::Some({let list_ident = __protocol.read_set_begin().await?;
                    let mut val = ::pilota::AHashSet::with_capacity(list_ident.size);
                    for _ in 0..list_ident.size {
                        val.insert(__protocol.read_i16().await?);
                    };
                    __protocol.read_set_end().await?;
                    val});

                },::std::option::Option::Some(10) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_10 = ::std::option::Option::Some(<Nested as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },::std::option::Option::Some(11) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_11 = ::std::option::Option::Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<Nested as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },::std::option::Option::Some(12) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_12 = ::std::option::Option::Some(__protocol.read_faststr().await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `CodecMessage` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let data = Self {
                        integer: var_1,
                        flag: var_2,
                        real: var_3,
                        text: var_4,
                        blob: var_5,
                        numbers: var_6,
                        nested: var_7,
                        note: var_8,
                        ids: var_9,
                        child: var_10,
                        children: var_11,
                        default_text: var_12,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "CodecMessage",
                }) + self.integer.as_ref().map_or(0, |value| {
                    __protocol.i64_field_len(::std::option::Option::Some(1), *value)
                }) + self.flag.as_ref().map_or(0, |value| {
                    __protocol.bool_field_len(::std::option::Option::Some(2), *value)
                }) + self.real.as_ref().map_or(0, |value| {
                    __protocol.double_field_len(::std::option::Option::Some(3), *value)
                }) + self.text.as_ref().map_or(0, |value| {
                    __protocol.faststr_field_len(::std::option::Option::Some(4), value)
                }) + self.blob.as_ref().map_or(0, |value| {
                    __protocol.bytes_field_len(::std::option::Option::Some(5), value)
                }) + self.numbers.as_ref().map_or(0, |value| {
                    __protocol.list_field_len(
                        ::std::option::Option::Some(6),
                        ::pilota::thrift::TType::I32,
                        value,
                        |__protocol, el| __protocol.i32_len(*el),
                    )
                }) + self.nested.as_ref().map_or(0, |value| {
                    __protocol.map_field_len(
                        ::std::option::Option::Some(7),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        value,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::I64,
                                val,
                                |__protocol, el| __protocol.i64_len(*el),
                            )
                        },
                    )
                }) + self.note.as_ref().map_or(0, |value| {
                    __protocol.faststr_field_len(::std::option::Option::Some(8), value)
                }) + self.ids.as_ref().map_or(0, |value| {
                    __protocol.set_field_len(
                        ::std::option::Option::Some(9),
                        ::pilota::thrift::TType::I16,
                        value,
                        |__protocol, el| __protocol.i16_len(*el),
                    )
                }) + self.child.as_ref().map_or(0, |value| {
                    __protocol.struct_field_len(::std::option::Option::Some(10), value)
                }) + self.children.as_ref().map_or(0, |value| {
                    __protocol.list_field_len(
                        ::std::option::Option::Some(11),
                        ::pilota::thrift::TType::Struct,
                        value,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                }) + self.default_text.as_ref().map_or(0, |value| {
                    __protocol.faststr_field_len(::std::option::Option::Some(12), value)
                }) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct CodecServiceUnaryArgsRecv {
            pub request: CodecMessage,
        }
        impl ::pilota::thrift::Message for CodecServiceUnaryArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "CodecServiceUnaryArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!("decode struct `CodecServiceUnaryArgsRecv` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        <CodecMessage as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `CodecServiceUnaryArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "CodecServiceUnaryArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for CodecServiceUnaryResultSend {
            fn default() -> Self {
                CodecServiceUnaryResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum CodecServiceUnaryResultSend {
            Ok(CodecMessage),
        }

        impl ::pilota::thrift::Message for CodecServiceUnaryResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "CodecServiceUnaryResultSend",
                })?;
                match self {
                    CodecServiceUnaryResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                let mut ret = ::std::option::Option::None;
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
                        ::std::option::Option::Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(CodecServiceUnaryResultSend::Ok(
                                    field_ident,
                                ));
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
                if let ::std::option::Option::Some(ret) = ret {
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            ::std::option::Option::Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <CodecMessage as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(
                                        CodecServiceUnaryResultSend::Ok(field_ident),
                                    );
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
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
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
                    name: "CodecServiceUnaryResultSend",
                }) + match self {
                    CodecServiceUnaryResultSend::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct CodecServicePlainArgsRecv {
            pub request: CodecMessage,
        }
        impl ::pilota::thrift::Message for CodecServicePlainArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "CodecServicePlainArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!("decode struct `CodecServicePlainArgsRecv` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        <CodecMessage as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `CodecServicePlainArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "CodecServicePlainArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for CodecServicePlainResultSend {
            fn default() -> Self {
                CodecServicePlainResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum CodecServicePlainResultSend {
            Ok(CodecMessage),
        }

        impl ::pilota::thrift::Message for CodecServicePlainResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "CodecServicePlainResultSend",
                })?;
                match self {
                    CodecServicePlainResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                let mut ret = ::std::option::Option::None;
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
                        ::std::option::Option::Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(CodecServicePlainResultSend::Ok(
                                    field_ident,
                                ));
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
                if let ::std::option::Option::Some(ret) = ret {
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            ::std::option::Option::Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <CodecMessage as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(
                                        CodecServicePlainResultSend::Ok(field_ident),
                                    );
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
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
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
                    name: "CodecServicePlainResultSend",
                }) + match self {
                    CodecServicePlainResultSend::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct MessageAlias(pub CodecMessage);

        impl ::std::ops::Deref for MessageAlias {
            type Target = CodecMessage;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<CodecMessage> for MessageAlias {
            fn from(v: CodecMessage) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for MessageAlias {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct((&**self))?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                ::std::result::Result::Ok(MessageAlias(::pilota::thrift::Message::decode(
                    __protocol,
                )?))
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
                    ::std::result::Result::Ok(MessageAlias(
                        <CodecMessage as ::pilota::thrift::Message>::decode_async(__protocol)
                            .await?,
                    ))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_len(&**self)
            }
        }
        pub trait CodecService {}

        impl ::std::default::Default for CodecServiceUnaryResultRecv {
            fn default() -> Self {
                CodecServiceUnaryResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum CodecServiceUnaryResultRecv {
            Ok(CodecMessage),
        }

        impl ::pilota::thrift::Message for CodecServiceUnaryResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "CodecServiceUnaryResultRecv",
                })?;
                match self {
                    CodecServiceUnaryResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                let mut ret = ::std::option::Option::None;
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
                        ::std::option::Option::Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(CodecServiceUnaryResultRecv::Ok(
                                    field_ident,
                                ));
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
                if let ::std::option::Option::Some(ret) = ret {
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            ::std::option::Option::Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <CodecMessage as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(
                                        CodecServiceUnaryResultRecv::Ok(field_ident),
                                    );
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
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
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
                    name: "CodecServiceUnaryResultRecv",
                }) + match self {
                    CodecServiceUnaryResultRecv::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for CodecServicePlainResultRecv {
            fn default() -> Self {
                CodecServicePlainResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum CodecServicePlainResultRecv {
            Ok(CodecMessage),
        }

        impl ::pilota::thrift::Message for CodecServicePlainResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "CodecServicePlainResultRecv",
                })?;
                match self {
                    CodecServicePlainResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                let mut ret = ::std::option::Option::None;
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
                        ::std::option::Option::Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(CodecServicePlainResultRecv::Ok(
                                    field_ident,
                                ));
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
                if let ::std::option::Option::Some(ret) = ret {
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            ::std::option::Option::Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <CodecMessage as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(
                                        CodecServicePlainResultRecv::Ok(field_ident),
                                    );
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
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
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
                    name: "CodecServicePlainResultRecv",
                }) + match self {
                    CodecServicePlainResultRecv::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
