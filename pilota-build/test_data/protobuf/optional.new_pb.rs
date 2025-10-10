pub mod optional {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct SearchRequest {
        pub page_number: ::std::option::Option<i32>,
    }
    impl ::pilota::pb::Message for SearchRequest {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.page_number.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::int32::encoded_len(2, value)
            })
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.page_number.as_ref() {
                ::pilota::pb::encoding::int32::encode(2, _pilota_inner_value, buf);
            };
        }

        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::pilota::pb::encoding::WireType,
            buf: &mut ::pilota::Bytes,
            ctx: &mut ::pilota::pb::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
            const STRUCT_NAME: &'static str = stringify!(SearchRequest);

            match tag {
                2 => {
                    let mut _inner_pilota_value = &mut self.page_number;
                    ::pilota::pb::encoding::int32::merge(
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
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
}
