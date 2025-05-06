pub mod unknown_fields_pb {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    #[derive(
        PartialOrd,
        Hash,
        Eq,
        Ord,
        Debug,
        Default,
        ::pilota::serde::Serialize,
        ::pilota::serde::Deserialize,
        Clone,
        PartialEq,
    )]
    pub struct A {
        pub a: ::std::option::Option<::pilota::FastStr>,

        pub b: ::pilota::FastStr,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::prost::Message for A {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.a.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::faststr::encoded_len(1, value)
            }) + ::pilota::prost::encoding::faststr::encoded_len(2, &self.b)
                + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.a.as_ref() {
                ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
            };
            ::pilota::prost::encoding::faststr::encode(2, &self.b, buf);
            for bytes in self._unknown_fields.list.iter() {
                buf.put_slice(bytes.as_ref());
            }
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

            let mut _unknown_fields = &mut self._unknown_fields;
            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::prost::encoding::faststr::merge(
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
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(b));
                        error
                    })
                }
                _ => {
                    let tag_value = (tag << 3) | wire_type as u32;
                    let tag_len = ::pilota::prost::encoding::encoded_len_varint(tag_value as u64);
                    let begin = unsafe { buf.chunk().as_ptr().sub(tag_len) };
                    ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx)?;
                    let end = buf.chunk().as_ptr();
                    _unknown_fields.push_back(::pilota::Bytes::copy_from_slice(unsafe {
                        std::slice::from_raw_parts(begin, end as usize - begin as usize)
                    }));
                    Ok(())
                }
            }
        }
    }
}
