pub mod bytes {
    #![allow(warnings, clippy::all)]
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct A {
        pub a: ::std::option::Option<::pilota::Bytes>,
    }
    impl ::pilota::prost::Message for A {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.a.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::bytes::encoded_len(1, value)
            })
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.a.as_ref() {
                ::pilota::prost::encoding::bytes::encode(1, _pilota_inner_value, buf);
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
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::prost::encoding::bytes::merge(
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
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
}
