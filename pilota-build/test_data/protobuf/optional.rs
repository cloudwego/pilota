pub mod optional {
    #![allow(warnings, clippy::all)]
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct SearchRequest {
        pub page_number: ::std::option::Option<i32>,
    }
    impl ::pilota::prost::Message for SearchRequest {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.page_number.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::int32::encoded_len(2, value)
            })
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.page_number.as_ref() {
                ::pilota::prost::encoding::int32::encode(2, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(SearchRequest);
            match tag {
                2 => {
                    let mut _inner_pilota_value = &mut self.page_number;
                    ::pilota::prost::encoding::int32::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(page_number));
                        error
                    })
                }

                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
}
