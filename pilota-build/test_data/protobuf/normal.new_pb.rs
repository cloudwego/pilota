pub mod normal {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct A {
        pub a: i32,
    }
    impl ::pilota::pb::Message for A {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::pb::encoding::int32::encoded_len(1, &self.a)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::int32::encode(1, &self.a, buf);
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

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.a;
                    ::pilota::pb::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(a));
                            error
                        })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct SubMessage {
        pub value: ::std::option::Option<::pilota::FastStr>,
    }
    impl ::pilota::pb::Message for SubMessage {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.value.as_ref().map_or(0, |value| {
                ::pilota::pb::encoding::faststr::encoded_len(2, value)
            })
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.value.as_ref() {
                ::pilota::pb::encoding::faststr::encode(2, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(SubMessage);

            match tag {
                2 => {
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
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }

    pub trait TestService {}
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct ObjReq {
        pub msg: ::std::option::Option<Message>,

        pub msg_map: ::std::vec::Vec<obj_req::MsgMapEntry>,

        pub sub_msgs: ::std::vec::Vec<SubMessage>,

        pub msg_set: ::std::vec::Vec<Message>,

        pub flag_msg: ::pilota::FastStr,

        pub mock_cost: ::std::option::Option<::pilota::FastStr>,
    }
    impl ::pilota::pb::Message for ObjReq {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.msg.as_ref().map_or(0, |msg| {
                ::pilota::pb::encoding::message::encoded_len(1, msg)
            }) + ::pilota::pb::encoding::message::encoded_len_repeated(2, &self.msg_map)
                + ::pilota::pb::encoding::message::encoded_len_repeated(3, &self.sub_msgs)
                + ::pilota::pb::encoding::message::encoded_len_repeated(4, &self.msg_set)
                + ::pilota::pb::encoding::faststr::encoded_len(5, &self.flag_msg)
                + self.mock_cost.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::faststr::encoded_len(6, value)
                })
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.msg.as_ref() {
                ::pilota::pb::encoding::message::encode(1, _pilota_inner_value, buf);
            }
            for msg in &self.msg_map {
                ::pilota::pb::encoding::message::encode(2, msg, buf);
            }
            for msg in &self.sub_msgs {
                ::pilota::pb::encoding::message::encode(3, msg, buf);
            }
            for msg in &self.msg_set {
                ::pilota::pb::encoding::message::encode(4, msg, buf);
            }
            ::pilota::pb::encoding::faststr::encode(5, &self.flag_msg, buf);
            if let Some(_pilota_inner_value) = self.mock_cost.as_ref() {
                ::pilota::pb::encoding::faststr::encode(6, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(ObjReq);

            match tag {
                1 => {
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
                2 => {
                    let mut _inner_pilota_value = &mut self.msg_map;
                    ::pilota::pb::encoding::message::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(msg_map));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.sub_msgs;
                    ::pilota::pb::encoding::message::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(sub_msgs));
                        error
                    })
                }
                4 => {
                    let mut _inner_pilota_value = &mut self.msg_set;
                    ::pilota::pb::encoding::message::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(msg_set));
                        error
                    })
                }
                5 => {
                    let mut _inner_pilota_value = &mut self.flag_msg;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(flag_msg));
                            error
                        })
                }
                6 => {
                    let mut _inner_pilota_value = &mut self.mock_cost;
                    ::pilota::pb::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(mock_cost));
                        error
                    })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct B {
        pub a: ::std::option::Option<A>,
    }
    impl ::pilota::pb::Message for B {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.a.as_ref().map_or(0, |msg| {
                ::pilota::pb::encoding::message::encoded_len(2, msg)
            })
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            if let Some(_pilota_inner_value) = self.a.as_ref() {
                ::pilota::pb::encoding::message::encode(2, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(B);

            match tag {
                2 => {
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
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct Message {
        pub uid: ::pilota::FastStr,

        pub value: ::std::option::Option<::pilota::FastStr>,

        pub sub_messages: ::std::vec::Vec<SubMessage>,
    }
    impl ::pilota::pb::Message for Message {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len(1, &self.uid)
                + self.value.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::faststr::encoded_len(2, value)
                })
                + ::pilota::pb::encoding::message::encoded_len_repeated(3, &self.sub_messages)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode(1, &self.uid, buf);
            if let Some(_pilota_inner_value) = self.value.as_ref() {
                ::pilota::pb::encoding::faststr::encode(2, _pilota_inner_value, buf);
            };
            for msg in &self.sub_messages {
                ::pilota::pb::encoding::message::encode(3, msg, buf);
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
            const STRUCT_NAME: &'static str = stringify!(Message);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.uid;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(uid));
                            error
                        })
                }
                2 => {
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
                3 => {
                    let mut _inner_pilota_value = &mut self.sub_messages;
                    ::pilota::pb::encoding::message::merge_repeated(
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
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }

    pub mod obj_req {
        use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct MsgMapEntry {
            pub key: ::std::option::Option<super::Message>,

            pub value: ::std::option::Option<super::SubMessage>,
        }
        impl ::pilota::pb::Message for MsgMapEntry {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.key.as_ref().map_or(0, |msg| {
                    ::pilota::pb::encoding::message::encoded_len(1, msg)
                }) + self.value.as_ref().map_or(0, |msg| {
                    ::pilota::pb::encoding::message::encoded_len(2, msg)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.key.as_ref() {
                    ::pilota::pb::encoding::message::encode(1, _pilota_inner_value, buf);
                }
                if let Some(_pilota_inner_value) = self.value.as_ref() {
                    ::pilota::pb::encoding::message::encode(2, _pilota_inner_value, buf);
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
                const STRUCT_NAME: &'static str = stringify!(MsgMapEntry);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.key;
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(key));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.value;
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(value));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
    }
}
