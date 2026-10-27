pub mod option_shadow {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    pub mod option_shadow {
        use ::pilota::{Buf as _, BufMut as _};
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Inner {
            pub id: i32,
        }
        impl ::pilota::pb::Message for Inner {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::int32::encoded_len(ctx, 1, &self.id)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::int32::encode(1, &self.id, buf);
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
                        let mut _inner_pilota_value = &mut self.id;
                        ::pilota::pb::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(id));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }

        pub trait ItemService {}
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct Item {
            pub name: ::std::option::Option<::pilota::FastStr>,

            pub some_code: Some,

            pub none_code: None,

            pub nums: ::std::vec::Vec<i32>,

            pub tags: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,

            pub inner: ::std::option::Option<Inner>,

            pub choice: ::std::option::Option<item::Choice>,
        }
        impl ::pilota::pb::Message for Item {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + self.name.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, value)
                }) + ::pilota::pb::encoding::int32::encoded_len(ctx, 2, &self.some_code)
                    + ::pilota::pb::encoding::int32::encoded_len(ctx, 3, &self.none_code)
                    + ::pilota::pb::encoding::int32::encoded_len_packed_convert(ctx, 4, &self.nums)
                    + ::pilota::pb::encoding::hash_map::encoded_len(
                        ctx,
                        ::pilota::pb::encoding::faststr::encoded_len,
                        ::pilota::pb::encoding::faststr::encoded_len,
                        5,
                        &self.tags,
                    )
                    + self.inner.as_ref().map_or(0, |msg| {
                        ::pilota::pb::encoding::message::encoded_len(ctx, 6, msg)
                    })
                    + self.choice.as_ref().map_or(0, |msg| msg.encoded_len(ctx))
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let ::std::option::Option::Some(_pilota_inner_value) = self.name.as_ref() {
                    ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                };
                ::pilota::pb::encoding::int32::encode(2, &self.some_code, buf);
                ::pilota::pb::encoding::int32::encode(3, &self.none_code, buf);
                ::pilota::pb::encoding::int32::encode_packed_convert(4, &self.nums, buf);
                ::pilota::pb::encoding::hash_map::encode(
                    ::pilota::pb::encoding::faststr::encode,
                    ::pilota::pb::encoding::faststr::encoded_len,
                    ::pilota::pb::encoding::faststr::encode,
                    ::pilota::pb::encoding::faststr::encoded_len,
                    5,
                    &self.tags,
                    buf,
                );
                if let ::std::option::Option::Some(_pilota_inner_value) = self.inner.as_ref() {
                    ::pilota::pb::encoding::message::encode(6, _pilota_inner_value, buf);
                }
                if let ::std::option::Option::Some(_pilota_inner_value) = self.choice.as_ref() {
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
                const STRUCT_NAME: &'static str = stringify!(Item);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.name;
                        ::pilota::pb::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(name));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.some_code;
                        ::pilota::pb::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(some_code));
                            error
                        })
                    }
                    3 => {
                        let mut _inner_pilota_value = &mut self.none_code;
                        ::pilota::pb::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(none_code));
                            error
                        })
                    }
                    4 => {
                        let mut _inner_pilota_value = &mut self.nums;
                        ::pilota::pb::encoding::int32::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(nums));
                            error
                        })
                    }
                    5 => {
                        let mut _inner_pilota_value = &mut self.tags;
                        ::pilota::pb::encoding::hash_map::merge(
                            ::pilota::pb::encoding::faststr::merge,
                            ::pilota::pb::encoding::faststr::merge,
                            &mut _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(tags));
                            error
                        })
                    }
                    6 => {
                        let mut _inner_pilota_value = &mut self.inner;
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(inner));
                            error
                        })
                    }
                    7 | 8 => {
                        let mut _inner_pilota_value = &mut self.choice;
                        item::Choice::merge(_inner_pilota_value, tag, wire_type, buf, ctx).map_err(
                            |mut error| {
                                error.push(STRUCT_NAME, stringify!(choice));
                                error
                            },
                        )
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct None(i32);

        impl None {
            pub const NONE_ZERO: Self = Self(0);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("NONE_ZERO"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => ::std::option::Option::Some(Self::NONE_ZERO),
                    _ => ::std::option::Option::None,
                }
            }
        }

        impl ::std::convert::From<i32> for None {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<None> for i32 {
            fn from(value: None) -> i32 {
                value.0
            }
        }

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Some(i32);

        impl Some {
            pub const SOME_UNSPECIFIED: Self = Self(0);
            pub const SOME_ONE: Self = Self(1);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("SOME_UNSPECIFIED"),
                    Self(1) => ::std::string::String::from("SOME_ONE"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => ::std::option::Option::Some(Self::SOME_UNSPECIFIED),
                    1 => ::std::option::Option::Some(Self::SOME_ONE),
                    _ => ::std::option::Option::None,
                }
            }
        }

        impl ::std::convert::From<i32> for Some {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Some> for i32 {
            fn from(value: Some) -> i32 {
                value.0
            }
        }

        pub mod item {
            use ::pilota::{Buf as _, BufMut as _};

            impl ::std::default::Default for Choice {
                fn default() -> Self {
                    Choice::Text(::std::default::Default::default())
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
            pub enum Choice {
                Text(::pilota::FastStr),

                Other(super::Inner),
            }

            impl Choice {
                pub fn encode(&self, buf: &mut ::pilota::LinkedBytes) {
                    match self {
                        Choice::Text(value) => {
                            ::pilota::pb::encoding::faststr::encode(7, &*value, buf);
                        }
                        Choice::Other(value) => {
                            ::pilota::pb::encoding::message::encode(8, (&*value), buf);
                        }
                    }
                }

                #[inline]
                pub fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                    match self {
                        Choice::Text(value) => {
                            ::pilota::pb::encoding::faststr::encoded_len(ctx, 7, &*value)
                        }
                        Choice::Other(value) => {
                            ::pilota::pb::encoding::message::encoded_len(ctx, 8, &*value)
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
                        7 => match field {
                            ::core::option::Option::Some(Choice::Text(value)) => {
                                ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                                *field = ::core::option::Option::Some(Choice::Text(owned_value));
                            }
                        },
                        8 => match field {
                            ::core::option::Option::Some(Choice::Other(value)) => {
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::message::merge(wire_type, value, buf, ctx)?;
                                *field = ::core::option::Option::Some(Choice::Other(owned_value));
                            }
                        },
                        _ => unreachable!(concat!("invalid ", stringify!(Choice), " tag: {}"), tag),
                    };
                    ::core::result::Result::Ok(())
                }
            }
        }
    }
}
