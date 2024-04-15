pub mod nested_message {
    #![allow(warnings, clippy::all)]
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct Tt1 {
        pub t2: tt1::T2,

        pub t3: tt1::Label,

        pub t4: tt1::t2::Tt3,
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
            ::pilota::prost::encoding::message::encode(1, (&self.t2), buf);
            ::pilota::prost::encoding::int32::encode(2, &self.t3, buf);
            ::pilota::prost::encoding::message::encode(4, (&self.t4), buf);
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

    pub mod tt1 {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Label(i32);

        impl Label {
            pub const LABEL_OPTIONAL: Self = Self(1);
            pub const LABEL_REQUIRED: Self = Self(2);
            pub const LABEL_REPEATED: Self = Self(3);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn as_str(&self) -> &'static str {
                match self {
                    Self(1) => stringify!(LABEL_OPTIONAL),
                    Self(2) => stringify!(LABEL_REQUIRED),
                    Self(3) => stringify!(LABEL_REPEATED),
                    _ => panic!(
                        "{} unknown fields val {}",
                        std::any::type_name::<Self>(),
                        self.0
                    ),
                }
            }
        }

        impl ::std::convert::From<i32> for Label {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Label> for i32 {
            fn from(value: Label) -> i32 {
                value.0
            }
        }

        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct T2 {
            pub t3: t2::Tt3,
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
                ::pilota::prost::encoding::message::encode(1, (&self.t3), buf);
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

        pub mod t2 {
            #[derive(Debug, Default, Clone, PartialEq)]
            pub struct Tt3 {
                pub a: ::std::option::Option<i32>,

                pub m: ::pilota::AHashMap<i32, super::T2>,
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
                    };
                    ::pilota::prost::encoding::hash_map::encode(
                        ::pilota::prost::encoding::int32::encode,
                        ::pilota::prost::encoding::int32::encoded_len,
                        ::pilota::prost::encoding::message::encode,
                        ::pilota::prost::encoding::message::encoded_len,
                        2,
                        &self.m,
                        buf,
                    );
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
