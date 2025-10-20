pub mod nested {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};
    pub mod example {
        use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};

        static FILE_DESCRIPTOR_BYTES_NESTED: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\n\x0cnested.proto\x12\x07example\"\x91\x01\n\x0bUserContact\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x1aI\n\x07Contact\x12\x16\n\x05email\x18\x02 \x01(\tH\0R\x05email\x12\x16\n\x05phone\x18\x03 \x01(\tH\0R\x05phoneB\x0e\n\x0ccontact_info\"#\n\x0bContactType\x12\t\n\x05EMAIL\x10\0\x12\t\n\x05PHONE\x10\x01b\x06proto3");
        static FILE_DESCRIPTOR_PROTO_NESTED: ::std::sync::LazyLock<
            ::pilota::pb::descriptor::FileDescriptorProto,
        > = ::std::sync::LazyLock::new(|| {
            let data: &[u8] = FILE_DESCRIPTOR_BYTES_NESTED.as_ref();
            ::pilota::pb::PbMessage::parse_from_bytes(data)
                .expect("Failed to decode file descriptor")
        });
        pub fn file_descriptor_proto_nested()
        -> &'static ::pilota::pb::descriptor::FileDescriptorProto {
            &*FILE_DESCRIPTOR_PROTO_NESTED
        }

        static FILE_DESCRIPTOR_NESTED: ::std::sync::LazyLock<
            ::pilota::pb::reflect::FileDescriptor,
        > = ::std::sync::LazyLock::new(|| {
            let mut deps = ::std::vec::Vec::new();

            ::pilota::pb::reflect::FileDescriptor::new_dynamic(
                file_descriptor_proto_nested().clone(),
                &deps,
            )
            .expect("Failed to build dynamic FileDescriptor")
        });

        pub fn file_descriptor_nested() -> &'static ::pilota::pb::reflect::FileDescriptor {
            &*FILE_DESCRIPTOR_NESTED
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct UserContact {
            pub name: ::pilota::FastStr,
        }
        impl UserContact {
            fn get_descriptor_proto() -> Option<&'static ::pilota::pb::descriptor::DescriptorProto>
            {
                let file_descriptor = file_descriptor_proto_nested();
                file_descriptor.get_message_descriptor_proto("UserContact")
            }
        }

        impl ::pilota::pb::Message for UserContact {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.name)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.name, buf);
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
                const STRUCT_NAME: &'static str = stringify!(UserContact);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.name;
                        ::pilota::pb::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(name));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }

        pub mod user_contact {
            use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
            #[repr(transparent)]
            pub struct ContactType(i32);

            impl ContactType {
                pub const EMAIL: Self = Self(0);
                pub const PHONE: Self = Self(1);

                pub fn inner(&self) -> i32 {
                    self.0
                }

                pub fn to_string(&self) -> ::std::string::String {
                    match self {
                        Self(0) => ::std::string::String::from("EMAIL"),
                        Self(1) => ::std::string::String::from("PHONE"),
                        Self(val) => val.to_string(),
                    }
                }

                pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                    match value {
                        0 => Some(Self::EMAIL),
                        1 => Some(Self::PHONE),
                        _ => None,
                    }
                }
            }

            impl ::pilota::pb::EnumMessage for ContactType {
                fn inner(&self) -> i32 {
                    self.inner()
                }

                fn to_string(&self) -> ::std::string::String {
                    self.to_string()
                }

                fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                    ContactType::try_from_i32(value)
                }
            }

            impl ::std::convert::From<i32> for ContactType {
                fn from(value: i32) -> Self {
                    Self(value)
                }
            }

            impl ::std::convert::From<ContactType> for i32 {
                fn from(value: ContactType) -> i32 {
                    value.0
                }
            }

            impl ContactType {
                fn get_descriptor_proto()
                -> Option<&'static ::pilota::pb::descriptor::EnumDescriptorProto> {
                    let message_descriptor = super::UserContact::get_descriptor_proto()?;
                    message_descriptor.get_enum_descriptor_proto("ContactType")
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct Contact {
                pub contact_info: ::std::option::Option<contact::ContactInfo>,
            }
            impl Contact {
                fn get_descriptor_proto()
                -> Option<&'static ::pilota::pb::descriptor::DescriptorProto> {
                    let message_descriptor = super::UserContact::get_descriptor_proto()?;
                    message_descriptor.get_message_descriptor_proto("Contact")
                }
            }

            impl ::pilota::pb::Message for Contact {
                #[inline]
                fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                    0 + self
                        .contact_info
                        .as_ref()
                        .map_or(0, |msg| msg.encoded_len(ctx))
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.contact_info.as_ref() {
                        _pilota_inner_value.encode(buf);
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
                    const STRUCT_NAME: &'static str = stringify!(Contact);

                    match tag {
                        2 | 3 => {
                            let mut _inner_pilota_value = &mut self.contact_info;
                            contact::ContactInfo::merge(
                                &mut _inner_pilota_value,
                                tag,
                                wire_type,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(contact_info));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }

            pub mod contact {
                use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};

                impl ::std::default::Default for ContactInfo {
                    fn default() -> Self {
                        ContactInfo::Email(::std::default::Default::default())
                    }
                }
                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
                pub enum ContactInfo {
                    Email(::pilota::FastStr),

                    Phone(::pilota::FastStr),
                }

                impl ContactInfo {
                    fn get_descriptor_proto()
                    -> Option<&'static ::pilota::pb::descriptor::OneofDescriptorProto>
                    {
                        let message_descriptor = super::Contact::get_descriptor_proto()?;
                        message_descriptor.get_oneof_descriptor_proto("contact_info")
                    }
                }

                impl ContactInfo {
                    pub fn encode(&self, buf: &mut ::pilota::LinkedBytes) {
                        match self {
                            ContactInfo::Email(value) => {
                                ::pilota::pb::encoding::faststr::encode(2, &*value, buf);
                            }
                            ContactInfo::Phone(value) => {
                                ::pilota::pb::encoding::faststr::encode(3, &*value, buf);
                            }
                        }
                    }

                    #[inline]
                    pub fn encoded_len(
                        &self,
                        ctx: &mut ::pilota::pb::EncodeLengthContext,
                    ) -> usize {
                        match self {
                            ContactInfo::Email(value) => {
                                ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &*value)
                            }
                            ContactInfo::Phone(value) => {
                                ::pilota::pb::encoding::faststr::encoded_len(ctx, 3, &*value)
                            }
                        }
                    }

                    #[inline]
                    pub fn merge(
                        field: &mut ::core::option::Option<Self>,
                        tag: u32,
                        wire_type: ::pilota::pb::encoding::WireType,
                        buf: &mut ::pilota::Bytes,
                        ctx: &mut ::pilota::pb::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                        match tag {
                            2 => match field {
                                ::core::option::Option::Some(ContactInfo::Email(value)) => {
                                    ::pilota::pb::encoding::faststr::merge(
                                        wire_type, value, buf, ctx,
                                    )?;
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::pilota::pb::encoding::faststr::merge(
                                        wire_type, value, buf, ctx,
                                    )?;
                                    *field = ::core::option::Option::Some(ContactInfo::Email(
                                        owned_value,
                                    ));
                                }
                            },
                            3 => match field {
                                ::core::option::Option::Some(ContactInfo::Phone(value)) => {
                                    ::pilota::pb::encoding::faststr::merge(
                                        wire_type, value, buf, ctx,
                                    )?;
                                }
                                _ => {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::pilota::pb::encoding::faststr::merge(
                                        wire_type, value, buf, ctx,
                                    )?;
                                    *field = ::core::option::Option::Some(ContactInfo::Phone(
                                        owned_value,
                                    ));
                                }
                            },
                            _ => unreachable!(
                                concat!("invalid ", stringify!(ContactInfo), " tag: {}"),
                                tag
                            ),
                        };
                        ::core::result::Result::Ok(())
                    }
                }
            }
        }
    }
}
