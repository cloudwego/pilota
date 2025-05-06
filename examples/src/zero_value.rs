pub mod zero_value {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct A {
        pub str_map: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,

        pub s1: ::pilota::FastStr,

        pub s2: ::std::option::Option<::pilota::FastStr>,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for A {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::pb::encoding::hash_map::encoded_len(
                ::pilota::pb::encoding::faststr::encoded_len,
                ::pilota::pb::encoding::faststr::encoded_len,
                1,
                &self.str_map,
            ) + ::pilota::pb::encoding::faststr::encoded_len(2, &self.s1)
                + self.s2.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::faststr::encoded_len(2047, value)
                })
                + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::hash_map::encode(
                ::pilota::pb::encoding::faststr::encode,
                ::pilota::pb::encoding::faststr::encoded_len,
                ::pilota::pb::encoding::faststr::encode,
                ::pilota::pb::encoding::faststr::encoded_len,
                1,
                &self.str_map,
                buf,
            );
            ::pilota::pb::encoding::faststr::encode(2, &self.s1, buf);
            if let Some(_pilota_inner_value) = self.s2.as_ref() {
                ::pilota::pb::encoding::faststr::encode(2047, _pilota_inner_value, buf);
            };
            for bytes in self._unknown_fields.list.iter() {
                buf.put_slice(bytes.as_ref());
            }
        }

        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::pilota::pb::encoding::WireType,
            buf: &mut ::pilota::Bytes,
            ctx: &mut ::pilota::pb::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
            const STRUCT_NAME: &'static str = stringify!(A);

            let mut _unknown_fields = &mut self._unknown_fields;
            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.str_map;
                    ::pilota::pb::encoding::hash_map::merge(
                        ::pilota::pb::encoding::faststr::merge,
                        ::pilota::pb::encoding::faststr::merge,
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
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(s1));
                            error
                        })
                }
                2047 => {
                    let mut _inner_pilota_value = &mut self.s2;
                    ::pilota::pb::encoding::faststr::merge(
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
                _ => {
                    ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx)?;
                    let end = buf.chunk().as_ptr();
                    let len = end as usize - ctx.raw_bytes_cursor();
                    let val = ctx.raw_bytes_split_to(len);
                    _unknown_fields.push_back(val);
                    Ok(())
                }
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct UnknownA {
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for UnknownA {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            for bytes in self._unknown_fields.list.iter() {
                buf.put_slice(bytes.as_ref());
            }
        }

        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::pilota::pb::encoding::WireType,
            buf: &mut ::pilota::Bytes,
            ctx: &mut ::pilota::pb::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
            let mut _unknown_fields = &mut self._unknown_fields;
            match tag {
                _ => {
                    ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx)?;
                    let end = buf.chunk().as_ptr();
                    let len = end as usize - ctx.raw_bytes_cursor();
                    let val = ctx.raw_bytes_split_to(len);
                    _unknown_fields.push_back(val);
                    Ok(())
                }
            }
        }
    }
}
