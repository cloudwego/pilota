pub mod repeated_numeric_proto2 {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    pub mod cov2 {
        use ::pilota::{Buf as _, BufMut as _};
        #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
        pub struct RepeatedNumeric2 {
            pub v_int32: ::std::vec::Vec<i32>,

            pub v_float: ::std::vec::Vec<f32>,
        }
        impl ::pilota::pb::Message for RepeatedNumeric2 {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::int32::encoded_len_repeated(ctx, 1, &self.v_int32)
                    + ::pilota::pb::encoding::float::encoded_len_repeated(ctx, 2, &self.v_float)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::int32::encode_repeated(1, &self.v_int32, buf);
                ::pilota::pb::encoding::float::encode_repeated(2, &self.v_float, buf);
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
                const STRUCT_NAME: &'static str = stringify!(RepeatedNumeric2);

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
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
    }
}
