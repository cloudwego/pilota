pub mod oneof {
    #![allow(warnings, clippy::all)]
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct Test {
        pub c: i32,

        pub r#type: ::std::option::Option<test::Type>,

        pub j: i64,

        pub test: ::std::option::Option<test::Test>,

        pub e: ::std::option::Option<Enum>,
    }
    impl ::pilota::prost::Message for Test {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::int32::encoded_len(1, &self.c)
                + self.r#type.as_ref().map_or(0, |msg| msg.encoded_len())
                + ::pilota::prost::encoding::int64::encoded_len(5, &self.j)
                + self.test.as_ref().map_or(0, |msg| msg.encoded_len())
                + self.e.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::int32::encoded_len(10, value)
                })
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::int32::encode(1, &self.c, buf);
            if let Some(_pilota_inner_value) = self.r#type.as_ref() {
                _pilota_inner_value.encode(buf);
            }
            ::pilota::prost::encoding::int64::encode(5, &self.j, buf);
            if let Some(_pilota_inner_value) = self.test.as_ref() {
                _pilota_inner_value.encode(buf);
            }
            if let Some(_pilota_inner_value) = self.e.as_ref() {
                ::pilota::prost::encoding::int32::encode(10, _pilota_inner_value, buf);
            };
        }

        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::pilota::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::pilota::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
        where
            B: ::pilota::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = stringify!(Test);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.c;
                    ::pilota::prost::encoding::int32::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(c));
                        error
                    })
                }
                2 | 4 => {
                    let mut _inner_pilota_value = &mut self.r#type;
                    test::Type::merge(&mut _inner_pilota_value, tag, wire_type, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, stringify!(r#type));
                            error
                        },
                    )
                }
                5 => {
                    let mut _inner_pilota_value = &mut self.j;
                    ::pilota::prost::encoding::int64::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(j));
                        error
                    })
                }
                6 | 8 => {
                    let mut _inner_pilota_value = &mut self.test;
                    test::Test::merge(&mut _inner_pilota_value, tag, wire_type, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, stringify!(test));
                            error
                        },
                    )
                }
                10 => {
                    let mut _inner_pilota_value = &mut self.e;
                    ::pilota::prost::encoding::int32::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(e));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct Enum(i32);

    impl Enum {
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

        pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
            match value {
                0 => Some(Self::A),
                1 => Some(Self::B),
                _ => None,
            }
        }
    }

    impl ::std::convert::From<i32> for Enum {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl ::std::convert::From<Enum> for i32 {
        fn from(value: Enum) -> i32 {
            value.0
        }
    }

    pub mod test {

        impl ::std::default::Default for Test {
            fn default() -> Self {
                Test::A(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum Test {
            A(::pilota::FastStr),

            B(i32),
        }
        impl Test {
            pub fn encode<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                match self {
                    Test::A(value) => {
                        ::pilota::prost::encoding::faststr::encode(6, &*value, buf);
                    }
                    Test::B(value) => {
                        ::pilota::prost::encoding::int32::encode(8, &*value, buf);
                    }
                }
            }

            #[inline]
            pub fn encoded_len(&self) -> usize {
                match self {
                    Test::A(value) => ::pilota::prost::encoding::faststr::encoded_len(6, &*value),
                    Test::B(value) => ::pilota::prost::encoding::int32::encoded_len(8, &*value),
                }
            }

            #[inline]
            pub fn merge<B>(
                field: &mut ::core::option::Option<Self>,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                match tag {
                    6 => match field {
                        ::core::option::Option::Some(Test::A(value)) => {
                            ::pilota::prost::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Test::A(owned_value));
                        }
                    },
                    8 => match field {
                        ::core::option::Option::Some(Test::B(value)) => {
                            ::pilota::prost::encoding::int32::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::int32::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Test::B(owned_value));
                        }
                    },
                    _ => unreachable!(concat!("invalid ", stringify!(Test), " tag: {}"), tag),
                };
                ::core::result::Result::Ok(())
            }
        }
        impl ::std::default::Default for Type {
            fn default() -> Self {
                Type::S(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum Type {
            S(::pilota::FastStr),

            I(i32),
        }
        impl Type {
            pub fn encode<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                match self {
                    Type::S(value) => {
                        ::pilota::prost::encoding::faststr::encode(2, &*value, buf);
                    }
                    Type::I(value) => {
                        ::pilota::prost::encoding::int32::encode(4, &*value, buf);
                    }
                }
            }

            #[inline]
            pub fn encoded_len(&self) -> usize {
                match self {
                    Type::S(value) => ::pilota::prost::encoding::faststr::encoded_len(2, &*value),
                    Type::I(value) => ::pilota::prost::encoding::int32::encoded_len(4, &*value),
                }
            }

            #[inline]
            pub fn merge<B>(
                field: &mut ::core::option::Option<Self>,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                match tag {
                    2 => match field {
                        ::core::option::Option::Some(Type::S(value)) => {
                            ::pilota::prost::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Type::S(owned_value));
                        }
                    },
                    4 => match field {
                        ::core::option::Option::Some(Type::I(value)) => {
                            ::pilota::prost::encoding::int32::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::int32::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Type::I(owned_value));
                        }
                    },
                    _ => unreachable!(concat!("invalid ", stringify!(Type), " tag: {}"), tag),
                };
                ::core::result::Result::Ok(())
            }
        }
    }
}
