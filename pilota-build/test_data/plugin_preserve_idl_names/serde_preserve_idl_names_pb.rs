pub mod serde_preserve_idl_names_pb {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    #[derive(
        PartialOrd,
        Hash,
        Eq,
        Ord,
        Debug,
        Default,
        ::pilota::serde::Serialize,
        ::pilota::serde::Deserialize,
        Clone,
        PartialEq,
    )]
    pub struct A {
        #[serde(rename = "SomeField")]
        pub some_field: ::pilota::FastStr,

        #[serde(rename = "type")]
        pub r#type: ::pilota::FastStr,

        #[serde(rename = "TestOneof")]
        pub test_oneof: ::std::option::Option<a::TestOneof>,
    }
    impl ::pilota::pb::Message for A {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.some_field)
                + ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &self.r#type)
                + self
                    .test_oneof
                    .as_ref()
                    .map_or(0, |msg| msg.encoded_len(ctx))
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode(1, &self.some_field, buf);
            ::pilota::pb::encoding::faststr::encode(2, &self.r#type, buf);
            if let Some(_pilota_inner_value) = self.test_oneof.as_ref() {
                _pilota_inner_value.encode(buf);
            }
        }

        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::pilota::pb::encoding::WireType,
            buf: &mut ::pilota::Bytes,
            ctx: &mut ::pilota::pb::encoding::DecodeContext,
            is_root: bool,
        ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
            const STRUCT_NAME: &'static str = stringify!(A);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.some_field;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(some_field));
                            error
                        })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.r#type;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(r#type));
                            error
                        })
                }
                3 | 4 | 5 => {
                    let mut _inner_pilota_value = &mut self.test_oneof;
                    a::TestOneof::merge(_inner_pilota_value, tag, wire_type, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, stringify!(test_oneof));
                            error
                        },
                    )
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }

    pub mod a {
        use ::pilota::{Buf as _, BufMut as _};

        impl ::std::default::Default for TestOneof {
            fn default() -> Self {
                TestOneof::StringValue(::std::default::Default::default())
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub enum TestOneof {
            #[serde(rename = "StringValue")]
            StringValue(::pilota::FastStr),

            #[serde(rename = "pub")]
            Pub(::pilota::FastStr),

            #[serde(rename = "int_value")]
            IntValue(i32),
        }

        impl TestOneof {
            pub fn encode(&self, buf: &mut ::pilota::LinkedBytes) {
                match self {
                    TestOneof::StringValue(value) => {
                        ::pilota::pb::encoding::faststr::encode(3, &*value, buf);
                    }
                    TestOneof::Pub(value) => {
                        ::pilota::pb::encoding::faststr::encode(4, &*value, buf);
                    }
                    TestOneof::IntValue(value) => {
                        ::pilota::pb::encoding::int32::encode(5, &*value, buf);
                    }
                }
            }

            #[inline]
            pub fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                match self {
                    TestOneof::StringValue(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len(ctx, 3, &*value)
                    }
                    TestOneof::Pub(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len(ctx, 4, &*value)
                    }
                    TestOneof::IntValue(value) => {
                        ::pilota::pb::encoding::int32::encoded_len(ctx, 5, &*value)
                    }
                }
            }

            #[inline]
            pub fn merge(
                field: &mut ::core::option::Option<Self>,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                match tag {
                    3 => match field {
                        ::core::option::Option::Some(TestOneof::StringValue(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field =
                                ::core::option::Option::Some(TestOneof::StringValue(owned_value));
                        }
                    },
                    4 => match field {
                        ::core::option::Option::Some(TestOneof::Pub(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(TestOneof::Pub(owned_value));
                        }
                    },
                    5 => match field {
                        ::core::option::Option::Some(TestOneof::IntValue(value)) => {
                            ::pilota::pb::encoding::int32::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::int32::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(TestOneof::IntValue(owned_value));
                        }
                    },
                    _ => unreachable!(concat!("invalid ", stringify!(TestOneof), " tag: {}"), tag),
                };
                ::core::result::Result::Ok(())
            }
        }
    }
}
