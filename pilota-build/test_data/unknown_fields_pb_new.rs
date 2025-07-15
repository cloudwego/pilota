pub mod unknown_fields_pb_new {
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
        pub a: i32,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for A {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::pb::encoding::int32::encoded_len(1, &self.a) + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::int32::encode(1, &self.a, buf);
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
            is_root: bool,
        ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
            const STRUCT_NAME: &'static str = stringify!(A);
            let mut _unknown_fields = &mut self._unknown_fields;
            // short circuit
            if is_root && !matches!(tag, 1) && ctx.root_decoded_fields_num() == 1 {
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
                1 => {
                    if is_root {
                        ctx.inc_root_decoded_fields_num(tag);
                    }
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::pb::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
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
        pub value: ::std::option::Option<::pilota::FastStr>,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for SubMessage {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.value.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::faststr::encoded_len(2, value)
            }) + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.value.as_ref() {
                ::pilota::pb::encoding::faststr::encode(2, _pilota_inner_value, buf);
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
            is_root: bool,
        ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
            const STRUCT_NAME: &'static str = stringify!(SubMessage);
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
                    let mut _inner_pilota_value = &mut self.value;
                    ::pilota::pb::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(value));
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
        pub msg: ::std::option::Option<Message>,

        pub flag_msg: ::pilota::FastStr,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for ObjReq {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.msg.as_ref().map_or(0, |msg| {
                ::pilota::pb::encoding::message::encoded_len(1, msg)
            }) + ::pilota::pb::encoding::faststr::encoded_len(5, &self.flag_msg)
                + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.msg.as_ref() {
                ::pilota::pb::encoding::message::encode(1, _pilota_inner_value, buf);
            }
            ::pilota::pb::encoding::faststr::encode(5, &self.flag_msg, buf);
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
            is_root: bool,
        ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
            const STRUCT_NAME: &'static str = stringify!(ObjReq);
            let mut _unknown_fields = &mut self._unknown_fields;
            // short circuit
            if is_root && !matches!(tag, 1 | 5) && ctx.root_decoded_fields_num() == 2 {
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
                1 => {
                    if is_root {
                        ctx.inc_root_decoded_fields_num(tag);
                    }
                    let mut _inner_pilota_value = &mut self.msg;
                    ::pilota::pb::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(msg));
                        error
                    })
                }
                5 => {
                    if is_root {
                        ctx.inc_root_decoded_fields_num(tag);
                    }
                    let mut _inner_pilota_value = &mut self.flag_msg;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(flag_msg));
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
    pub struct B {
        pub a: ::std::option::Option<A>,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for B {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.a.as_ref().map_or(0, |msg| {
                ::pilota::pb::encoding::message::encoded_len(2, msg)
            }) + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.a.as_ref() {
                ::pilota::pb::encoding::message::encode(2, _pilota_inner_value, buf);
            }
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
        pub uid: ::pilota::FastStr,

        pub value: ::pilota::FastStr,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::pb::Message for Message {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len(1, &self.uid)
                + ::pilota::pb::encoding::faststr::encoded_len(2, &self.value)
                + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode(1, &self.uid, buf);
            ::pilota::pb::encoding::faststr::encode(2, &self.value, buf);
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
            is_root: bool,
        ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
            const STRUCT_NAME: &'static str = stringify!(Message);
            let mut _unknown_fields = &mut self._unknown_fields;
            // short circuit
            if is_root && !matches!(tag, 1 | 2) && ctx.root_decoded_fields_num() == 2 {
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
                1 => {
                    if is_root {
                        ctx.inc_root_decoded_fields_num(tag);
                    }
                    let mut _inner_pilota_value = &mut self.uid;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(uid));
                            error
                        })
                }
                2 => {
                    if is_root {
                        ctx.inc_root_decoded_fields_num(tag);
                    }
                    let mut _inner_pilota_value = &mut self.value;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(value));
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
}
