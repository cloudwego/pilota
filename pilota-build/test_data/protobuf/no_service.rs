pub mod no_service {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct Person {
        pub name: ::pilota::FastStr,

        pub age: i32,

        pub contact: ::std::option::Option<person::Contact>,
    }
    impl ::pilota::pb::Message for Person {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.name)
                + ::pilota::pb::encoding::int32::encoded_len(ctx, 2, &self.age)
                + self.contact.as_ref().map_or(0, |msg| {
                    ::pilota::pb::encoding::message::encoded_len(ctx, 3, msg)
                })
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode(1, &self.name, buf);
            ::pilota::pb::encoding::int32::encode(2, &self.age, buf);
            if let Some(_pilota_inner_value) = self.contact.as_ref() {
                ::pilota::pb::encoding::message::encode(3, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(Person);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.name;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(name));
                            error
                        })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.age;
                    ::pilota::pb::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(age));
                            error
                        })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.contact;
                    ::pilota::pb::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(contact));
                        error
                    })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct Address {
        pub street: ::pilota::FastStr,

        pub city: ::pilota::FastStr,
    }
    impl ::pilota::pb::Message for Address {
        #[inline]
        fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.street)
                + ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &self.city)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode(1, &self.street, buf);
            ::pilota::pb::encoding::faststr::encode(2, &self.city, buf);
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
            const STRUCT_NAME: &'static str = stringify!(Address);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.street;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(street));
                            error
                        })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.city;
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(city));
                            error
                        })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }

    pub mod person {
        use ::pilota::{Buf as _, BufMut as _};
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Contact {
            pub email: ::pilota::FastStr,

            pub label: Label,
        }
        impl ::pilota::pb::Message for Contact {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.email)
                    + ::pilota::pb::encoding::int32::encoded_len(ctx, 2, &self.label)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.email, buf);
                ::pilota::pb::encoding::int32::encode(2, &self.label, buf);
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
                const STRUCT_NAME: &'static str = stringify!(Contact);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.email;
                        ::pilota::pb::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(email));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.label;
                        ::pilota::pb::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(label));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Label(i32);

        impl Label {
            pub const LABEL_UNKNOWN: Self = Self(0);
            pub const LABEL_HOME: Self = Self(1);
            pub const LABEL_WORK: Self = Self(2);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("LABEL_UNKNOWN"),
                    Self(1) => ::std::string::String::from("LABEL_HOME"),
                    Self(2) => ::std::string::String::from("LABEL_WORK"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => Some(Self::LABEL_UNKNOWN),
                    1 => Some(Self::LABEL_HOME),
                    2 => Some(Self::LABEL_WORK),
                    _ => None,
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
    }
}
