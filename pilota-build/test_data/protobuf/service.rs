pub mod service {
    #![allow(warnings, clippy::all)]

    pub mod service {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct EchoRequest {
            pub message: ::pilota::FastStr,
        }
        impl ::pilota::prost::Message for EchoRequest {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.message)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                ::pilota::prost::encoding::faststr::encode(1, &self.message, buf);
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
                const STRUCT_NAME: &'static str = stringify!(EchoRequest);
                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.message;
                        ::pilota::prost::encoding::faststr::merge(
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
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }

        #[::async_trait::async_trait]
        pub trait Echo {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct EchoResponse {
            pub message: ::pilota::FastStr,
        }
        impl ::pilota::prost::Message for EchoResponse {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.message)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                ::pilota::prost::encoding::faststr::encode(1, &self.message, buf);
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
                const STRUCT_NAME: &'static str = stringify!(EchoResponse);
                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.message;
                        ::pilota::prost::encoding::faststr::merge(
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
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
    }
}
