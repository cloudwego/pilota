pub mod struct_init_with_field_mask {
    #![allow(warnings, clippy::all)]

    pub mod struct_init_with_field_mask {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Item {
            pub id: i64,

            pub title: ::pilota::FastStr,
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
                        __protocol.write_faststr_field(2, (&self.title).clone())?;
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }
                } else {
                    let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Item" };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_i64_field(1, *&self.id)?;
                    __protocol.write_faststr_field(2, (&self.title).clone())?;
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
                        "field title is required".to_string(),
                    ));
                };

                let data = Self {
                    id: var_1,
                    title: var_2,
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
                                "field title is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        id: var_1,
                        title: var_2,
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
                                    __protocol.faststr_field_len(Some(2), &self.title)
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
                        + __protocol.faststr_field_len(Some(2), &self.title)
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
        }
        impl Item {
            pub fn get_descriptor()
            -> Option<&'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor>
            {
                let file_descriptor = get_file_descriptor_struct_init_with_field_mask();
                file_descriptor.find_struct_by_name("Item")
            }

            pub fn set_field_mask(&mut self, field_mask: ::pilota_thrift_fieldmask::FieldMask) {
                self._field_mask = Some(field_mask.clone());
            }
        }
        pub const DEFAULT_ITEM: Item = Item {
            id: 1i64,
            title: ::pilota::FastStr::from_static_str("a"),
            _field_mask: ::std::option::Option::None,
        };
        pub static DEFAULT_ITEM_MAP: ::std::sync::LazyLock<::pilota::AHashMap<&'static str, Item>> =
            ::std::sync::LazyLock::new(|| {
                let mut map = ::pilota::AHashMap::with_capacity(1);
                map.insert(
                    "a",
                    Item {
                        id: 1i64,
                        title: ::pilota::FastStr::from_static_str("a"),
                        _field_mask: ::std::option::Option::None,
                    },
                );
                map
            });

        impl ::std::default::Default for GetItemRequest {
            fn default() -> Self {
                GetItemRequest {
                    id: ::std::default::Default::default(),
                    item_opt: Some(DEFAULT_ITEM),
                    item_opt2: Some(Item {
                        id: 1i64,
                        title: ::pilota::FastStr::from_static_str("a"),
                        _field_mask: ::std::option::Option::None,
                    }),
                    test_map: ::std::default::Default::default(),
                    test_map2: ::std::default::Default::default(),
                    _field_mask: ::std::option::Option::None,
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetItemRequest {
            pub id: i64,

            pub item_opt: ::std::option::Option<Item>,

            pub item_opt2: ::std::option::Option<Item>,

            pub test_map: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,

            pub test_map2: ::pilota::AHashMap<i64, ::pilota::FastStr>,
            pub _field_mask: ::std::option::Option<::pilota_thrift_fieldmask::FieldMask>,
        }
        impl ::pilota::thrift::Message for GetItemRequest {
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
                            name: "GetItemRequest",
                        };
                        __protocol.write_struct_begin(&struct_ident)?;
                        let (field_fm, _) = struct_fm.field(1);
                        __protocol.write_i64_field(1, *&self.id)?;
                        if let Some(value) = self.item_opt.as_ref() {
                            let (field_fm, exist) = struct_fm.field(2);
                            if exist {
                                __protocol.write_struct_field(
                                    2,
                                    value,
                                    ::pilota::thrift::TType::Struct,
                                )?;
                            }
                        }
                        if let Some(value) = self.item_opt2.as_ref() {
                            let (field_fm, exist) = struct_fm.field(3);
                            if exist {
                                __protocol.write_struct_field(
                                    3,
                                    value,
                                    ::pilota::thrift::TType::Struct,
                                )?;
                            }
                        }
                        let (field_fm, _) = struct_fm.field(4);
                        if let Some(map_fm) = field_fm {
                            __protocol.write_field_begin(::pilota::thrift::TType::Map, 4)?;
                            __protocol.write_map_begin(::pilota::thrift::TMapIdentifier {
                                key_type: ::pilota::thrift::TType::Binary,
                                value_type: ::pilota::thrift::TType::Binary,
                                size: (&self.test_map)
                                    .keys()
                                    .filter(|key| map_fm.str(key).1)
                                    .count(),
                            })?;
                            for (key, val) in &self.test_map {
                                let (item_fm, is_exist) = map_fm.str(key);
                                if is_exist {
                                    __protocol.write_faststr((key).clone())?;
                                    __protocol.write_faststr((val).clone())?;
                                }
                            }
                            __protocol.write_map_end()?;
                            __protocol.write_field_end()?;
                        } else {
                            __protocol.write_map_field(
                                4,
                                ::pilota::thrift::TType::Binary,
                                ::pilota::thrift::TType::Binary,
                                &&self.test_map,
                                |__protocol, key| {
                                    __protocol.write_faststr((key).clone())?;
                                    ::std::result::Result::Ok(())
                                },
                                |__protocol, val| {
                                    __protocol.write_faststr((val).clone())?;
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                        }
                        let (field_fm, _) = struct_fm.field(5);
                        if let Some(map_fm) = field_fm {
                            __protocol.write_field_begin(::pilota::thrift::TType::Map, 5)?;
                            __protocol.write_map_begin(::pilota::thrift::TMapIdentifier {
                                key_type: ::pilota::thrift::TType::I64,
                                value_type: ::pilota::thrift::TType::Binary,
                                size: (&self.test_map2)
                                    .keys()
                                    .filter(|key| map_fm.int(**key as i32).1)
                                    .count(),
                            })?;
                            for (key, val) in &self.test_map2 {
                                let (item_fm, is_exist) = map_fm.int(*key as i32);
                                if is_exist {
                                    __protocol.write_i64(*key)?;
                                    __protocol.write_faststr((val).clone())?;
                                }
                            }
                            __protocol.write_map_end()?;
                            __protocol.write_field_end()?;
                        } else {
                            __protocol.write_map_field(
                                5,
                                ::pilota::thrift::TType::I64,
                                ::pilota::thrift::TType::Binary,
                                &&self.test_map2,
                                |__protocol, key| {
                                    __protocol.write_i64(*key)?;
                                    ::std::result::Result::Ok(())
                                },
                                |__protocol, val| {
                                    __protocol.write_faststr((val).clone())?;
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                        }
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }
                } else {
                    let struct_ident = ::pilota::thrift::TStructIdentifier {
                        name: "GetItemRequest",
                    };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_i64_field(1, *&self.id)?;
                    if let Some(value) = self.item_opt.as_ref() {
                        __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
                    }
                    if let Some(value) = self.item_opt2.as_ref() {
                        __protocol.write_struct_field(3, value, ::pilota::thrift::TType::Struct)?;
                    }
                    __protocol.write_map_field(
                        4,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &&self.test_map,
                        |__protocol, key| {
                            __protocol.write_faststr((key).clone())?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    __protocol.write_map_field(
                        5,
                        ::pilota::thrift::TType::I64,
                        ::pilota::thrift::TType::Binary,
                        &&self.test_map2,
                        |__protocol, key| {
                            __protocol.write_i64(*key)?;
                            ::std::result::Result::Ok(())
                        },
                        |__protocol, val| {
                            __protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
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
                let mut var_2 = Some(DEFAULT_ITEM);
                let mut var_3 = Some(Item {
                    id: 1i64,
                    title: ::pilota::FastStr::from_static_str("a"),
                    _field_mask: ::std::option::Option::None,
                });
                let mut var_4 = None;
                let mut var_5 = None;

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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_2 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(3)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_3 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_4 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_5 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_i64()?,
                                            __protocol.read_faststr()?,
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
                            "decode struct `GetItemRequest` field(#{}) failed, caused by: ",
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
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field test_map is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field test_map2 is required".to_string(),
                    ));
                };

                let data = Self {
                    id: var_1,
                    item_opt: var_2,
                    item_opt2: var_3,
                    test_map: var_4,
                    test_map2: var_5,
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
                    let mut var_2 = Some(DEFAULT_ITEM);
                    let mut var_3 = Some(Item {
                        id: 1i64,
                        title: ::pilota::FastStr::from_static_str("a"),
                        _field_mask: ::std::option::Option::None,
                    });
                    let mut var_4 = None;
                    let mut var_5 = None;

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
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_2 = Some(
                                        <Item as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                Some(3)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_3 = Some(
                                        <Item as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                Some(4)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_4 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val =
                                            ::pilota::AHashMap::with_capacity(map_ident.size);
                                        for _ in 0..map_ident.size {
                                            val.insert(
                                                __protocol.read_faststr().await?,
                                                __protocol.read_faststr().await?,
                                            );
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
                                }
                                Some(5)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_5 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val =
                                            ::pilota::AHashMap::with_capacity(map_ident.size);
                                        for _ in 0..map_ident.size {
                                            val.insert(
                                                __protocol.read_i64().await?,
                                                __protocol.read_faststr().await?,
                                            );
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
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
                                "decode struct `GetItemRequest` field(#{}) failed, caused by: ",
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
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field test_map is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field test_map2 is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        id: var_1,
                        item_opt: var_2,
                        item_opt2: var_3,
                        test_map: var_4,
                        test_map2: var_5,
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
                            name: "GetItemRequest",
                        }) + {
                            let (field_fm, exist) = struct_fm.field(1);
                            if exist {
                                __protocol.i64_field_len(Some(1), *&self.id)
                            } else {
                                0
                            }
                        } + self.item_opt.as_ref().map_or(0, |value| {
                            let (field_fm, exist) = struct_fm.field(2);
                            if exist {
                                __protocol.struct_field_len(Some(2), value)
                            } else {
                                0
                            }
                        }) + self.item_opt2.as_ref().map_or(0, |value| {
                            let (field_fm, exist) = struct_fm.field(3);
                            if exist {
                                __protocol.struct_field_len(Some(3), value)
                            } else {
                                0
                            }
                        }) + {
                            let (field_fm, exist) = struct_fm.field(4);
                            if exist {
                                if let Some(map_fm) = field_fm {
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::Map, None)
                                        + __protocol.field_end_len()
                                        + __protocol.map_begin_len(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::Binary,
                                                value_type: ::pilota::thrift::TType::Binary,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in &self.test_map {
                                        let (item_fm, exist) = map_fm.str(key);
                                        if exist {
                                            size += __protocol.faststr_len(key);
                                            size += __protocol.faststr_len(val);
                                        }
                                    }
                                    size
                                } else {
                                    __protocol.map_field_len(
                                        Some(4),
                                        ::pilota::thrift::TType::Binary,
                                        ::pilota::thrift::TType::Binary,
                                        &self.test_map,
                                        |__protocol, key| __protocol.faststr_len(key),
                                        |__protocol, val| __protocol.faststr_len(val),
                                    )
                                }
                            } else {
                                0
                            }
                        } + {
                            let (field_fm, exist) = struct_fm.field(5);
                            if exist {
                                if let Some(map_fm) = field_fm {
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::Map, None)
                                        + __protocol.field_end_len()
                                        + __protocol.map_begin_len(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::I64,
                                                value_type: ::pilota::thrift::TType::Binary,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in &self.test_map2 {
                                        let (item_fm, is_exist) = map_fm.int(*key as i32);
                                        if is_exist {
                                            size += __protocol.i64_len(*key);
                                            size += __protocol.faststr_len(val);
                                        }
                                    }
                                    size
                                } else {
                                    __protocol.map_field_len(
                                        Some(5),
                                        ::pilota::thrift::TType::I64,
                                        ::pilota::thrift::TType::Binary,
                                        &self.test_map2,
                                        |__protocol, key| __protocol.i64_len(*key),
                                        |__protocol, val| __protocol.faststr_len(val),
                                    )
                                }
                            } else {
                                0
                            }
                        } + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                } else {
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                        name: "GetItemRequest",
                    }) + __protocol.i64_field_len(Some(1), *&self.id)
                        + self
                            .item_opt
                            .as_ref()
                            .map_or(0, |value| __protocol.struct_field_len(Some(2), value))
                        + self
                            .item_opt2
                            .as_ref()
                            .map_or(0, |value| __protocol.struct_field_len(Some(3), value))
                        + __protocol.map_field_len(
                            Some(4),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            &self.test_map,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                        + __protocol.map_field_len(
                            Some(5),
                            ::pilota::thrift::TType::I64,
                            ::pilota::thrift::TType::Binary,
                            &self.test_map2,
                            |__protocol, key| __protocol.i64_len(*key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
        }
        impl GetItemRequest {
            pub fn get_descriptor()
            -> Option<&'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor>
            {
                let file_descriptor = get_file_descriptor_struct_init_with_field_mask();
                file_descriptor.find_struct_by_name("GetItemRequest")
            }

            pub fn set_field_mask(&mut self, field_mask: ::pilota_thrift_fieldmask::FieldMask) {
                self._field_mask = Some(field_mask.clone());
                if let Some(value) = &mut self.item_opt {
                    if let Some(fm) = field_mask.field(2).0 {
                        value.set_field_mask(fm.clone());
                    }
                }
                if let Some(value) = &mut self.item_opt2 {
                    if let Some(fm) = field_mask.field(3).0 {
                        value.set_field_mask(fm.clone());
                    }
                }
            }
        }
    }
}
