pub mod zero_value {
    #![allow(warnings, clippy::all)]
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct A {
        pub str_map: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,

        pub s1: ::pilota::FastStr,

        pub s2: ::std::option::Option<::pilota::FastStr>,
    }
    impl ::pilota::prost::Message for A {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::hash_map::encoded_len(
                ::pilota::prost::encoding::faststr::encoded_len,
                ::pilota::prost::encoding::faststr::encoded_len,
                1,
                &self.str_map,
            ) + ::pilota::prost::encoding::faststr::encoded_len(2, &self.s1)
                + self.s2.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(3, value)
                })
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
            ::pilota::prost::encoding::faststr::encode(2, &self.s1, buf);
            if let Some(_pilota_inner_value) = self.s2.as_ref() {
                ::pilota::prost::encoding::faststr::encode(3, _pilota_inner_value, buf);
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
                    let mut _inner_pilota_value = &mut self.s1;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(s1));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.s2;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(s2));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
}
