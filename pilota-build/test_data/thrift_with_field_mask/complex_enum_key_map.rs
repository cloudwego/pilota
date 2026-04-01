pub mod complex_enum_key_map {
    #![allow(warnings, clippy::all)]

    pub mod complex_enum_key_map {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Priority(i32);

        impl Priority {
            pub const LOW: Self = Self(1);
            pub const MEDIUM: Self = Self(2);
            pub const HIGH: Self = Self(3);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(1) => ::std::string::String::from("LOW"),
                    Self(2) => ::std::string::String::from("MEDIUM"),
                    Self(3) => ::std::string::String::from("HIGH"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    1 => Some(Self::LOW),
                    2 => Some(Self::MEDIUM),
                    3 => Some(Self::HIGH),
                    _ => None,
                }
            }
        }

        impl ::std::convert::From<i32> for Priority {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Priority> for i32 {
            fn from(value: Priority) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for Priority {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_i32(self.inner())?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                let value = __protocol.read_i32()?;
                ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(
                    |err| {
                        ::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            format!("invalid enum value for Priority, value: {}", value),
                        )
                    },
                )?)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let value = __protocol.read_i32().await?;
                    ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(
                        |err| {
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                format!("invalid enum value for Priority, value: {}", value),
                            )
                        },
                    )?)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.i32_len(self.inner())
            }
        }

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct NestedItem {
            pub value: i32,

            pub label: ::std::option::Option<::pilota::FastStr>,
            pub _field_mask: ::std::option::Option<::pilota_thrift_fieldmask::FieldMask>,
        }
        impl ::pilota::thrift::Message for NestedItem {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.exist() {
                        ::std::result::Result::Ok(())
                    } else {
                        let struct_ident =
                            ::pilota::thrift::TStructIdentifier { name: "NestedItem" };
                        __protocol.write_struct_begin(&struct_ident)?;
                        let (field_fm, _) = struct_fm.field(1);
                        __protocol.write_i32_field(1, *&self.value)?;
                        if let Some(value) = self.label.as_ref() {
                            let (field_fm, exist) = struct_fm.field(2);
                            if exist {
                                __protocol.write_faststr_field(2, (value).clone())?;
                            }
                        }
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }
                } else {
                    let struct_ident = ::pilota::thrift::TStructIdentifier { name: "NestedItem" };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_i32_field(1, *&self.value)?;
                    if let Some(value) = self.label.as_ref() {
                        __protocol.write_faststr_field(2, (value).clone())?;
                    }
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = None;
                let mut var_2 = None;

                let mut __pilota_decoding_field_id = None;

                __protocol.read_struct_begin()?;
                if let ::std::result::Result::Err(mut err) = (|| {
                    loop {
                        let field_ident = __protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __protocol.field_stop_len();
                            break;
                        } else {
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                var_1 = Some(__protocol.read_i32()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_2 = Some(__protocol.read_faststr()?);
                            }
                            _ => {
                                __protocol.skip(field_ident.field_type)?;
                            }
                        }

                        __protocol.read_field_end()?;
                        __protocol.field_end_len();
                    }
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `NestedItem` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };

                let data = Self {
                    value: var_1,
                    label: var_2,
                    _field_mask: ::std::option::Option::None,
                };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut var_1 = None;
                    let mut var_2 = None;

                    let mut __pilota_decoding_field_id = None;

                    __protocol.read_struct_begin().await?;
                    if let ::std::result::Result::Err(mut err) = async {
                        loop {
                            let field_ident = __protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_1 = Some(__protocol.read_i32().await?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_2 = Some(__protocol.read_faststr().await?);
                                }
                                _ => {
                                    __protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            __protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `NestedItem` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        value: var_1,
                        label: var_2,
                        _field_mask: ::std::option::Option::None,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.exist() {
                        0
                    } else {
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "NestedItem",
                        }) + {
                            let (field_fm, exist) = struct_fm.field(1);
                            if exist {
                                __protocol.i32_field_len(Some(1), *&self.value)
                            } else {
                                0
                            }
                        } + self.label.as_ref().map_or(0, |value| {
                            let (field_fm, exist) = struct_fm.field(2);
                            if exist {
                                __protocol.faststr_field_len(Some(2), value)
                            } else {
                                0
                            }
                        }) + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                } else {
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "NestedItem",
                    }) + __protocol.i32_field_len(Some(1), *&self.value)
                        + self
                            .label
                            .as_ref()
                            .map_or(0, |value| __protocol.faststr_field_len(Some(2), value))
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
        }
        impl NestedItem {
            pub fn set_field_mask(&mut self, field_mask: ::pilota_thrift_fieldmask::FieldMask) {
                self._field_mask = Some(field_mask.clone());
            }
        }

        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct ComplexEnumKeyMapTest {
            pub priority_counts: ::pilota::AHashMap<Priority, i32>,

            pub priority_items: ::pilota::AHashMap<Priority, NestedItem>,

            pub nested_maps:
                ::pilota::AHashMap<Priority, ::pilota::AHashMap<::pilota::FastStr, i32>>,

            pub priority_item_lists:
                ::std::option::Option<::pilota::AHashMap<Priority, ::std::vec::Vec<NestedItem>>>,
            pub _field_mask: ::std::option::Option<::pilota_thrift_fieldmask::FieldMask>,
        }
        impl ::pilota::thrift::Message for ComplexEnumKeyMapTest {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.exist() {
                        ::std::result::Result::Ok(())
                    } else {
                        let struct_ident = ::pilota::thrift::TStructIdentifier {
                            name: "ComplexEnumKeyMapTest",
                        };
                        __protocol.write_struct_begin(&struct_ident)?;
                        let (field_fm, _) = struct_fm.field(1);
                        if let Some(map_fm) = field_fm {
                            __protocol.write_field_begin(::pilota::thrift::TType::Map, 1)?;
                            __protocol.write_map_begin(::pilota::thrift::TMapIdentifier {
                                key_type: ::pilota::thrift::TType::I32,
                                value_type: ::pilota::thrift::TType::I32,
                                size: (&self.priority_counts)
                                    .keys()
                                    .filter(|key| map_fm.int(key.inner() as i32).1)
                                    .count(),
                            })?;
                            for (key, val) in &self.priority_counts {
                                let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                if is_exist {
                                    __protocol.write_struct(key)?;
                                    __protocol.write_i32(*val)?;
                                }
                            }
                            __protocol.write_map_end()?;
                            __protocol.write_field_end()?;
                        } else {
                            __protocol.write_map_field(
                                1,
                                ::pilota::thrift::TType::I32,
                                ::pilota::thrift::TType::I32,
                                &&self.priority_counts,
                                |__protocol, key| {
                                    __protocol.write_struct(key)?;
                                    ::std::result::Result::Ok(())
                                },
                                |__protocol, val| {
                                    __protocol.write_i32(*val)?;
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                        }
                        let (field_fm, _) = struct_fm.field(2);
                        if let Some(map_fm) = field_fm {
                            __protocol.write_field_begin(::pilota::thrift::TType::Map, 2)?;
                            __protocol.write_map_begin(::pilota::thrift::TMapIdentifier {
                                key_type: ::pilota::thrift::TType::I32,
                                value_type: ::pilota::thrift::TType::Struct,
                                size: (&self.priority_items)
                                    .keys()
                                    .filter(|key| map_fm.int(key.inner() as i32).1)
                                    .count(),
                            })?;
                            for (key, val) in &self.priority_items {
                                let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                if is_exist {
                                    __protocol.write_struct(key)?;
                                    __protocol.write_struct(val)?;
                                }
                            }
                            __protocol.write_map_end()?;
                            __protocol.write_field_end()?;
                        } else {
                            __protocol.write_map_field(
                                2,
                                ::pilota::thrift::TType::I32,
                                ::pilota::thrift::TType::Struct,
                                &&self.priority_items,
                                |__protocol, key| {
                                    __protocol.write_struct(key)?;
                                    ::std::result::Result::Ok(())
                                },
                                |__protocol, val| {
                                    __protocol.write_struct(val)?;
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                        }
                        let (field_fm, _) = struct_fm.field(3);
                        if let Some(map_fm) = field_fm {
                            __protocol.write_field_begin(::pilota::thrift::TType::Map, 3)?;
                            __protocol.write_map_begin(::pilota::thrift::TMapIdentifier {
                                key_type: ::pilota::thrift::TType::I32,
                                value_type: ::pilota::thrift::TType::Map,
                                size: (&self.nested_maps)
                                    .keys()
                                    .filter(|key| map_fm.int(key.inner() as i32).1)
                                    .count(),
                            })?;
                            for (key, val) in &self.nested_maps {
                                let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                if is_exist {
                                    __protocol.write_struct(key)?;
                                    if let Some(map_fm) = item_fm {
                                        __protocol.write_map_begin(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::Binary,
                                                value_type: ::pilota::thrift::TType::I32,
                                                size: (val)
                                                    .keys()
                                                    .filter(|key| map_fm.str(key).1)
                                                    .count(),
                                            },
                                        )?;
                                        for (key, val) in val {
                                            let (item_fm, exist) = map_fm.str(key.as_str());
                                            if exist {
                                                __protocol.write_faststr((key).clone())?;
                                                __protocol.write_i32(*val)?;
                                            }
                                        }
                                        __protocol.write_map_end()?;
                                    } else {
                                        __protocol.write_map(
                                            ::pilota::thrift::TType::Binary,
                                            ::pilota::thrift::TType::I32,
                                            &val,
                                            |__protocol, key| {
                                                __protocol.write_faststr((key).clone())?;
                                                ::std::result::Result::Ok(())
                                            },
                                            |__protocol, val| {
                                                __protocol.write_i32(*val)?;
                                                ::std::result::Result::Ok(())
                                            },
                                        )?;
                                    }
                                }
                            }
                            __protocol.write_map_end()?;
                            __protocol.write_field_end()?;
                        } else {
                            __protocol.write_map_field(
                                3,
                                ::pilota::thrift::TType::I32,
                                ::pilota::thrift::TType::Map,
                                &&self.nested_maps,
                                |__protocol, key| {
                                    __protocol.write_struct(key)?;
                                    ::std::result::Result::Ok(())
                                },
                                |__protocol, val| {
                                    __protocol.write_map(
                                        ::pilota::thrift::TType::Binary,
                                        ::pilota::thrift::TType::I32,
                                        &val,
                                        |__protocol, key| {
                                            __protocol.write_faststr((key).clone())?;
                                            ::std::result::Result::Ok(())
                                        },
                                        |__protocol, val| {
                                            __protocol.write_i32(*val)?;
                                            ::std::result::Result::Ok(())
                                        },
                                    )?;
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                        }
                        if let Some(value) = self.priority_item_lists.as_ref() {
                            let (field_fm, exist) = struct_fm.field(4);
                            if exist {
                                if let Some(map_fm) = field_fm {
                                    __protocol
                                        .write_field_begin(::pilota::thrift::TType::Map, 4)?;
                                    __protocol.write_map_begin(
                                        ::pilota::thrift::TMapIdentifier {
                                            key_type: ::pilota::thrift::TType::I32,
                                            value_type: ::pilota::thrift::TType::List,
                                            size: (value)
                                                .keys()
                                                .filter(|key| map_fm.int(key.inner() as i32).1)
                                                .count(),
                                        },
                                    )?;
                                    for (key, val) in value {
                                        let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                        if is_exist {
                                            __protocol.write_struct(key)?;
                                            if let Some(list_fm) = item_fm {
                                                __protocol.write_list_begin(
                                                    ::pilota::thrift::TListIdentifier {
                                                        element_type:
                                                            ::pilota::thrift::TType::Struct,
                                                        size: (0..(val).len())
                                                            .filter(|idx| {
                                                                list_fm.int(*idx as i32).1
                                                            })
                                                            .count(),
                                                    },
                                                )?;
                                                let mut idx = 0;
                                                for val in val {
                                                    let (item_fm, exist) = list_fm.int(idx as i32);
                                                    if exist {
                                                        __protocol.write_struct(val)?;
                                                    }
                                                    idx += 1;
                                                }
                                                __protocol.write_list_end()?;
                                            } else {
                                                __protocol.write_list(
                                                    ::pilota::thrift::TType::Struct,
                                                    &val,
                                                    |__protocol, val| {
                                                        __protocol.write_struct(val)?;
                                                        ::std::result::Result::Ok(())
                                                    },
                                                )?;
                                            }
                                        }
                                    }
                                    __protocol.write_map_end()?;
                                    __protocol.write_field_end()?;
                                } else {
                                    __protocol.write_map_field(
                                        4,
                                        ::pilota::thrift::TType::I32,
                                        ::pilota::thrift::TType::List,
                                        &value,
                                        |__protocol, key| {
                                            __protocol.write_struct(key)?;
                                            ::std::result::Result::Ok(())
                                        },
                                        |__protocol, val| {
                                            __protocol.write_list(
                                                ::pilota::thrift::TType::Struct,
                                                &val,
                                                |__protocol, val| {
                                                    __protocol.write_struct(val)?;
                                                    ::std::result::Result::Ok(())
                                                },
                                            )?;
                                            ::std::result::Result::Ok(())
                                        },
                                    )?;
                                }
                            }
                        }
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }
                } else {
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "ComplexEnumKeyMapTest",
                    };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_map_field(
                        1,
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::I32,
                        &&self.priority_counts,
                        |__protocol, key| {
                            __protocol.write_struct(key)?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    __protocol.write_map_field(
                        2,
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::Struct,
                        &&self.priority_items,
                        |__protocol, key| {
                            __protocol.write_struct(key)?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_struct(val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    __protocol.write_map_field(
                        3,
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::Map,
                        &&self.nested_maps,
                        |__protocol, key| {
                            __protocol.write_struct(key)?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_map(
                                ::pilota::thrift::TType::Binary,
                                ::pilota::thrift::TType::I32,
                                &val,
                                |__protocol, key| {
                                    __protocol.write_faststr((key).clone())?;
                                    ::std::result::Result::Ok(())
                                },
                                |__protocol, val| {
                                    __protocol.write_i32(*val)?;
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    if let Some(value) = self.priority_item_lists.as_ref() {
                        __protocol.write_map_field(
                            4,
                            ::pilota::thrift::TType::I32,
                            ::pilota::thrift::TType::List,
                            &value,
                            |__protocol, key| {
                                __protocol.write_struct(key)?;
                                ::std::result::Result::Ok(())
                            },
                            |__protocol, val| {
                                __protocol.write_list(
                                    ::pilota::thrift::TType::Struct,
                                    &val,
                                    |__protocol, val| {
                                        __protocol.write_struct(val)?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;

                let mut __pilota_decoding_field_id = None;

                __protocol.read_struct_begin()?;
                if let ::std::result::Result::Err(mut err) = (|| {
                    loop {
                        let field_ident = __protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __protocol.field_stop_len();
                            break;
                        } else {
                            __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_1 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            ::pilota::thrift::Message::decode(__protocol)?,
                                            __protocol.read_i32()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_2 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            ::pilota::thrift::Message::decode(__protocol)?,
                                            ::pilota::thrift::Message::decode(__protocol)?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_3 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            ::pilota::thrift::Message::decode(__protocol)?,
                                            {
                                                let map_ident = __protocol.read_map_begin()?;
                                                let mut val = ::pilota::AHashMap::with_capacity(
                                                    map_ident.size,
                                                );
                                                for _ in 0..map_ident.size {
                                                    val.insert(
                                                        __protocol.read_faststr()?,
                                                        __protocol.read_i32()?,
                                                    );
                                                }
                                                __protocol.read_map_end()?;
                                                val
                                            },
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_4 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            ::pilota::thrift::Message::decode(__protocol)?,
                                            unsafe {
                                                let list_ident = __protocol.read_list_begin()?;
                                                let mut val: ::std::vec::Vec<NestedItem> =
                                                    ::std::vec::Vec::with_capacity(list_ident.size);
                                                for i in 0..list_ident.size {
                                                    val.as_mut_ptr().offset(i as isize).write(
                                                        ::pilota::thrift::Message::decode(
                                                            __protocol,
                                                        )?,
                                                    );
                                                }
                                                val.set_len(list_ident.size);
                                                __protocol.read_list_end()?;
                                                val
                                            },
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            _ => {
                                __protocol.skip(field_ident.field_type)?;
                            }
                        }

                        __protocol.read_field_end()?;
                        __protocol.field_end_len();
                    }
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `ComplexEnumKeyMapTest` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field priority_counts is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field priority_items is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field nested_maps is required".to_string(),
                    ));
                };

                let data = Self {
                    priority_counts: var_1,
                    priority_items: var_2,
                    nested_maps: var_3,
                    priority_item_lists: var_4,
                    _field_mask: ::std::option::Option::None,
                };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut var_1 = None;
                    let mut var_2 = None;
                    let mut var_3 = None;
                    let mut var_4 = None;

                    let mut __pilota_decoding_field_id = None;

                    __protocol.read_struct_begin().await?;
                    if let ::std::result::Result::Err(mut err) = async {
                    loop {


                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_1 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(<Priority as ::pilota::thrift::Message>::decode_async(__protocol).await?, __protocol.read_i32().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_2 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(<Priority as ::pilota::thrift::Message>::decode_async(__protocol).await?, <NestedItem as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_3 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(<Priority as ::pilota::thrift::Message>::decode_async(__protocol).await?, {
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_i32().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_4 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(<Priority as ::pilota::thrift::Message>::decode_async(__protocol).await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<NestedItem as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `ComplexEnumKeyMapTest` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field priority_counts is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field priority_items is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field nested_maps is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        priority_counts: var_1,
                        priority_items: var_2,
                        nested_maps: var_3,
                        priority_item_lists: var_4,
                        _field_mask: ::std::option::Option::None,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.exist() {
                        0
                    } else {
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "ComplexEnumKeyMapTest",
                        }) + {
                            let (field_fm, exist) = struct_fm.field(1);
                            if exist {
                                if let Some(map_fm) = field_fm {
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::Map, None)
                                        + __protocol.field_end_len()
                                        + __protocol.map_begin_len(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::I32,
                                                value_type: ::pilota::thrift::TType::I32,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in &self.priority_counts {
                                        let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                        if is_exist {
                                            size += __protocol.struct_len(key);
                                            size += __protocol.i32_len(*val);
                                        }
                                    }
                                    size
                                } else {
                                    __protocol.map_field_len(
                                        Some(1),
                                        ::pilota::thrift::TType::I32,
                                        ::pilota::thrift::TType::I32,
                                        &self.priority_counts,
                                        |__protocol, key| __protocol.struct_len(key),
                                        |__protocol, val| __protocol.i32_len(*val),
                                    )
                                }
                            } else {
                                0
                            }
                        } + {
                            let (field_fm, exist) = struct_fm.field(2);
                            if exist {
                                if let Some(map_fm) = field_fm {
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::Map, None)
                                        + __protocol.field_end_len()
                                        + __protocol.map_begin_len(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::I32,
                                                value_type: ::pilota::thrift::TType::Struct,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in &self.priority_items {
                                        let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                        if is_exist {
                                            size += __protocol.struct_len(key);
                                            size += __protocol.struct_len(val);
                                        }
                                    }
                                    size
                                } else {
                                    __protocol.map_field_len(
                                        Some(2),
                                        ::pilota::thrift::TType::I32,
                                        ::pilota::thrift::TType::Struct,
                                        &self.priority_items,
                                        |__protocol, key| __protocol.struct_len(key),
                                        |__protocol, val| __protocol.struct_len(val),
                                    )
                                }
                            } else {
                                0
                            }
                        } + {
                            let (field_fm, exist) = struct_fm.field(3);
                            if exist {
                                if let Some(map_fm) = field_fm {
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::Map, None)
                                        + __protocol.field_end_len()
                                        + __protocol.map_begin_len(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::I32,
                                                value_type: ::pilota::thrift::TType::Map,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in &self.nested_maps {
                                        let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                        if is_exist {
                                            size += __protocol.struct_len(key);
                                            size += if let Some(map_fm) = item_fm {
                                                let mut size = __protocol.map_begin_len(
                                                    ::pilota::thrift::TMapIdentifier {
                                                        key_type: ::pilota::thrift::TType::Binary,
                                                        value_type: ::pilota::thrift::TType::I32,
                                                        size: 0,
                                                    },
                                                ) + __protocol.map_end_len();
                                                for (key, val) in val {
                                                    let (item_fm, exist) = map_fm.str(key);
                                                    if exist {
                                                        size += __protocol.faststr_len(key);
                                                        size += __protocol.i32_len(*val);
                                                    }
                                                }
                                                size
                                            } else {
                                                __protocol.map_len(
                                                    ::pilota::thrift::TType::Binary,
                                                    ::pilota::thrift::TType::I32,
                                                    val,
                                                    |__protocol, key| __protocol.faststr_len(key),
                                                    |__protocol, val| __protocol.i32_len(*val),
                                                )
                                            };
                                        }
                                    }
                                    size
                                } else {
                                    __protocol.map_field_len(
                                        Some(3),
                                        ::pilota::thrift::TType::I32,
                                        ::pilota::thrift::TType::Map,
                                        &self.nested_maps,
                                        |__protocol, key| __protocol.struct_len(key),
                                        |__protocol, val| {
                                            __protocol.map_len(
                                                ::pilota::thrift::TType::Binary,
                                                ::pilota::thrift::TType::I32,
                                                val,
                                                |__protocol, key| __protocol.faststr_len(key),
                                                |__protocol, val| __protocol.i32_len(*val),
                                            )
                                        },
                                    )
                                }
                            } else {
                                0
                            }
                        } + self.priority_item_lists.as_ref().map_or(0, |value| {
                            let (field_fm, exist) = struct_fm.field(4);
                            if exist {
                                if let Some(map_fm) = field_fm {
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::Map, None)
                                        + __protocol.field_end_len()
                                        + __protocol.map_begin_len(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::I32,
                                                value_type: ::pilota::thrift::TType::List,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in value {
                                        let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                        if is_exist {
                                            size += __protocol.struct_len(key);
                                            size += if let Some(list_fm) = item_fm {
                                                let mut idx = 0;
                                                let mut size = __protocol.list_begin_len(
                                                    ::pilota::thrift::TListIdentifier {
                                                        element_type:
                                                            ::pilota::thrift::TType::Struct,
                                                        size: 0,
                                                    },
                                                ) + __protocol.list_end_len();
                                                for el in val {
                                                    let item_fm = list_fm.int(idx as i32);
                                                    size += __protocol.struct_len(el);
                                                    idx += 1;
                                                }
                                                size
                                            } else {
                                                __protocol.list_len(
                                                    ::pilota::thrift::TType::Struct,
                                                    val,
                                                    |__protocol, el| __protocol.struct_len(el),
                                                )
                                            };
                                        }
                                    }
                                    size
                                } else {
                                    __protocol.map_field_len(
                                        Some(4),
                                        ::pilota::thrift::TType::I32,
                                        ::pilota::thrift::TType::List,
                                        value,
                                        |__protocol, key| __protocol.struct_len(key),
                                        |__protocol, val| {
                                            __protocol.list_len(
                                                ::pilota::thrift::TType::Struct,
                                                val,
                                                |__protocol, el| __protocol.struct_len(el),
                                            )
                                        },
                                    )
                                }
                            } else {
                                0
                            }
                        }) + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                } else {
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "ComplexEnumKeyMapTest",
                    }) + __protocol.map_field_len(
                        Some(1),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::I32,
                        &self.priority_counts,
                        |__protocol, key| __protocol.struct_len(key),
                        |__protocol, val| __protocol.i32_len(*val),
                    ) + __protocol.map_field_len(
                        Some(2),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::Struct,
                        &self.priority_items,
                        |__protocol, key| __protocol.struct_len(key),
                        |__protocol, val| __protocol.struct_len(val),
                    ) + __protocol.map_field_len(
                        Some(3),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::Map,
                        &self.nested_maps,
                        |__protocol, key| __protocol.struct_len(key),
                        |__protocol, val| {
                            __protocol.map_len(
                                ::pilota::thrift::TType::Binary,
                                ::pilota::thrift::TType::I32,
                                val,
                                |__protocol, key| __protocol.faststr_len(key),
                                |__protocol, val| __protocol.i32_len(*val),
                            )
                        },
                    ) + self.priority_item_lists.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(4),
                            ::pilota::thrift::TType::I32,
                            ::pilota::thrift::TType::List,
                            value,
                            |__protocol, key| __protocol.struct_len(key),
                            |__protocol, val| {
                                __protocol.list_len(
                                    ::pilota::thrift::TType::Struct,
                                    val,
                                    |__protocol, el| __protocol.struct_len(el),
                                )
                            },
                        )
                    }) + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
        }
        impl ComplexEnumKeyMapTest {
            pub fn set_field_mask(&mut self, field_mask: ::pilota_thrift_fieldmask::FieldMask) {
                self._field_mask = Some(field_mask.clone());
                if let Some(map_mask) = field_mask.field(2).0 {
                    if !map_mask.all() {
                        for (key, item) in self.priority_items.iter_mut() {
                            if let Some(item_fm) = map_mask.int(key.inner() as i32).0 {
                                item.set_field_mask(item_fm.clone());
                            }
                        }
                    }
                }
                if let Some(value) = &mut self.priority_item_lists {
                    if let Some(map_mask) = field_mask.field(4).0 {
                        if !map_mask.all() {
                            for (key, item) in value.iter_mut() {
                                if let Some(item_fm) = map_mask.int(key.inner() as i32).0 {
                                    if !item_fm.all() {
                                        for (idx, item) in item.iter_mut().enumerate() {
                                            if let Some(item_fm) = item_fm.int(idx as i32).0 {
                                                item.set_field_mask(item_fm.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
