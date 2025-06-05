pub mod const_val {
    #![allow(warnings, clippy::all)]

    pub fn find_mod_file_descriptor(
        path: &str,
    ) -> Option<&'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor> {
        match path {

                "/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift" => Some(
            const_val::get_file_descriptor()),

                _ => None,
            }
    }

    pub mod const_val {

        static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\r\0\x02\x0b\x0b\0\0\0\0\r\0\x03\x0b\x0b\0\0\0\0\x0f\0\x04\x0c\0\0\0\0\x0f\0\x05\x0c\0\0\0\x01\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x04Test\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x04name\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x03map\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x06string\0\x0c\0\x04\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x06string\0\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\x01\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x05Index\x0f\0\x03\x0c\0\0\0\x02\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x01A\n\0\x03\0\0\0\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x01B\n\0\x03\0\0\0\0\0\0\0\x01\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x08\x0c\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\x04\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x08TEST_MAP\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x03map\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x05Index\0\x0c\0\x04\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x06string\0\0\x0c\0\x04\x08\0\x01\0\0\0\x05\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\r\0\x07\x0c\x0c\0\0\0\x02\x08\0\x01\0\0\0\x06\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0b\0\x08\0\0\0\x07Index.A\0\x08\0\x01\0\0\0\x02\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\x05hello\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\x08\0\x01\0\0\0\x06\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0b\0\x08\0\0\0\x07Index.B\0\x08\0\x01\0\0\0\x02\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\x05world\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\0\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\tTEST_LIST\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x06string\0\0\x0c\0\x04\x08\0\x01\0\0\0\x04\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0f\0\x06\x0c\0\0\0\x02\x08\0\x01\0\0\0\x02\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\x05hello\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\x08\0\x01\0\0\0\x02\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\x05world\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\0\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\rTEST_MAP_LIST\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x03map\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x03i32\0\x0c\0\x04\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x06string\0\0\0\x0c\0\x04\x08\0\x01\0\0\0\x05\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\r\0\x07\x0c\x0c\0\0\0\x01\x08\0\x01\0\0\0\x01\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\x01\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\x08\0\x01\0\0\0\x04\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0f\0\x06\x0c\0\0\0\x01\x08\0\x01\0\0\0\x02\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\x05hello\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\0\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x0bTEST_STRUCT\x0c\0\x03\x0b\0\x01\0\0\0R/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/const_val.thrift\x0b\0\x02\0\0\0\x04Test\0\x0c\0\x04\x08\0\x01\0\0\0\x05\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\r\0\x07\x0c\x0c\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\0\0");

        pub static FILE_DESCRIPTOR: ::std::sync::LazyLock<
            ::pilota_thrift_reflect::thrift_reflection::FileDescriptor,
        > = ::std::sync::LazyLock::new(|| {
            let descriptor =
                ::pilota_thrift_reflect::thrift_reflection::FileDescriptor::deserialize(
                    FILE_DESCRIPTOR_BYTES.clone(),
                )
                .expect("Failed to decode file descriptor");
            ::pilota_thrift_reflect::service::Register::register(
                descriptor.filepath.clone(),
                descriptor.clone(),
            );

            for (key, include) in descriptor.includes.iter() {
                let path = include.as_str();
                if ::pilota_thrift_reflect::service::Register::contains(path) {
                    continue;
                }

                let include_file_descriptor = super::find_mod_file_descriptor(path)
                    .expect("include file descriptor must exist");
                ::pilota_thrift_reflect::service::Register::register(
                    include_file_descriptor.filepath.clone(),
                    include_file_descriptor.clone(),
                );
            }
            descriptor
        });

        pub fn get_file_descriptor(
        ) -> &'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor {
            &*FILE_DESCRIPTOR
        }
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
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

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
        impl Test {
            pub fn get_descriptor(
                &self,
            ) -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor {
                let file_descriptor = get_file_descriptor();
                file_descriptor.find_struct_by_name("Test").unwrap()
            }
        }
    }
}
