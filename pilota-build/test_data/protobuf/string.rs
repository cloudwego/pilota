pub mod string {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct A {
        pub a: ::std::option::Option<::pilota::FastStr>,

        pub b: ::pilota::FastStr,
    }
    impl ::pilota::pb::Message for A {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + self.a.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, value)
            }) + ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &self.b)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.a.as_ref() {
                ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
            };
            ::pilota::pb::encoding::faststr::encode(2, &self.b, buf);
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
            const STRUCT_NAME: &'static str = stringify!(A);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::pb::encoding::faststr::merge(
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
                2 => {
                    let mut _inner_pilota_value = &mut self.b;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(b));
                            error
                        })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
}
