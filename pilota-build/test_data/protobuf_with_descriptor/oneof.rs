pub mod oneof {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};
    pub mod example {
        use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};

        static FILE_DESCRIPTOR_BYTES_ONEOF: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\n\x0boneof.proto\x12\x07example\"\\\n\x0bUserContact\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12\x16\n\x05email\x18\x02 \x01(\tH\0R\x05email\x12\x16\n\x05phone\x18\x03 \x01(\tH\0R\x05phoneB\t\n\x07contactb\x06proto3");
        static FILE_DESCRIPTOR_PROTO_ONEOF: ::std::sync::LazyLock<
            ::pilota::pb::descriptor::FileDescriptorProto,
        > = ::std::sync::LazyLock::new(|| {
            let data: &[u8] = FILE_DESCRIPTOR_BYTES_ONEOF.as_ref();
            ::pilota::pb::PbMessage::parse_from_bytes(data)
                .expect("Failed to decode file descriptor")
        });
        pub fn file_descriptor_proto_oneof()
        -> &'static ::pilota::pb::descriptor::FileDescriptorProto {
            &*FILE_DESCRIPTOR_PROTO_ONEOF
        }

        static FILE_DESCRIPTOR_ONEOF: ::std::sync::LazyLock<::pilota::pb::reflect::FileDescriptor> =
            ::std::sync::LazyLock::new(|| {
                let mut deps = ::std::vec::Vec::new();

                ::pilota::pb::reflect::FileDescriptor::new_dynamic(
                    file_descriptor_proto_oneof().clone(),
                    &deps,
                )
                .expect("Failed to build dynamic FileDescriptor")
            });

        pub fn file_descriptor_oneof() -> &'static ::pilota::pb::reflect::FileDescriptor {
            &*FILE_DESCRIPTOR_ONEOF
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct UserContact {
            pub name: ::pilota::FastStr,

            pub contact: ::std::option::Option<user_contact::Contact>,
        }
        impl UserContact {
            fn get_descriptor_proto() -> Option<&'static ::pilota::pb::descriptor::DescriptorProto>
            {
                let file_descriptor = file_descriptor_proto_oneof();
                file_descriptor.get_message_descriptor_proto("UserContact")
            }
        }

        impl ::pilota::pb::Message for UserContact {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(1, &self.name)
                    + self.contact.as_ref().map_or(0, |msg| msg.encoded_len())
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.name, buf);
                if let Some(_pilota_inner_value) = self.contact.as_ref() {
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
                    2 | 3 => {
                        let mut _inner_pilota_value = &mut self.contact;
                        user_contact::Contact::merge(
                            &mut _inner_pilota_value,
                            tag,
                            wire_type,
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

        pub mod user_contact {
            use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};

            impl ::std::default::Default for Contact {
                fn default() -> Self {
                    Contact::Email(::std::default::Default::default())
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
            pub enum Contact {
                Email(::pilota::FastStr),

                Phone(::pilota::FastStr),
            }

            impl Contact {
                fn get_descriptor_proto()
                -> Option<&'static ::pilota::pb::descriptor::OneofDescriptorProto> {
                    let message_descriptor = super::UserContact::get_descriptor_proto()?;
                    message_descriptor.get_oneof_descriptor_proto("contact")
                }
            }

            impl Contact {
                pub fn encode(&self, buf: &mut ::pilota::LinkedBytes) {
                    match self {
                        Contact::Email(value) => {
                            ::pilota::pb::encoding::faststr::encode(2, &*value, buf);
                        }
                        Contact::Phone(value) => {
                            ::pilota::pb::encoding::faststr::encode(3, &*value, buf);
                        }
                    }
                }

                #[inline]
                pub fn encoded_len(&self) -> usize {
                    match self {
                        Contact::Email(value) => {
                            ::pilota::pb::encoding::faststr::encoded_len(2, &*value)
                        }
                        Contact::Phone(value) => {
                            ::pilota::pb::encoding::faststr::encoded_len(3, &*value)
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
                            ::core::option::Option::Some(Contact::Email(value)) => {
                                ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                                *field = ::core::option::Option::Some(Contact::Email(owned_value));
                            }
                        },
                        3 => match field {
                            ::core::option::Option::Some(Contact::Phone(value)) => {
                                ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                            }
                            _ => {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::pilota::pb::encoding::faststr::merge(wire_type, value, buf, ctx)?;
                                *field = ::core::option::Option::Some(Contact::Phone(owned_value));
                            }
                        },
                        _ => {
                            unreachable!(concat!("invalid ", stringify!(Contact), " tag: {}"), tag)
                        }
                    };
                    ::core::result::Result::Ok(())
                }
            }
        }
    }
}
