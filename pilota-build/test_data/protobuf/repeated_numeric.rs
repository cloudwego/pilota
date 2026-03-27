pub mod repeated_numeric {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    pub mod cov {
        use ::pilota::{Buf as _, BufMut as _};
        #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
        pub struct RepeatedNumeric {
            pub v_int32: ::std::vec::Vec<i32>,

            pub v_float: ::std::vec::Vec<f32>,

            pub v_u64: ::std::vec::Vec<u64>,

            pub v_enum: ::std::vec::Vec<E>,
        }
        impl ::pilota::pb::Message for RepeatedNumeric {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::int32::encoded_len_packed_convert(ctx, 1, &self.v_int32)
                    + ::pilota::pb::encoding::float::encoded_len_packed(ctx, 2, &self.v_float)
                    + ::pilota::pb::encoding::uint64::encoded_len_packed(ctx, 3, &self.v_u64)
                    + ::pilota::pb::encoding::int32::encoded_len_packed_convert(
                        ctx,
                        4,
                        &self.v_enum,
                    )
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::int32::encode_packed_convert(1, &self.v_int32, buf);
                ::pilota::pb::encoding::float::encode_packed(2, &self.v_float, buf);
                ::pilota::pb::encoding::uint64::encode_packed(3, &self.v_u64, buf);
                ::pilota::pb::encoding::int32::encode_packed_convert(4, &self.v_enum, buf);
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
                const STRUCT_NAME: &'static str = stringify!(RepeatedNumeric);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.v_int32;
                        ::pilota::pb::encoding::int32::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(v_int32));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.v_float;
                        ::pilota::pb::encoding::float::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(v_float));
                            error
                        })
                    }
                    3 => {
                        let mut _inner_pilota_value = &mut self.v_u64;
                        ::pilota::pb::encoding::uint64::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(v_u64));
                            error
                        })
                    }
                    4 => {
                        let mut _inner_pilota_value = &mut self.v_enum;
                        ::pilota::pb::encoding::int32::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(v_enum));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct E(i32);

        impl E {
            pub const E0: Self = Self(0);
            pub const E1: Self = Self(1);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("E0"),
                    Self(1) => ::std::string::String::from("E1"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => Some(Self::E0),
                    1 => Some(Self::E1),
                    _ => None,
                }
            }
        }

        impl ::std::convert::From<i32> for E {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<E> for i32 {
            fn from(value: E) -> i32 {
                value.0
            }
        }
    }
}
