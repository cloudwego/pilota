pub mod service {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};

    pub mod service {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct EchoRequest {
            pub message: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for EchoRequest {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(1, &self.message)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.message, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(EchoRequest);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.message;
                        ::pilota::pb::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(message));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }

        pub trait Echo {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct EchoResponse {
            pub message: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for EchoResponse {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(1, &self.message)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.message, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(EchoResponse);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.message;
                        ::pilota::pb::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(message));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
    }
}
