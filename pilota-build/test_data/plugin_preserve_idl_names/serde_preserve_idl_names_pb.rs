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
        #[serde(rename = "type")]
        pub r#type: ::pilota::FastStr,

        #[serde(rename = "SomeField")]
        pub some_field: ::pilota::FastStr,

        #[serde(rename = "CC")]
        pub c: ::pilota::FastStr,

        #[serde(alias = "renamed_value")]
        #[serde(rename = "d")]
        pub d: ::pilota::FastStr,

        #[serde(rename(serialize = "EE"))]
        #[serde(rename(deserialize = "SerOnly"))]
        pub ser_only: ::pilota::FastStr,

        #[serde(rename(deserialize = "FF"))]
        #[serde(rename(serialize = "DeOnly"))]
        pub de_only: ::pilota::FastStr,

        #[serde(rename(serialize = "GS", deserialize = "GD"))]
        pub both_dirs: ::pilota::FastStr,

        #[serde(rename = "TestOneof")]
        pub test_oneof: ::std::option::Option<a::TestOneof>,
    }
    impl ::pilota::pb::Message for A {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.r#type)
                + ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &self.some_field)
                + ::pilota::pb::encoding::faststr::encoded_len(ctx, 3, &self.c)
                + ::pilota::pb::encoding::faststr::encoded_len(ctx, 4, &self.d)
                + ::pilota::pb::encoding::faststr::encoded_len(ctx, 5, &self.ser_only)
                + ::pilota::pb::encoding::faststr::encoded_len(ctx, 6, &self.de_only)
                + ::pilota::pb::encoding::faststr::encoded_len(ctx, 7, &self.both_dirs)
                + self
                    .test_oneof
                    .as_ref()
                    .map_or(0, |msg| msg.encoded_len(ctx))
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode(1, &self.r#type, buf);
            ::pilota::pb::encoding::faststr::encode(2, &self.some_field, buf);
            ::pilota::pb::encoding::faststr::encode(3, &self.c, buf);
            ::pilota::pb::encoding::faststr::encode(4, &self.d, buf);
            ::pilota::pb::encoding::faststr::encode(5, &self.ser_only, buf);
            ::pilota::pb::encoding::faststr::encode(6, &self.de_only, buf);
            ::pilota::pb::encoding::faststr::encode(7, &self.both_dirs, buf);
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
                    let mut _inner_pilota_value = &mut self.r#type;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(r#type));
                            error
                        })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.some_field;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(some_field));
                            error
                        })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.c;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(c));
                            error
                        })
                }
                4 => {
                    let mut _inner_pilota_value = &mut self.d;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(d));
                            error
                        })
                }
                5 => {
                    let mut _inner_pilota_value = &mut self.ser_only;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(ser_only));
                            error
                        })
                }
                6 => {
                    let mut _inner_pilota_value = &mut self.de_only;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(de_only));
                            error
                        })
                }
                7 => {
                    let mut _inner_pilota_value = &mut self.both_dirs;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(both_dirs));
                            error
                        })
                }
                8 | 9 | 10 | 11 | 12 | 13 | 14 => {
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
                TestOneof::Pub(::std::default::Default::default())
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
            #[serde(rename = "pub")]
            Pub(::pilota::FastStr),

            #[serde(rename = "some_field")]
            SomeField(::pilota::FastStr),

            #[serde(rename = "CC")]
            E(::pilota::FastStr),

            #[serde(alias = "renamed_value")]
            #[serde(rename = "f")]
            F(::pilota::FastStr),

            #[serde(rename(serialize = "EE"))]
            #[serde(rename(deserialize = "ser_only"))]
            SerOnly(::pilota::FastStr),

            #[serde(rename(deserialize = "FF"))]
            #[serde(rename(serialize = "de_only"))]
            DeOnly(::pilota::FastStr),

            #[serde(rename(serialize = "GS", deserialize = "GD"))]
            BothDirs(::pilota::FastStr),
        }

        impl TestOneof {
            pub fn encode(&self, buf: &mut ::pilota::LinkedBytes) {
                match self {
                    TestOneof::Pub(value) => {
                        ::pilota::pb::encoding::faststr::encode(8, &*value, buf);
                    }
                    TestOneof::SomeField(value) => {
                        ::pilota::pb::encoding::faststr::encode(9, &*value, buf);
                    }
                    TestOneof::E(value) => {
                        ::pilota::pb::encoding::faststr::encode(10, &*value, buf);
                    }
                    TestOneof::F(value) => {
                        ::pilota::pb::encoding::faststr::encode(11, &*value, buf);
                    }
                    TestOneof::SerOnly(value) => {
                        ::pilota::pb::encoding::faststr::encode(12, &*value, buf);
                    }
                    TestOneof::DeOnly(value) => {
                        ::pilota::pb::encoding::faststr::encode(13, &*value, buf);
                    }
                    TestOneof::BothDirs(value) => {
                        ::pilota::pb::encoding::faststr::encode(14, &*value, buf);
                    }
                }
            }

            #[inline]
            pub fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                match self {
                    TestOneof::Pub(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len(ctx, 8, &*value)
                    }
                    TestOneof::SomeField(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len(ctx, 9, &*value)
                    }
                    TestOneof::E(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len(ctx, 10, &*value)
                    }
                    TestOneof::F(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len(ctx, 11, &*value)
                    }
                    TestOneof::SerOnly(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len(ctx, 12, &*value)
                    }
                    TestOneof::DeOnly(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len(ctx, 13, &*value)
                    }
                    TestOneof::BothDirs(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len(ctx, 14, &*value)
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
                    8 => match field {
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
                    9 => match field {
                        ::core::option::Option::Some(TestOneof::SomeField(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field =
                                ::core::option::Option::Some(TestOneof::SomeField(owned_value));
                        }
                    },
                    10 => match field {
                        ::core::option::Option::Some(TestOneof::E(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(TestOneof::E(owned_value));
                        }
                    },
                    11 => match field {
                        ::core::option::Option::Some(TestOneof::F(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(TestOneof::F(owned_value));
                        }
                    },
                    12 => match field {
                        ::core::option::Option::Some(TestOneof::SerOnly(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(TestOneof::SerOnly(owned_value));
                        }
                    },
                    13 => match field {
                        ::core::option::Option::Some(TestOneof::DeOnly(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(TestOneof::DeOnly(owned_value));
                        }
                    },
                    14 => match field {
                        ::core::option::Option::Some(TestOneof::BothDirs(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(TestOneof::BothDirs(owned_value));
                        }
                    },
                    _ => unreachable!(concat!("invalid ", stringify!(TestOneof), " tag: {}"), tag),
                };
                ::core::result::Result::Ok(())
            }
        }
    }

    pub mod pilota {
        use ::pilota::{Buf as _, BufMut as _};
    }
}
