pub mod enum_map {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};

    pub mod enum_map {
<<<<<<< HEAD
<<<<<<< HEAD
        use ::pilota::{Buf as _, BufMut as _};
=======

        static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\r\0\x02\x0b\x0b\0\0\0\0\r\0\x03\x0b\x0b\0\0\0\0\x0f\0\x04\x0c\0\0\0\0\x0f\0\x05\x0c\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x0f\0\x08\x0c\0\0\0\x02\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0c\0\x02\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x03i32\0\x0b\0\x03\0\0\0\x05TypeB\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0c\0\x02\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x03\0\0\0\x05TypeA\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\x05\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x06TypeB1\x0c\0\x03\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x05TypeB\0\x0c\0\x04\x08\0\x01\0\0\0\x01\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\x01\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\0\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x06TypeB2\x0c\0\x03\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x05TypeB\0\x0c\0\x04\x08\0\x01\0\0\0\x01\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\x02\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\0\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x06TypeA1\x0c\0\x03\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x05TypeA\0\x0c\0\x04\x08\0\x01\0\0\0\x02\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\x02a1\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\0\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x06TypeA2\x0c\0\x03\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x05TypeA\0\x0c\0\x04\x08\0\x01\0\0\0\x02\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\x02a2\x02\0\x05\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\0\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x08TypeAMap\x0c\0\x03\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x03map\x0c\0\x03\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x05TypeB\0\x0c\0\x04\x0b\0\x01\0\0\0Q/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift/enum_map.thrift\x0b\0\x02\0\0\0\x05TypeA\0\0\x0c\0\x04\x08\0\x01\0\0\0\x05\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\r\0\x07\x0c\x0c\0\0\0\x02\x08\0\x01\0\0\0\x06\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0b\0\x08\0\0\0\x06TypeB1\0\x08\0\x01\0\0\0\x06\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0b\0\x08\0\0\0\x06TypeA1\0\x08\0\x01\0\0\0\x06\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0b\0\x08\0\0\0\x06TypeB2\0\x08\0\x01\0\0\0\x06\x04\0\x02\0\0\0\0\0\0\0\0\n\0\x03\0\0\0\0\0\0\0\0\x0b\0\x04\0\0\0\0\x02\0\x05\0\x0b\0\x08\0\0\0\x06TypeA2\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\0\0");

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
=======
>>>>>>> 0314c00 (feat(pilota-build): codegen fieldmask)
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypeB(pub i32);

        impl ::std::ops::Deref for TypeB {
            type Target = i32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<i32> for TypeB {
            fn from(v: i32) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for TypeB {
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
                ::std::result::Result::Ok(TypeB(__protocol.read_i32()?))
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
                    ::std::result::Result::Ok(TypeB(__protocol.read_i32().await?))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.i32_len(*&**self)
            }
        }
        pub const TYPE_A2: TypeA = TypeA(::pilota::FastStr::from_static_str("a2"));
        pub const TYPE_B2: TypeB = TypeB(2i32);
        pub const TYPE_A1: TypeA = TypeA(::pilota::FastStr::from_static_str("a1"));
        pub const TYPE_B1: TypeB = TypeB(1i32);
        pub static TYPE_A_MAP: ::std::sync::LazyLock<::pilota::AHashMap<TypeB, TypeA>> =
            ::std::sync::LazyLock::new(|| {
                let mut map = ::pilota::AHashMap::with_capacity(2);
                map.insert(TYPE_B1, TYPE_A1);
                map.insert(TYPE_B2, TYPE_A2);
                map
            });
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypeA(pub ::pilota::FastStr);

        impl ::std::ops::Deref for TypeA {
            type Target = ::pilota::FastStr;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<::pilota::FastStr> for TypeA {
            fn from(v: ::pilota::FastStr) -> Self {
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
                __protocol.write_faststr((&**self).clone())?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                ::std::result::Result::Ok(TypeA(__protocol.read_faststr()?))
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
                    ::std::result::Result::Ok(TypeA(__protocol.read_faststr().await?))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.faststr_len(&**self)
            }
        }
    }
}
