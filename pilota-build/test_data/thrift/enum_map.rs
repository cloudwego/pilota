pub mod enum_map {
    #![allow(warnings, clippy::all)]

    pub mod enum_map {

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
        pub const TYPE_A2: TypeA = TypeA("a2");
        pub const TYPE_B2: TypeB = TypeB(2i32);
        pub const TYPE_A1: TypeA = TypeA("a1");
        pub const TYPE_B1: TypeB = TypeB(1i32);
        pub static TYPE_A_MAP: ::std::sync::LazyLock<&'static ::pilota::AHashMap<TypeB, TypeA>> =
            ::std::sync::LazyLock::new(|| {
                pub static INNER_MAP: ::std::sync::LazyLock<::pilota::AHashMap<TypeB, TypeA>> =
                    ::std::sync::LazyLock::new(|| {
                        let mut map = ::pilota::AHashMap::with_capacity(2);
                        map.insert(TYPE_B1, TYPE_A1);
                        map.insert(TYPE_B2, TYPE_A2);
                        map
                    });

                &*INNER_MAP
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
