pub mod enum_key_map {
    #![allow(warnings, clippy::all)]

    pub mod enum_key_map {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Status(i32);

        impl Status {
            pub const ACTIVE: Self = Self(1);
            pub const INACTIVE: Self = Self(2);
            pub const PENDING: Self = Self(3);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(1) => ::std::string::String::from("ACTIVE"),
                    Self(2) => ::std::string::String::from("INACTIVE"),
                    Self(3) => ::std::string::String::from("PENDING"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    1 => Some(Self::ACTIVE),
                    2 => Some(Self::INACTIVE),
                    3 => Some(Self::PENDING),
                    _ => None,
                }
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

        impl ::pilota::thrift::Message for Status {
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
                            format!("invalid enum value for Status, value: {}", value),
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
                                format!("invalid enum value for Status, value: {}", value),
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
        pub struct Item {
            pub id: i64,

            pub name: ::pilota::FastStr,
            pub _field_mask: ::std::option::Option<::pilota_thrift_fieldmask::FieldMask>,
        }
        impl ::pilota::thrift::Message for Item {
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
                        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Item" };
                        __protocol.write_struct_begin(&struct_ident)?;
                        let (field_fm, _) = struct_fm.field(1);
                        __protocol.write_i64_field(1, *&self.id)?;
                        let (field_fm, _) = struct_fm.field(2);
                        __protocol.write_faststr_field(2, (&self.name).clone())?;
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }
                } else {
                    let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Item" };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_i64_field(1, *&self.id)?;
                    __protocol.write_faststr_field(2, (&self.name).clone())?;
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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                                var_1 = Some(__protocol.read_i64()?);
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
                            "decode struct `Item` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field id is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };

                let data = Self {
                    id: var_1,
                    name: var_2,
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
                                    if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                                {
                                    var_1 = Some(__protocol.read_i64().await?);
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
                                "decode struct `Item` field(#{}) failed, caused by: ",
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
                                "field id is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        id: var_1,
                        name: var_2,
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
                        __protocol
                            .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Item" })
                            + {
                                let (field_fm, exist) = struct_fm.field(1);
                                if exist {
                                    __protocol.i64_field_len(Some(1), *&self.id)
                                } else {
                                    0
                                }
                            }
                            + {
                                let (field_fm, exist) = struct_fm.field(2);
                                if exist {
                                    __protocol.faststr_field_len(Some(2), &self.name)
                                } else {
                                    0
                                }
                            }
                            + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                } else {
                    __protocol
                        .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Item" })
                        + __protocol.i64_field_len(Some(1), *&self.id)
                        + __protocol.faststr_field_len(Some(2), &self.name)
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
        }
        impl Item {
            pub fn set_field_mask(&mut self, field_mask: ::pilota_thrift_fieldmask::FieldMask) {
                self._field_mask = Some(field_mask.clone());
            }
        }

        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct EnumKeyMapTest {
            pub status_map: ::pilota::AHashMap<Status, ::pilota::FastStr>,

            pub status_item_map: ::pilota::AHashMap<Status, Item>,

            pub status_list_map:
                ::std::option::Option<::pilota::AHashMap<Status, ::std::vec::Vec<Item>>>,
            pub _field_mask: ::std::option::Option<::pilota_thrift_fieldmask::FieldMask>,
        }
        impl ::pilota::thrift::Message for EnumKeyMapTest {
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
                            name: "EnumKeyMapTest",
                        };
                        __protocol.write_struct_begin(&struct_ident)?;
                        let (field_fm, _) = struct_fm.field(1);
                        if let Some(map_fm) = field_fm {
                            __protocol.write_field_begin(::pilota::thrift::TType::Map, 1)?;
                            __protocol.write_map_begin(::pilota::thrift::TMapIdentifier {
                                key_type: ::pilota::thrift::TType::I32,
                                value_type: ::pilota::thrift::TType::Binary,
                                size: (&self.status_map)
                                    .keys()
                                    .filter(|key| map_fm.int(key.inner() as i32).1)
                                    .count(),
                            })?;
                            for (key, val) in &self.status_map {
                                let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                if is_exist {
                                    __protocol.write_struct(key)?;
                                    __protocol.write_faststr((val).clone())?;
                                }
                            }
                            __protocol.write_map_end()?;
                            __protocol.write_field_end()?;
                        } else {
                            __protocol.write_map_field(
                                1,
                                ::pilota::thrift::TType::I32,
                                ::pilota::thrift::TType::Binary,
                                &&self.status_map,
                                |__protocol, key| {
                                    __protocol.write_struct(key)?;
                                    ::std::result::Result::Ok(())
                                },
                                |__protocol, val| {
                                    __protocol.write_faststr((val).clone())?;
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
                                size: (&self.status_item_map)
                                    .keys()
                                    .filter(|key| map_fm.int(key.inner() as i32).1)
                                    .count(),
                            })?;
                            for (key, val) in &self.status_item_map {
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
                                &&self.status_item_map,
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
                        if let Some(value) = self.status_list_map.as_ref() {
                            let (field_fm, exist) = struct_fm.field(3);
                            if exist {
                                if let Some(map_fm) = field_fm {
                                    __protocol
                                        .write_field_begin(::pilota::thrift::TType::Map, 3)?;
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
                                        3,
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
                        name: "EnumKeyMapTest",
                    };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_map_field(
                        1,
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::Binary,
                        &&self.status_map,
                        |__protocol, key| {
                            __protocol.write_struct(key)?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    __protocol.write_map_field(
                        2,
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::Struct,
                        &&self.status_item_map,
                        |__protocol, key| {
                            __protocol.write_struct(key)?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_struct(val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    if let Some(value) = self.status_list_map.as_ref() {
                        __protocol.write_map_field(
                            3,
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
                                            __protocol.read_faststr()?,
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
                                            unsafe {
                                                let list_ident = __protocol.read_list_begin()?;
                                                let mut val: ::std::vec::Vec<Item> =
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
                            "decode struct `EnumKeyMapTest` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field status_map is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field status_item_map is required".to_string(),
                    ));
                };

                let data = Self {
                    status_map: var_1,
                    status_item_map: var_2,
                    status_list_map: var_3,
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
                            val.insert(<Status as ::pilota::thrift::Message>::decode_async(__protocol).await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_2 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(<Status as ::pilota::thrift::Message>::decode_async(__protocol).await?, <Item as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_3 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(<Status as ::pilota::thrift::Message>::decode_async(__protocol).await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<Item as ::pilota::thrift::Message>::decode_async(__protocol).await?);
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
                    err.prepend_msg(&format!("decode struct `EnumKeyMapTest` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field status_map is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field status_item_map is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        status_map: var_1,
                        status_item_map: var_2,
                        status_list_map: var_3,
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
                            name: "EnumKeyMapTest",
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
                                                value_type: ::pilota::thrift::TType::Binary,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in &self.status_map {
                                        let (item_fm, is_exist) = map_fm.int(key.inner() as i32);
                                        if is_exist {
                                            size += __protocol.struct_len(key);
                                            size += __protocol.faststr_len(val);
                                        }
                                    }
                                    size
                                } else {
                                    __protocol.map_field_len(
                                        Some(1),
                                        ::pilota::thrift::TType::I32,
                                        ::pilota::thrift::TType::Binary,
                                        &self.status_map,
                                        |__protocol, key| __protocol.struct_len(key),
                                        |__protocol, val| __protocol.faststr_len(val),
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
                                    for (key, val) in &self.status_item_map {
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
                                        &self.status_item_map,
                                        |__protocol, key| __protocol.struct_len(key),
                                        |__protocol, val| __protocol.struct_len(val),
                                    )
                                }
                            } else {
                                0
                            }
                        } + self.status_list_map.as_ref().map_or(0, |value| {
                            let (field_fm, exist) = struct_fm.field(3);
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
                                        Some(3),
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
                        name: "EnumKeyMapTest",
                    }) + __protocol.map_field_len(
                        Some(1),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::Binary,
                        &self.status_map,
                        |__protocol, key| __protocol.struct_len(key),
                        |__protocol, val| __protocol.faststr_len(val),
                    ) + __protocol.map_field_len(
                        Some(2),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::Struct,
                        &self.status_item_map,
                        |__protocol, key| __protocol.struct_len(key),
                        |__protocol, val| __protocol.struct_len(val),
                    ) + self.status_list_map.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(3),
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
        impl EnumKeyMapTest {
            pub fn set_field_mask(&mut self, field_mask: ::pilota_thrift_fieldmask::FieldMask) {
                self._field_mask = Some(field_mask.clone());
                if let Some(map_mask) = field_mask.field(2).0 {
                    if !map_mask.all() {
                        for (key, item) in self.status_item_map.iter_mut() {
                            if let Some(item_fm) = map_mask.int(key.inner() as i32).0 {
                                item.set_field_mask(item_fm.clone());
                            }
                        }
                    }
                }
                if let Some(value) = &mut self.status_list_map {
                    if let Some(map_mask) = field_mask.field(3).0 {
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
