pub mod const_val {
    #![allow(warnings, clippy::all)]

    pub mod const_val {

        impl ::std::convert::From<Index> for i32 {
            fn from(e: Index) -> Self {
                e as _
            }
        }

        impl ::std::convert::TryFrom<i32> for Index {
            type Error = ::pilota::EnumConvertError<i32>;

            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> ::std::result::Result<Self, ::pilota::EnumConvertError<i32>> {
                const A: i32 = Index::A as i32;
                const B: i32 = Index::B as i32;
                match v {
                    A => ::std::result::Result::Ok(Index::A),
                    B => ::std::result::Result::Ok(Index::B),

                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(
                        v, "Index",
                    )),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum Index {
            #[derivative(Default)]
            A = 0,

            B = 1,
        }

        impl ::pilota::thrift::Message for Index {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(*self as i32)?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let value = protocol.read_i32()?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for Index, value: {}", value),
                    )
                })?)
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
                    let value = protocol.read_i32().await?;
                    Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                        ::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                            format!("invalid enum value for Index, value: {}", value),
                        )
                    })?)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.i32_len(*self as i32)
            }
        }
        ::pilota::lazy_static::lazy_static! {
            pub static ref TEST_MAP_LIST: ::std::collections::HashMap<i32, ::std::vec::Vec<&'static str>, ::pilota::Hasher> = {
            let mut map = ::std::collections::HashMap::with_capacity_and_hasher(
                1,
                ::pilota::Hasher::new(),
            );
            map.insert(1i32, ::std::vec!["hello"]);
            map
        };
        }
        pub const TEST_LIST: [&'static str; 2] = ["hello", "world"];
        ::pilota::lazy_static::lazy_static! {
            pub static ref TEST_MAP: ::std::collections::HashMap<Index, &'static str, ::pilota::Hasher> = {
            let mut map = ::std::collections::HashMap::with_capacity_and_hasher(
                2,
                ::pilota::Hasher::new(),
            );
            map.insert(Index::A, "hello");map.insert(Index::B, "world");
            map
        };
        }
    }
}
