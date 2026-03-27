pub mod repeated_nonpackable {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    pub mod covnp {
        use ::pilota::{Buf as _, BufMut as _};
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct RepeatedNonPackable {
            pub v_str: ::std::vec::Vec<::pilota::FastStr>,

            pub v_bytes: ::std::vec::Vec<::pilota::Bytes>,
        }
        impl ::pilota::pb::Message for RepeatedNonPackable {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len_repeated(ctx, 1, &self.v_str)
                    + ::pilota::pb::encoding::bytes::encoded_len_repeated(ctx, 2, &self.v_bytes)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode_repeated(1, &self.v_str, buf);
                ::pilota::pb::encoding::bytes::encode_repeated(2, &self.v_bytes, buf);
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
                const STRUCT_NAME: &'static str = stringify!(RepeatedNonPackable);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.v_str;
                        ::pilota::pb::encoding::faststr::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(v_str));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.v_bytes;
                        ::pilota::pb::encoding::bytes::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(v_bytes));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
    }
}
