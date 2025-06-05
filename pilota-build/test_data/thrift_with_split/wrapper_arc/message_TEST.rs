#[derive(Debug, Default, Clone, PartialEq)]
pub struct Test {
    pub id: ::pilota::FastStr,

    pub name2: ::std::vec::Vec<::std::vec::Vec<::std::sync::Arc<A>>>,

    pub name3: ::pilota::AHashMap<i32, ::std::vec::Vec<::std::sync::Arc<A>>>,
}
impl ::pilota::thrift::Message for Test {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Test" };

        __protocol.write_struct_begin(&struct_ident)?;
        __protocol.write_faststr_field(1, (&self.id).clone())?;
        __protocol.write_list_field(
            2,
            ::pilota::thrift::TType::List,
            &&self.name2,
            |__protocol, val| {
                __protocol.write_list(
                    ::pilota::thrift::TType::Struct,
                    &val,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                ::std::result::Result::Ok(())
            },
        )?;
        __protocol.write_map_field(
            3,
            ::pilota::thrift::TType::I32,
            ::pilota::thrift::TType::List,
            &&self.name3,
            |__protocol, key| {
                __protocol.write_i32(*key)?;
                ::std::result::Result::Ok(())
            },
            |__protocol, val| {
                __protocol.write_list(
                    ::pilota::thrift::TType::Struct,
                    &val,
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                ::std::result::Result::Ok(())
            },
        )?;
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_1 = Some(__protocol.read_faststr()?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        var_2 = Some(unsafe {
                            let list_ident = __protocol.read_list_begin()?;
                            let mut val: ::std::vec::Vec<::std::vec::Vec<::std::sync::Arc<A>>> =
                                ::std::vec::Vec::with_capacity(list_ident.size);
                            for i in 0..list_ident.size {
                                val.as_mut_ptr().offset(i as isize).write(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<::std::sync::Arc<A>> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr().offset(i as isize).write(
                                            ::std::sync::Arc::new(
                                                ::pilota::thrift::Message::decode(__protocol)?,
                                            ),
                                        );
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                });
                            }
                            val.set_len(list_ident.size);
                            __protocol.read_list_end()?;
                            val
                        });
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                        var_3 = Some({
                            let map_ident = __protocol.read_map_begin()?;
                            let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                            for _ in 0..map_ident.size {
                                val.insert(__protocol.read_i32()?, unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<::std::sync::Arc<A>> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr().offset(i as isize).write(
                                            ::std::sync::Arc::new(
                                                ::pilota::thrift::Message::decode(__protocol)?,
                                            ),
                                        );
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
                    "decode struct `Test` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let Some(var_1) = var_1 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field id is required".to_string(),
            ));
        };
        let Some(var_2) = var_2 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field name2 is required".to_string(),
            ));
        };
        let Some(var_3) = var_3 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field name3 is required".to_string(),
            ));
        };

        let data = Self {
            id: var_1,
            name2: var_2,
            name3: var_3,
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
                        Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_1 = Some(__protocol.read_faststr().await?);
                        }
                        Some(2) if field_ident.field_type == ::pilota::thrift::TType::List => {
                            var_2 = Some({
                                let list_ident = __protocol.read_list_begin().await?;
                                let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                                for _ in 0..list_ident.size {
                                    val.push({
                                        let list_ident = __protocol.read_list_begin().await?;
                                        let mut val =
                                            ::std::vec::Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(::std::sync::Arc::new(
                                                <A as ::pilota::thrift::Message>::decode_async(
                                                    __protocol,
                                                )
                                                .await?,
                                            ));
                                        }
                                        __protocol.read_list_end().await?;
                                        val
                                    });
                                }
                                __protocol.read_list_end().await?;
                                val
                            });
                        }
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                            var_3 = Some({
                                let map_ident = __protocol.read_map_begin().await?;
                                let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                for _ in 0..map_ident.size {
                                    val.insert(__protocol.read_i32().await?, {
                                        let list_ident = __protocol.read_list_begin().await?;
                                        let mut val =
                                            ::std::vec::Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(::std::sync::Arc::new(
                                                <A as ::pilota::thrift::Message>::decode_async(
                                                    __protocol,
                                                )
                                                .await?,
                                            ));
                                        }
                                        __protocol.read_list_end().await?;
                                        val
                                    });
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
                    err.prepend_msg(&format!(
                        "decode struct `Test` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let Some(var_1) = var_1 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field id is required".to_string(),
                ));
            };
            let Some(var_2) = var_2 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field name2 is required".to_string(),
                ));
            };
            let Some(var_3) = var_3 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field name3 is required".to_string(),
                ));
            };

            let data = Self {
                id: var_1,
                name2: var_2,
                name3: var_3,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Test" })
            + __protocol.faststr_field_len(Some(1), &self.id)
            + __protocol.list_field_len(
                Some(2),
                ::pilota::thrift::TType::List,
                &self.name2,
                |__protocol, el| {
                    __protocol.list_len(::pilota::thrift::TType::Struct, el, |__protocol, el| {
                        __protocol.struct_len(el)
                    })
                },
            )
            + __protocol.map_field_len(
                Some(3),
                ::pilota::thrift::TType::I32,
                ::pilota::thrift::TType::List,
                &self.name3,
                |__protocol, key| __protocol.i32_len(*key),
                |__protocol, val| {
                    __protocol.list_len(::pilota::thrift::TType::Struct, val, |__protocol, el| {
                        __protocol.struct_len(el)
                    })
                },
            )
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
impl Test {
    pub fn get_descriptor(
        &self,
    ) -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor {
        let file_descriptor = get_file_descriptor();
        file_descriptor.find_struct_by_name("Test").unwrap()
    }
}
