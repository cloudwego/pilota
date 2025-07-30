pub mod const_val {
    #![allow(warnings, clippy::all)]

    pub mod const_val {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
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

            pub fn try_from(value: i32) -> Option<Self> {
                match value {
                    0 => Some(Self::A),
                    1 => Some(Self::B),
                    _ => None,
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
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
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
        pub const TEST_STRUCT: Test = Test { name: None };
        pub static TEST_MAP: ::std::sync::LazyLock<::pilota::AHashMap<Index, &'static str>> =
            ::std::sync::LazyLock::new(|| {
                let mut map = ::pilota::AHashMap::with_capacity(2);
                map.insert(Index::A, "hello");
                map.insert(Index::B, "world");
                map
            });
        pub const TEST_LIST: [&'static str; 2] = ["hello", "world"];
        pub static TEST_MAP_LIST: ::std::sync::LazyLock<
            ::pilota::AHashMap<i32, ::std::vec::Vec<&'static str>>,
        > = ::std::sync::LazyLock::new(|| {
            let mut map = ::pilota::AHashMap::with_capacity(1);
            map.insert(1i32, ::std::vec!["hello"]);
            map
        });
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct Test {
            pub name:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,
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
                if let Some(value) = self.name.as_ref() {
                    __protocol.write_map_field(
                        1,
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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_1 = Some({
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
                            "decode struct `Test` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self { name: var_1 };
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
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_1 = Some({
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
                            err.prepend_msg(&format!(
                                "decode struct `Test` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self { name: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Test" })
                    + self.name.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(1),
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
