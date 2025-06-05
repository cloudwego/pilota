pub mod serde {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};

    pub fn find_mod_file_descriptor(
        path: &str,
    ) -> Option<&'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor> {
        match path {
            "/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift" => {
                Some(serde::get_file_descriptor())
            }

            _ => None,
        }
    }

    pub mod serde {
<<<<<<< HEAD
        use ::pilota::{Buf as _, BufMut as _};
=======

        static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\r\0\x02\x0b\x0b\0\0\0\0\r\0\x03\x0b\x0b\0\0\0\0\x0f\0\x04\x0c\0\0\0\0\x0f\0\x05\x0c\0\0\0\x01\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0b\0\x02\0\0\0\x01A\x0f\0\x03\x0c\0\0\0\x02\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0b\0\x02\0\0\0\x01a\x0c\0\x03\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\x01\0\0\0\x16pilota.serde_attribute\x0b\0\0\0\x01\0\0\0\x19#[serde(rename = \\\"AA\\\")]\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0b\0\x02\0\0\0\x01b\x0c\0\x03\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0b\0\x02\0\0\0\x03i32\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\x01\0\0\0\x16pilota.serde_attribute\x0b\0\0\0\x01\0\0\0$#[serde(rename_all = \\\"camelCase\\\")]\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\x01\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0b\0\x02\0\0\0\x01C\x0f\0\x03\x0c\0\0\0\x02\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0b\0\x02\0\0\0\x01D\n\0\x03\xff\xff\xff\xff\xff\xff\xff\xff\r\0\x04\x0b\x0f\0\0\0\x01\0\0\0\x16pilota.serde_attribute\x0b\0\0\0\x01\0\0\0\x19#[serde(rename = \\\"DD\\\")]\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0b\0\x02\0\0\0\x01E\n\0\x03\xff\xff\xff\xff\xff\xff\xff\xff\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\x01\0\0\0\x16pilota.serde_attribute\x0b\0\0\0\x01\0\0\0\x12#[serde(untagged)]\x0b\0\x05\0\0\0\0\0\x0f\0\x08\x0c\0\0\0\x01\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0c\0\x02\x0b\0\x01\0\0\0N/data02/home/giggle/projects/pilota/pilota-build/test_data/plugin/serde.thrift\x0b\0\x02\0\0\0\x03i32\0\x0b\0\x03\0\0\0\x01B\r\0\x04\x0b\x0f\0\0\0\x01\0\0\0\x16pilota.serde_attribute\x0b\0\0\0\x01\0\0\0\x19#[serde(rename = \\\"BB\\\")]\x0b\0\x05\0\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\0\0");

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
>>>>>>> ae87e76 (feat(pilota-build): codegen file descriptor)
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
        )]
        #[serde(rename_all = "camelCase")]
        #[derive(Clone, PartialEq)]
        pub struct A {
            #[serde(rename = "AA")]
            pub a: ::pilota::FastStr,

            pub b: i32,
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
                __protocol.write_faststr_field(1, (&self.a).clone())?;
                __protocol.write_i32_field(2, *&self.b)?;
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
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                var_2 = Some(__protocol.read_i32()?);
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
                        "field a is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field b is required".to_string(),
                    ));
                };

                let data = Self { a: var_1, b: var_2 };
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
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_2 = Some(__protocol.read_i32().await?);
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
                                "field a is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field b is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { a: var_1, b: var_2 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + __protocol.faststr_field_len(Some(1), &self.a)
                    + __protocol.i32_field_len(Some(2), *&self.b)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl A {
            pub fn get_descriptor(
                &self,
            ) -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor {
                let file_descriptor = get_file_descriptor();
                file_descriptor.find_struct_by_name("A").unwrap()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
        )]
        #[serde(untagged)]
        #[serde(transparent)]
        #[derive(Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct C(i32);

        impl C {
            pub const D: Self = Self(0);
            pub const E: Self = Self(1);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("D"),
                    Self(1) => ::std::string::String::from("E"),
                    Self(val) => val.to_string(),
                }
            }
        }

        impl ::std::convert::From<i32> for C {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<C> for i32 {
            fn from(value: C) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for C {
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
                            format!("invalid enum value for C, value: {}", value),
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
                                format!("invalid enum value for C, value: {}", value),
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
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
        )]
        #[serde(rename = "BB")]
        #[derive(Clone, PartialEq)]
        pub struct B(pub i32);

        impl ::std::ops::Deref for B {
            type Target = i32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<i32> for B {
            fn from(v: i32) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_i32(*(&**self))?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                ::std::result::Result::Ok(B(__protocol.read_i32()?))
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
                    ::std::result::Result::Ok(B(__protocol.read_i32().await?))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.i32_len(*&**self)
            }
        }
    }
}
