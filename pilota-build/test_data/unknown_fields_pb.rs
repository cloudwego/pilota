pub mod unknown_fields_pb {
    #![allow(warnings, clippy::all)]
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
    impl ::pilota::prost::Message for A {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::int32::encoded_len(1, &self.a)
                + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::int32::encode(1, &self.a, buf);
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
                    ::pilota::prost::encoding::int32::merge(
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
    impl ::pilota::prost::Message for ObjReq {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
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
            let mut _unknown_fields = &mut self._unknown_fields;
            match tag {
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
    impl ::pilota::prost::Message for B {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.a.as_ref().map_or(0, |msg| {
                ::pilota::prost::encoding::message::encoded_len(2, msg)
            }) + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.a.as_ref() {
                ::pilota::prost::encoding::message::encode(2, _pilota_inner_value, buf);
            }
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
            const STRUCT_NAME: &'static str = stringify!(B);

            let mut _unknown_fields = &mut self._unknown_fields;
            match tag {
                2 => {
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::prost::encoding::message::merge(
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
    impl ::pilota::prost::Message for SubMessage {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.value.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::faststr::encoded_len(2, value)
            }) + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.value.as_ref() {
                ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
            };
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
            const STRUCT_NAME: &'static str = stringify!(SubMessage);

            let mut _unknown_fields = &mut self._unknown_fields;
            match tag {
                2 => {
                    let mut _inner_pilota_value = &mut self.value;
                    ::pilota::prost::encoding::faststr::merge(
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

        pub value: ::std::option::Option<::pilota::FastStr>,

        pub sub_messages: ::std::vec::Vec<SubMessage>,
        pub _unknown_fields: ::pilota::BytesVec,
    }
    impl ::pilota::prost::Message for Message {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.uid)
                + self.value.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(2, value)
                })
                + ::pilota::prost::encoding::message::encoded_len_repeated(3, &self.sub_messages)
                + self._unknown_fields.size()
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.uid, buf);
            if let Some(_pilota_inner_value) = self.value.as_ref() {
                ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
            };
            for msg in &self.sub_messages {
                ::pilota::prost::encoding::message::encode(3, msg, buf);
            }
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
            const STRUCT_NAME: &'static str = stringify!(Message);

            let mut _unknown_fields = &mut self._unknown_fields;
            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.uid;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(uid));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.value;
                    ::pilota::prost::encoding::faststr::merge(
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
                3 => {
                    let mut _inner_pilota_value = &mut self.sub_messages;
                    ::pilota::prost::encoding::message::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(sub_messages));
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
