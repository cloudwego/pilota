pub mod serde_pb {
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
    )]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq)]
    pub struct A {
        #[serde(rename = "AA")]
        pub a: ::pilota::FastStr,

        pub b: i32,
    }
    impl ::pilota::pb::Message for A {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::pb::encoding::faststr::encoded_len(1, &self.a)
                + ::pilota::pb::encoding::int32::encoded_len(2, &self.b)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::faststr::encode(1, &self.a, buf);
            ::pilota::pb::encoding::int32::encode(2, &self.b, buf);
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
                    ::pilota::pb::encoding::faststr::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(a));
                            error
                        })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.b;
                    ::pilota::pb::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(b));
                            error
                        })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
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
    )]
    #[serde(untagged)]
    #[serde(transparent)]
    #[derive(Clone, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct C(i32);

    impl C {
        pub const UNSPECIFIED: Self = Self(0);
        pub const D: Self = Self(1);
        pub const E: Self = Self(2);

        pub fn inner(&self) -> i32 {
            self.0
        }

        pub fn to_string(&self) -> ::std::string::String {
            match self {
                Self(0) => ::std::string::String::from("UNSPECIFIED"),
                Self(1) => ::std::string::String::from("D"),
                Self(2) => ::std::string::String::from("E"),
                Self(val) => val.to_string(),
            }
        }

        pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
            match value {
                0 => Some(Self::UNSPECIFIED),
                1 => Some(Self::D),
                2 => Some(Self::E),
                _ => None,
            }
        }
    }

    impl ::std::convert::From<i32> for C {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl ::std::convert::From<C> for i32 {
        fn from(value: C) -> i32 {
            value.0
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
    )]
    #[serde(rename = "BB")]
    #[derive(Clone, PartialEq)]
    pub struct B {
        pub value: i32,
    }
    impl ::pilota::pb::Message for B {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::pb::encoding::int32::encoded_len(1, &self.value)
        }

        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
            ::pilota::pb::encoding::int32::encode(1, &self.value, buf);
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
                1 => {
                    let mut _inner_pilota_value = &mut self.value;
                    ::pilota::pb::encoding::int32::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(value));
                            error
                        })
                }
                _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }

    pub mod google {
        use ::pilota::{Buf as _, BufMut as _};

        pub mod protobuf {
            use ::pilota::{Buf as _, BufMut as _};
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct FileDescriptorSet {
                pub file: ::std::vec::Vec<FileDescriptorProto>,
            }
            impl ::pilota::pb::Message for FileDescriptorSet {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::pb::encoding::message::encoded_len_repeated(1, &self.file)
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    for msg in &self.file {
                        ::pilota::pb::encoding::message::encode(1, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(FileDescriptorSet);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.file;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(file));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct FieldDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub number: ::std::option::Option<i32>,

                pub label: ::std::option::Option<field_descriptor_proto::Label>,

                pub r#type: ::std::option::Option<field_descriptor_proto::Type>,

                pub type_name: ::std::option::Option<::pilota::FastStr>,

                pub extendee: ::std::option::Option<::pilota::FastStr>,

                pub default_value: ::std::option::Option<::pilota::FastStr>,

                pub oneof_index: ::std::option::Option<i32>,

                pub json_name: ::std::option::Option<::pilota::FastStr>,

                pub options: ::std::option::Option<FieldOptions>,

                pub proto3_optional: ::std::option::Option<bool>,
            }
            impl ::pilota::pb::Message for FieldDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(1, value)
                    }) + self.number.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::int32::encoded_len(3, value)
                    }) + self.label.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::int32::encoded_len(4, value)
                    }) + self.r#type.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::int32::encoded_len(5, value)
                    }) + self.type_name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(6, value)
                    }) + self.extendee.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(2, value)
                    }) + self.default_value.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(7, value)
                    }) + self.oneof_index.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::int32::encoded_len(9, value)
                    }) + self.json_name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(10, value)
                    }) + self.options.as_ref().map_or(0, |msg| {
                        ::pilota::pb::encoding::message::encoded_len(8, msg)
                    }) + self.proto3_optional.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(17, value)
                    })
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.number.as_ref() {
                        ::pilota::pb::encoding::int32::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.label.as_ref() {
                        ::pilota::pb::encoding::int32::encode(4, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.r#type.as_ref() {
                        ::pilota::pb::encoding::int32::encode(5, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.type_name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(6, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.extendee.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.default_value.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(7, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.oneof_index.as_ref() {
                        ::pilota::pb::encoding::int32::encode(9, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.json_name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(10, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::pb::encoding::message::encode(8, _pilota_inner_value, buf);
                    }
                    if let Some(_pilota_inner_value) = self.proto3_optional.as_ref() {
                        ::pilota::pb::encoding::bool::encode(17, _pilota_inner_value, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(FieldDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.number;
                            ::pilota::pb::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(number));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.label;
                            ::pilota::pb::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(label));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.r#type;
                            ::pilota::pb::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(r#type));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.type_name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(type_name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.extendee;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(extendee));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.default_value;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(default_value));
                                error
                            })
                        }
                        9 => {
                            let mut _inner_pilota_value = &mut self.oneof_index;
                            ::pilota::pb::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(oneof_index));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.json_name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(json_name));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::pb::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        17 => {
                            let mut _inner_pilota_value = &mut self.proto3_optional;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(proto3_optional));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct MethodDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub input_type: ::std::option::Option<::pilota::FastStr>,

                pub output_type: ::std::option::Option<::pilota::FastStr>,

                pub options: ::std::option::Option<MethodOptions>,

                pub client_streaming: ::std::option::Option<bool>,

                pub server_streaming: ::std::option::Option<bool>,
            }
            impl ::pilota::pb::Message for MethodDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(1, value)
                    }) + self.input_type.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(2, value)
                    }) + self.output_type.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(3, value)
                    }) + self.options.as_ref().map_or(0, |msg| {
                        ::pilota::pb::encoding::message::encoded_len(4, msg)
                    }) + self.client_streaming.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(5, value)
                    }) + self.server_streaming.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(6, value)
                    })
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.input_type.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.output_type.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::pb::encoding::message::encode(4, _pilota_inner_value, buf);
                    }
                    if let Some(_pilota_inner_value) = self.client_streaming.as_ref() {
                        ::pilota::pb::encoding::bool::encode(5, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.server_streaming.as_ref() {
                        ::pilota::pb::encoding::bool::encode(6, _pilota_inner_value, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(MethodDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.input_type;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(input_type));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.output_type;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(output_type));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::pb::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.client_streaming;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(client_streaming));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.server_streaming;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(server_streaming));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
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
            pub struct SourceCodeInfo {
                pub location: ::std::vec::Vec<source_code_info::Location>,
            }
            impl ::pilota::pb::Message for SourceCodeInfo {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::pb::encoding::message::encoded_len_repeated(1, &self.location)
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    for msg in &self.location {
                        ::pilota::pb::encoding::message::encode(1, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(SourceCodeInfo);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.location;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(location));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct DescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub field: ::std::vec::Vec<FieldDescriptorProto>,

                pub extension: ::std::vec::Vec<FieldDescriptorProto>,

                pub nested_type: ::std::vec::Vec<DescriptorProto>,

                pub enum_type: ::std::vec::Vec<EnumDescriptorProto>,

                pub extension_range: ::std::vec::Vec<descriptor_proto::ExtensionRange>,

                pub oneof_decl: ::std::vec::Vec<OneofDescriptorProto>,

                pub options: ::std::option::Option<MessageOptions>,

                pub reserved_range: ::std::vec::Vec<descriptor_proto::ReservedRange>,

                pub reserved_name: ::std::vec::Vec<::pilota::FastStr>,
            }
            impl ::pilota::pb::Message for DescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(1, value)
                    }) + ::pilota::pb::encoding::message::encoded_len_repeated(2, &self.field)
                        + ::pilota::pb::encoding::message::encoded_len_repeated(6, &self.extension)
                        + ::pilota::pb::encoding::message::encoded_len_repeated(
                            3,
                            &self.nested_type,
                        )
                        + ::pilota::pb::encoding::message::encoded_len_repeated(4, &self.enum_type)
                        + ::pilota::pb::encoding::message::encoded_len_repeated(
                            5,
                            &self.extension_range,
                        )
                        + ::pilota::pb::encoding::message::encoded_len_repeated(8, &self.oneof_decl)
                        + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::pb::encoding::message::encoded_len(7, msg)
                        })
                        + ::pilota::pb::encoding::message::encoded_len_repeated(
                            9,
                            &self.reserved_range,
                        )
                        + ::pilota::pb::encoding::faststr::encoded_len_repeated(
                            10,
                            &self.reserved_name,
                        )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    for msg in &self.field {
                        ::pilota::pb::encoding::message::encode(2, msg, buf);
                    }
                    for msg in &self.extension {
                        ::pilota::pb::encoding::message::encode(6, msg, buf);
                    }
                    for msg in &self.nested_type {
                        ::pilota::pb::encoding::message::encode(3, msg, buf);
                    }
                    for msg in &self.enum_type {
                        ::pilota::pb::encoding::message::encode(4, msg, buf);
                    }
                    for msg in &self.extension_range {
                        ::pilota::pb::encoding::message::encode(5, msg, buf);
                    }
                    for msg in &self.oneof_decl {
                        ::pilota::pb::encoding::message::encode(8, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::pb::encoding::message::encode(7, _pilota_inner_value, buf);
                    }
                    for msg in &self.reserved_range {
                        ::pilota::pb::encoding::message::encode(9, msg, buf);
                    }
                    ::pilota::pb::encoding::faststr::encode_repeated(10, &self.reserved_name, buf);
                }

                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::pilota::pb::encoding::WireType,
                    buf: &mut ::pilota::Bytes,
                    ctx: &mut ::pilota::pb::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                    const STRUCT_NAME: &'static str = stringify!(DescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.field;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(field));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.extension;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(extension));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.nested_type;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(nested_type));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.enum_type;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(enum_type));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.extension_range;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(extension_range));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.oneof_decl;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(oneof_decl));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::pb::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        9 => {
                            let mut _inner_pilota_value = &mut self.reserved_range;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(reserved_range));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.reserved_name;
                            ::pilota::pb::encoding::faststr::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(reserved_name));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct EnumOptions {
                pub allow_alias: ::std::option::Option<bool>,

                pub deprecated: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::pb::Message for EnumOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.allow_alias.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(2, value)
                    }) + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(3, value)
                    }) + ::pilota::pb::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.allow_alias.as_ref() {
                        ::pilota::pb::encoding::bool::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::pb::encoding::bool::encode(3, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::pb::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(EnumOptions);

                    match tag {
                        2 => {
                            let mut _inner_pilota_value = &mut self.allow_alias;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(allow_alias));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct MessageOptions {
                pub message_set_wire_format: ::std::option::Option<bool>,

                pub no_standard_descriptor_accessor: ::std::option::Option<bool>,

                pub deprecated: ::std::option::Option<bool>,

                pub map_entry: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::pb::Message for MessageOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.message_set_wire_format.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(1, value)
                    }) + self
                        .no_standard_descriptor_accessor
                        .as_ref()
                        .map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(2, value)
                        })
                        + self.deprecated.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(3, value)
                        })
                        + self.map_entry.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(7, value)
                        })
                        + ::pilota::pb::encoding::message::encoded_len_repeated(
                            999,
                            &self.uninterpreted_option,
                        )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.message_set_wire_format.as_ref() {
                        ::pilota::pb::encoding::bool::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.no_standard_descriptor_accessor.as_ref()
                    {
                        ::pilota::pb::encoding::bool::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::pb::encoding::bool::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.map_entry.as_ref() {
                        ::pilota::pb::encoding::bool::encode(7, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::pb::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(MessageOptions);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.message_set_wire_format;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(message_set_wire_format));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.no_standard_descriptor_accessor;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error
                                    .push(STRUCT_NAME, stringify!(no_standard_descriptor_accessor));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.map_entry;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(map_entry));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct EnumDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub value: ::std::vec::Vec<EnumValueDescriptorProto>,

                pub options: ::std::option::Option<EnumOptions>,

                pub reserved_range: ::std::vec::Vec<enum_descriptor_proto::EnumReservedRange>,

                pub reserved_name: ::std::vec::Vec<::pilota::FastStr>,
            }
            impl ::pilota::pb::Message for EnumDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(1, value)
                    }) + ::pilota::pb::encoding::message::encoded_len_repeated(2, &self.value)
                        + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::pb::encoding::message::encoded_len(3, msg)
                        })
                        + ::pilota::pb::encoding::message::encoded_len_repeated(
                            4,
                            &self.reserved_range,
                        )
                        + ::pilota::pb::encoding::faststr::encoded_len_repeated(
                            5,
                            &self.reserved_name,
                        )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    for msg in &self.value {
                        ::pilota::pb::encoding::message::encode(2, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::pb::encoding::message::encode(3, _pilota_inner_value, buf);
                    }
                    for msg in &self.reserved_range {
                        ::pilota::pb::encoding::message::encode(4, msg, buf);
                    }
                    ::pilota::pb::encoding::faststr::encode_repeated(5, &self.reserved_name, buf);
                }

                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::pilota::pb::encoding::WireType,
                    buf: &mut ::pilota::Bytes,
                    ctx: &mut ::pilota::pb::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                    const STRUCT_NAME: &'static str = stringify!(EnumDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.value;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(value));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::pb::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.reserved_range;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(reserved_range));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.reserved_name;
                            ::pilota::pb::encoding::faststr::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(reserved_name));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct ServiceOptions {
                pub deprecated: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::pb::Message for ServiceOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(33, value)
                    }) + ::pilota::pb::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::pb::encoding::bool::encode(33, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::pb::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(ServiceOptions);

                    match tag {
                        33 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct UninterpretedOption {
                pub name: ::std::vec::Vec<uninterpreted_option::NamePart>,

                pub identifier_value: ::std::option::Option<::pilota::FastStr>,

                pub positive_int_value: ::std::option::Option<u64>,

                pub negative_int_value: ::std::option::Option<i64>,

                pub double_value: ::std::option::Option<f64>,

                pub string_value: ::std::option::Option<::pilota::Bytes>,

                pub aggregate_value: ::std::option::Option<::pilota::FastStr>,
            }
            impl ::pilota::pb::Message for UninterpretedOption {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::pb::encoding::message::encoded_len_repeated(2, &self.name)
                        + self.identifier_value.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(3, value)
                        })
                        + self.positive_int_value.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::uint64::encoded_len(4, value)
                        })
                        + self.negative_int_value.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::int64::encoded_len(5, value)
                        })
                        + self.double_value.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::double::encoded_len(6, value)
                        })
                        + self.string_value.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bytes::encoded_len(7, value)
                        })
                        + self.aggregate_value.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(8, value)
                        })
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    for msg in &self.name {
                        ::pilota::pb::encoding::message::encode(2, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.identifier_value.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.positive_int_value.as_ref() {
                        ::pilota::pb::encoding::uint64::encode(4, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.negative_int_value.as_ref() {
                        ::pilota::pb::encoding::int64::encode(5, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.double_value.as_ref() {
                        ::pilota::pb::encoding::double::encode(6, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.string_value.as_ref() {
                        ::pilota::pb::encoding::bytes::encode(7, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.aggregate_value.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(8, _pilota_inner_value, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(UninterpretedOption);

                    match tag {
                        2 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::pb::encoding::message::merge_repeated(
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
                        3 => {
                            let mut _inner_pilota_value = &mut self.identifier_value;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(identifier_value));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.positive_int_value;
                            ::pilota::pb::encoding::uint64::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(positive_int_value));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.negative_int_value;
                            ::pilota::pb::encoding::int64::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(negative_int_value));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.double_value;
                            ::pilota::pb::encoding::double::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(double_value));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.string_value;
                            ::pilota::pb::encoding::bytes::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(string_value));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.aggregate_value;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(aggregate_value));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct ExtensionRangeOptions {
                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::pb::Message for ExtensionRangeOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::pb::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    for msg in &self.uninterpreted_option {
                        ::pilota::pb::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(ExtensionRangeOptions);

                    match tag {
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct ServiceDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub method: ::std::vec::Vec<MethodDescriptorProto>,

                pub options: ::std::option::Option<ServiceOptions>,
            }
            impl ::pilota::pb::Message for ServiceDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(1, value)
                    }) + ::pilota::pb::encoding::message::encoded_len_repeated(2, &self.method)
                        + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::pb::encoding::message::encoded_len(3, msg)
                        })
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    for msg in &self.method {
                        ::pilota::pb::encoding::message::encode(2, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
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
                ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                    const STRUCT_NAME: &'static str = stringify!(ServiceDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.method;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(method));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::pb::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct FileDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub package: ::std::option::Option<::pilota::FastStr>,

                pub dependency: ::std::vec::Vec<::pilota::FastStr>,

                pub public_dependency: ::std::vec::Vec<i32>,

                pub weak_dependency: ::std::vec::Vec<i32>,

                pub message_type: ::std::vec::Vec<DescriptorProto>,

                pub enum_type: ::std::vec::Vec<EnumDescriptorProto>,

                pub service: ::std::vec::Vec<ServiceDescriptorProto>,

                pub extension: ::std::vec::Vec<FieldDescriptorProto>,

                pub options: ::std::option::Option<FileOptions>,

                pub source_code_info: ::std::option::Option<SourceCodeInfo>,

                pub syntax: ::std::option::Option<::pilota::FastStr>,
            }
            impl ::pilota::pb::Message for FileDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(1, value)
                    }) + self.package.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(2, value)
                    }) + ::pilota::pb::encoding::faststr::encoded_len_repeated(3, &self.dependency)
                        + ::pilota::pb::encoding::int32::encoded_len_repeated(
                            10,
                            &self.public_dependency,
                        )
                        + ::pilota::pb::encoding::int32::encoded_len_repeated(
                            11,
                            &self.weak_dependency,
                        )
                        + ::pilota::pb::encoding::message::encoded_len_repeated(
                            4,
                            &self.message_type,
                        )
                        + ::pilota::pb::encoding::message::encoded_len_repeated(5, &self.enum_type)
                        + ::pilota::pb::encoding::message::encoded_len_repeated(6, &self.service)
                        + ::pilota::pb::encoding::message::encoded_len_repeated(7, &self.extension)
                        + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::pb::encoding::message::encoded_len(8, msg)
                        })
                        + self.source_code_info.as_ref().map_or(0, |msg| {
                            ::pilota::pb::encoding::message::encoded_len(9, msg)
                        })
                        + self.syntax.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(12, value)
                        })
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.package.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(2, _pilota_inner_value, buf);
                    };
                    ::pilota::pb::encoding::faststr::encode_repeated(3, &self.dependency, buf);
                    ::pilota::pb::encoding::int32::encode_repeated(
                        10,
                        &self.public_dependency,
                        buf,
                    );
                    ::pilota::pb::encoding::int32::encode_repeated(11, &self.weak_dependency, buf);
                    for msg in &self.message_type {
                        ::pilota::pb::encoding::message::encode(4, msg, buf);
                    }
                    for msg in &self.enum_type {
                        ::pilota::pb::encoding::message::encode(5, msg, buf);
                    }
                    for msg in &self.service {
                        ::pilota::pb::encoding::message::encode(6, msg, buf);
                    }
                    for msg in &self.extension {
                        ::pilota::pb::encoding::message::encode(7, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::pb::encoding::message::encode(8, _pilota_inner_value, buf);
                    }
                    if let Some(_pilota_inner_value) = self.source_code_info.as_ref() {
                        ::pilota::pb::encoding::message::encode(9, _pilota_inner_value, buf);
                    }
                    if let Some(_pilota_inner_value) = self.syntax.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(12, _pilota_inner_value, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(FileDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.package;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(package));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.dependency;
                            ::pilota::pb::encoding::faststr::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(dependency));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.public_dependency;
                            ::pilota::pb::encoding::int32::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(public_dependency));
                                error
                            })
                        }
                        11 => {
                            let mut _inner_pilota_value = &mut self.weak_dependency;
                            ::pilota::pb::encoding::int32::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(weak_dependency));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.message_type;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(message_type));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.enum_type;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(enum_type));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.service;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(service));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.extension;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(extension));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::pb::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        9 => {
                            let mut _inner_pilota_value = &mut self.source_code_info;
                            ::pilota::pb::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(source_code_info));
                                error
                            })
                        }
                        12 => {
                            let mut _inner_pilota_value = &mut self.syntax;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(syntax));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct OneofOptions {
                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::pb::Message for OneofOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::pb::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    for msg in &self.uninterpreted_option {
                        ::pilota::pb::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(OneofOptions);

                    match tag {
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct OneofDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub options: ::std::option::Option<OneofOptions>,
            }
            impl ::pilota::pb::Message for OneofDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(1, value)
                    }) + self.options.as_ref().map_or(0, |msg| {
                        ::pilota::pb::encoding::message::encoded_len(2, msg)
                    })
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
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
                    const STRUCT_NAME: &'static str = stringify!(OneofDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::pb::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct FileOptions {
                pub java_package: ::std::option::Option<::pilota::FastStr>,

                pub java_outer_classname: ::std::option::Option<::pilota::FastStr>,

                pub java_multiple_files: ::std::option::Option<bool>,

                #[deprecated]
                pub java_generate_equals_and_hash: ::std::option::Option<bool>,

                pub java_string_check_utf8: ::std::option::Option<bool>,

                pub optimize_for: ::std::option::Option<file_options::OptimizeMode>,

                pub go_package: ::std::option::Option<::pilota::FastStr>,

                pub cc_generic_services: ::std::option::Option<bool>,

                pub java_generic_services: ::std::option::Option<bool>,

                pub py_generic_services: ::std::option::Option<bool>,

                pub php_generic_services: ::std::option::Option<bool>,

                pub deprecated: ::std::option::Option<bool>,

                pub cc_enable_arenas: ::std::option::Option<bool>,

                pub objc_class_prefix: ::std::option::Option<::pilota::FastStr>,

                pub csharp_namespace: ::std::option::Option<::pilota::FastStr>,

                pub swift_prefix: ::std::option::Option<::pilota::FastStr>,

                pub php_class_prefix: ::std::option::Option<::pilota::FastStr>,

                pub php_namespace: ::std::option::Option<::pilota::FastStr>,

                pub php_metadata_namespace: ::std::option::Option<::pilota::FastStr>,

                pub ruby_package: ::std::option::Option<::pilota::FastStr>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::pb::Message for FileOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.java_package.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(1, value)
                    }) + self.java_outer_classname.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(8, value)
                    }) + self.java_multiple_files.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(10, value)
                    }) + self
                        .java_generate_equals_and_hash
                        .as_ref()
                        .map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(20, value)
                        })
                        + self.java_string_check_utf8.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(27, value)
                        })
                        + self.optimize_for.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::int32::encoded_len(9, value)
                        })
                        + self.go_package.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(11, value)
                        })
                        + self.cc_generic_services.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(16, value)
                        })
                        + self.java_generic_services.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(17, value)
                        })
                        + self.py_generic_services.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(18, value)
                        })
                        + self.php_generic_services.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(42, value)
                        })
                        + self.deprecated.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(23, value)
                        })
                        + self.cc_enable_arenas.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::bool::encoded_len(31, value)
                        })
                        + self.objc_class_prefix.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(36, value)
                        })
                        + self.csharp_namespace.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(37, value)
                        })
                        + self.swift_prefix.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(39, value)
                        })
                        + self.php_class_prefix.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(40, value)
                        })
                        + self.php_namespace.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(41, value)
                        })
                        + self.php_metadata_namespace.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(44, value)
                        })
                        + self.ruby_package.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::faststr::encoded_len(45, value)
                        })
                        + ::pilota::pb::encoding::message::encoded_len_repeated(
                            999,
                            &self.uninterpreted_option,
                        )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.java_package.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_outer_classname.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(8, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_multiple_files.as_ref() {
                        ::pilota::pb::encoding::bool::encode(10, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_generate_equals_and_hash.as_ref() {
                        ::pilota::pb::encoding::bool::encode(20, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_string_check_utf8.as_ref() {
                        ::pilota::pb::encoding::bool::encode(27, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.optimize_for.as_ref() {
                        ::pilota::pb::encoding::int32::encode(9, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.go_package.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(11, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.cc_generic_services.as_ref() {
                        ::pilota::pb::encoding::bool::encode(16, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_generic_services.as_ref() {
                        ::pilota::pb::encoding::bool::encode(17, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.py_generic_services.as_ref() {
                        ::pilota::pb::encoding::bool::encode(18, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.php_generic_services.as_ref() {
                        ::pilota::pb::encoding::bool::encode(42, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::pb::encoding::bool::encode(23, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.cc_enable_arenas.as_ref() {
                        ::pilota::pb::encoding::bool::encode(31, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.objc_class_prefix.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(36, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.csharp_namespace.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(37, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.swift_prefix.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(39, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.php_class_prefix.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(40, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.php_namespace.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(41, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.php_metadata_namespace.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(44, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.ruby_package.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(45, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::pb::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(FileOptions);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.java_package;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_package));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.java_outer_classname;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_outer_classname));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.java_multiple_files;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_multiple_files));
                                error
                            })
                        }
                        20 => {
                            let mut _inner_pilota_value = &mut self.java_generate_equals_and_hash;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_generate_equals_and_hash));
                                error
                            })
                        }
                        27 => {
                            let mut _inner_pilota_value = &mut self.java_string_check_utf8;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_string_check_utf8));
                                error
                            })
                        }
                        9 => {
                            let mut _inner_pilota_value = &mut self.optimize_for;
                            ::pilota::pb::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(optimize_for));
                                error
                            })
                        }
                        11 => {
                            let mut _inner_pilota_value = &mut self.go_package;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(go_package));
                                error
                            })
                        }
                        16 => {
                            let mut _inner_pilota_value = &mut self.cc_generic_services;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(cc_generic_services));
                                error
                            })
                        }
                        17 => {
                            let mut _inner_pilota_value = &mut self.java_generic_services;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_generic_services));
                                error
                            })
                        }
                        18 => {
                            let mut _inner_pilota_value = &mut self.py_generic_services;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(py_generic_services));
                                error
                            })
                        }
                        42 => {
                            let mut _inner_pilota_value = &mut self.php_generic_services;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(php_generic_services));
                                error
                            })
                        }
                        23 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        31 => {
                            let mut _inner_pilota_value = &mut self.cc_enable_arenas;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(cc_enable_arenas));
                                error
                            })
                        }
                        36 => {
                            let mut _inner_pilota_value = &mut self.objc_class_prefix;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(objc_class_prefix));
                                error
                            })
                        }
                        37 => {
                            let mut _inner_pilota_value = &mut self.csharp_namespace;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(csharp_namespace));
                                error
                            })
                        }
                        39 => {
                            let mut _inner_pilota_value = &mut self.swift_prefix;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(swift_prefix));
                                error
                            })
                        }
                        40 => {
                            let mut _inner_pilota_value = &mut self.php_class_prefix;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(php_class_prefix));
                                error
                            })
                        }
                        41 => {
                            let mut _inner_pilota_value = &mut self.php_namespace;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(php_namespace));
                                error
                            })
                        }
                        44 => {
                            let mut _inner_pilota_value = &mut self.php_metadata_namespace;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(php_metadata_namespace));
                                error
                            })
                        }
                        45 => {
                            let mut _inner_pilota_value = &mut self.ruby_package;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(ruby_package));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct EnumValueOptions {
                pub deprecated: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::pb::Message for EnumValueOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(1, value)
                    }) + ::pilota::pb::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::pb::encoding::bool::encode(1, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::pb::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(EnumValueOptions);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct FieldOptions {
                pub ctype: ::std::option::Option<field_options::CType>,

                pub packed: ::std::option::Option<bool>,

                pub jstype: ::std::option::Option<field_options::JsType>,

                pub lazy: ::std::option::Option<bool>,

                pub deprecated: ::std::option::Option<bool>,

                pub weak: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::pb::Message for FieldOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.ctype.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::int32::encoded_len(1, value)
                    }) + self.packed.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(2, value)
                    }) + self.jstype.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::int32::encoded_len(6, value)
                    }) + self.lazy.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(5, value)
                    }) + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(3, value)
                    }) + self.weak.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(10, value)
                    }) + ::pilota::pb::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.ctype.as_ref() {
                        ::pilota::pb::encoding::int32::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.packed.as_ref() {
                        ::pilota::pb::encoding::bool::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.jstype.as_ref() {
                        ::pilota::pb::encoding::int32::encode(6, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.lazy.as_ref() {
                        ::pilota::pb::encoding::bool::encode(5, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::pb::encoding::bool::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.weak.as_ref() {
                        ::pilota::pb::encoding::bool::encode(10, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::pb::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(FieldOptions);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.ctype;
                            ::pilota::pb::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(ctype));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.packed;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(packed));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.jstype;
                            ::pilota::pb::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(jstype));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.lazy;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(lazy));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.weak;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(weak));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
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
            pub struct GeneratedCodeInfo {
                pub annotation: ::std::vec::Vec<generated_code_info::Annotation>,
            }
            impl ::pilota::pb::Message for GeneratedCodeInfo {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::pb::encoding::message::encoded_len_repeated(1, &self.annotation)
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    for msg in &self.annotation {
                        ::pilota::pb::encoding::message::encode(1, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(GeneratedCodeInfo);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.annotation;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(annotation));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct MethodOptions {
                pub deprecated: ::std::option::Option<bool>,

                pub idempotency_level: ::std::option::Option<method_options::IdempotencyLevel>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::pb::Message for MethodOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::bool::encoded_len(33, value)
                    }) + self.idempotency_level.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::int32::encoded_len(34, value)
                    }) + ::pilota::pb::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::pb::encoding::bool::encode(33, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.idempotency_level.as_ref() {
                        ::pilota::pb::encoding::int32::encode(34, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::pb::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(MethodOptions);

                    match tag {
                        33 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::pb::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        34 => {
                            let mut _inner_pilota_value = &mut self.idempotency_level;
                            ::pilota::pb::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(idempotency_level));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::pb::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(
                PartialOrd,
                Debug,
                Default,
                ::pilota::serde::Serialize,
                ::pilota::serde::Deserialize,
                Clone,
                PartialEq,
            )]
            pub struct EnumValueDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub number: ::std::option::Option<i32>,

                pub options: ::std::option::Option<EnumValueOptions>,
            }
            impl ::pilota::pb::Message for EnumValueDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::faststr::encoded_len(1, value)
                    }) + self.number.as_ref().map_or(0, |value| {
                        ::pilota::pb::encoding::int32::encoded_len(2, value)
                    }) + self.options.as_ref().map_or(0, |msg| {
                        ::pilota::pb::encoding::message::encoded_len(3, msg)
                    })
                }

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::pb::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.number.as_ref() {
                        ::pilota::pb::encoding::int32::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
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
                ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                    const STRUCT_NAME: &'static str = stringify!(EnumValueDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::pb::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.number;
                            ::pilota::pb::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(number));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::pb::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }

            pub mod descriptor_proto {
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
                pub struct ReservedRange {
                    pub start: ::std::option::Option<i32>,

                    pub end: ::std::option::Option<i32>,
                }
                impl ::pilota::pb::Message for ReservedRange {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + self.start.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::int32::encoded_len(1, value)
                        }) + self.end.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::int32::encoded_len(2, value)
                        })
                    }

                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                        if let Some(_pilota_inner_value) = self.start.as_ref() {
                            ::pilota::pb::encoding::int32::encode(1, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.end.as_ref() {
                            ::pilota::pb::encoding::int32::encode(2, _pilota_inner_value, buf);
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
                        const STRUCT_NAME: &'static str = stringify!(ReservedRange);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.start;
                                ::pilota::pb::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(start));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.end;
                                ::pilota::pb::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(end));
                                    error
                                })
                            }
                            _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
                #[derive(
                    PartialOrd,
                    Debug,
                    Default,
                    ::pilota::serde::Serialize,
                    ::pilota::serde::Deserialize,
                    Clone,
                    PartialEq,
                )]
                pub struct ExtensionRange {
                    pub start: ::std::option::Option<i32>,

                    pub end: ::std::option::Option<i32>,

                    pub options: ::std::option::Option<super::ExtensionRangeOptions>,
                }
                impl ::pilota::pb::Message for ExtensionRange {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + self.start.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::int32::encoded_len(1, value)
                        }) + self.end.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::int32::encoded_len(2, value)
                        }) + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::pb::encoding::message::encoded_len(3, msg)
                        })
                    }

                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                        if let Some(_pilota_inner_value) = self.start.as_ref() {
                            ::pilota::pb::encoding::int32::encode(1, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.end.as_ref() {
                            ::pilota::pb::encoding::int32::encode(2, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.options.as_ref() {
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
                    ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                        const STRUCT_NAME: &'static str = stringify!(ExtensionRange);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.start;
                                ::pilota::pb::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(start));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.end;
                                ::pilota::pb::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(end));
                                    error
                                })
                            }
                            3 => {
                                let mut _inner_pilota_value = &mut self.options;
                                ::pilota::pb::encoding::message::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(options));
                                    error
                                })
                            }
                            _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }

            pub mod enum_descriptor_proto {
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
                pub struct EnumReservedRange {
                    pub start: ::std::option::Option<i32>,

                    pub end: ::std::option::Option<i32>,
                }
                impl ::pilota::pb::Message for EnumReservedRange {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + self.start.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::int32::encoded_len(1, value)
                        }) + self.end.as_ref().map_or(0, |value| {
                            ::pilota::pb::encoding::int32::encoded_len(2, value)
                        })
                    }

                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                        if let Some(_pilota_inner_value) = self.start.as_ref() {
                            ::pilota::pb::encoding::int32::encode(1, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.end.as_ref() {
                            ::pilota::pb::encoding::int32::encode(2, _pilota_inner_value, buf);
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
                        const STRUCT_NAME: &'static str = stringify!(EnumReservedRange);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.start;
                                ::pilota::pb::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(start));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.end;
                                ::pilota::pb::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(end));
                                    error
                                })
                            }
                            _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }

            pub mod field_descriptor_proto {
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
                )]
                #[serde(transparent)]
                #[derive(Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct Type(i32);

                impl Type {
                    pub const TYPE_DOUBLE: Self = Self(1);
                    pub const TYPE_FLOAT: Self = Self(2);
                    pub const TYPE_INT64: Self = Self(3);
                    pub const TYPE_UINT64: Self = Self(4);
                    pub const TYPE_INT32: Self = Self(5);
                    pub const TYPE_FIXED64: Self = Self(6);
                    pub const TYPE_FIXED32: Self = Self(7);
                    pub const TYPE_BOOL: Self = Self(8);
                    pub const TYPE_STRING: Self = Self(9);
                    pub const TYPE_GROUP: Self = Self(10);
                    pub const TYPE_MESSAGE: Self = Self(11);
                    pub const TYPE_BYTES: Self = Self(12);
                    pub const TYPE_UINT32: Self = Self(13);
                    pub const TYPE_ENUM: Self = Self(14);
                    pub const TYPE_SFIXED32: Self = Self(15);
                    pub const TYPE_SFIXED64: Self = Self(16);
                    pub const TYPE_SINT32: Self = Self(17);
                    pub const TYPE_SINT64: Self = Self(18);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(1) => ::std::string::String::from("TYPE_DOUBLE"),
                            Self(2) => ::std::string::String::from("TYPE_FLOAT"),
                            Self(3) => ::std::string::String::from("TYPE_INT64"),
                            Self(4) => ::std::string::String::from("TYPE_UINT64"),
                            Self(5) => ::std::string::String::from("TYPE_INT32"),
                            Self(6) => ::std::string::String::from("TYPE_FIXED64"),
                            Self(7) => ::std::string::String::from("TYPE_FIXED32"),
                            Self(8) => ::std::string::String::from("TYPE_BOOL"),
                            Self(9) => ::std::string::String::from("TYPE_STRING"),
                            Self(10) => ::std::string::String::from("TYPE_GROUP"),
                            Self(11) => ::std::string::String::from("TYPE_MESSAGE"),
                            Self(12) => ::std::string::String::from("TYPE_BYTES"),
                            Self(13) => ::std::string::String::from("TYPE_UINT32"),
                            Self(14) => ::std::string::String::from("TYPE_ENUM"),
                            Self(15) => ::std::string::String::from("TYPE_SFIXED32"),
                            Self(16) => ::std::string::String::from("TYPE_SFIXED64"),
                            Self(17) => ::std::string::String::from("TYPE_SINT32"),
                            Self(18) => ::std::string::String::from("TYPE_SINT64"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            1 => Some(Self::TYPE_DOUBLE),
                            2 => Some(Self::TYPE_FLOAT),
                            3 => Some(Self::TYPE_INT64),
                            4 => Some(Self::TYPE_UINT64),
                            5 => Some(Self::TYPE_INT32),
                            6 => Some(Self::TYPE_FIXED64),
                            7 => Some(Self::TYPE_FIXED32),
                            8 => Some(Self::TYPE_BOOL),
                            9 => Some(Self::TYPE_STRING),
                            10 => Some(Self::TYPE_GROUP),
                            11 => Some(Self::TYPE_MESSAGE),
                            12 => Some(Self::TYPE_BYTES),
                            13 => Some(Self::TYPE_UINT32),
                            14 => Some(Self::TYPE_ENUM),
                            15 => Some(Self::TYPE_SFIXED32),
                            16 => Some(Self::TYPE_SFIXED64),
                            17 => Some(Self::TYPE_SINT32),
                            18 => Some(Self::TYPE_SINT64),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for Type {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<Type> for i32 {
                    fn from(value: Type) -> i32 {
                        value.0
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
                )]
                #[serde(transparent)]
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

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(1) => ::std::string::String::from("LABEL_OPTIONAL"),
                            Self(2) => ::std::string::String::from("LABEL_REQUIRED"),
                            Self(3) => ::std::string::String::from("LABEL_REPEATED"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            1 => Some(Self::LABEL_OPTIONAL),
                            2 => Some(Self::LABEL_REQUIRED),
                            3 => Some(Self::LABEL_REPEATED),
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

            pub mod field_options {
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
                )]
                #[serde(transparent)]
                #[derive(Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct JsType(i32);

                impl JsType {
                    pub const JS_NORMAL: Self = Self(0);
                    pub const JS_STRING: Self = Self(1);
                    pub const JS_NUMBER: Self = Self(2);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(0) => ::std::string::String::from("JS_NORMAL"),
                            Self(1) => ::std::string::String::from("JS_STRING"),
                            Self(2) => ::std::string::String::from("JS_NUMBER"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            0 => Some(Self::JS_NORMAL),
                            1 => Some(Self::JS_STRING),
                            2 => Some(Self::JS_NUMBER),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for JsType {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<JsType> for i32 {
                    fn from(value: JsType) -> i32 {
                        value.0
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
                )]
                #[serde(transparent)]
                #[derive(Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct CType(i32);

                impl CType {
                    pub const STRING: Self = Self(0);
                    pub const CORD: Self = Self(1);
                    pub const STRING_PIECE: Self = Self(2);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(0) => ::std::string::String::from("STRING"),
                            Self(1) => ::std::string::String::from("CORD"),
                            Self(2) => ::std::string::String::from("STRING_PIECE"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            0 => Some(Self::STRING),
                            1 => Some(Self::CORD),
                            2 => Some(Self::STRING_PIECE),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for CType {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<CType> for i32 {
                    fn from(value: CType) -> i32 {
                        value.0
                    }
                }
            }

            pub mod file_options {
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
                )]
                #[serde(transparent)]
                #[derive(Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct OptimizeMode(i32);

                impl OptimizeMode {
                    pub const SPEED: Self = Self(1);
                    pub const CODE_SIZE: Self = Self(2);
                    pub const LITE_RUNTIME: Self = Self(3);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(1) => ::std::string::String::from("SPEED"),
                            Self(2) => ::std::string::String::from("CODE_SIZE"),
                            Self(3) => ::std::string::String::from("LITE_RUNTIME"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            1 => Some(Self::SPEED),
                            2 => Some(Self::CODE_SIZE),
                            3 => Some(Self::LITE_RUNTIME),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for OptimizeMode {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<OptimizeMode> for i32 {
                    fn from(value: OptimizeMode) -> i32 {
                        value.0
                    }
                }
            }

            pub mod generated_code_info {
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
                pub struct Annotation {
                    pub path: ::std::vec::Vec<i32>,

                    pub source_file: ::std::option::Option<::pilota::FastStr>,

                    pub begin: ::std::option::Option<i32>,

                    pub end: ::std::option::Option<i32>,
                }
                impl ::pilota::pb::Message for Annotation {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + ::pilota::pb::encoding::int32::encoded_len_repeated(1, &self.path)
                            + self.source_file.as_ref().map_or(0, |value| {
                                ::pilota::pb::encoding::faststr::encoded_len(2, value)
                            })
                            + self.begin.as_ref().map_or(0, |value| {
                                ::pilota::pb::encoding::int32::encoded_len(3, value)
                            })
                            + self.end.as_ref().map_or(0, |value| {
                                ::pilota::pb::encoding::int32::encoded_len(4, value)
                            })
                    }

                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                        ::pilota::pb::encoding::int32::encode_repeated(1, &self.path, buf);
                        if let Some(_pilota_inner_value) = self.source_file.as_ref() {
                            ::pilota::pb::encoding::faststr::encode(2, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.begin.as_ref() {
                            ::pilota::pb::encoding::int32::encode(3, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.end.as_ref() {
                            ::pilota::pb::encoding::int32::encode(4, _pilota_inner_value, buf);
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
                        const STRUCT_NAME: &'static str = stringify!(Annotation);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.path;
                                ::pilota::pb::encoding::int32::merge_repeated(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(path));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.source_file;
                                ::pilota::pb::encoding::faststr::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(source_file));
                                    error
                                })
                            }
                            3 => {
                                let mut _inner_pilota_value = &mut self.begin;
                                ::pilota::pb::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(begin));
                                    error
                                })
                            }
                            4 => {
                                let mut _inner_pilota_value = &mut self.end;
                                ::pilota::pb::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(end));
                                    error
                                })
                            }
                            _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }

            pub mod method_options {
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
                )]
                #[serde(transparent)]
                #[derive(Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct IdempotencyLevel(i32);

                impl IdempotencyLevel {
                    pub const IDEMPOTENCY_UNKNOWN: Self = Self(0);
                    pub const NO_SIDE_EFFECTS: Self = Self(1);
                    pub const IDEMPOTENT: Self = Self(2);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(0) => ::std::string::String::from("IDEMPOTENCY_UNKNOWN"),
                            Self(1) => ::std::string::String::from("NO_SIDE_EFFECTS"),
                            Self(2) => ::std::string::String::from("IDEMPOTENT"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            0 => Some(Self::IDEMPOTENCY_UNKNOWN),
                            1 => Some(Self::NO_SIDE_EFFECTS),
                            2 => Some(Self::IDEMPOTENT),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for IdempotencyLevel {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<IdempotencyLevel> for i32 {
                    fn from(value: IdempotencyLevel) -> i32 {
                        value.0
                    }
                }
            }

            pub mod source_code_info {
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
                pub struct Location {
                    pub path: ::std::vec::Vec<i32>,

                    pub span: ::std::vec::Vec<i32>,

                    pub leading_comments: ::std::option::Option<::pilota::FastStr>,

                    pub trailing_comments: ::std::option::Option<::pilota::FastStr>,

                    pub leading_detached_comments: ::std::vec::Vec<::pilota::FastStr>,
                }
                impl ::pilota::pb::Message for Location {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + ::pilota::pb::encoding::int32::encoded_len_repeated(1, &self.path)
                            + ::pilota::pb::encoding::int32::encoded_len_repeated(2, &self.span)
                            + self.leading_comments.as_ref().map_or(0, |value| {
                                ::pilota::pb::encoding::faststr::encoded_len(3, value)
                            })
                            + self.trailing_comments.as_ref().map_or(0, |value| {
                                ::pilota::pb::encoding::faststr::encoded_len(4, value)
                            })
                            + ::pilota::pb::encoding::faststr::encoded_len_repeated(
                                6,
                                &self.leading_detached_comments,
                            )
                    }

                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                        ::pilota::pb::encoding::int32::encode_repeated(1, &self.path, buf);
                        ::pilota::pb::encoding::int32::encode_repeated(2, &self.span, buf);
                        if let Some(_pilota_inner_value) = self.leading_comments.as_ref() {
                            ::pilota::pb::encoding::faststr::encode(3, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.trailing_comments.as_ref() {
                            ::pilota::pb::encoding::faststr::encode(4, _pilota_inner_value, buf);
                        };
                        ::pilota::pb::encoding::faststr::encode_repeated(
                            6,
                            &self.leading_detached_comments,
                            buf,
                        );
                    }

                    #[allow(unused_variables)]
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        wire_type: ::pilota::pb::encoding::WireType,
                        buf: &mut ::pilota::Bytes,
                        ctx: &mut ::pilota::pb::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                        const STRUCT_NAME: &'static str = stringify!(Location);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.path;
                                ::pilota::pb::encoding::int32::merge_repeated(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(path));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.span;
                                ::pilota::pb::encoding::int32::merge_repeated(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(span));
                                    error
                                })
                            }
                            3 => {
                                let mut _inner_pilota_value = &mut self.leading_comments;
                                ::pilota::pb::encoding::faststr::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(leading_comments));
                                    error
                                })
                            }
                            4 => {
                                let mut _inner_pilota_value = &mut self.trailing_comments;
                                ::pilota::pb::encoding::faststr::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(trailing_comments));
                                    error
                                })
                            }
                            6 => {
                                let mut _inner_pilota_value = &mut self.leading_detached_comments;
                                ::pilota::pb::encoding::faststr::merge_repeated(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(leading_detached_comments));
                                    error
                                })
                            }
                            _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }

            pub mod uninterpreted_option {
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
                pub struct NamePart {
                    pub name_part: ::pilota::FastStr,

                    pub is_extension: bool,
                }
                impl ::pilota::pb::Message for NamePart {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + ::pilota::pb::encoding::faststr::encoded_len(1, &self.name_part)
                            + ::pilota::pb::encoding::bool::encoded_len(2, &self.is_extension)
                    }

                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                        ::pilota::pb::encoding::faststr::encode(1, &self.name_part, buf);
                        ::pilota::pb::encoding::bool::encode(2, &self.is_extension, buf);
                    }

                    #[allow(unused_variables)]
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        wire_type: ::pilota::pb::encoding::WireType,
                        buf: &mut ::pilota::Bytes,
                        ctx: &mut ::pilota::pb::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                        const STRUCT_NAME: &'static str = stringify!(NamePart);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.name_part;
                                ::pilota::pb::encoding::faststr::merge(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(name_part));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.is_extension;
                                ::pilota::pb::encoding::bool::merge(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(is_extension));
                                    error
                                })
                            }
                            _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }
        }
    }

    pub mod pilota {
        use ::pilota::{Buf as _, BufMut as _};
    }
}
