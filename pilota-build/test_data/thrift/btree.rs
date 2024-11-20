pub mod btree {
    #![allow(warnings, clippy::all)]

    pub mod btree {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {}
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                __protocol.write_struct_begin(&struct_ident)?;

                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

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

                let data = Self {};
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

                    let data = Self {};
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypeA(
            pub ::std::collections::BTreeMap<::std::collections::BTreeSet<i32>, ::pilota::FastStr>,
        );

        impl ::std::ops::Deref for TypeA {
            type Target =
                ::std::collections::BTreeMap<::std::collections::BTreeSet<i32>, ::pilota::FastStr>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl
            From<::std::collections::BTreeMap<::std::collections::BTreeSet<i32>, ::pilota::FastStr>>
            for TypeA
        {
            fn from(
                v: ::std::collections::BTreeMap<
                    ::std::collections::BTreeSet<i32>,
                    ::pilota::FastStr,
                >,
            ) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for TypeA {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_btree_map(
                    ::pilota::thrift::TType::Set,
                    ::pilota::thrift::TType::Binary,
                    &(&**self),
                    |__protocol, key| {
                        __protocol.write_btree_set(
                            ::pilota::thrift::TType::I32,
                            &key,
                            |__protocol, val| {
                                __protocol.write_i32(*val)?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_faststr((val).clone())?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                ::std::result::Result::Ok(TypeA({
                    let map_ident = __protocol.read_map_begin()?;
                    let mut val = ::std::collections::BTreeMap::new();
                    for _ in 0..map_ident.size {
                        val.insert(
                            {
                                let list_ident = __protocol.read_set_begin()?;
                                let mut val = ::std::collections::BTreeSet::new();
                                for _ in 0..list_ident.size {
                                    val.insert(__protocol.read_i32()?);
                                }
                                __protocol.read_set_end()?;
                                val
                            },
                            __protocol.read_faststr()?,
                        );
                    }
                    __protocol.read_map_end()?;
                    val
                }))
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
                    ::std::result::Result::Ok(TypeA({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::std::collections::BTreeMap::new();
                        for _ in 0..map_ident.size {
                            val.insert(
                                {
                                    let list_ident = __protocol.read_set_begin().await?;
                                    let mut val = ::std::collections::BTreeSet::new();
                                    for _ in 0..list_ident.size {
                                        val.insert(__protocol.read_i32().await?);
                                    }
                                    __protocol.read_set_end().await?;
                                    val
                                },
                                __protocol.read_faststr().await?,
                            );
                        }
                        __protocol.read_map_end().await?;
                        val
                    }))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.btree_map_len(
                    ::pilota::thrift::TType::Set,
                    ::pilota::thrift::TType::Binary,
                    &**self,
                    |__protocol, key| {
                        __protocol.btree_set_len(
                            ::pilota::thrift::TType::I32,
                            key,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    },
                    |__protocol, val| __protocol.faststr_len(val),
                )
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct B {
            pub m: ::std::collections::BTreeMap<i32, ::std::vec::Vec<::std::sync::Arc<A>>>,

            pub s: ::std::collections::BTreeSet<i32>,

            pub m2: ::std::collections::BTreeMap<
                ::std::vec::Vec<
                    ::std::collections::BTreeMap<::std::collections::BTreeSet<i32>, i32>,
                >,
                ::std::collections::BTreeSet<i32>,
            >,
        }
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "B" };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_btree_map_field(
                    1,
                    ::pilota::thrift::TType::I32,
                    ::pilota::thrift::TType::List,
                    &&self.m,
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
                __protocol.write_btree_set_field(
                    2,
                    ::pilota::thrift::TType::I32,
                    &&self.s,
                    |__protocol, val| {
                        __protocol.write_i32(*val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_btree_map_field(
                    3,
                    ::pilota::thrift::TType::List,
                    ::pilota::thrift::TType::Set,
                    &&self.m2,
                    |__protocol, key| {
                        __protocol.write_list(
                            ::pilota::thrift::TType::Map,
                            &key,
                            |__protocol, val| {
                                __protocol.write_btree_map(
                                    ::pilota::thrift::TType::Set,
                                    ::pilota::thrift::TType::I32,
                                    &val,
                                    |__protocol, key| {
                                        __protocol.write_btree_set(
                                            ::pilota::thrift::TType::I32,
                                            &key,
                                            |__protocol, val| {
                                                __protocol.write_i32(*val)?;
                                                ::std::result::Result::Ok(())
                                            },
                                        )?;
                                        ::std::result::Result::Ok(())
                                    },
                                    |__protocol, val| {
                                        __protocol.write_i32(*val)?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_btree_set(
                            ::pilota::thrift::TType::I32,
                            &val,
                            |__protocol, val| {
                                __protocol.write_i32(*val)?;
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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_1 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::std::collections::BTreeMap::new();
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_i32()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: Vec<::std::sync::Arc<A>> =
                                                Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr().offset(i as isize).write(
                                                    ::std::sync::Arc::new(
                                                        ::pilota::thrift::Message::decode(
                                                            __protocol,
                                                        )?,
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
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                var_2 = Some({
                                    let list_ident = __protocol.read_set_begin()?;
                                    let mut val = ::std::collections::BTreeSet::new();
                                    for _ in 0..list_ident.size {
                                        val.insert(__protocol.read_i32()?);
                                    }
                                    __protocol.read_set_end()?;
                                    val
                                });
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_3 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::std::collections::BTreeMap::new();
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            unsafe {
                                                let list_ident = __protocol.read_list_begin()?;
                                                let mut val: Vec<
                                                    ::std::collections::BTreeMap<
                                                        ::std::collections::BTreeSet<i32>,
                                                        i32,
                                                    >,
                                                > = Vec::with_capacity(list_ident.size);
                                                for i in 0..list_ident.size {
                                                    val.as_mut_ptr().offset(i as isize).write({
                        let map_ident = __protocol.read_map_begin()?;
                        let mut val = ::std::collections::BTreeMap::new();
                        for _ in 0..map_ident.size {
                            val.insert({let list_ident = __protocol.read_set_begin()?;
                    let mut val = ::std::collections::BTreeSet::new();
                    for _ in 0..list_ident.size {
                        val.insert(__protocol.read_i32()?);
                    };
                    __protocol.read_set_end()?;
                    val}, __protocol.read_i32()?);
                        }
                        __protocol.read_map_end()?;
                        val
                    });
                                                }
                                                val.set_len(list_ident.size);
                                                __protocol.read_list_end()?;
                                                val
                                            },
                                            {
                                                let list_ident = __protocol.read_set_begin()?;
                                                let mut val = ::std::collections::BTreeSet::new();
                                                for _ in 0..list_ident.size {
                                                    val.insert(__protocol.read_i32()?);
                                                }
                                                __protocol.read_set_end()?;
                                                val
                                            },
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
                            "decode struct `B` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field m is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field s is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field m2 is required".to_string(),
                    ));
                };

                let data = Self {
                    m: var_1,
                    s: var_2,
                    m2: var_3,
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_1 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::std::collections::BTreeMap::new();
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_i32().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(::std::sync::Arc::new(<A as ::pilota::thrift::Message>::decode_async(__protocol).await?));
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Set  => {
                    var_2 = Some({let list_ident = __protocol.read_set_begin().await?;
                    let mut val = ::std::collections::BTreeSet::new();
                    for _ in 0..list_ident.size {
                        val.insert(__protocol.read_i32().await?);
                    };
                    __protocol.read_set_end().await?;
                    val});

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_3 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::std::collections::BTreeMap::new();
                        for _ in 0..map_ident.size {
                            val.insert({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::std::collections::BTreeMap::new();
                        for _ in 0..map_ident.size {
                            val.insert({let list_ident = __protocol.read_set_begin().await?;
                    let mut val = ::std::collections::BTreeSet::new();
                    for _ in 0..list_ident.size {
                        val.insert(__protocol.read_i32().await?);
                    };
                    __protocol.read_set_end().await?;
                    val}, __protocol.read_i32().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });
                            };
                            __protocol.read_list_end().await?;
                            val
                        }, {let list_ident = __protocol.read_set_begin().await?;
                    let mut val = ::std::collections::BTreeSet::new();
                    for _ in 0..list_ident.size {
                        val.insert(__protocol.read_i32().await?);
                    };
                    __protocol.read_set_end().await?;
                    val});
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
                    err.prepend_msg(&format!("decode struct `B` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field m is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field s is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field m2 is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        m: var_1,
                        s: var_2,
                        m2: var_3,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "B" })
                    + __protocol.btree_map_field_len(
                        Some(1),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::List,
                        &self.m,
                        |__protocol, key| __protocol.i32_len(*key),
                        |__protocol, val| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Struct,
                                val,
                                |__protocol, el| __protocol.struct_len(el),
                            )
                        },
                    )
                    + __protocol.btree_set_field_len(
                        Some(2),
                        ::pilota::thrift::TType::I32,
                        &self.s,
                        |__protocol, el| __protocol.i32_len(*el),
                    )
                    + __protocol.btree_map_field_len(
                        Some(3),
                        ::pilota::thrift::TType::List,
                        ::pilota::thrift::TType::Set,
                        &self.m2,
                        |__protocol, key| {
                            __protocol.list_len(
                                ::pilota::thrift::TType::Map,
                                key,
                                |__protocol, el| {
                                    __protocol.btree_map_len(
                                        ::pilota::thrift::TType::Set,
                                        ::pilota::thrift::TType::I32,
                                        el,
                                        |__protocol, key| {
                                            __protocol.btree_set_len(
                                                ::pilota::thrift::TType::I32,
                                                key,
                                                |__protocol, el| __protocol.i32_len(*el),
                                            )
                                        },
                                        |__protocol, val| __protocol.i32_len(*val),
                                    )
                                },
                            )
                        },
                        |__protocol, val| {
                            __protocol.btree_set_len(
                                ::pilota::thrift::TType::I32,
                                val,
                                |__protocol, el| __protocol.i32_len(*el),
                            )
                        },
                    )
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub static TEST_MAP_LIST: ::std::sync::LazyLock<
            ::std::collections::BTreeMap<i32, ::std::vec::Vec<&'static str>>,
        > = ::std::sync::LazyLock::new(|| {
            let mut map = ::std::collections::BTreeMap::new();
            map.insert(1i32, ::std::vec!["hello"]);
            map
        });

        pub static TEST_MAP: ::std::sync::LazyLock<
            ::std::collections::BTreeMap<Index, &'static str>,
        > = ::std::sync::LazyLock::new(|| {
            let mut map = ::std::collections::BTreeMap::new();
            map.insert(Index::A, "hello");
            map.insert(Index::B, "world");
            map
        });
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Index(i32);

        impl Index {
            pub const A: Self = Self(0);
            pub const B: Self = Self(1);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("A"),
                    Self(1) => ::std::string::String::from("B"),
                    Self(val) => val.to_string(),
                }
            }
        }

        impl ::std::convert::From<i32> for Index {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Index> for i32 {
            fn from(value: Index) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for Index {
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
                            format!("invalid enum value for Index, value: {}", value),
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
                                format!("invalid enum value for Index, value: {}", value),
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
    }
}
