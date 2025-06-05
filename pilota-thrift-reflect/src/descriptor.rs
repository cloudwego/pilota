pub mod descriptor {
    #![allow(warnings, clippy::all)]

    pub mod thrift_reflection {
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TypeDescriptor {
            pub filepath: ::pilota::FastStr,

            pub name: ::pilota::FastStr,

            pub key_type: ::std::option::Option<::std::boxed::Box<TypeDescriptor>>,

            pub value_type: ::std::option::Option<::std::boxed::Box<TypeDescriptor>>,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for TypeDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypeDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_faststr_field(2, (&self.name).clone())?;
                if let Some(value) = self.key_type.as_ref() {
                    __protocol.write_struct_field(3, value, ::pilota::thrift::TType::Struct)?;
                }
                if let Some(value) = self.value_type.as_ref() {
                    __protocol.write_struct_field(4, value, ::pilota::thrift::TType::Struct)?;
                }
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        5,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;

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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_3 = Some(::std::boxed::Box::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
                            }
                            Some(4)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_4 = Some(::std::boxed::Box::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
                            }
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_5 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `TypeDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    name: var_2,
                    key_type: var_3,
                    value_type: var_4,
                    extra: var_5,
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
                    let mut var_5 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_3 = Some(::std::boxed::Box::new(<TypeDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?));

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_4 = Some(::std::boxed::Box::new(<TypeDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?));

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_5 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

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
                    err.prepend_msg(&format!("decode struct `TypeDescriptor` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        name: var_2,
                        key_type: var_3,
                        value_type: var_4,
                        extra: var_5,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypeDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.faststr_field_len(Some(2), &self.name)
                    + self
                        .key_type
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(3), value))
                    + self
                        .value_type
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(4), value))
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(5),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct EnumValueDescriptor {
            pub filepath: ::pilota::FastStr,

            pub name: ::pilota::FastStr,

            pub value: i64,

            pub annotations:
                ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<::pilota::FastStr>>,

            pub comments: ::pilota::FastStr,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for EnumValueDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "EnumValueDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_faststr_field(2, (&self.name).clone())?;
                __protocol.write_i64_field(3, *&self.value)?;
                __protocol.write_map_field(
                    4,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::List,
                    &&self.annotations,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_faststr_field(5, (&self.comments).clone())?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        6,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;

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
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                                var_3 = Some(__protocol.read_i64()?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_4 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::pilota::FastStr> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(__protocol.read_faststr()?);
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
                            Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_5 = Some(__protocol.read_faststr()?);
                            }
                            Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_6 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `EnumValueDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field annotations is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field comments is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    name: var_2,
                    value: var_3,
                    annotations: var_4,
                    comments: var_5,
                    extra: var_6,
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
                    let mut var_5 = None;
                    let mut var_6 = None;

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
                                    if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                                {
                                    var_3 = Some(__protocol.read_i64().await?);
                                }
                                Some(4)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_4 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val =
                                            ::pilota::AHashMap::with_capacity(map_ident.size);
                                        for _ in 0..map_ident.size {
                                            val.insert(__protocol.read_faststr().await?, {
                                                let list_ident =
                                                    __protocol.read_list_begin().await?;
                                                let mut val =
                                                    ::std::vec::Vec::with_capacity(list_ident.size);
                                                for _ in 0..list_ident.size {
                                                    val.push(__protocol.read_faststr().await?);
                                                }
                                                __protocol.read_list_end().await?;
                                                val
                                            });
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
                                }
                                Some(5)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_5 = Some(__protocol.read_faststr().await?);
                                }
                                Some(6)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_6 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val =
                                            ::pilota::AHashMap::with_capacity(map_ident.size);
                                        for _ in 0..map_ident.size {
                                            val.insert(
                                                __protocol.read_faststr().await?,
                                                __protocol.read_faststr().await?,
                                            );
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
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
                            err.prepend_msg(&format!("decode struct `EnumValueDescriptor` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field annotations is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field comments is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        name: var_2,
                        value: var_3,
                        annotations: var_4,
                        comments: var_5,
                        extra: var_6,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "EnumValueDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.faststr_field_len(Some(2), &self.name)
                    + __protocol.i64_field_len(Some(3), *&self.value)
                    + __protocol.map_field_len(
                        Some(4),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        &self.annotations,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Binary,
                                val,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        },
                    )
                    + __protocol.faststr_field_len(Some(5), &self.comments)
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(6),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct ConstDescriptor {
            pub filepath: ::pilota::FastStr,

            pub name: ::pilota::FastStr,

            pub r#type: TypeDescriptor,

            pub value: ConstValueDescriptor,

            pub annotations:
                ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<::pilota::FastStr>>,

            pub comments: ::pilota::FastStr,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for ConstDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ConstDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_faststr_field(2, (&self.name).clone())?;
                __protocol.write_struct_field(3, &self.r#type, ::pilota::thrift::TType::Struct)?;
                __protocol.write_struct_field(4, &self.value, ::pilota::thrift::TType::Struct)?;
                __protocol.write_map_field(
                    5,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::List,
                    &&self.annotations,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_faststr_field(6, (&self.comments).clone())?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        7,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;
                let mut var_7 = None;

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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_3 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(4)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_4 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_5 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::pilota::FastStr> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(__protocol.read_faststr()?);
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
                            Some(6)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_6 = Some(__protocol.read_faststr()?);
                            }
                            Some(7) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_7 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `ConstDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field r#type is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field annotations is required".to_string(),
                    ));
                };
                let Some(var_6) = var_6 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field comments is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    name: var_2,
                    r#type: var_3,
                    value: var_4,
                    annotations: var_5,
                    comments: var_6,
                    extra: var_7,
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
                    let mut var_5 = None;
                    let mut var_6 = None;
                    let mut var_7 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_3 = Some(<TypeDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_4 = Some(<ConstValueDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_5 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_faststr().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_6 = Some(__protocol.read_faststr().await?);

                },Some(7) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_7 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

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
                    err.prepend_msg(&format!("decode struct `ConstDescriptor` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field r#type is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field annotations is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_6) = var_6 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field comments is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        name: var_2,
                        r#type: var_3,
                        value: var_4,
                        annotations: var_5,
                        comments: var_6,
                        extra: var_7,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ConstDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.faststr_field_len(Some(2), &self.name)
                    + __protocol.struct_field_len(Some(3), &self.r#type)
                    + __protocol.struct_field_len(Some(4), &self.value)
                    + __protocol.map_field_len(
                        Some(5),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        &self.annotations,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Binary,
                                val,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        },
                    )
                    + __protocol.faststr_field_len(Some(6), &self.comments)
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(7),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct FieldDescriptor {
            pub filepath: ::pilota::FastStr,

            pub name: ::pilota::FastStr,

            pub r#type: TypeDescriptor,

            pub requiredness: ::pilota::FastStr,

            pub id: i32,

            pub default_value: ::std::option::Option<ConstValueDescriptor>,

            pub annotations:
                ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<::pilota::FastStr>>,

            pub comments: ::pilota::FastStr,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for FieldDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "FieldDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_faststr_field(2, (&self.name).clone())?;
                __protocol.write_struct_field(3, &self.r#type, ::pilota::thrift::TType::Struct)?;
                __protocol.write_faststr_field(4, (&self.requiredness).clone())?;
                __protocol.write_i32_field(5, *&self.id)?;
                if let Some(value) = self.default_value.as_ref() {
                    __protocol.write_struct_field(6, value, ::pilota::thrift::TType::Struct)?;
                }
                __protocol.write_map_field(
                    7,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::List,
                    &&self.annotations,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_faststr_field(8, (&self.comments).clone())?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        9,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;
                let mut var_7 = None;
                let mut var_8 = None;
                let mut var_9 = None;

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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_3 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(4)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_4 = Some(__protocol.read_faststr()?);
                            }
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                var_5 = Some(__protocol.read_i32()?);
                            }
                            Some(6)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_6 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(7) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_7 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::pilota::FastStr> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(__protocol.read_faststr()?);
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
                            Some(8)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_8 = Some(__protocol.read_faststr()?);
                            }
                            Some(9) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_9 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `FieldDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field r#type is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field requiredness is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field id is required".to_string(),
                    ));
                };
                let Some(var_7) = var_7 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field annotations is required".to_string(),
                    ));
                };
                let Some(var_8) = var_8 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field comments is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    name: var_2,
                    r#type: var_3,
                    requiredness: var_4,
                    id: var_5,
                    default_value: var_6,
                    annotations: var_7,
                    comments: var_8,
                    extra: var_9,
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
                    let mut var_5 = None;
                    let mut var_6 = None;
                    let mut var_7 = None;
                    let mut var_8 = None;
                    let mut var_9 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_3 = Some(<TypeDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_4 = Some(__protocol.read_faststr().await?);

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32  => {
                    var_5 = Some(__protocol.read_i32().await?);

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_6 = Some(<ConstValueDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(7) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_7 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_faststr().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(8) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_8 = Some(__protocol.read_faststr().await?);

                },Some(9) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_9 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

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
                    err.prepend_msg(&format!("decode struct `FieldDescriptor` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field r#type is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field requiredness is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field id is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_7) = var_7 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field annotations is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_8) = var_8 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field comments is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        name: var_2,
                        r#type: var_3,
                        requiredness: var_4,
                        id: var_5,
                        default_value: var_6,
                        annotations: var_7,
                        comments: var_8,
                        extra: var_9,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "FieldDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.faststr_field_len(Some(2), &self.name)
                    + __protocol.struct_field_len(Some(3), &self.r#type)
                    + __protocol.faststr_field_len(Some(4), &self.requiredness)
                    + __protocol.i32_field_len(Some(5), *&self.id)
                    + self
                        .default_value
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(6), value))
                    + __protocol.map_field_len(
                        Some(7),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        &self.annotations,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Binary,
                                val,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        },
                    )
                    + __protocol.faststr_field_len(Some(8), &self.comments)
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(9),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct ConstValueType(i32);

        impl ConstValueType {
            pub const DOUBLE: Self = Self(0);
            pub const INT: Self = Self(1);
            pub const STRING: Self = Self(2);
            pub const BOOL: Self = Self(3);
            pub const LIST: Self = Self(4);
            pub const MAP: Self = Self(5);
            pub const IDENTIFIER: Self = Self(6);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("DOUBLE"),
                    Self(1) => ::std::string::String::from("INT"),
                    Self(2) => ::std::string::String::from("STRING"),
                    Self(3) => ::std::string::String::from("BOOL"),
                    Self(4) => ::std::string::String::from("LIST"),
                    Self(5) => ::std::string::String::from("MAP"),
                    Self(6) => ::std::string::String::from("IDENTIFIER"),
                    Self(val) => val.to_string(),
                }
            }
        }

        impl ::std::convert::From<i32> for ConstValueType {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<ConstValueType> for i32 {
            fn from(value: ConstValueType) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for ConstValueType {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_i32(self.inner())?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let value = __protocol.read_i32()?;
                ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(
                    |err| {
                        ::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            format!("invalid enum value for ConstValueType, value: {}", value),
                        )
                    },
                )?)
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
                    let value = __protocol.read_i32().await?;
                    ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(
                        |err| {
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                format!("invalid enum value for ConstValueType, value: {}", value),
                            )
                        },
                    )?)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.i32_len(self.inner())
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct StructDescriptor {
            pub filepath: ::pilota::FastStr,

            pub name: ::pilota::FastStr,

            pub fields: ::std::vec::Vec<FieldDescriptor>,

            pub annotations:
                ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<::pilota::FastStr>>,

            pub comments: ::pilota::FastStr,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for StructDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "StructDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_faststr_field(2, (&self.name).clone())?;
                __protocol.write_list_field(
                    3,
                    ::pilota::thrift::TType::Struct,
                    &&self.fields,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_map_field(
                    4,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::List,
                    &&self.annotations,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_faststr_field(5, (&self.comments).clone())?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        6,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;

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
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_3 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<FieldDescriptor> =
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
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_4 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::pilota::FastStr> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(__protocol.read_faststr()?);
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
                            Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_5 = Some(__protocol.read_faststr()?);
                            }
                            Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_6 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `StructDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field fields is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field annotations is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field comments is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    name: var_2,
                    fields: var_3,
                    annotations: var_4,
                    comments: var_5,
                    extra: var_6,
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
                    let mut var_5 = None;
                    let mut var_6 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_3 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<FieldDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_4 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_faststr().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_5 = Some(__protocol.read_faststr().await?);

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_6 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

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
                    err.prepend_msg(&format!("decode struct `StructDescriptor` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field fields is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field annotations is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field comments is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        name: var_2,
                        fields: var_3,
                        annotations: var_4,
                        comments: var_5,
                        extra: var_6,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "StructDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.faststr_field_len(Some(2), &self.name)
                    + __protocol.list_field_len(
                        Some(3),
                        ::pilota::thrift::TType::Struct,
                        &self.fields,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.map_field_len(
                        Some(4),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        &self.annotations,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Binary,
                                val,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        },
                    )
                    + __protocol.faststr_field_len(Some(5), &self.comments)
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(6),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct MethodDescriptor {
            pub filepath: ::pilota::FastStr,

            pub name: ::pilota::FastStr,

            pub response: ::std::option::Option<TypeDescriptor>,

            pub args: ::std::vec::Vec<FieldDescriptor>,

            pub annotations:
                ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<::pilota::FastStr>>,

            pub comments: ::pilota::FastStr,

            pub throw_exceptions: ::std::vec::Vec<FieldDescriptor>,

            pub is_oneway: bool,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for MethodDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "MethodDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_faststr_field(2, (&self.name).clone())?;
                if let Some(value) = self.response.as_ref() {
                    __protocol.write_struct_field(3, value, ::pilota::thrift::TType::Struct)?;
                }
                __protocol.write_list_field(
                    4,
                    ::pilota::thrift::TType::Struct,
                    &&self.args,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_map_field(
                    5,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::List,
                    &&self.annotations,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_faststr_field(6, (&self.comments).clone())?;
                __protocol.write_list_field(
                    7,
                    ::pilota::thrift::TType::Struct,
                    &&self.throw_exceptions,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_bool_field(8, *&self.is_oneway)?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        9,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;
                let mut var_7 = None;
                let mut var_8 = None;
                let mut var_9 = None;

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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_3 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_4 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<FieldDescriptor> =
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
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_5 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::pilota::FastStr> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(__protocol.read_faststr()?);
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
                            Some(6)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_6 = Some(__protocol.read_faststr()?);
                            }
                            Some(7) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_7 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<FieldDescriptor> =
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
                            Some(8) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                                var_8 = Some(__protocol.read_bool()?);
                            }
                            Some(9) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_9 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `MethodDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field args is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field annotations is required".to_string(),
                    ));
                };
                let Some(var_6) = var_6 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field comments is required".to_string(),
                    ));
                };
                let Some(var_7) = var_7 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field throw_exceptions is required".to_string(),
                    ));
                };
                let Some(var_8) = var_8 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field is_oneway is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    name: var_2,
                    response: var_3,
                    args: var_4,
                    annotations: var_5,
                    comments: var_6,
                    throw_exceptions: var_7,
                    is_oneway: var_8,
                    extra: var_9,
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
                    let mut var_5 = None;
                    let mut var_6 = None;
                    let mut var_7 = None;
                    let mut var_8 = None;
                    let mut var_9 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_3 = Some(<TypeDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_4 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<FieldDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_5 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_faststr().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_6 = Some(__protocol.read_faststr().await?);

                },Some(7) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_7 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<FieldDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(8) if field_ident.field_type == ::pilota::thrift::TType::Bool  => {
                    var_8 = Some(__protocol.read_bool().await?);

                },Some(9) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_9 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

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
                    err.prepend_msg(&format!("decode struct `MethodDescriptor` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field args is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field annotations is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_6) = var_6 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field comments is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_7) = var_7 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field throw_exceptions is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_8) = var_8 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field is_oneway is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        name: var_2,
                        response: var_3,
                        args: var_4,
                        annotations: var_5,
                        comments: var_6,
                        throw_exceptions: var_7,
                        is_oneway: var_8,
                        extra: var_9,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "MethodDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.faststr_field_len(Some(2), &self.name)
                    + self
                        .response
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(3), value))
                    + __protocol.list_field_len(
                        Some(4),
                        ::pilota::thrift::TType::Struct,
                        &self.args,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.map_field_len(
                        Some(5),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        &self.annotations,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Binary,
                                val,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        },
                    )
                    + __protocol.faststr_field_len(Some(6), &self.comments)
                    + __protocol.list_field_len(
                        Some(7),
                        ::pilota::thrift::TType::Struct,
                        &self.throw_exceptions,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.bool_field_len(Some(8), *&self.is_oneway)
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(9),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ConstValueDescriptor {
            pub r#type: ConstValueType,

            pub value_double: ::pilota::OrderedFloat<f64>,

            pub value_int: i64,

            pub value_string: ::pilota::FastStr,

            pub value_bool: bool,

            pub value_list: ::std::option::Option<::std::vec::Vec<ConstValueDescriptor>>,

            pub value_map: ::std::option::Option<
                ::std::collections::BTreeMap<ConstValueDescriptor, ConstValueDescriptor>,
            >,

            pub value_identifier: ::pilota::FastStr,

            pub extra: ::std::option::Option<
                ::std::collections::BTreeMap<::pilota::FastStr, ::pilota::FastStr>,
            >,
        }
        impl ::pilota::thrift::Message for ConstValueDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ConstValueDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_i32_field(1, (&self.r#type).inner())?;
                __protocol.write_double_field(2, *&self.value_double.0)?;
                __protocol.write_i64_field(3, *&self.value_int)?;
                __protocol.write_faststr_field(4, (&self.value_string).clone())?;
                __protocol.write_bool_field(5, *&self.value_bool)?;
                if let Some(value) = self.value_list.as_ref() {
                    __protocol.write_list_field(
                        6,
                        ::pilota::thrift::TType::Struct,
                        &value,
                        |__protocol, val| {
                            __protocol.write_struct(val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let Some(value) = self.value_map.as_ref() {
                    __protocol.write_btree_map_field(
                        7,
                        ::pilota::thrift::TType::Struct,
                        ::pilota::thrift::TType::Struct,
                        &value,
                        |__protocol, key| {
                            __protocol.write_struct(key)?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_struct(val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_faststr_field(8, (&self.value_identifier).clone())?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_btree_map_field(
                        9,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;
                let mut var_7 = None;
                let mut var_8 = None;
                let mut var_9 = None;

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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Double =>
                            {
                                var_2 = Some(::pilota::OrderedFloat(__protocol.read_double()?));
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                                var_3 = Some(__protocol.read_i64()?);
                            }
                            Some(4)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_4 = Some(__protocol.read_faststr()?);
                            }
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                                var_5 = Some(__protocol.read_bool()?);
                            }
                            Some(6) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_6 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<ConstValueDescriptor> =
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
                            Some(7) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_7 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::std::collections::BTreeMap::new();
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            ::pilota::thrift::Message::decode(__protocol)?,
                                            ::pilota::thrift::Message::decode(__protocol)?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(8)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_8 = Some(__protocol.read_faststr()?);
                            }
                            Some(9) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_9 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::std::collections::BTreeMap::new();
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `ConstValueDescriptor` field(#{}) failed, caused by: ",
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
                        "field value_double is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value_int is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value_string is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value_bool is required".to_string(),
                    ));
                };
                let Some(var_8) = var_8 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value_identifier is required".to_string(),
                    ));
                };

                let data = Self {
                    r#type: var_1,
                    value_double: var_2,
                    value_int: var_3,
                    value_string: var_4,
                    value_bool: var_5,
                    value_list: var_6,
                    value_map: var_7,
                    value_identifier: var_8,
                    extra: var_9,
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
                    let mut var_5 = None;
                    let mut var_6 = None;
                    let mut var_7 = None;
                    let mut var_8 = None;
                    let mut var_9 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32  => {
                    var_1 = Some(<ConstValueType as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Double  => {
                    var_2 = Some(::pilota::OrderedFloat(__protocol.read_double().await?));

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::I64  => {
                    var_3 = Some(__protocol.read_i64().await?);

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_4 = Some(__protocol.read_faststr().await?);

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Bool  => {
                    var_5 = Some(__protocol.read_bool().await?);

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_6 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<ConstValueDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(7) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_7 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::std::collections::BTreeMap::new();
                        for _ in 0..map_ident.size {
                            val.insert(<ConstValueDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?, <ConstValueDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(8) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_8 = Some(__protocol.read_faststr().await?);

                },Some(9) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_9 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::std::collections::BTreeMap::new();
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

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
                    err.prepend_msg(&format!("decode struct `ConstValueDescriptor` field(#{}) failed, caused by: ", field_id));
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
                                "field value_double is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value_int is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value_string is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value_bool is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_8) = var_8 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value_identifier is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        r#type: var_1,
                        value_double: var_2,
                        value_int: var_3,
                        value_string: var_4,
                        value_bool: var_5,
                        value_list: var_6,
                        value_map: var_7,
                        value_identifier: var_8,
                        extra: var_9,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ConstValueDescriptor",
                }) + __protocol.i32_field_len(Some(1), (&self.r#type).inner())
                    + __protocol.double_field_len(Some(2), *&self.value_double.0)
                    + __protocol.i64_field_len(Some(3), *&self.value_int)
                    + __protocol.faststr_field_len(Some(4), &self.value_string)
                    + __protocol.bool_field_len(Some(5), *&self.value_bool)
                    + self.value_list.as_ref().map_or(0, |value| {
                        __protocol.list_field_len(
                            Some(6),
                            ::pilota::thrift::TType::Struct,
                            value,
                            |__protocol, el| __protocol.struct_len(el),
                        )
                    })
                    + self.value_map.as_ref().map_or(0, |value| {
                        __protocol.btree_map_field_len(
                            Some(7),
                            ::pilota::thrift::TType::Struct,
                            ::pilota::thrift::TType::Struct,
                            value,
                            |__protocol, key| __protocol.struct_len(key),
                            |__protocol, val| __protocol.struct_len(val),
                        )
                    })
                    + __protocol.faststr_field_len(Some(8), &self.value_identifier)
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.btree_map_field_len(
                            Some(9),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ServiceDescriptor {
            fn default() -> Self {
                ServiceDescriptor {
                    filepath: ::std::default::Default::default(),
                    name: ::std::default::Default::default(),
                    methods: ::std::default::Default::default(),
                    annotations: ::std::default::Default::default(),
                    comments: ::std::default::Default::default(),
                    extra: ::std::default::Default::default(),
                    base: Some(::pilota::FastStr::from_static_str("")),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ServiceDescriptor {
            pub filepath: ::pilota::FastStr,

            pub name: ::pilota::FastStr,

            pub methods: ::std::vec::Vec<MethodDescriptor>,

            pub annotations:
                ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<::pilota::FastStr>>,

            pub comments: ::pilota::FastStr,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,

            pub base: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::thrift::Message for ServiceDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ServiceDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_faststr_field(2, (&self.name).clone())?;
                __protocol.write_list_field(
                    3,
                    ::pilota::thrift::TType::Struct,
                    &&self.methods,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_map_field(
                    4,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::List,
                    &&self.annotations,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_faststr_field(5, (&self.comments).clone())?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        6,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let Some(value) = self.base.as_ref() {
                    __protocol.write_faststr_field(7, (value).clone())?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;
                let mut var_7 = Some(::pilota::FastStr::from_static_str(""));

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
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_3 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<MethodDescriptor> =
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
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_4 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::pilota::FastStr> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(__protocol.read_faststr()?);
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
                            Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_5 = Some(__protocol.read_faststr()?);
                            }
                            Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_6 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(7)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_7 = Some(__protocol.read_faststr()?);
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
                            "decode struct `ServiceDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field methods is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field annotations is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field comments is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    name: var_2,
                    methods: var_3,
                    annotations: var_4,
                    comments: var_5,
                    extra: var_6,
                    base: var_7,
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
                    let mut var_5 = None;
                    let mut var_6 = None;
                    let mut var_7 = Some(::pilota::FastStr::from_static_str(""));

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_3 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<MethodDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_4 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_faststr().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_5 = Some(__protocol.read_faststr().await?);

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_6 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(7) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_7 = Some(__protocol.read_faststr().await?);

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
                    err.prepend_msg(&format!("decode struct `ServiceDescriptor` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field methods is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field annotations is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field comments is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        name: var_2,
                        methods: var_3,
                        annotations: var_4,
                        comments: var_5,
                        extra: var_6,
                        base: var_7,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ServiceDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.faststr_field_len(Some(2), &self.name)
                    + __protocol.list_field_len(
                        Some(3),
                        ::pilota::thrift::TType::Struct,
                        &self.methods,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.map_field_len(
                        Some(4),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        &self.annotations,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Binary,
                                val,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        },
                    )
                    + __protocol.faststr_field_len(Some(5), &self.comments)
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(6),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + self
                        .base
                        .as_ref()
                        .map_or(0, |value| __protocol.faststr_field_len(Some(7), value))
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TypedefDescriptor {
            pub filepath: ::pilota::FastStr,

            pub r#type: TypeDescriptor,

            pub alias: ::pilota::FastStr,

            pub annotations:
                ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<::pilota::FastStr>>,

            pub comments: ::pilota::FastStr,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for TypedefDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypedefDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_struct_field(2, &self.r#type, ::pilota::thrift::TType::Struct)?;
                __protocol.write_faststr_field(3, (&self.alias).clone())?;
                __protocol.write_map_field(
                    4,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::List,
                    &&self.annotations,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_faststr_field(5, (&self.comments).clone())?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        6,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;

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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_2 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(3)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_3 = Some(__protocol.read_faststr()?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_4 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::pilota::FastStr> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(__protocol.read_faststr()?);
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
                            Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_5 = Some(__protocol.read_faststr()?);
                            }
                            Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_6 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `TypedefDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field r#type is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field alias is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field annotations is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field comments is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    r#type: var_2,
                    alias: var_3,
                    annotations: var_4,
                    comments: var_5,
                    extra: var_6,
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
                    let mut var_5 = None;
                    let mut var_6 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_2 = Some(<TypeDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_3 = Some(__protocol.read_faststr().await?);

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_4 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_faststr().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_5 = Some(__protocol.read_faststr().await?);

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_6 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

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
                    err.prepend_msg(&format!("decode struct `TypedefDescriptor` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field r#type is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field alias is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field annotations is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field comments is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        r#type: var_2,
                        alias: var_3,
                        annotations: var_4,
                        comments: var_5,
                        extra: var_6,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypedefDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.struct_field_len(Some(2), &self.r#type)
                    + __protocol.faststr_field_len(Some(3), &self.alias)
                    + __protocol.map_field_len(
                        Some(4),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        &self.annotations,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Binary,
                                val,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        },
                    )
                    + __protocol.faststr_field_len(Some(5), &self.comments)
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(6),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct FileDescriptor {
            pub filepath: ::pilota::FastStr,

            pub includes: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,

            pub namespaces: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,

            pub services: ::std::vec::Vec<ServiceDescriptor>,

            pub structs: ::std::vec::Vec<StructDescriptor>,

            pub exceptions: ::std::vec::Vec<StructDescriptor>,

            pub enums: ::std::vec::Vec<EnumDescriptor>,

            pub typedefs: ::std::vec::Vec<TypedefDescriptor>,

            pub unions: ::std::vec::Vec<StructDescriptor>,

            pub consts: ::std::vec::Vec<ConstDescriptor>,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for FileDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "FileDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_map_field(
                    2,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::Binary,
                    &&self.includes,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_faststr((val).clone())?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_map_field(
                    3,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::Binary,
                    &&self.namespaces,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_faststr((val).clone())?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_list_field(
                    4,
                    ::pilota::thrift::TType::Struct,
                    &&self.services,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_list_field(
                    5,
                    ::pilota::thrift::TType::Struct,
                    &&self.structs,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_list_field(
                    6,
                    ::pilota::thrift::TType::Struct,
                    &&self.exceptions,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_list_field(
                    7,
                    ::pilota::thrift::TType::Struct,
                    &&self.enums,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_list_field(
                    8,
                    ::pilota::thrift::TType::Struct,
                    &&self.typedefs,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_list_field(
                    9,
                    ::pilota::thrift::TType::Struct,
                    &&self.unions,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_list_field(
                    10,
                    ::pilota::thrift::TType::Struct,
                    &&self.consts,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        11,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;
                let mut var_7 = None;
                let mut var_8 = None;
                let mut var_9 = None;
                let mut var_10 = None;
                let mut var_11 = None;

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
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_2 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_3 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_4 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<ServiceDescriptor> =
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
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_5 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<StructDescriptor> =
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
                            Some(6) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_6 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<StructDescriptor> =
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
                            Some(7) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_7 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<EnumDescriptor> =
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
                            Some(8) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_8 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<TypedefDescriptor> =
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
                            Some(9) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_9 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<StructDescriptor> =
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
                            Some(10) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_10 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<ConstDescriptor> =
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
                            Some(11) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_11 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `FileDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field includes is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field namespaces is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field services is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field structs is required".to_string(),
                    ));
                };
                let Some(var_6) = var_6 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field exceptions is required".to_string(),
                    ));
                };
                let Some(var_7) = var_7 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field enums is required".to_string(),
                    ));
                };
                let Some(var_8) = var_8 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field typedefs is required".to_string(),
                    ));
                };
                let Some(var_9) = var_9 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field unions is required".to_string(),
                    ));
                };
                let Some(var_10) = var_10 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field consts is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    includes: var_2,
                    namespaces: var_3,
                    services: var_4,
                    structs: var_5,
                    exceptions: var_6,
                    enums: var_7,
                    typedefs: var_8,
                    unions: var_9,
                    consts: var_10,
                    extra: var_11,
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
                    let mut var_5 = None;
                    let mut var_6 = None;
                    let mut var_7 = None;
                    let mut var_8 = None;
                    let mut var_9 = None;
                    let mut var_10 = None;
                    let mut var_11 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_2 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_3 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_4 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<ServiceDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_5 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<StructDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_6 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<StructDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(7) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_7 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<EnumDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(8) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_8 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<TypedefDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(9) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_9 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<StructDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(10) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_10 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<ConstDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(11) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_11 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

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
                    err.prepend_msg(&format!("decode struct `FileDescriptor` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field includes is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field namespaces is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field services is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field structs is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_6) = var_6 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field exceptions is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_7) = var_7 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field enums is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_8) = var_8 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field typedefs is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_9) = var_9 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field unions is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_10) = var_10 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field consts is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        includes: var_2,
                        namespaces: var_3,
                        services: var_4,
                        structs: var_5,
                        exceptions: var_6,
                        enums: var_7,
                        typedefs: var_8,
                        unions: var_9,
                        consts: var_10,
                        extra: var_11,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "FileDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.map_field_len(
                        Some(2),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &self.includes,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| __protocol.faststr_len(val),
                    )
                    + __protocol.map_field_len(
                        Some(3),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &self.namespaces,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| __protocol.faststr_len(val),
                    )
                    + __protocol.list_field_len(
                        Some(4),
                        ::pilota::thrift::TType::Struct,
                        &self.services,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.list_field_len(
                        Some(5),
                        ::pilota::thrift::TType::Struct,
                        &self.structs,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.list_field_len(
                        Some(6),
                        ::pilota::thrift::TType::Struct,
                        &self.exceptions,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.list_field_len(
                        Some(7),
                        ::pilota::thrift::TType::Struct,
                        &self.enums,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.list_field_len(
                        Some(8),
                        ::pilota::thrift::TType::Struct,
                        &self.typedefs,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.list_field_len(
                        Some(9),
                        ::pilota::thrift::TType::Struct,
                        &self.unions,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.list_field_len(
                        Some(10),
                        ::pilota::thrift::TType::Struct,
                        &self.consts,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(11),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct EnumDescriptor {
            pub filepath: ::pilota::FastStr,

            pub name: ::pilota::FastStr,

            pub values: ::std::vec::Vec<EnumValueDescriptor>,

            pub annotations:
                ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<::pilota::FastStr>>,

            pub comments: ::pilota::FastStr,

            pub extra:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for EnumDescriptor {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "EnumDescriptor",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.filepath).clone())?;
                __protocol.write_faststr_field(2, (&self.name).clone())?;
                __protocol.write_list_field(
                    3,
                    ::pilota::thrift::TType::Struct,
                    &&self.values,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_map_field(
                    4,
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::List,
                    &&self.annotations,
                    |__protocol, key| {
                        __protocol.write_faststr((key).clone())?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_faststr_field(5, (&self.comments).clone())?;
                if let Some(value) = self.extra.as_ref() {
                    __protocol.write_map_field(
                        6,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;

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
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_3 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<EnumValueDescriptor> =
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
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_4 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<::pilota::FastStr> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(__protocol.read_faststr()?);
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
                            Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_5 = Some(__protocol.read_faststr()?);
                            }
                            Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_6 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
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
                            "decode struct `EnumDescriptor` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field filepath is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field values is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field annotations is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field comments is required".to_string(),
                    ));
                };

                let data = Self {
                    filepath: var_1,
                    name: var_2,
                    values: var_3,
                    annotations: var_4,
                    comments: var_5,
                    extra: var_6,
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
                    let mut var_5 = None;
                    let mut var_6 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_1 = Some(__protocol.read_faststr().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_3 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<EnumValueDescriptor as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_4 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_faststr().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_5 = Some(__protocol.read_faststr().await?);

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_6 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

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
                    err.prepend_msg(&format!("decode struct `EnumDescriptor` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field filepath is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field values is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field annotations is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field comments is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        filepath: var_1,
                        name: var_2,
                        values: var_3,
                        annotations: var_4,
                        comments: var_5,
                        extra: var_6,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "EnumDescriptor",
                }) + __protocol.faststr_field_len(Some(1), &self.filepath)
                    + __protocol.faststr_field_len(Some(2), &self.name)
                    + __protocol.list_field_len(
                        Some(3),
                        ::pilota::thrift::TType::Struct,
                        &self.values,
                        |__protocol, el| __protocol.struct_len(el),
                    )
                    + __protocol.map_field_len(
                        Some(4),
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::List,
                        &self.annotations,
                        |__protocol, key| __protocol.faststr_len(key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Binary,
                                val,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        },
                    )
                    + __protocol.faststr_field_len(Some(5), &self.comments)
                    + self.extra.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(6),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
