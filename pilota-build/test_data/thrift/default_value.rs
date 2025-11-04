pub mod default_value {
    #![allow(warnings, clippy::all)]

    pub mod default_value {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct B(i32);

        impl B {
            pub const READ: Self = Self(1);
            pub const WRITE: Self = Self(2);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(1) => ::std::string::String::from("READ"),
                    Self(2) => ::std::string::String::from("WRITE"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    1 => Some(Self::READ),
                    2 => Some(Self::WRITE),
                    _ => None,
                }
            }
        }

        impl ::std::convert::From<i32> for B {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<B> for i32 {
            fn from(value: B) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for B {
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
                            format!("invalid enum value for B, value: {}", value),
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
                                format!("invalid enum value for B, value: {}", value),
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

        impl ::std::default::Default for A {
            fn default() -> Self {
                A {
                    faststr: ::pilota::FastStr::from_static_str("hello world"),
                    string: "test".to_string(),
                    a: Some(false),
                    test_b: Some(B::READ),
                    test_b2: Some(B::WRITE),
                    test_b3: Some((B::READ.inner() as i8)),
                    map: Some({
                        let mut map = ::pilota::AHashMap::with_capacity(1);
                        map.insert(
                            ::pilota::FastStr::from_static_str("hello"),
                            ::pilota::FastStr::from_static_str("world"),
                        );
                        map
                    }),
                    test_double: Some(1f64),
                    test_double2: Some(1.2f64),
                    alias_str: Some(::pilota::FastStr::from_static_str(A_S)),
                    empty: ::pilota::Bytes::from_static("".as_bytes()),
                    test_map: {
                        let mut map = ::pilota::AHashMap::with_capacity(1);
                        map.insert(::pilota::OrderedFloat(1f64), 2f64);
                        map
                    },
                    test_set: ::pilota::AHashSet::from([::pilota::OrderedFloat(1f64)]),
                    a2: Some(true),
                    map2: Some(::pilota::AHashMap::new()),
                    commit_ids_raw: Some({
                        (DEFAULT_COMMIT_IDS.clone())
                            .0
                            .iter()
                            .map(|el| (el.clone()).0)
                            .collect::<::std::vec::Vec<_>>()
                    }),
                    commit_ids: Some(CommitIdList(::std::vec![CommitId(3i32), CommitId(4i32)])),
                    default_i16: Some(16i16),
                    default_i64: Some(64i64),
                    list_literal: Some(::std::vec![5i32, 6i32, 7i32]),
                    set_empty: Some(::pilota::AHashSet::from([])),
                    set_from_const: Some(INT_SET_CONST.clone()),
                    btree_set_literal: Some(::std::collections::BTreeSet::from([4i32, 5i32])),
                    btree_set_empty: Some(::std::collections::BTreeSet::from([])),
                    btree_set_from_const: Some(INT_BTREE_SET_CONST.clone()),
                    map_literal_i32: Some({
                        let mut map = ::pilota::AHashMap::with_capacity(2);
                        map.insert(::pilota::FastStr::from_static_str("one"), 1i32);
                        map.insert(::pilota::FastStr::from_static_str("two"), 2i32);
                        map
                    }),
                    map_from_const: Some({
                        STR_I32_MAP
                            .clone()
                            .iter()
                            .map(|(k, v)| {
                                (::pilota::FastStr::from_static_str(k.clone()), v.clone())
                            })
                            .collect::<::pilota::AHashMap<_, _>>()
                    }),
                    btree_map_literal: Some({
                        let mut map = ::std::collections::BTreeMap::new();
                        map.insert(::pilota::FastStr::from_static_str("three"), 3i32);
                        map
                    }),
                    btree_map_empty: Some(::std::collections::BTreeMap::new()),
                    btree_map_from_const: Some({
                        STR_I32_BTREE_MAP
                            .clone()
                            .iter()
                            .map(|(k, v)| {
                                (::pilota::FastStr::from_static_str(k.clone()), v.clone())
                            })
                            .collect::<::std::collections::BTreeMap<_, _>>()
                    }),
                    struct_literal: Some(C {
                        off: Some(::pilota::FastStr::from_static_str("nested")),
                        test_byte: Some(7i8),
                    }),
                    struct_partial: Some(C {
                        off: Some(::pilota::FastStr::from_static_str("partial")),
                        test_byte: None,
                    }),
                    struct_from_const: Some(DEFAULT_C),
                    binary_from_const: Some(DEFAULT_BINARY),
                    newtype_map_literal: Some(NameScoreMap({
                        let mut map = ::pilota::AHashMap::with_capacity(1);
                        map.insert(
                            NameId(::pilota::FastStr::from_static_str("carol")),
                            Score(5i64),
                        );
                        map
                    })),
                    newtype_map_from_const: Some(NameScoreMap((NAME_SCORE_LITERAL.clone()).0)),
                    newtype_map_empty: Some(NameScoreMap(::pilota::AHashMap::new())),
                    newtype_map_from_empty_const: Some(NameScoreMap((NAME_SCORE_EMPTY.clone()).0)),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct A {
            pub faststr: ::pilota::FastStr,

            pub string: ::std::string::String,

            pub a: ::std::option::Option<bool>,

            pub test_b: ::std::option::Option<B>,

            pub test_b2: ::std::option::Option<B>,

            pub test_b3: ::std::option::Option<i8>,

            pub map:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,

            pub test_double: ::std::option::Option<f64>,

            pub test_double2: ::std::option::Option<f64>,

            pub alias_str: ::std::option::Option<::pilota::FastStr>,

            pub empty: ::pilota::Bytes,

            pub test_map: ::pilota::AHashMap<::pilota::OrderedFloat<f64>, f64>,

            pub test_set: ::pilota::AHashSet<::pilota::OrderedFloat<f64>>,

            pub a2: ::std::option::Option<bool>,

            pub map2:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,

            pub commit_ids_raw: ::std::option::Option<::std::vec::Vec<i32>>,

            pub commit_ids: ::std::option::Option<CommitIdList>,

            pub default_i16: ::std::option::Option<i16>,

            pub default_i64: ::std::option::Option<i64>,

            pub list_literal: ::std::option::Option<::std::vec::Vec<i32>>,

            pub set_empty: ::std::option::Option<::pilota::AHashSet<i32>>,

            pub set_from_const: ::std::option::Option<::pilota::AHashSet<i32>>,

            pub btree_set_literal: ::std::option::Option<::std::collections::BTreeSet<i32>>,

            pub btree_set_empty: ::std::option::Option<::std::collections::BTreeSet<i32>>,

            pub btree_set_from_const: ::std::option::Option<::std::collections::BTreeSet<i32>>,

            pub map_literal_i32: ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, i32>>,

            pub map_from_const: ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, i32>>,

            pub btree_map_literal:
                ::std::option::Option<::std::collections::BTreeMap<::pilota::FastStr, i32>>,

            pub btree_map_empty:
                ::std::option::Option<::std::collections::BTreeMap<::pilota::FastStr, i32>>,

            pub btree_map_from_const:
                ::std::option::Option<::std::collections::BTreeMap<::pilota::FastStr, i32>>,

            pub struct_literal: ::std::option::Option<C>,

            pub struct_partial: ::std::option::Option<C>,

            pub struct_from_const: ::std::option::Option<C>,

            pub binary_from_const: ::std::option::Option<::pilota::Bytes>,

            pub newtype_map_literal: ::std::option::Option<NameScoreMap>,

            pub newtype_map_from_const: ::std::option::Option<NameScoreMap>,

            pub newtype_map_empty: ::std::option::Option<NameScoreMap>,

            pub newtype_map_from_empty_const: ::std::option::Option<NameScoreMap>,
        }
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_faststr_field(1, (&self.faststr).clone())?;
                __protocol.write_string_field(2, &self.string)?;
                if let Some(value) = self.a.as_ref() {
                    __protocol.write_bool_field(3, *value)?;
                }
                if let Some(value) = self.test_b.as_ref() {
                    __protocol.write_i32_field(4, (value).inner())?;
                }
                if let Some(value) = self.test_b2.as_ref() {
                    __protocol.write_i32_field(5, (value).inner())?;
                }
                if let Some(value) = self.test_b3.as_ref() {
                    __protocol.write_i8_field(6, *value)?;
                }
                if let Some(value) = self.map.as_ref() {
                    __protocol.write_map_field(
                        7,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
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
                if let Some(value) = self.test_double.as_ref() {
                    __protocol.write_double_field(8, *value)?;
                }
                if let Some(value) = self.test_double2.as_ref() {
                    __protocol.write_double_field(9, *value)?;
                }
                if let Some(value) = self.alias_str.as_ref() {
                    __protocol.write_faststr_field(10, (value).clone())?;
                }
                __protocol.write_bytes_field(11, (&self.empty).clone())?;
                __protocol.write_map_field(
                    12,
                    ::pilota::thrift::TType::Double,
                    ::pilota::thrift::TType::Double,
                    &&self.test_map,
                    |__protocol, key| {
                        __protocol.write_double(key.0)?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_double(*val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_set_field(
                    13,
                    ::pilota::thrift::TType::Double,
                    &&self.test_set,
                    |__protocol, val| {
                        __protocol.write_double(val.0)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                if let Some(value) = self.a2.as_ref() {
                    __protocol.write_bool_field(14, *value)?;
                }
                if let Some(value) = self.map2.as_ref() {
                    __protocol.write_map_field(
                        15,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::Binary,
                        &value,
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
                if let Some(value) = self.commit_ids_raw.as_ref() {
                    __protocol.write_list_field(
                        16,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let Some(value) = self.commit_ids.as_ref() {
                    __protocol.write_struct_field(17, value, ::pilota::thrift::TType::List)?;
                }
                if let Some(value) = self.default_i16.as_ref() {
                    __protocol.write_i16_field(18, *value)?;
                }
                if let Some(value) = self.default_i64.as_ref() {
                    __protocol.write_i64_field(19, *value)?;
                }
                if let Some(value) = self.list_literal.as_ref() {
                    __protocol.write_list_field(
                        20,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let Some(value) = self.set_empty.as_ref() {
                    __protocol.write_set_field(
                        21,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let Some(value) = self.set_from_const.as_ref() {
                    __protocol.write_set_field(
                        22,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let Some(value) = self.btree_set_literal.as_ref() {
                    __protocol.write_btree_set_field(
                        23,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let Some(value) = self.btree_set_empty.as_ref() {
                    __protocol.write_btree_set_field(
                        24,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let Some(value) = self.btree_set_from_const.as_ref() {
                    __protocol.write_btree_set_field(
                        25,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let Some(value) = self.map_literal_i32.as_ref() {
                    __protocol.write_map_field(
                        26,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::I32,
                        &value,
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
                if let Some(value) = self.map_from_const.as_ref() {
                    __protocol.write_map_field(
                        27,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::I32,
                        &value,
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
                if let Some(value) = self.btree_map_literal.as_ref() {
                    __protocol.write_btree_map_field(
                        28,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::I32,
                        &value,
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
                if let Some(value) = self.btree_map_empty.as_ref() {
                    __protocol.write_btree_map_field(
                        29,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::I32,
                        &value,
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
                if let Some(value) = self.btree_map_from_const.as_ref() {
                    __protocol.write_btree_map_field(
                        30,
                        ::pilota::thrift::TType::Binary,
                        ::pilota::thrift::TType::I32,
                        &value,
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
                if let Some(value) = self.struct_literal.as_ref() {
                    __protocol.write_struct_field(31, value, ::pilota::thrift::TType::Struct)?;
                }
                if let Some(value) = self.struct_partial.as_ref() {
                    __protocol.write_struct_field(32, value, ::pilota::thrift::TType::Struct)?;
                }
                if let Some(value) = self.struct_from_const.as_ref() {
                    __protocol.write_struct_field(33, value, ::pilota::thrift::TType::Struct)?;
                }
                if let Some(value) = self.binary_from_const.as_ref() {
                    __protocol.write_bytes_field(34, (value).clone())?;
                }
                if let Some(value) = self.newtype_map_literal.as_ref() {
                    __protocol.write_struct_field(35, value, ::pilota::thrift::TType::Map)?;
                }
                if let Some(value) = self.newtype_map_from_const.as_ref() {
                    __protocol.write_struct_field(36, value, ::pilota::thrift::TType::Map)?;
                }
                if let Some(value) = self.newtype_map_empty.as_ref() {
                    __protocol.write_struct_field(37, value, ::pilota::thrift::TType::Map)?;
                }
                if let Some(value) = self.newtype_map_from_empty_const.as_ref() {
                    __protocol.write_struct_field(38, value, ::pilota::thrift::TType::Map)?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::pilota::FastStr::from_static_str("hello world");
                let mut var_2 = None;
                let mut var_3 = Some(false);
                let mut var_4 = Some(B::READ);
                let mut var_5 = Some(B::WRITE);
                let mut var_6 = Some((B::READ.inner() as i8));
                let mut var_7 = None;
                let mut var_8 = Some(1f64);
                let mut var_9 = Some(1.2f64);
                let mut var_10 = Some(::pilota::FastStr::from_static_str(A_S));
                let mut var_11 = ::pilota::Bytes::from_static("".as_bytes());
                let mut var_12 = None;
                let mut var_13 = None;
                let mut var_14 = Some(true);
                let mut var_15 = None;
                let mut var_16 = None;
                let mut var_17 = None;
                let mut var_18 = Some(16i16);
                let mut var_19 = Some(64i64);
                let mut var_20 = None;
                let mut var_21 = None;
                let mut var_22 = None;
                let mut var_23 = None;
                let mut var_24 = None;
                let mut var_25 = None;
                let mut var_26 = None;
                let mut var_27 = None;
                let mut var_28 = None;
                let mut var_29 = None;
                let mut var_30 = None;
                let mut var_31 = Some(C {
                    off: Some(::pilota::FastStr::from_static_str("nested")),
                    test_byte: Some(7i8),
                });
                let mut var_32 = Some(C {
                    off: Some(::pilota::FastStr::from_static_str("partial")),
                    test_byte: None,
                });
                let mut var_33 = Some(DEFAULT_C);
                let mut var_34 = Some(DEFAULT_BINARY);
                let mut var_35 = None;
                let mut var_36 = None;
                let mut var_37 = None;
                let mut var_38 = None;

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
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_1 = __protocol.read_faststr()?;
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_2 = Some(__protocol.read_string()?);
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                                var_3 = Some(__protocol.read_bool()?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                var_4 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                var_5 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(6) if field_ident.field_type == ::pilota::thrift::TType::I8 => {
                                var_6 = Some(__protocol.read_i8()?);
                            }
                            Some(7) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_7 = Some({
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
                            Some(8)
                                if field_ident.field_type == ::pilota::thrift::TType::Double =>
                            {
                                var_8 = Some(__protocol.read_double()?);
                            }
                            Some(9)
                                if field_ident.field_type == ::pilota::thrift::TType::Double =>
                            {
                                var_9 = Some(__protocol.read_double()?);
                            }
                            Some(10)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_10 = Some(__protocol.read_faststr()?);
                            }
                            Some(11)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_11 = __protocol.read_bytes()?;
                            }
                            Some(12) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_12 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            ::pilota::OrderedFloat(__protocol.read_double()?),
                                            __protocol.read_double()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(13) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                var_13 = Some({
                                    let list_ident = __protocol.read_set_begin()?;
                                    let mut val =
                                        ::pilota::AHashSet::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.insert(::pilota::OrderedFloat(
                                            __protocol.read_double()?,
                                        ));
                                    }
                                    __protocol.read_set_end()?;
                                    val
                                });
                            }
                            Some(14) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                                var_14 = Some(__protocol.read_bool()?);
                            }
                            Some(15) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_15 = Some({
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
                            Some(16) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_16 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<i32> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(__protocol.read_i32()?);
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                });
                            }
                            Some(17) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_17 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(18) if field_ident.field_type == ::pilota::thrift::TType::I16 => {
                                var_18 = Some(__protocol.read_i16()?);
                            }
                            Some(19) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                                var_19 = Some(__protocol.read_i64()?);
                            }
                            Some(20) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_20 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<i32> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(__protocol.read_i32()?);
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                });
                            }
                            Some(21) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                var_21 = Some({
                                    let list_ident = __protocol.read_set_begin()?;
                                    let mut val =
                                        ::pilota::AHashSet::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.insert(__protocol.read_i32()?);
                                    }
                                    __protocol.read_set_end()?;
                                    val
                                });
                            }
                            Some(22) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                var_22 = Some({
                                    let list_ident = __protocol.read_set_begin()?;
                                    let mut val =
                                        ::pilota::AHashSet::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.insert(__protocol.read_i32()?);
                                    }
                                    __protocol.read_set_end()?;
                                    val
                                });
                            }
                            Some(23) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                var_23 = Some({
                                    let list_ident = __protocol.read_set_begin()?;
                                    let mut val = ::std::collections::BTreeSet::new();
                                    for _ in 0..list_ident.size {
                                        val.insert(__protocol.read_i32()?);
                                    }
                                    __protocol.read_set_end()?;
                                    val
                                });
                            }
                            Some(24) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                var_24 = Some({
                                    let list_ident = __protocol.read_set_begin()?;
                                    let mut val = ::std::collections::BTreeSet::new();
                                    for _ in 0..list_ident.size {
                                        val.insert(__protocol.read_i32()?);
                                    }
                                    __protocol.read_set_end()?;
                                    val
                                });
                            }
                            Some(25) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                var_25 = Some({
                                    let list_ident = __protocol.read_set_begin()?;
                                    let mut val = ::std::collections::BTreeSet::new();
                                    for _ in 0..list_ident.size {
                                        val.insert(__protocol.read_i32()?);
                                    }
                                    __protocol.read_set_end()?;
                                    val
                                });
                            }
                            Some(26) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_26 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_i32()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(27) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_27 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_i32()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(28) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_28 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::std::collections::BTreeMap::new();
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_i32()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(29) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_29 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::std::collections::BTreeMap::new();
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_i32()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(30) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_30 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::std::collections::BTreeMap::new();
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            __protocol.read_i32()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(31)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_31 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(32)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_32 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(33)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_33 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(34)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_34 = Some(__protocol.read_bytes()?);
                            }
                            Some(35) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_35 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(36) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_36 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(37) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_37 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(38) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_38 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `A` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let var_2 = var_2.unwrap_or_else(|| "test".to_string());
                if var_7.is_none() {
                    var_7 = Some({
                        let mut map = ::pilota::AHashMap::with_capacity(1);
                        map.insert(
                            ::pilota::FastStr::from_static_str("hello"),
                            ::pilota::FastStr::from_static_str("world"),
                        );
                        map
                    });
                }
                let var_12 = var_12.unwrap_or_else(|| {
                    let mut map = ::pilota::AHashMap::with_capacity(1);
                    map.insert(::pilota::OrderedFloat(1f64), 2f64);
                    map
                });
                let var_13 = var_13
                    .unwrap_or_else(|| ::pilota::AHashSet::from([::pilota::OrderedFloat(1f64)]));
                if var_15.is_none() {
                    var_15 = Some(::pilota::AHashMap::new());
                }
                if var_16.is_none() {
                    var_16 = Some({
                        (DEFAULT_COMMIT_IDS.clone())
                            .0
                            .iter()
                            .map(|el| (el.clone()).0)
                            .collect::<::std::vec::Vec<_>>()
                    });
                }
                if var_17.is_none() {
                    var_17 = Some(CommitIdList(::std::vec![CommitId(3i32), CommitId(4i32)]));
                }
                if var_20.is_none() {
                    var_20 = Some(::std::vec![5i32, 6i32, 7i32]);
                }
                if var_21.is_none() {
                    var_21 = Some(::pilota::AHashSet::from([]));
                }
                if var_22.is_none() {
                    var_22 = Some(INT_SET_CONST.clone());
                }
                if var_23.is_none() {
                    var_23 = Some(::std::collections::BTreeSet::from([4i32, 5i32]));
                }
                if var_24.is_none() {
                    var_24 = Some(::std::collections::BTreeSet::from([]));
                }
                if var_25.is_none() {
                    var_25 = Some(INT_BTREE_SET_CONST.clone());
                }
                if var_26.is_none() {
                    var_26 = Some({
                        let mut map = ::pilota::AHashMap::with_capacity(2);
                        map.insert(::pilota::FastStr::from_static_str("one"), 1i32);
                        map.insert(::pilota::FastStr::from_static_str("two"), 2i32);
                        map
                    });
                }
                if var_27.is_none() {
                    var_27 = Some({
                        STR_I32_MAP
                            .clone()
                            .iter()
                            .map(|(k, v)| {
                                (::pilota::FastStr::from_static_str(k.clone()), v.clone())
                            })
                            .collect::<::pilota::AHashMap<_, _>>()
                    });
                }
                if var_28.is_none() {
                    var_28 = Some({
                        let mut map = ::std::collections::BTreeMap::new();
                        map.insert(::pilota::FastStr::from_static_str("three"), 3i32);
                        map
                    });
                }
                if var_29.is_none() {
                    var_29 = Some(::std::collections::BTreeMap::new());
                }
                if var_30.is_none() {
                    var_30 = Some({
                        STR_I32_BTREE_MAP
                            .clone()
                            .iter()
                            .map(|(k, v)| {
                                (::pilota::FastStr::from_static_str(k.clone()), v.clone())
                            })
                            .collect::<::std::collections::BTreeMap<_, _>>()
                    });
                }
                if var_35.is_none() {
                    var_35 = Some(NameScoreMap({
                        let mut map = ::pilota::AHashMap::with_capacity(1);
                        map.insert(
                            NameId(::pilota::FastStr::from_static_str("carol")),
                            Score(5i64),
                        );
                        map
                    }));
                }
                if var_36.is_none() {
                    var_36 = Some(NameScoreMap((NAME_SCORE_LITERAL.clone()).0));
                }
                if var_37.is_none() {
                    var_37 = Some(NameScoreMap(::pilota::AHashMap::new()));
                }
                if var_38.is_none() {
                    var_38 = Some(NameScoreMap((NAME_SCORE_EMPTY.clone()).0));
                }

                let data = Self {
                    faststr: var_1,
                    string: var_2,
                    a: var_3,
                    test_b: var_4,
                    test_b2: var_5,
                    test_b3: var_6,
                    map: var_7,
                    test_double: var_8,
                    test_double2: var_9,
                    alias_str: var_10,
                    empty: var_11,
                    test_map: var_12,
                    test_set: var_13,
                    a2: var_14,
                    map2: var_15,
                    commit_ids_raw: var_16,
                    commit_ids: var_17,
                    default_i16: var_18,
                    default_i64: var_19,
                    list_literal: var_20,
                    set_empty: var_21,
                    set_from_const: var_22,
                    btree_set_literal: var_23,
                    btree_set_empty: var_24,
                    btree_set_from_const: var_25,
                    map_literal_i32: var_26,
                    map_from_const: var_27,
                    btree_map_literal: var_28,
                    btree_map_empty: var_29,
                    btree_map_from_const: var_30,
                    struct_literal: var_31,
                    struct_partial: var_32,
                    struct_from_const: var_33,
                    binary_from_const: var_34,
                    newtype_map_literal: var_35,
                    newtype_map_from_const: var_36,
                    newtype_map_empty: var_37,
                    newtype_map_from_empty_const: var_38,
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
                    let mut var_1 = ::pilota::FastStr::from_static_str("hello world");
                    let mut var_2 = None;
                    let mut var_3 = Some(false);
                    let mut var_4 = Some(B::READ);
                    let mut var_5 = Some(B::WRITE);
                    let mut var_6 = Some((B::READ.inner() as i8));
                    let mut var_7 = None;
                    let mut var_8 = Some(1f64);
                    let mut var_9 = Some(1.2f64);
                    let mut var_10 = Some(::pilota::FastStr::from_static_str(A_S));
                    let mut var_11 = ::pilota::Bytes::from_static("".as_bytes());
                    let mut var_12 = None;
                    let mut var_13 = None;
                    let mut var_14 = Some(true);
                    let mut var_15 = None;
                    let mut var_16 = None;
                    let mut var_17 = None;
                    let mut var_18 = Some(16i16);
                    let mut var_19 = Some(64i64);
                    let mut var_20 = None;
                    let mut var_21 = None;
                    let mut var_22 = None;
                    let mut var_23 = None;
                    let mut var_24 = None;
                    let mut var_25 = None;
                    let mut var_26 = None;
                    let mut var_27 = None;
                    let mut var_28 = None;
                    let mut var_29 = None;
                    let mut var_30 = None;
                    let mut var_31 = Some(C {
                        off: Some(::pilota::FastStr::from_static_str("nested")),
                        test_byte: Some(7i8),
                    });
                    let mut var_32 = Some(C {
                        off: Some(::pilota::FastStr::from_static_str("partial")),
                        test_byte: None,
                    });
                    let mut var_33 = Some(DEFAULT_C);
                    let mut var_34 = Some(DEFAULT_BINARY);
                    let mut var_35 = None;
                    let mut var_36 = None;
                    let mut var_37 = None;
                    let mut var_38 = None;

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
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_1 = __protocol.read_faststr().await?;
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_2 = Some(__protocol.read_string().await?);
                                }
                                Some(3)
                                    if field_ident.field_type == ::pilota::thrift::TType::Bool =>
                                {
                                    var_3 = Some(__protocol.read_bool().await?);
                                }
                                Some(4)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_4 = Some(
                                        <B as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    );
                                }
                                Some(5)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_5 = Some(
                                        <B as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    );
                                }
                                Some(6)
                                    if field_ident.field_type == ::pilota::thrift::TType::I8 =>
                                {
                                    var_6 = Some(__protocol.read_i8().await?);
                                }
                                Some(7)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_7 = Some({
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
                                Some(8)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Double =>
                                {
                                    var_8 = Some(__protocol.read_double().await?);
                                }
                                Some(9)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Double =>
                                {
                                    var_9 = Some(__protocol.read_double().await?);
                                }
                                Some(10)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_10 = Some(__protocol.read_faststr().await?);
                                }
                                Some(11)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_11 = __protocol.read_bytes().await?;
                                }
                                Some(12)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_12 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val =
                                            ::pilota::AHashMap::with_capacity(map_ident.size);
                                        for _ in 0..map_ident.size {
                                            val.insert(
                                                ::pilota::OrderedFloat(
                                                    __protocol.read_double().await?,
                                                ),
                                                __protocol.read_double().await?,
                                            );
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
                                }
                                Some(13)
                                    if field_ident.field_type == ::pilota::thrift::TType::Set =>
                                {
                                    var_13 = Some({
                                        let list_ident = __protocol.read_set_begin().await?;
                                        let mut val =
                                            ::pilota::AHashSet::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.insert(::pilota::OrderedFloat(
                                                __protocol.read_double().await?,
                                            ));
                                        }
                                        __protocol.read_set_end().await?;
                                        val
                                    });
                                }
                                Some(14)
                                    if field_ident.field_type == ::pilota::thrift::TType::Bool =>
                                {
                                    var_14 = Some(__protocol.read_bool().await?);
                                }
                                Some(15)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_15 = Some({
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
                                Some(16)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_16 = Some({
                                        let list_ident = __protocol.read_list_begin().await?;
                                        let mut val =
                                            ::std::vec::Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(__protocol.read_i32().await?);
                                        }
                                        __protocol.read_list_end().await?;
                                        val
                                    });
                                }
                                Some(17)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_17 = Some(
                                        <CommitIdList as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                Some(18)
                                    if field_ident.field_type == ::pilota::thrift::TType::I16 =>
                                {
                                    var_18 = Some(__protocol.read_i16().await?);
                                }
                                Some(19)
                                    if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                                {
                                    var_19 = Some(__protocol.read_i64().await?);
                                }
                                Some(20)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_20 = Some({
                                        let list_ident = __protocol.read_list_begin().await?;
                                        let mut val =
                                            ::std::vec::Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(__protocol.read_i32().await?);
                                        }
                                        __protocol.read_list_end().await?;
                                        val
                                    });
                                }
                                Some(21)
                                    if field_ident.field_type == ::pilota::thrift::TType::Set =>
                                {
                                    var_21 = Some({
                                        let list_ident = __protocol.read_set_begin().await?;
                                        let mut val =
                                            ::pilota::AHashSet::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.insert(__protocol.read_i32().await?);
                                        }
                                        __protocol.read_set_end().await?;
                                        val
                                    });
                                }
                                Some(22)
                                    if field_ident.field_type == ::pilota::thrift::TType::Set =>
                                {
                                    var_22 = Some({
                                        let list_ident = __protocol.read_set_begin().await?;
                                        let mut val =
                                            ::pilota::AHashSet::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.insert(__protocol.read_i32().await?);
                                        }
                                        __protocol.read_set_end().await?;
                                        val
                                    });
                                }
                                Some(23)
                                    if field_ident.field_type == ::pilota::thrift::TType::Set =>
                                {
                                    var_23 = Some({
                                        let list_ident = __protocol.read_set_begin().await?;
                                        let mut val = ::std::collections::BTreeSet::new();
                                        for _ in 0..list_ident.size {
                                            val.insert(__protocol.read_i32().await?);
                                        }
                                        __protocol.read_set_end().await?;
                                        val
                                    });
                                }
                                Some(24)
                                    if field_ident.field_type == ::pilota::thrift::TType::Set =>
                                {
                                    var_24 = Some({
                                        let list_ident = __protocol.read_set_begin().await?;
                                        let mut val = ::std::collections::BTreeSet::new();
                                        for _ in 0..list_ident.size {
                                            val.insert(__protocol.read_i32().await?);
                                        }
                                        __protocol.read_set_end().await?;
                                        val
                                    });
                                }
                                Some(25)
                                    if field_ident.field_type == ::pilota::thrift::TType::Set =>
                                {
                                    var_25 = Some({
                                        let list_ident = __protocol.read_set_begin().await?;
                                        let mut val = ::std::collections::BTreeSet::new();
                                        for _ in 0..list_ident.size {
                                            val.insert(__protocol.read_i32().await?);
                                        }
                                        __protocol.read_set_end().await?;
                                        val
                                    });
                                }
                                Some(26)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_26 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val =
                                            ::pilota::AHashMap::with_capacity(map_ident.size);
                                        for _ in 0..map_ident.size {
                                            val.insert(
                                                __protocol.read_faststr().await?,
                                                __protocol.read_i32().await?,
                                            );
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
                                }
                                Some(27)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_27 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val =
                                            ::pilota::AHashMap::with_capacity(map_ident.size);
                                        for _ in 0..map_ident.size {
                                            val.insert(
                                                __protocol.read_faststr().await?,
                                                __protocol.read_i32().await?,
                                            );
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
                                }
                                Some(28)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_28 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val = ::std::collections::BTreeMap::new();
                                        for _ in 0..map_ident.size {
                                            val.insert(
                                                __protocol.read_faststr().await?,
                                                __protocol.read_i32().await?,
                                            );
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
                                }
                                Some(29)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_29 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val = ::std::collections::BTreeMap::new();
                                        for _ in 0..map_ident.size {
                                            val.insert(
                                                __protocol.read_faststr().await?,
                                                __protocol.read_i32().await?,
                                            );
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
                                }
                                Some(30)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_30 = Some({
                                        let map_ident = __protocol.read_map_begin().await?;
                                        let mut val = ::std::collections::BTreeMap::new();
                                        for _ in 0..map_ident.size {
                                            val.insert(
                                                __protocol.read_faststr().await?,
                                                __protocol.read_i32().await?,
                                            );
                                        }
                                        __protocol.read_map_end().await?;
                                        val
                                    });
                                }
                                Some(31)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_31 = Some(
                                        <C as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    );
                                }
                                Some(32)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_32 = Some(
                                        <C as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    );
                                }
                                Some(33)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_33 = Some(
                                        <C as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    );
                                }
                                Some(34)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_34 = Some(__protocol.read_bytes().await?);
                                }
                                Some(35)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_35 = Some(
                                        <NameScoreMap as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                Some(36)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_36 = Some(
                                        <NameScoreMap as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                Some(37)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_37 = Some(
                                        <NameScoreMap as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                Some(38)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_38 = Some(
                                        <NameScoreMap as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
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
                                "decode struct `A` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let var_2 = var_2.unwrap_or_else(|| "test".to_string());
                    if var_7.is_none() {
                        var_7 = Some({
                            let mut map = ::pilota::AHashMap::with_capacity(1);
                            map.insert(
                                ::pilota::FastStr::from_static_str("hello"),
                                ::pilota::FastStr::from_static_str("world"),
                            );
                            map
                        });
                    }
                    let var_12 = var_12.unwrap_or_else(|| {
                        let mut map = ::pilota::AHashMap::with_capacity(1);
                        map.insert(::pilota::OrderedFloat(1f64), 2f64);
                        map
                    });
                    let var_13 = var_13.unwrap_or_else(|| {
                        ::pilota::AHashSet::from([::pilota::OrderedFloat(1f64)])
                    });
                    if var_15.is_none() {
                        var_15 = Some(::pilota::AHashMap::new());
                    }
                    if var_16.is_none() {
                        var_16 = Some({
                            (DEFAULT_COMMIT_IDS.clone())
                                .0
                                .iter()
                                .map(|el| (el.clone()).0)
                                .collect::<::std::vec::Vec<_>>()
                        });
                    }
                    if var_17.is_none() {
                        var_17 = Some(CommitIdList(::std::vec![CommitId(3i32), CommitId(4i32)]));
                    }
                    if var_20.is_none() {
                        var_20 = Some(::std::vec![5i32, 6i32, 7i32]);
                    }
                    if var_21.is_none() {
                        var_21 = Some(::pilota::AHashSet::from([]));
                    }
                    if var_22.is_none() {
                        var_22 = Some(INT_SET_CONST.clone());
                    }
                    if var_23.is_none() {
                        var_23 = Some(::std::collections::BTreeSet::from([4i32, 5i32]));
                    }
                    if var_24.is_none() {
                        var_24 = Some(::std::collections::BTreeSet::from([]));
                    }
                    if var_25.is_none() {
                        var_25 = Some(INT_BTREE_SET_CONST.clone());
                    }
                    if var_26.is_none() {
                        var_26 = Some({
                            let mut map = ::pilota::AHashMap::with_capacity(2);
                            map.insert(::pilota::FastStr::from_static_str("one"), 1i32);
                            map.insert(::pilota::FastStr::from_static_str("two"), 2i32);
                            map
                        });
                    }
                    if var_27.is_none() {
                        var_27 = Some({
                            STR_I32_MAP
                                .clone()
                                .iter()
                                .map(|(k, v)| {
                                    (::pilota::FastStr::from_static_str(k.clone()), v.clone())
                                })
                                .collect::<::pilota::AHashMap<_, _>>()
                        });
                    }
                    if var_28.is_none() {
                        var_28 = Some({
                            let mut map = ::std::collections::BTreeMap::new();
                            map.insert(::pilota::FastStr::from_static_str("three"), 3i32);
                            map
                        });
                    }
                    if var_29.is_none() {
                        var_29 = Some(::std::collections::BTreeMap::new());
                    }
                    if var_30.is_none() {
                        var_30 = Some({
                            STR_I32_BTREE_MAP
                                .clone()
                                .iter()
                                .map(|(k, v)| {
                                    (::pilota::FastStr::from_static_str(k.clone()), v.clone())
                                })
                                .collect::<::std::collections::BTreeMap<_, _>>()
                        });
                    }
                    if var_35.is_none() {
                        var_35 = Some(NameScoreMap({
                            let mut map = ::pilota::AHashMap::with_capacity(1);
                            map.insert(
                                NameId(::pilota::FastStr::from_static_str("carol")),
                                Score(5i64),
                            );
                            map
                        }));
                    }
                    if var_36.is_none() {
                        var_36 = Some(NameScoreMap((NAME_SCORE_LITERAL.clone()).0));
                    }
                    if var_37.is_none() {
                        var_37 = Some(NameScoreMap(::pilota::AHashMap::new()));
                    }
                    if var_38.is_none() {
                        var_38 = Some(NameScoreMap((NAME_SCORE_EMPTY.clone()).0));
                    }

                    let data = Self {
                        faststr: var_1,
                        string: var_2,
                        a: var_3,
                        test_b: var_4,
                        test_b2: var_5,
                        test_b3: var_6,
                        map: var_7,
                        test_double: var_8,
                        test_double2: var_9,
                        alias_str: var_10,
                        empty: var_11,
                        test_map: var_12,
                        test_set: var_13,
                        a2: var_14,
                        map2: var_15,
                        commit_ids_raw: var_16,
                        commit_ids: var_17,
                        default_i16: var_18,
                        default_i64: var_19,
                        list_literal: var_20,
                        set_empty: var_21,
                        set_from_const: var_22,
                        btree_set_literal: var_23,
                        btree_set_empty: var_24,
                        btree_set_from_const: var_25,
                        map_literal_i32: var_26,
                        map_from_const: var_27,
                        btree_map_literal: var_28,
                        btree_map_empty: var_29,
                        btree_map_from_const: var_30,
                        struct_literal: var_31,
                        struct_partial: var_32,
                        struct_from_const: var_33,
                        binary_from_const: var_34,
                        newtype_map_literal: var_35,
                        newtype_map_from_const: var_36,
                        newtype_map_empty: var_37,
                        newtype_map_from_empty_const: var_38,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + __protocol.faststr_field_len(Some(1), &self.faststr)
                    + __protocol.string_field_len(Some(2), &&self.string)
                    + self
                        .a
                        .as_ref()
                        .map_or(0, |value| __protocol.bool_field_len(Some(3), *value))
                    + self.test_b.as_ref().map_or(0, |value| {
                        __protocol.i32_field_len(Some(4), (value).inner())
                    })
                    + self.test_b2.as_ref().map_or(0, |value| {
                        __protocol.i32_field_len(Some(5), (value).inner())
                    })
                    + self
                        .test_b3
                        .as_ref()
                        .map_or(0, |value| __protocol.i8_field_len(Some(6), *value))
                    + self.map.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(7),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + self
                        .test_double
                        .as_ref()
                        .map_or(0, |value| __protocol.double_field_len(Some(8), *value))
                    + self
                        .test_double2
                        .as_ref()
                        .map_or(0, |value| __protocol.double_field_len(Some(9), *value))
                    + self
                        .alias_str
                        .as_ref()
                        .map_or(0, |value| __protocol.faststr_field_len(Some(10), value))
                    + __protocol.bytes_field_len(Some(11), &self.empty)
                    + __protocol.map_field_len(
                        Some(12),
                        ::pilota::thrift::TType::Double,
                        ::pilota::thrift::TType::Double,
                        &self.test_map,
                        |__protocol, key| __protocol.double_len(key.0),
                        |__protocol, val| __protocol.double_len(*val),
                    )
                    + __protocol.set_field_len(
                        Some(13),
                        ::pilota::thrift::TType::Double,
                        &self.test_set,
                        |__protocol, el| __protocol.double_len(el.0),
                    )
                    + self
                        .a2
                        .as_ref()
                        .map_or(0, |value| __protocol.bool_field_len(Some(14), *value))
                    + self.map2.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(15),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + self.commit_ids_raw.as_ref().map_or(0, |value| {
                        __protocol.list_field_len(
                            Some(16),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + self
                        .commit_ids
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(17), value))
                    + self
                        .default_i16
                        .as_ref()
                        .map_or(0, |value| __protocol.i16_field_len(Some(18), *value))
                    + self
                        .default_i64
                        .as_ref()
                        .map_or(0, |value| __protocol.i64_field_len(Some(19), *value))
                    + self.list_literal.as_ref().map_or(0, |value| {
                        __protocol.list_field_len(
                            Some(20),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + self.set_empty.as_ref().map_or(0, |value| {
                        __protocol.set_field_len(
                            Some(21),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + self.set_from_const.as_ref().map_or(0, |value| {
                        __protocol.set_field_len(
                            Some(22),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + self.btree_set_literal.as_ref().map_or(0, |value| {
                        __protocol.btree_set_field_len(
                            Some(23),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + self.btree_set_empty.as_ref().map_or(0, |value| {
                        __protocol.btree_set_field_len(
                            Some(24),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + self.btree_set_from_const.as_ref().map_or(0, |value| {
                        __protocol.btree_set_field_len(
                            Some(25),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + self.map_literal_i32.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(26),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.i32_len(*val),
                        )
                    })
                    + self.map_from_const.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(27),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.i32_len(*val),
                        )
                    })
                    + self.btree_map_literal.as_ref().map_or(0, |value| {
                        __protocol.btree_map_field_len(
                            Some(28),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.i32_len(*val),
                        )
                    })
                    + self.btree_map_empty.as_ref().map_or(0, |value| {
                        __protocol.btree_map_field_len(
                            Some(29),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.i32_len(*val),
                        )
                    })
                    + self.btree_map_from_const.as_ref().map_or(0, |value| {
                        __protocol.btree_map_field_len(
                            Some(30),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.i32_len(*val),
                        )
                    })
                    + self
                        .struct_literal
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(31), value))
                    + self
                        .struct_partial
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(32), value))
                    + self
                        .struct_from_const
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(33), value))
                    + self
                        .binary_from_const
                        .as_ref()
                        .map_or(0, |value| __protocol.bytes_field_len(Some(34), value))
                    + self
                        .newtype_map_literal
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(35), value))
                    + self
                        .newtype_map_from_const
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(36), value))
                    + self
                        .newtype_map_empty
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(37), value))
                    + self
                        .newtype_map_from_empty_const
                        .as_ref()
                        .map_or(0, |value| __protocol.struct_field_len(Some(38), value))
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub const DEFAULT_BINARY: ::pilota::Bytes = ::pilota::Bytes::from_static("bin".as_bytes());
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct NameScoreMap(pub ::pilota::AHashMap<NameId, Score>);

        impl ::std::ops::Deref for NameScoreMap {
            type Target = ::pilota::AHashMap<NameId, Score>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<::pilota::AHashMap<NameId, Score>> for NameScoreMap {
            fn from(v: ::pilota::AHashMap<NameId, Score>) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for NameScoreMap {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_map(
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::I64,
                    &(&**self),
                    |__protocol, key| {
                        __protocol.write_struct(key)?;
                        ::std::result::Result::Ok(())
                    },
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                ::std::result::Result::Ok(NameScoreMap({
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
                }))
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
                    ::std::result::Result::Ok(NameScoreMap({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(
                                <NameId as ::pilota::thrift::Message>::decode_async(__protocol)
                                    .await?,
                                <Score as ::pilota::thrift::Message>::decode_async(__protocol)
                                    .await?,
                            );
                        }
                        __protocol.read_map_end().await?;
                        val
                    }))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.map_len(
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::I64,
                    &**self,
                    |__protocol, key| __protocol.struct_len(key),
                    |__protocol, val| __protocol.struct_len(val),
                )
            }
        }
        pub static DEFAULT_COMMIT_IDS: ::std::sync::LazyLock<CommitIdList> =
            ::std::sync::LazyLock::new(|| {
                CommitIdList(::std::vec![CommitId(1i32), CommitId(2i32)])
            });
        pub const A_S: &'static str = "string";
        pub static NAME_SCORE_LITERAL: ::std::sync::LazyLock<NameScoreMap> =
            ::std::sync::LazyLock::new(|| {
                NameScoreMap({
                    let mut map = ::pilota::AHashMap::with_capacity(2);
                    map.insert(
                        NameId(::pilota::FastStr::from_static_str("alice")),
                        Score(3i64),
                    );
                    map.insert(
                        NameId(::pilota::FastStr::from_static_str("bob")),
                        Score(4i64),
                    );
                    map
                })
            });

        pub static STR_I32_MAP: ::std::sync::LazyLock<::pilota::AHashMap<&'static str, i32>> =
            ::std::sync::LazyLock::new(|| {
                let mut map = ::pilota::AHashMap::with_capacity(2);
                map.insert("hello", 1i32);
                map.insert("world", 2i32);
                map
            });
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct CommitId(pub i32);

        impl ::std::ops::Deref for CommitId {
            type Target = i32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<i32> for CommitId {
            fn from(v: i32) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for CommitId {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_i32(*(&**self))?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                ::std::result::Result::Ok(CommitId(__protocol.read_i32()?))
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
                    ::std::result::Result::Ok(CommitId(__protocol.read_i32().await?))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.i32_len(*&**self)
            }
        }
        pub static NAME_SCORE_EMPTY: ::std::sync::LazyLock<NameScoreMap> =
            ::std::sync::LazyLock::new(|| NameScoreMap(::pilota::AHashMap::new()));

        pub static STR_I32_BTREE_MAP: ::std::sync::LazyLock<
            ::std::collections::BTreeMap<&'static str, i32>,
        > = ::std::sync::LazyLock::new(|| {
            let mut map = ::std::collections::BTreeMap::new();
            map.insert("alpha", 10i32);
            map.insert("beta", 20i32);
            map
        });
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct CommitIdList(pub ::std::vec::Vec<CommitId>);

        impl ::std::ops::Deref for CommitIdList {
            type Target = ::std::vec::Vec<CommitId>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<::std::vec::Vec<CommitId>> for CommitIdList {
            fn from(v: ::std::vec::Vec<CommitId>) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for CommitIdList {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_list(
                    ::pilota::thrift::TType::I32,
                    &(&**self),
                    |__protocol, val| {
                        __protocol.write_struct(val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                ::std::result::Result::Ok(CommitIdList(unsafe {
                    let list_ident = __protocol.read_list_begin()?;
                    let mut val: ::std::vec::Vec<CommitId> =
                        ::std::vec::Vec::with_capacity(list_ident.size);
                    for i in 0..list_ident.size {
                        val.as_mut_ptr()
                            .offset(i as isize)
                            .write(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    val.set_len(list_ident.size);
                    __protocol.read_list_end()?;
                    val
                }))
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
                    ::std::result::Result::Ok(CommitIdList({
                        let list_ident = __protocol.read_list_begin().await?;
                        let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                        for _ in 0..list_ident.size {
                            val.push(
                                <CommitId as ::pilota::thrift::Message>::decode_async(__protocol)
                                    .await?,
                            );
                        }
                        __protocol.read_list_end().await?;
                        val
                    }))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.list_len(::pilota::thrift::TType::I32, &**self, |__protocol, el| {
                    __protocol.struct_len(el)
                })
            }
        }

        impl ::std::default::Default for C {
            fn default() -> Self {
                C {
                    off: Some(::pilota::FastStr::from_static_str("off")),
                    test_byte: Some(0i8),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub struct C {
            pub off: ::std::option::Option<::pilota::FastStr>,

            pub test_byte: ::std::option::Option<i8>,
        }
        impl ::pilota::thrift::Message for C {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "C" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.off.as_ref() {
                    __protocol.write_faststr_field(1, (value).clone())?;
                }
                if let Some(value) = self.test_byte.as_ref() {
                    __protocol.write_i8_field(2, *value)?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = Some(::pilota::FastStr::from_static_str("off"));
                let mut var_2 = Some(0i8);

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
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_1 = Some(__protocol.read_faststr()?);
                            }
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::I8 => {
                                var_2 = Some(__protocol.read_i8()?);
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
                            "decode struct `C` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self {
                    off: var_1,
                    test_byte: var_2,
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
                    let mut var_1 = Some(::pilota::FastStr::from_static_str("off"));
                    let mut var_2 = Some(0i8);

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
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_1 = Some(__protocol.read_faststr().await?);
                                }
                                Some(2)
                                    if field_ident.field_type == ::pilota::thrift::TType::I8 =>
                                {
                                    var_2 = Some(__protocol.read_i8().await?);
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
                                "decode struct `C` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self {
                        off: var_1,
                        test_byte: var_2,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "C" })
                    + self
                        .off
                        .as_ref()
                        .map_or(0, |value| __protocol.faststr_field_len(Some(1), value))
                    + self
                        .test_byte
                        .as_ref()
                        .map_or(0, |value| __protocol.i8_field_len(Some(2), *value))
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub static INT_SET_CONST: ::std::sync::LazyLock<::pilota::AHashSet<i32>> =
            ::std::sync::LazyLock::new(|| ::pilota::AHashSet::from([7i32, 8i32]));
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct NameId(pub ::pilota::FastStr);

        impl ::std::ops::Deref for NameId {
            type Target = ::pilota::FastStr;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<::pilota::FastStr> for NameId {
            fn from(v: ::pilota::FastStr) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for NameId {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_faststr((&**self).clone())?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                ::std::result::Result::Ok(NameId(__protocol.read_faststr()?))
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
                    ::std::result::Result::Ok(NameId(__protocol.read_faststr().await?))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.faststr_len(&**self)
            }
        }
        pub const DEFAULT_C: C = C {
            off: Some(::pilota::FastStr::from_static_str("const")),
            test_byte: Some(9i8),
        };
        pub static INT_BTREE_SET_CONST: ::std::sync::LazyLock<::std::collections::BTreeSet<i32>> =
            ::std::sync::LazyLock::new(|| ::std::collections::BTreeSet::from([9i32, 10i32]));
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Score(pub i64);

        impl ::std::ops::Deref for Score {
            type Target = i64;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<i64> for Score {
            fn from(v: i64) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for Score {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_i64(*(&**self))?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                ::std::result::Result::Ok(Score(__protocol.read_i64()?))
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
                    ::std::result::Result::Ok(Score(__protocol.read_i64().await?))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.i64_len(*&**self)
            }
        }
    }
}
