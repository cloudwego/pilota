pub mod nested_message {
    #![allow(warnings, clippy::all)]

    pub mod tt1 {
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct Tt1 {
            pub t2: t2::T2,

            pub t3: Label,

            pub t4: t2::Tt3,
        }
        impl ::pilota::prost::Message for Tt1 {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::prost::encoding::message::encoded_len(1, &self.t2)
                    + ::pilota::prost::encoding::int32::encoded_len(2, &self.t3)
                    + ::pilota::prost::encoding::message::encoded_len(4, &self.t4)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                ::pilota::prost::encoding::message::encode(1, &self.t2, buf);
                ::pilota::prost::encoding::int32::encode(2, &self.t3, buf);
                ::pilota::prost::encoding::message::encode(4, &self.t4, buf);
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
                const STRUCT_NAME: &'static str = stringify!(Tt1);
                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.t2;
                        ::pilota::prost::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(t2));
                            error
                        })
                    }

                    2 => {
                        let mut _inner_pilota_value = &mut self.t3;
                        ::pilota::prost::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(t3));
                            error
                        })
                    }

                    4 => {
                        let mut _inner_pilota_value = &mut self.t4;
                        ::pilota::prost::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(t4));
                            error
                        })
                    }

                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }

        impl ::std::convert::From<Label> for i32 {
            fn from(e: Label) -> Self {
                e as _
            }
        }

        impl ::std::convert::TryFrom<i32> for Label {
            type Error = ::pilota::EnumConvertError<i32>;

            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> Result<Self, ::pilota::EnumConvertError<i32>> {
                const LabelOptional: i32 = Label::LabelOptional as i32;
                const LabelRequired: i32 = Label::LabelRequired as i32;
                const LabelRepeated: i32 = Label::LabelRepeated as i32;
                match v {
                    LabelOptional => ::std::result::Result::Ok(Label::LabelOptional),
                    LabelRequired => ::std::result::Result::Ok(Label::LabelRequired),
                    LabelRepeated => ::std::result::Result::Ok(Label::LabelRepeated),

                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(
                        v, "Label",
                    )),
                }
            }
        }
        #[derivative(Default)]
        #[derive(
            ::pilota::derivative::Derivative, Debug, Hash, Eq, Ord, PartialOrd, Clone, PartialEq,
        )]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum Label {
            #[derivative(Default)]
            LabelOptional = 1,

            LabelRequired = 2,

            LabelRepeated = 3,
        }

        pub mod t2 {
            #[derive(Debug, Default, Clone, PartialEq)]
            pub struct T2 {
                pub t3: Tt3,
            }
            impl ::pilota::prost::Message for T2 {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::prost::encoding::message::encoded_len(1, &self.t3)
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    ::pilota::prost::encoding::message::encode(1, &self.t3, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(T2);
                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.t3;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(t3));
                                error
                            })
                        }

                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(Debug, Default, Clone, PartialEq)]
            pub struct Tt3 {
                pub a: ::std::option::Option<i32>,

                pub m: ::std::collections::HashMap<i32, T2>,
            }
            impl ::pilota::prost::Message for Tt3 {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.a.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::int32::encoded_len(1, value)
                    }) + ::pilota::prost::encoding::hash_map::encoded_len(
                        ::pilota::prost::encoding::int32::encoded_len,
                        ::pilota::prost::encoding::message::encoded_len,
                        2,
                        &self.m,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.a.as_ref() {
                        ::pilota::prost::encoding::int32::encode(1, _pilota_inner_value, buf);
                    }
                    ::pilota::prost::encoding::hash_map::encode(
                        ::pilota::prost::encoding::int32::encode,
                        ::pilota::prost::encoding::int32::encoded_len,
                        ::pilota::prost::encoding::message::encode,
                        ::pilota::prost::encoding::message::encoded_len,
                        2,
                        &self.m,
                        buf,
                    )
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
                    const STRUCT_NAME: &'static str = stringify!(Tt3);
                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.a;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(a));
                                error
                            })
                        }

                        2 => {
                            let mut _inner_pilota_value = &mut self.m;
                            ::pilota::prost::encoding::hash_map::merge(
                                ::pilota::prost::encoding::int32::merge,
                                ::pilota::prost::encoding::message::merge,
                                &mut _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(m));
                                error
                            })
                        }

                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
        }
    }
}
