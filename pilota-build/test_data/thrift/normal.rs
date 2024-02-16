pub mod normal {
    #![allow(warnings, clippy::all)]

    pub mod normal {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub a: ::std::option::Option<i32>,
        }
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.a.as_ref() {
                    protocol.write_i32_field(1, *value)?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut a = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                a = Some(protocol.read_i32()?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `A` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let data = Self { a };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut a = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    a = Some(protocol.read_i32().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!("decode struct `A` field(#{}) failed", field_id),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let data = Self { a };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + self
                        .a
                        .as_ref()
                        .map_or(0, |value| protocol.i32_field_len(Some(1), *value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        pub trait Test {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TestTest123ArgsSend {}
        impl ::pilota::thrift::Message for TestTest123ArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
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
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `TestTest123ArgsSend` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
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
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `TestTest123ArgsSend` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
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
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct ObjReq {
            pub msg: Message,

            pub msg_map: ::pilota::AHashMap<Message, SubMessage>,

            pub sub_msgs: ::std::vec::Vec<SubMessage>,

            pub msg_set: ::std::option::Option<::pilota::AHashSet<Message>>,

            pub flag_msg: ::pilota::FastStr,

            pub mock_cost: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::thrift::Message for ObjReq {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "ObjReq" };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.msg, ::pilota::thrift::TType::Struct)?;
                protocol.write_map_field(
                    2,
                    ::pilota::thrift::TType::Struct,
                    ::pilota::thrift::TType::Struct,
                    &&self.msg_map,
                    |protocol, key| {
                        protocol.write_struct(key)?;
                        Ok(())
                    },
                    |protocol, val| {
                        protocol.write_struct(val)?;
                        Ok(())
                    },
                )?;
                protocol.write_list_field(
                    3,
                    ::pilota::thrift::TType::Struct,
                    &&self.sub_msgs,
                    |protocol, val| {
                        protocol.write_struct(val)?;
                        Ok(())
                    },
                )?;
                if let Some(value) = self.msg_set.as_ref() {
                    protocol.write_set_field(
                        4,
                        ::pilota::thrift::TType::Struct,
                        &value,
                        |protocol, val| {
                            protocol.write_struct(val)?;
                            Ok(())
                        },
                    )?;
                }
                protocol.write_faststr_field(5, (&self.flag_msg).clone())?;
                if let Some(value) = self.mock_cost.as_ref() {
                    protocol.write_faststr_field(6, (value).clone())?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut msg = None;
                let mut msg_map = None;
                let mut sub_msgs = None;
                let mut msg_set = None;
                let mut flag_msg = None;
                let mut mock_cost = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                msg = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                msg_map = Some({
                                    let map_ident = protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            ::pilota::thrift::Message::decode(protocol)?,
                                            ::pilota::thrift::Message::decode(protocol)?,
                                        );
                                    }
                                    protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                sub_msgs = Some(unsafe {
                                    let list_ident = protocol.read_list_begin()?;
                                    let mut val: Vec<SubMessage> =
                                        Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(::pilota::thrift::Message::decode(protocol)?);
                                    }
                                    val.set_len(list_ident.size);
                                    protocol.read_list_end()?;
                                    val
                                });
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                msg_set = Some({
                                    let list_ident = protocol.read_set_begin()?;
                                    let mut val =
                                        ::pilota::AHashSet::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.insert(::pilota::thrift::Message::decode(protocol)?);
                                    }
                                    protocol.read_set_end()?;
                                    val
                                });
                            }
                            Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                flag_msg = Some(protocol.read_faststr()?);
                            }
                            Some(6)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                mock_cost = Some(protocol.read_faststr()?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `ObjReq` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(msg) = msg else {
                    return Err(::pilota::thrift::DecodeError::new_protocol(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field msg is required".to_string(),
                    ));
                };
                let Some(msg_map) = msg_map else {
                    return Err(::pilota::thrift::DecodeError::new_protocol(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field msg_map is required".to_string(),
                    ));
                };
                let Some(sub_msgs) = sub_msgs else {
                    return Err(::pilota::thrift::DecodeError::new_protocol(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field sub_msgs is required".to_string(),
                    ));
                };
                let Some(flag_msg) = flag_msg else {
                    return Err(::pilota::thrift::DecodeError::new_protocol(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field flag_msg is required".to_string(),
                    ));
                };

                let data = Self {
                    msg,
                    msg_map,
                    sub_msgs,
                    msg_set,
                    flag_msg,
                    mock_cost,
                };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut msg = None;
                    let mut msg_map = None;
                    let mut sub_msgs = None;
                    let mut msg_set = None;
                    let mut flag_msg = None;
                    let mut mock_cost = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                    loop {


                let field_ident = protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    msg = Some(<Message as ::pilota::thrift::Message>::decode_async(protocol).await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    msg_map = Some({
                        let map_ident = protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(<Message as ::pilota::thrift::Message>::decode_async(protocol).await?, <SubMessage as ::pilota::thrift::Message>::decode_async(protocol).await?);
                        }
                        protocol.read_map_end().await?;
                        val
                    });

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    sub_msgs = Some({
                            let list_ident = protocol.read_list_begin().await?;
                            let mut val = Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<SubMessage as ::pilota::thrift::Message>::decode_async(protocol).await?);
                            };
                            protocol.read_list_end().await?;
                            val
                        });

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Set  => {
                    msg_set = Some({let list_ident = protocol.read_set_begin().await?;
                    let mut val = ::pilota::AHashSet::with_capacity(list_ident.size);
                    for _ in 0..list_ident.size {
                        val.insert(<Message as ::pilota::thrift::Message>::decode_async(protocol).await?);
                    };
                    protocol.read_set_end().await?;
                    val});

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    flag_msg = Some(protocol.read_faststr().await?);

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    mock_cost = Some(protocol.read_faststr().await?);

                },
                    _ => {
                        protocol.skip(field_ident.field_type).await?;

                    },
                }

                protocol.read_field_end().await?;


            };
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `ObjReq` field(#{}) failed", field_id),
                    ));
                } else {
                    return Err(err)
                }
            };
                    protocol.read_struct_end().await?;

                    let Some(msg) = msg else {
                        return Err(::pilota::thrift::DecodeError::new_protocol(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field msg is required".to_string(),
                        ));
                    };
                    let Some(msg_map) = msg_map else {
                        return Err(::pilota::thrift::DecodeError::new_protocol(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field msg_map is required".to_string(),
                        ));
                    };
                    let Some(sub_msgs) = sub_msgs else {
                        return Err(::pilota::thrift::DecodeError::new_protocol(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field sub_msgs is required".to_string(),
                        ));
                    };
                    let Some(flag_msg) = flag_msg else {
                        return Err(::pilota::thrift::DecodeError::new_protocol(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field flag_msg is required".to_string(),
                        ));
                    };

                    let data = Self {
                        msg,
                        msg_map,
                        sub_msgs,
                        msg_set,
                        flag_msg,
                        mock_cost,
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "ObjReq" })
                    + protocol.struct_field_len(Some(1), &self.msg)
                    + protocol.map_field_len(
                        Some(2),
                        ::pilota::thrift::TType::Struct,
                        ::pilota::thrift::TType::Struct,
                        &self.msg_map,
                        |protocol, key| protocol.struct_len(key),
                        |protocol, val| protocol.struct_len(val),
                    )
                    + protocol.list_field_len(
                        Some(3),
                        ::pilota::thrift::TType::Struct,
                        &self.sub_msgs,
                        |protocol, el| protocol.struct_len(el),
                    )
                    + self.msg_set.as_ref().map_or(0, |value| {
                        protocol.set_field_len(
                            Some(4),
                            ::pilota::thrift::TType::Struct,
                            value,
                            |protocol, el| protocol.struct_len(el),
                        )
                    })
                    + protocol.faststr_field_len(Some(5), &self.flag_msg)
                    + self
                        .mock_cost
                        .as_ref()
                        .map_or(0, |value| protocol.faststr_field_len(Some(6), value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct B {
            pub a: ::std::option::Option<A>,
        }
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "b" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.a.as_ref() {
                    protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut a = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                a = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `b` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let data = Self { a };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut a = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    a = Some(
                                        <A as ::pilota::thrift::Message>::decode_async(protocol)
                                            .await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!("decode struct `b` field(#{}) failed", field_id),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let data = Self { a };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "b" })
                    + self
                        .a
                        .as_ref()
                        .map_or(0, |value| protocol.struct_field_len(Some(2), value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestTestExceptionArgsSend {
            pub req: ObjReq,
        }
        impl ::pilota::thrift::Message for TestTestExceptionArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionArgsSend",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `TestTestExceptionArgsSend` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new_protocol(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut req = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
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
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    req = Some(
                                        <ObjReq as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `TestTestExceptionArgsSend` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(req) = req else {
                        return Err(::pilota::thrift::DecodeError::new_protocol(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field req is required".to_string(),
                        ));
                    };

                    let data = Self { req };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionArgsSend",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TestTest123ArgsRecv {}
        impl ::pilota::thrift::Message for TestTest123ArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
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
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `TestTest123ArgsRecv` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
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
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `TestTest123ArgsRecv` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
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
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
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
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
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
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct StException {
            pub message: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::thrift::Message for StException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "STException",
                };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.message.as_ref() {
                    protocol.write_faststr_field(1, (value).clone())?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut message = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                                message = Some(protocol.read_faststr()?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `STException` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let data = Self { message };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut message = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
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
                                    message = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!("decode struct `STException` field(#{}) failed", field_id),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let data = Self { message };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "STException",
                }) + self
                    .message
                    .as_ref()
                    .map_or(0, |value| protocol.faststr_field_len(Some(1), value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct SubMessage {
            pub value: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::thrift::Message for SubMessage {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "SubMessage" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.value.as_ref() {
                    protocol.write_faststr_field(2, (value).clone())?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut value = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr()?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `SubMessage` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let data = Self { value };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut value = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    value = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!("decode struct `SubMessage` field(#{}) failed", field_id),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let data = Self { value };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol
                    .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "SubMessage" })
                    + self
                        .value
                        .as_ref()
                        .map_or(0, |value| protocol.faststr_field_len(Some(2), value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestTestExceptionArgsRecv {
            pub req: ObjReq,
        }
        impl ::pilota::thrift::Message for TestTestExceptionArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionArgsRecv",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!(
                                "decode struct `TestTestExceptionArgsRecv` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::DecodeError::new_protocol(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut req = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
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
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    req = Some(
                                        <ObjReq as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::DecodeError>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            return Err(::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::WithContext(
                                    ::std::boxed::Box::new(err),
                                ),
                                format!(
                                    "decode struct `TestTestExceptionArgsRecv` field(#{}) failed",
                                    field_id
                                ),
                            ));
                        } else {
                            return Err(err);
                        }
                    };
                    protocol.read_struct_end().await?;

                    let Some(req) = req else {
                        return Err(::pilota::thrift::DecodeError::new_protocol(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field req is required".to_string(),
                        ));
                    };

                    let data = Self { req };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionArgsRecv",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]

        pub enum TestTestExceptionException {
            #[derivative(Default)]
            StException(StException),
        }

        impl ::pilota::thrift::Message for TestTestExceptionException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionException",
                })?;
                match self {
                    TestTestExceptionException::StException(ref value) => {
                        protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
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
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionException::StException(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new_protocol(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
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
                    Err(::pilota::thrift::DecodeError::new_protocol(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
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
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <StException as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret =
                                        Some(TestTestExceptionException::StException(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new_protocol(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
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
                        Err(::pilota::thrift::DecodeError::new_protocol(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionException",
                }) + match self {
                    TestTestExceptionException::StException(ref value) => {
                        protocol.struct_field_len(Some(1), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]

        pub enum TestTestExceptionResultSend {
            #[derivative(Default)]
            Ok(ObjReq),

            StException(StException),
        }

        impl ::pilota::thrift::Message for TestTestExceptionResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionResultSend",
                })?;
                match self {
                    TestTestExceptionResultSend::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    TestTestExceptionResultSend::StException(ref value) => {
                        protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
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
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionResultSend::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new_protocol(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionResultSend::StException(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new_protocol(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
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
                    Err(::pilota::thrift::DecodeError::new_protocol(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
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
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <ObjReq as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret = Some(TestTestExceptionResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new_protocol(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <StException as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret =
                                        Some(TestTestExceptionResultSend::StException(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new_protocol(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
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
                        Err(::pilota::thrift::DecodeError::new_protocol(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionResultSend",
                }) + match self {
                    TestTestExceptionResultSend::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                    TestTestExceptionResultSend::StException(ref value) => {
                        protocol.struct_field_len(Some(1), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]

        pub enum TestTestExceptionResultRecv {
            #[derivative(Default)]
            Ok(ObjReq),

            StException(StException),
        }

        impl ::pilota::thrift::Message for TestTestExceptionResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionResultRecv",
                })?;
                match self {
                    TestTestExceptionResultRecv::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    TestTestExceptionResultRecv::StException(ref value) => {
                        protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
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
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionResultRecv::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new_protocol(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionResultRecv::StException(field_ident));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new_protocol(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
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
                    Err(::pilota::thrift::DecodeError::new_protocol(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
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
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <ObjReq as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret = Some(TestTestExceptionResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new_protocol(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <StException as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret =
                                        Some(TestTestExceptionResultRecv::StException(field_ident));
                                } else {
                                    return Err(::pilota::thrift::DecodeError::new_protocol(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
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
                        Err(::pilota::thrift::DecodeError::new_protocol(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionResultRecv",
                }) + match self {
                    TestTestExceptionResultRecv::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                    TestTestExceptionResultRecv::StException(ref value) => {
                        protocol.struct_field_len(Some(1), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
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
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
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
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
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
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
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
        pub struct Message {
            pub uid: ::std::option::Option<[u8; 16]>,

            pub value: ::std::option::Option<::pilota::FastStr>,

            pub sub_messages: ::std::option::Option<::std::vec::Vec<SubMessage>>,
        }
        impl ::pilota::thrift::Message for Message {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Message" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.uid.as_ref() {
                    protocol.write_uuid_field(1, *value)?;
                }
                if let Some(value) = self.value.as_ref() {
                    protocol.write_faststr_field(2, (value).clone())?;
                }
                if let Some(value) = self.sub_messages.as_ref() {
                    protocol.write_list_field(
                        3,
                        ::pilota::thrift::TType::Struct,
                        &value,
                        |protocol, val| {
                            protocol.write_struct(val)?;
                            Ok(())
                        },
                    )?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut uid = None;
                let mut value = None;
                let mut sub_messages = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::Uuid => {
                                uid = Some(protocol.read_uuid()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                value = Some(protocol.read_faststr()?);
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                sub_messages = Some(unsafe {
                                    let list_ident = protocol.read_list_begin()?;
                                    let mut val: Vec<SubMessage> =
                                        Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(::pilota::thrift::Message::decode(protocol)?);
                                    }
                                    val.set_len(list_ident.size);
                                    protocol.read_list_end()?;
                                    val
                                });
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `Message` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;

                let data = Self {
                    uid,
                    value,
                    sub_messages,
                };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::DecodeError>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut uid = None;
                    let mut value = None;
                    let mut sub_messages = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(err) = async {
                    loop {


                let field_ident = protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Uuid  => {
                    uid = Some(protocol.read_uuid().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    value = Some(protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    sub_messages = Some({
                            let list_ident = protocol.read_list_begin().await?;
                            let mut val = Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<SubMessage as ::pilota::thrift::Message>::decode_async(protocol).await?);
                            };
                            protocol.read_list_end().await?;
                            val
                        });

                },
                    _ => {
                        protocol.skip(field_ident.field_type).await?;

                    },
                }

                protocol.read_field_end().await?;


            };
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("decode struct `Message` field(#{}) failed", field_id),
                    ));
                } else {
                    return Err(err)
                }
            };
                    protocol.read_struct_end().await?;

                    let data = Self {
                        uid,
                        value,
                        sub_messages,
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Message" })
                    + self
                        .uid
                        .as_ref()
                        .map_or(0, |value| protocol.uuid_field_len(Some(1), *value))
                    + self
                        .value
                        .as_ref()
                        .map_or(0, |value| protocol.faststr_field_len(Some(2), value))
                    + self.sub_messages.as_ref().map_or(0, |value| {
                        protocol.list_field_len(
                            Some(3),
                            ::pilota::thrift::TType::Struct,
                            value,
                            |protocol, el| protocol.struct_len(el),
                        )
                    })
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
    }
}
