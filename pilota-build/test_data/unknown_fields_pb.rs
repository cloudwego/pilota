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
        pub a: ::std::vec::Vec<i32>,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for A {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::int32::encoded_len_repeated(ctx, 1, &self.a)
                + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::int32::encode_repeated(1, &self.a, buf);
            for bytes in self._unknown_fields.list.iter() {
                buf.insert(bytes.clone());
            }
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
            let mut _unknown_fields = &mut self._unknown_fields;

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::pb::encoding::int32::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(a));
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
    pub struct ObjReq {
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for ObjReq {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            for bytes in self._unknown_fields.list.iter() {
                buf.insert(bytes.clone());
            }
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
            let mut _unknown_fields = &mut self._unknown_fields;
            // short circuit
            if is_root && ctx.root_decoded_fields_num() == 0 {
                // advance buf
                let cur = buf.chunk().as_ptr();
                let len = ctx.raw_bytes_len() - (cur as usize - ctx.raw_bytes_cursor());
                buf.advance(len);

                // read rest bytes
                let val = ctx.raw_bytes_split_to(ctx.raw_bytes_len());
                _unknown_fields.push_back(val);
                return Ok(());
            }
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
    pub struct B {
        pub a: ::std::option::Option<A>,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for B {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + self.a.as_ref().map_or(0, |msg| {
                ::pilota::pb::encoding::message::encoded_len(ctx, 2, msg)
            }) + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.a.as_ref() {
                ::pilota::pb::encoding::message::encode(2, _pilota_inner_value, buf);
            }
            for bytes in self._unknown_fields.list.iter() {
                buf.insert(bytes.clone());
            }
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
            const STRUCT_NAME: &'static str = stringify!(B);
            let mut _unknown_fields = &mut self._unknown_fields;
            // short circuit
            if is_root && !matches!(tag, 2) && ctx.root_decoded_fields_num() == 1 {
                // advance buf
                let cur = buf.chunk().as_ptr();
                let len = ctx.raw_bytes_len() - (cur as usize - ctx.raw_bytes_cursor());
                buf.advance(len);

                // read rest bytes
                let val = ctx.raw_bytes_split_to(ctx.raw_bytes_len());
                _unknown_fields.push_back(val);
                return Ok(());
            }
            match tag {
                2 => {
                    if is_root {
                        ctx.inc_root_decoded_fields_num(tag);
                    }
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::pb::encoding::message::merge(
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

    pub trait TestService {}
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
    pub struct SubMessage {
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for SubMessage {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            for bytes in self._unknown_fields.list.iter() {
                buf.insert(bytes.clone());
            }
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
            let mut _unknown_fields = &mut self._unknown_fields;
            // short circuit
            if is_root && ctx.root_decoded_fields_num() == 0 {
                // advance buf
                let cur = buf.chunk().as_ptr();
                let len = ctx.raw_bytes_len() - (cur as usize - ctx.raw_bytes_cursor());
                buf.advance(len);

                // read rest bytes
                let val = ctx.raw_bytes_split_to(ctx.raw_bytes_len());
                _unknown_fields.push_back(val);
                return Ok(());
            }
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
    pub struct Message {
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for Message {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            for bytes in self._unknown_fields.list.iter() {
                buf.insert(bytes.clone());
            }
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
            let mut _unknown_fields = &mut self._unknown_fields;
            // short circuit
            if is_root && ctx.root_decoded_fields_num() == 0 {
                // advance buf
                let cur = buf.chunk().as_ptr();
                let len = ctx.raw_bytes_len() - (cur as usize - ctx.raw_bytes_cursor());
                buf.advance(len);

                // read rest bytes
                let val = ctx.raw_bytes_split_to(ctx.raw_bytes_len());
                _unknown_fields.push_back(val);
                return Ok(());
            }
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
