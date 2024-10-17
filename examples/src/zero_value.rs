pub mod zero_value {
    #![allow(warnings, clippy::all)]
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct A {
        pub str_map: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,

        pub int_map: ::pilota::AHashMap<::pilota::FastStr, i32>,
    }
    impl ::pilota::prost::Message for A {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::hash_map::encoded_len(
                ::pilota::prost::encoding::faststr::encoded_len,
                ::pilota::prost::encoding::faststr::encoded_len,
                1,
                &self.str_map,
            ) + ::pilota::prost::encoding::hash_map::encoded_len(
                ::pilota::prost::encoding::faststr::encoded_len,
                ::pilota::prost::encoding::int32::encoded_len,
                2,
                &self.int_map,
            )
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::hash_map::encode(
                ::pilota::prost::encoding::faststr::encode,
                ::pilota::prost::encoding::faststr::encoded_len,
                ::pilota::prost::encoding::faststr::encode,
                ::pilota::prost::encoding::faststr::encoded_len,
                1,
                &self.str_map,
                buf,
            );
            ::pilota::prost::encoding::hash_map::encode(
                ::pilota::prost::encoding::faststr::encode,
                ::pilota::prost::encoding::faststr::encoded_len,
                ::pilota::prost::encoding::int32::encode,
                ::pilota::prost::encoding::int32::encoded_len,
                2,
                &self.int_map,
                buf,
            );
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
            const STRUCT_NAME: &'static str = stringify!(A);
            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.str_map;
                    ::pilota::prost::encoding::hash_map::merge(
                        ::pilota::prost::encoding::faststr::merge,
                        ::pilota::prost::encoding::faststr::merge,
                        &mut _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(str_map));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.int_map;
                    ::pilota::prost::encoding::hash_map::merge(
                        ::pilota::prost::encoding::faststr::merge,
                        ::pilota::prost::encoding::int32::merge,
                        &mut _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(int_map));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
}
