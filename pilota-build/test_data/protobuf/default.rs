pub mod default {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct Outer {}
    impl ::pilota::pb::Message for Outer {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {}

        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::pilota::pb::encoding::WireType,
            buf: &mut ::pilota::Bytes,
            ctx: &mut ::pilota::pb::encoding::DecodeContext,
            is_root: bool,
        ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
            match tag {
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct Color(i32);

    impl Color {
        pub const RED: Self = Self(0);
        pub const GREEN: Self = Self(1);
        pub const BLUE: Self = Self(2);

        pub fn inner(&self) -> i32 {
            self.0
        }

        pub fn to_string(&self) -> ::std::string::String {
            match self {
                Self(0) => ::std::string::String::from("RED"),
                Self(1) => ::std::string::String::from("GREEN"),
                Self(2) => ::std::string::String::from("BLUE"),
                Self(val) => val.to_string(),
            }
        }

        pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
            match value {
                0 => Some(Self::RED),
                1 => Some(Self::GREEN),
                2 => Some(Self::BLUE),
                _ => None,
            }
        }
    }

    impl ::std::convert::From<i32> for Color {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl ::std::convert::From<Color> for i32 {
        fn from(value: Color) -> i32 {
            value.0
        }
    }

    impl ::std::default::Default for Defaults {
        fn default() -> Self {
            Defaults {
                a: Some(42i32),
                a64: Some(-9007199254740991i64),
                b: Some(3.5f32),
                c: Some(-1.25f64),
                d: Some(true),
                e: Some(::pilota::FastStr::from_static_str("hi")),
                color: Some(Color::GREEN),
                inner: ::std::default::Default::default(),
                zero: Some(0i32),
                dzero: Some(0f64),
            }
        }
    }
    #[derive(PartialOrd, Debug, Clone, PartialEq)]
    pub struct Defaults {
        pub a: ::std::option::Option<i32>,

        pub a64: ::std::option::Option<i64>,

        pub b: ::std::option::Option<f32>,

        pub c: ::std::option::Option<f64>,

        pub d: ::std::option::Option<bool>,

        pub e: ::std::option::Option<::pilota::FastStr>,

        pub color: ::std::option::Option<Color>,

        pub inner: outer::Inner,

        pub zero: ::std::option::Option<i32>,

        pub dzero: ::std::option::Option<f64>,
    }
    impl ::pilota::pb::Message for Defaults {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + self.a.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::int32::encoded_len(ctx, 1, value)
            }) + self.a64.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::int64::encoded_len(ctx, 2, value)
            }) + self.b.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::float::encoded_len(ctx, 3, value)
            }) + self.c.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::double::encoded_len(ctx, 4, value)
            }) + self.d.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::bool::encoded_len(ctx, 5, value)
            }) + self.e.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::faststr::encoded_len(ctx, 6, value)
            }) + self.color.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::int32::encoded_len(ctx, 7, value)
            }) + ::pilota::pb::encoding::message::encoded_len(ctx, 8, &self.inner)
                + self.zero.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::int32::encoded_len(ctx, 9, value)
                })
                + self.dzero.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::double::encoded_len(ctx, 10, value)
                })
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.a.as_ref() {
                ::pilota::pb::encoding::int32::encode(1, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.a64.as_ref() {
                ::pilota::pb::encoding::int64::encode(2, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.b.as_ref() {
                ::pilota::pb::encoding::float::encode(3, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.c.as_ref() {
                ::pilota::pb::encoding::double::encode(4, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.d.as_ref() {
                ::pilota::pb::encoding::bool::encode(5, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.e.as_ref() {
                ::pilota::pb::encoding::faststr::encode(6, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.color.as_ref() {
                ::pilota::pb::encoding::int32::encode(7, _pilota_inner_value, buf);
            };
            ::pilota::pb::encoding::message::encode(8, (&self.inner), buf);
            if let Some(_pilota_inner_value) = self.zero.as_ref() {
                ::pilota::pb::encoding::int32::encode(9, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.dzero.as_ref() {
                ::pilota::pb::encoding::double::encode(10, _pilota_inner_value, buf);
            };
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
            const STRUCT_NAME: &'static str = stringify!(Defaults);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::pb::encoding::int32::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(a));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.a64;
                    ::pilota::pb::encoding::int64::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(a64));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.b;
                    ::pilota::pb::encoding::float::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(b));
                        error
                    })
                }
                4 => {
                    let mut _inner_pilota_value = &mut self.c;
                    ::pilota::pb::encoding::double::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(c));
                        error
                    })
                }
                5 => {
                    let mut _inner_pilota_value = &mut self.d;
                    ::pilota::pb::encoding::bool::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(d));
                        error
                    })
                }
                6 => {
                    let mut _inner_pilota_value = &mut self.e;
                    ::pilota::pb::encoding::faststr::merge(
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
                7 => {
                    let mut _inner_pilota_value = &mut self.color;
                    ::pilota::pb::encoding::int32::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(color));
                        error
                    })
                }
                8 => {
                    let mut _inner_pilota_value = &mut self.inner;
                    ::pilota::pb::encoding::message::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(inner));
                            error
                        })
                }
                9 => {
                    let mut _inner_pilota_value = &mut self.zero;
                    ::pilota::pb::encoding::int32::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(zero));
                        error
                    })
                }
                10 => {
                    let mut _inner_pilota_value = &mut self.dzero;
                    ::pilota::pb::encoding::double::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(dzero));
                        error
                    })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }

    pub mod outer {
        use ::pilota::{Buf as _, BufMut as _};

        impl ::std::default::Default for Inner {
            fn default() -> Self {
                Inner {
                    st: Some(Status::OK),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub struct Inner {
            pub st: ::std::option::Option<Status>,
        }
        impl ::pilota::pb::Message for Inner {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + self.st.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::int32::encoded_len(ctx, 1, value)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.st.as_ref() {
                    ::pilota::pb::encoding::int32::encode(1, _pilota_inner_value, buf);
                };
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
                const STRUCT_NAME: &'static str = stringify!(Inner);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.st;
                        ::pilota::pb::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(st));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Status(i32);

        impl Status {
            pub const UNKNOWN: Self = Self(0);
            pub const OK: Self = Self(1);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("UNKNOWN"),
                    Self(1) => ::std::string::String::from("OK"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => Some(Self::UNKNOWN),
                    1 => Some(Self::OK),
                    _ => None,
                }
            }
        }

        impl ::std::convert::From<i32> for Status {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Status> for i32 {
            fn from(value: Status) -> i32 {
                value.0
            }
        }
    }
}
