pub mod message {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};
    pub mod example {
        use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};

        static FILE_DESCRIPTOR_BYTES_MESSAGE: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\n\rmessage.proto\x12\x07example\".\n\x06Person\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12\x10\n\x03age\x18\x02 \x01(\x05R\x03age\"L\n\x07Company\x12\x12\n\x04name\x18\x01 \x01(\tR\x04name\x12-\n\temployees\x18\x02 \x03(\x0b2\x0f.example.PersonR\temployees\"\x06\n\x04Self*/\n\x06Status\x12\x0b\n\x07UNKNOWN\x10\0\x12\n\n\x06ACTIVE\x10\x01\x12\x0c\n\x08INACTIVE\x10\x02b\x06proto3");
        static FILE_DESCRIPTOR_PROTO_MESSAGE: ::std::sync::LazyLock<
            ::pilota::pb::descriptor::FileDescriptorProto,
        > = ::std::sync::LazyLock::new(|| {
            let data: &[u8] = FILE_DESCRIPTOR_BYTES_MESSAGE.as_ref();
            ::pilota::pb::PbMessage::parse_from_bytes(data)
                .expect("Failed to decode file descriptor")
        });
        pub fn file_descriptor_proto_message()
        -> &'static ::pilota::pb::descriptor::FileDescriptorProto {
            &*FILE_DESCRIPTOR_PROTO_MESSAGE
        }

        static FILE_DESCRIPTOR_MESSAGE: ::std::sync::LazyLock<
            ::pilota::pb::reflect::FileDescriptor,
        > = ::std::sync::LazyLock::new(|| {
            let mut deps = ::std::vec::Vec::new();

            ::pilota::pb::reflect::FileDescriptor::new_dynamic(
                file_descriptor_proto_message().clone(),
                &deps,
            )
            .expect("Failed to build dynamic FileDescriptor")
        });

        pub fn file_descriptor_message() -> &'static ::pilota::pb::reflect::FileDescriptor {
            &*FILE_DESCRIPTOR_MESSAGE
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Person {
            pub name: ::pilota::FastStr,

            pub age: i32,
        }
        impl MessageDescriptorGetter for Person {
            fn get_descriptor_proto(&self) -> Option<&::pilota::pb::descriptor::DescriptorProto> {
                let file_descriptor = file_descriptor_proto_message();
                file_descriptor.get_message_descriptor_proto("Person")
            }
        }

        impl ::pilota::pb::Message for Person {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(1, &self.name)
                    + ::pilota::pb::encoding::int32::encoded_len(2, &self.age)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.name, buf);
                ::pilota::pb::encoding::int32::encode(2, &self.age, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(Person);

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
                    2 => {
                        let mut _inner_pilota_value = &mut self.age;
                        ::pilota::pb::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(age));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Company {
            pub name: ::pilota::FastStr,

            pub employees: ::std::vec::Vec<Person>,
        }
        impl MessageDescriptorGetter for Company {
            fn get_descriptor_proto(&self) -> Option<&::pilota::pb::descriptor::DescriptorProto> {
                let file_descriptor = file_descriptor_proto_message();
                file_descriptor.get_message_descriptor_proto("Company")
            }
        }

        impl ::pilota::pb::Message for Company {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(1, &self.name)
                    + ::pilota::pb::encoding::message::encoded_len_repeated(2, &self.employees)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.name, buf);
                for msg in &self.employees {
                    ::pilota::pb::encoding::message::encode(2, msg, buf);
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
                const STRUCT_NAME: &'static str = stringify!(Company);

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
                    2 => {
                        let mut _inner_pilota_value = &mut self.employees;
                        ::pilota::pb::encoding::message::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(employees));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Self_ {}
        impl MessageDescriptorGetter for Self_ {
            fn get_descriptor_proto(&self) -> Option<&::pilota::pb::descriptor::DescriptorProto> {
                let file_descriptor = file_descriptor_proto_message();
                file_descriptor.get_message_descriptor_proto("Self")
            }
        }

        impl ::pilota::pb::Message for Self_ {
            #[inline]
            fn encoded_len(&self) -> usize {
                0
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {}

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                match tag {
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Status(i32);

        impl Status {
            pub const UNKNOWN: Self = Self(0);
            pub const ACTIVE: Self = Self(1);
            pub const INACTIVE: Self = Self(2);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("UNKNOWN"),
                    Self(1) => ::std::string::String::from("ACTIVE"),
                    Self(2) => ::std::string::String::from("INACTIVE"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => Some(Self::UNKNOWN),
                    1 => Some(Self::ACTIVE),
                    2 => Some(Self::INACTIVE),
                    _ => None,
                }
            }
        }

        impl ::pilota::pb::EnumMessage for Status {
            fn inner(&self) -> i32 {
                self.inner()
            }

            fn to_string(&self) -> ::std::string::String {
                self.to_string()
            }

            fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                Status::try_from_i32(value)
            }
        }

        impl ::std::convert::From<i32> for Status {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Status> for i32 {
            fn from(value: Status) -> i32 {
                value.0
            }
        }

        impl EnumDescriptorGetter for Status {
            fn get_descriptor_proto(
                &self,
            ) -> Option<&::pilota::pb::descriptor::EnumDescriptorProto> {
                let file_descriptor = file_descriptor_proto_message();
                file_descriptor.get_enum_descriptor_proto("Status")
            }
        }
    }
}
