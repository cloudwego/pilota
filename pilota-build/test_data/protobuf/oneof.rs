pub mod oneof {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct Test {
        pub c: i32,

        pub r#type: ::std::option::Option<test::Type>,

        pub j: i64,

        pub test: ::std::option::Option<test::Test>,

        pub e: ::std::option::Option<Enum>,
    }
    impl ::pilota::pb::Message for Test {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::int32::encoded_len_if_not_default(ctx, 1, &self.c)
                + self.r#type.as_ref().map_or(0, |msg| msg.encoded_len(ctx))
                + ::pilota::pb::encoding::int64::encoded_len_if_not_default(ctx, 5, &self.j)
                + self.test.as_ref().map_or(0, |msg| msg.encoded_len(ctx))
                + self.e.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::int32::encoded_len_if_not_default(ctx, 10, value)
                })
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::int32::encode_if_not_default(1, &self.c, buf);
            if let Some(_pilota_inner_value) = self.r#type.as_ref() {
                _pilota_inner_value.encode(buf);
            }
            ::pilota::pb::encoding::int64::encode_if_not_default(5, &self.j, buf);
            if let Some(_pilota_inner_value) = self.test.as_ref() {
                _pilota_inner_value.encode(buf);
            }
            if let Some(_pilota_inner_value) = self.e.as_ref() {
                ::pilota::pb::encoding::int32::encode_if_not_default(10, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(Test);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.c;
                    ::pilota::pb::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(c));
                            error
                        })
                }
                2 | 4 => {
                    let mut _inner_pilota_value = &mut self.r#type;
                    test::Type::merge(_inner_pilota_value, tag, wire_type, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, stringify!(r#type));
                            error
                        },
                    )
                }
                5 => {
                    let mut _inner_pilota_value = &mut self.j;
                    ::pilota::pb::encoding::int64::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(j));
                            error
                        })
                }
                6 | 8 => {
                    let mut _inner_pilota_value = &mut self.test;
                    test::Test::merge(_inner_pilota_value, tag, wire_type, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, stringify!(test));
                            error
                        },
                    )
                }
                10 => {
                    let mut _inner_pilota_value = &mut self.e;
                    ::pilota::pb::encoding::int32::merge(
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
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct D {
        pub type_spec_class: d::F,

        pub a: ::std::option::Option<::std::boxed::Box<A>>,

        pub name: ::pilota::FastStr,

        pub num: i32,
    }
    impl ::pilota::pb::Message for D {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::int32::encoded_len_if_not_default(
                ctx,
                1,
                &self.type_spec_class,
            ) + self.a.as_ref().map_or(0, |msg| {
                ::pilota::pb::encoding::message::encoded_len(ctx, 2, msg)
            }) + ::pilota::pb::encoding::faststr::encoded_len_if_not_default(ctx, 3, &self.name)
                + ::pilota::pb::encoding::int32::encoded_len_if_not_default(ctx, 4, &self.num)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::int32::encode_if_not_default(1, &self.type_spec_class, buf);
            if let Some(_pilota_inner_value) = self.a.as_ref() {
                ::pilota::pb::encoding::message::encode(2, _pilota_inner_value, buf);
            }
            ::pilota::pb::encoding::faststr::encode_if_not_default(3, &self.name, buf);
            ::pilota::pb::encoding::int32::encode_if_not_default(4, &self.num, buf);
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
            const STRUCT_NAME: &'static str = stringify!(D);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.type_spec_class;
                    ::pilota::pb::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(type_spec_class));
                            error
                        })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::pb::encoding::message::merge(
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
                3 => {
                    let mut _inner_pilota_value = &mut self.name;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(name));
                            error
                        })
                }
                4 => {
                    let mut _inner_pilota_value = &mut self.num;
                    ::pilota::pb::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(num));
                            error
                        })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct TupleValue {
        pub values: ::std::vec::Vec<A>,
    }
    impl ::pilota::pb::Message for TupleValue {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::message::encoded_len_repeated(ctx, 1, &self.values)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            for msg in &self.values {
                ::pilota::pb::encoding::message::encode(1, msg, buf);
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
            const STRUCT_NAME: &'static str = stringify!(TupleValue);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.values;
                    ::pilota::pb::encoding::message::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(values));
                        error
                    })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct DictValue {
        pub fields: ::pilota::AHashMap<::pilota::FastStr, A>,
    }
    impl ::pilota::pb::Message for DictValue {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::hash_map::encoded_len(
                ctx,
                ::pilota::pb::encoding::faststr::encoded_len,
                ::pilota::pb::encoding::message::encoded_len,
                1,
                &self.fields,
            )
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::hash_map::encode(
                ::pilota::pb::encoding::faststr::encode,
                ::pilota::pb::encoding::faststr::encoded_len,
                ::pilota::pb::encoding::message::encode,
                ::pilota::pb::encoding::message::encoded_len,
                1,
                &self.fields,
                buf,
            );
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
            const STRUCT_NAME: &'static str = stringify!(DictValue);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.fields;
                    ::pilota::pb::encoding::hash_map::merge(
                        ::pilota::pb::encoding::faststr::merge,
                        ::pilota::pb::encoding::message::merge,
                        &mut _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(fields));
                        error
                    })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct A {
        pub b: ::std::option::Option<::std::boxed::Box<a::B>>,
    }
    impl ::pilota::pb::Message for A {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + self.b.as_ref().map_or(0, |msg| msg.encoded_len(ctx))
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.b.as_ref() {
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
                1 | 11 | 12 | 13 | 14 | 33 | 34 | 35 | 51 | 52 | 53 | 54 => {
                    let mut _inner_pilota_value = &mut self.b;
                    a::B::merge(_inner_pilota_value, tag, wire_type, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, stringify!(b));
                            error
                        },
                    )
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct PairValue {
        pub key: ::pilota::FastStr,

        pub value: ::std::option::Option<A>,
    }
    impl ::pilota::pb::Message for PairValue {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len_if_not_default(ctx, 1, &self.key)
                + self.value.as_ref().map_or(0, |msg| {
                    ::pilota::pb::encoding::message::encoded_len(ctx, 2, msg)
                })
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode_if_not_default(1, &self.key, buf);
            if let Some(_pilota_inner_value) = self.value.as_ref() {
                ::pilota::pb::encoding::message::encode(2, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(PairValue);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.key;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(key));
                            error
                        })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.value;
                    ::pilota::pb::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(value));
                        error
                    })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct NamedTupleValue {
        pub name: ::pilota::FastStr,

        pub values: ::std::vec::Vec<PairValue>,
    }
    impl ::pilota::pb::Message for NamedTupleValue {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len_if_not_default(ctx, 1, &self.name)
                + ::pilota::pb::encoding::message::encoded_len_repeated(ctx, 2, &self.values)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode_if_not_default(1, &self.name, buf);
            for msg in &self.values {
                ::pilota::pb::encoding::message::encode(2, msg, buf);
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
            const STRUCT_NAME: &'static str = stringify!(NamedTupleValue);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.name;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(name));
                            error
                        })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.values;
                    ::pilota::pb::encoding::message::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(values));
                        error
                    })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
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

    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct C {
        pub name: ::pilota::FastStr,
    }
    impl ::pilota::pb::Message for C {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len_if_not_default(ctx, 1, &self.name)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode_if_not_default(1, &self.name, buf);
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
            const STRUCT_NAME: &'static str = stringify!(C);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.name;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(name));
                            error
                        })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct NoneValue {}
    impl ::pilota::pb::Message for NoneValue {
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
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct E {
        pub name: ::pilota::FastStr,
    }
    impl ::pilota::pb::Message for E {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len_if_not_default(ctx, 1, &self.name)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode_if_not_default(1, &self.name, buf);
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
            const STRUCT_NAME: &'static str = stringify!(E);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.name;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(name));
                            error
                        })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct ListValue {
        pub values: ::std::vec::Vec<A>,
    }
    impl ::pilota::pb::Message for ListValue {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::message::encoded_len_repeated(ctx, 1, &self.values)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            for msg in &self.values {
                ::pilota::pb::encoding::message::encode(1, msg, buf);
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
            const STRUCT_NAME: &'static str = stringify!(ListValue);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.values;
                    ::pilota::pb::encoding::message::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(values));
                        error
                    })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }

    pub mod a {
        use ::pilota::{Buf as _, BufMut as _};

        impl ::std::default::Default for B {
            fn default() -> Self {
                B::NoneValue(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum B {
            NoneValue(super::NoneValue),

            Float64Value(f64),

            Int64Value(i64),

            StringValue(::pilota::FastStr),

            BoolValue(bool),

            C(super::C),

            D(super::D),

            E(super::E),

            ListValue(super::ListValue),

            TupleValue(super::TupleValue),

            DictValue(super::DictValue),

            NamedTupleValue(super::NamedTupleValue),
        }

        impl B {
            pub fn encode(&self, buf: &mut ::pilota::LinkedBytes) {
                match self {
                    B::NoneValue(value) => {
                        ::pilota::pb::encoding::message::encode(1, (&*value), buf);
                    }
                    B::Float64Value(value) => {
                        ::pilota::pb::encoding::double::encode_if_not_default(11, &*value, buf);
                    }
                    B::Int64Value(value) => {
                        ::pilota::pb::encoding::int64::encode_if_not_default(12, &*value, buf);
                    }
                    B::StringValue(value) => {
                        ::pilota::pb::encoding::faststr::encode_if_not_default(13, &*value, buf);
                    }
                    B::BoolValue(value) => {
                        ::pilota::pb::encoding::bool::encode_if_not_default(14, &*value, buf);
                    }
                    B::C(value) => {
                        ::pilota::pb::encoding::message::encode(33, (&*value), buf);
                    }
                    B::D(value) => {
                        ::pilota::pb::encoding::message::encode(34, (&*value), buf);
                    }
                    B::E(value) => {
                        ::pilota::pb::encoding::message::encode(35, (&*value), buf);
                    }
                    B::ListValue(value) => {
                        ::pilota::pb::encoding::message::encode(51, (&*value), buf);
                    }
                    B::TupleValue(value) => {
                        ::pilota::pb::encoding::message::encode(52, (&*value), buf);
                    }
                    B::DictValue(value) => {
                        ::pilota::pb::encoding::message::encode(53, (&*value), buf);
                    }
                    B::NamedTupleValue(value) => {
                        ::pilota::pb::encoding::message::encode(54, (&*value), buf);
                    }
                }
            }

            #[inline]
            pub fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                match self {
                    B::NoneValue(value) => {
                        ::pilota::pb::encoding::message::encoded_len(ctx, 1, &*value)
                    }
                    B::Float64Value(value) => {
                        ::pilota::pb::encoding::double::encoded_len_if_not_default(ctx, 11, &*value)
                    }
                    B::Int64Value(value) => {
                        ::pilota::pb::encoding::int64::encoded_len_if_not_default(ctx, 12, &*value)
                    }
                    B::StringValue(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len_if_not_default(
                            ctx, 13, &*value,
                        )
                    }
                    B::BoolValue(value) => {
                        ::pilota::pb::encoding::bool::encoded_len_if_not_default(ctx, 14, &*value)
                    }
                    B::C(value) => ::pilota::pb::encoding::message::encoded_len(ctx, 33, &*value),
                    B::D(value) => ::pilota::pb::encoding::message::encoded_len(ctx, 34, &*value),
                    B::E(value) => ::pilota::pb::encoding::message::encoded_len(ctx, 35, &*value),
                    B::ListValue(value) => {
                        ::pilota::pb::encoding::message::encoded_len(ctx, 51, &*value)
                    }
                    B::TupleValue(value) => {
                        ::pilota::pb::encoding::message::encoded_len(ctx, 52, &*value)
                    }
                    B::DictValue(value) => {
                        ::pilota::pb::encoding::message::encoded_len(ctx, 53, &*value)
                    }
                    B::NamedTupleValue(value) => {
                        ::pilota::pb::encoding::message::encoded_len(ctx, 54, &*value)
                    }
                }
            }

            #[inline]
            pub fn merge(
                field: &mut ::core::option::Option<::std::boxed::Box<Self>>,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                match tag {
                    1 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::NoneValue(value) => {
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::NoneValue(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(
                                B::NoneValue(owned_value),
                            ));
                        }
                    },
                    11 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::Float64Value(value) => {
                                ::pilota::pb::encoding::double::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::double::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::Float64Value(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::double::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(
                                B::Float64Value(owned_value),
                            ));
                        }
                    },
                    12 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::Int64Value(value) => {
                                ::pilota::pb::encoding::int64::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::int64::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::Int64Value(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::int64::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(
                                B::Int64Value(owned_value),
                            ));
                        }
                    },
                    13 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::StringValue(value) => {
                                ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::StringValue(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(
                                B::StringValue(owned_value),
                            ));
                        }
                    },
                    14 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::BoolValue(value) => {
                                ::pilota::pb::encoding::bool::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::bool::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::BoolValue(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::bool::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(
                                B::BoolValue(owned_value),
                            ));
                        }
                    },
                    33 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::C(value) => {
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::C(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(B::C(
                                owned_value,
                            )));
                        }
                    },
                    34 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::D(value) => {
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::D(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(B::D(
                                owned_value,
                            )));
                        }
                    },
                    35 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::E(value) => {
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::E(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(B::E(
                                owned_value,
                            )));
                        }
                    },
                    51 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::ListValue(value) => {
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::ListValue(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(
                                B::ListValue(owned_value),
                            ));
                        }
                    },
                    52 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::TupleValue(value) => {
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::TupleValue(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(
                                B::TupleValue(owned_value),
                            ));
                        }
                    },
                    53 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::DictValue(value) => {
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::DictValue(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(
                                B::DictValue(owned_value),
                            ));
                        }
                    },
                    54 => match field.as_mut() {
                        ::core::option::Option::Some(boxed) => match &mut **boxed {
                            B::NamedTupleValue(value) => {
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                                **boxed = B::NamedTupleValue(owned_value);
                            }
                        },
                        ::core::option::Option::None => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(::std::boxed::Box::new(
                                B::NamedTupleValue(owned_value),
                            ));
                        }
                    },
                    _ => unreachable!(concat!("invalid ", stringify!(B), " tag: {}"), tag),
                };
                ::core::result::Result::Ok(())
            }
        }
    }

    pub mod d {
        use ::pilota::{Buf as _, BufMut as _};
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct F(i32);

        impl F {
            pub const UNKNOWN: Self = Self(0);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("UNKNOWN"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => Some(Self::UNKNOWN),
                    _ => None,
                }
            }
        }

        impl ::std::convert::From<i32> for F {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<F> for i32 {
            fn from(value: F) -> i32 {
                value.0
            }
        }
    }

    pub mod test {
        use ::pilota::{Buf as _, BufMut as _};

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
            pub fn encode(&self, buf: &mut ::pilota::LinkedBytes) {
                match self {
                    Type::S(value) => {
                        ::pilota::pb::encoding::faststr::encode_if_not_default(2, &*value, buf);
                    }
                    Type::I(value) => {
                        ::pilota::pb::encoding::int32::encode_if_not_default(4, &*value, buf);
                    }
                }
            }

            #[inline]
            pub fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                match self {
                    Type::S(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len_if_not_default(ctx, 2, &*value)
                    }
                    Type::I(value) => {
                        ::pilota::pb::encoding::int32::encoded_len_if_not_default(ctx, 4, &*value)
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
                    2 => match field {
                        ::core::option::Option::Some(Type::S(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Type::S(owned_value));
                        }
                    },
                    4 => match field {
                        ::core::option::Option::Some(Type::I(value)) => {
                            ::pilota::pb::encoding::int32::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::int32::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Type::I(owned_value));
                        }
                    },
                    _ => unreachable!(concat!("invalid ", stringify!(Type), " tag: {}"), tag),
                };
                ::core::result::Result::Ok(())
            }
        }
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
            pub fn encode(&self, buf: &mut ::pilota::LinkedBytes) {
                match self {
                    Test::A(value) => {
                        ::pilota::pb::encoding::faststr::encode_if_not_default(6, &*value, buf);
                    }
                    Test::B(value) => {
                        ::pilota::pb::encoding::int32::encode_if_not_default(8, &*value, buf);
                    }
                }
            }

            #[inline]
            pub fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                match self {
                    Test::A(value) => {
                        ::pilota::pb::encoding::faststr::encoded_len_if_not_default(ctx, 6, &*value)
                    }
                    Test::B(value) => {
                        ::pilota::pb::encoding::int32::encoded_len_if_not_default(ctx, 8, &*value)
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
                    6 => match field {
                        ::core::option::Option::Some(Test::A(value)) => {
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Test::A(owned_value));
                        }
                    },
                    8 => match field {
                        ::core::option::Option::Some(Test::B(value)) => {
                            ::pilota::pb::encoding::int32::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::pb::encoding::int32::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Test::B(owned_value));
                        }
                    },
                    _ => unreachable!(concat!("invalid ", stringify!(Test), " tag: {}"), tag),
                };
                ::core::result::Result::Ok(())
            }
        }
    }
}
