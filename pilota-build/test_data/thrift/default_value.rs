pub mod default_value {
    #![allow(warnings, clippy::all)]

    pub mod default_value {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq, Copy)]
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
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
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
        impl Default for C {
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
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut off = Some(::pilota::FastStr::from_static_str("off"));
                let mut test_byte = Some(0i8);

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
                                off = Some(__protocol.read_faststr()?);
                            }
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::I8 => {
                                test_byte = Some(__protocol.read_i8()?);
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

                let data = Self { off, test_byte };
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
                    let mut off = Some(::pilota::FastStr::from_static_str("off"));
                    let mut test_byte = Some(0i8);

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
                                    off = Some(__protocol.read_faststr().await?);
                                }
                                Some(2)
                                    if field_ident.field_type == ::pilota::thrift::TType::I8 =>
                                {
                                    test_byte = Some(__protocol.read_i8().await?);
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

                    let data = Self { off, test_byte };
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
        impl Default for A {
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
                    __protocol.write_i8_field(5, *value)?;
                }
                if let Some(value) = self.map.as_ref() {
                    __protocol.write_map_field(
                        6,
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
                    __protocol.write_double_field(7, *value)?;
                }
                if let Some(value) = self.test_double2.as_ref() {
                    __protocol.write_double_field(8, *value)?;
                }
                if let Some(value) = self.alias_str.as_ref() {
                    __protocol.write_faststr_field(9, (value).clone())?;
                }
                __protocol.write_bytes_field(10, (&self.empty).clone())?;
                __protocol.write_map_field(
                    11,
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
                    12,
                    ::pilota::thrift::TType::Double,
                    &&self.test_set,
                    |__protocol, val| {
                        __protocol.write_double(val.0)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                if let Some(value) = self.a2.as_ref() {
                    __protocol.write_bool_field(13, *value)?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut faststr = ::pilota::FastStr::from_static_str("hello world");
                let mut string = None;
                let mut a = Some(false);
                let mut test_b = Some(B::READ);
                let mut test_b2 = Some(B::WRITE);
                let mut test_b3 = Some((B::READ.inner() as i8));
                let mut map = None;
                let mut test_double = Some(1f64);
                let mut test_double2 = Some(1.2f64);
                let mut alias_str = Some(::pilota::FastStr::from_static_str(A_S));
                let mut empty = ::pilota::Bytes::from_static("".as_bytes());
                let mut test_map = None;
                let mut test_set = None;
                let mut a2 = Some(true);

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
                                faststr = __protocol.read_faststr()?;
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                string = Some(__protocol.read_string()?);
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                                a = Some(__protocol.read_bool()?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                test_b = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                test_b2 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::I8 => {
                                test_b3 = Some(__protocol.read_i8()?);
                            }
                            Some(6) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                map = Some({
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
                            Some(7)
                                if field_ident.field_type == ::pilota::thrift::TType::Double =>
                            {
                                test_double = Some(__protocol.read_double()?);
                            }
                            Some(8)
                                if field_ident.field_type == ::pilota::thrift::TType::Double =>
                            {
                                test_double2 = Some(__protocol.read_double()?);
                            }
                            Some(9)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                alias_str = Some(__protocol.read_faststr()?);
                            }
                            Some(10)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                empty = __protocol.read_bytes()?;
                            }
                            Some(11) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                test_map = Some({
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
                            Some(12) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                test_set = Some({
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
                            Some(13) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                                a2 = Some(__protocol.read_bool()?);
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

                let string = string.unwrap_or_else(|| "test".to_string());
                if map.is_none() {
                    map = Some({
                        let mut map = ::pilota::AHashMap::with_capacity(1);
                        map.insert(
                            ::pilota::FastStr::from_static_str("hello"),
                            ::pilota::FastStr::from_static_str("world"),
                        );
                        map
                    });
                }
                let test_map = test_map.unwrap_or_else(|| {
                    let mut map = ::pilota::AHashMap::with_capacity(1);
                    map.insert(::pilota::OrderedFloat(1f64), 2f64);
                    map
                });
                let test_set = test_set
                    .unwrap_or_else(|| ::pilota::AHashSet::from([::pilota::OrderedFloat(1f64)]));

                let data = Self {
                    faststr,
                    string,
                    a,
                    test_b,
                    test_b2,
                    test_b3,
                    map,
                    test_double,
                    test_double2,
                    alias_str,
                    empty,
                    test_map,
                    test_set,
                    a2,
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
                    let mut faststr = ::pilota::FastStr::from_static_str("hello world");
                    let mut string = None;
                    let mut a = Some(false);
                    let mut test_b = Some(B::READ);
                    let mut test_b2 = Some(B::WRITE);
                    let mut test_b3 = Some((B::READ.inner() as i8));
                    let mut map = None;
                    let mut test_double = Some(1f64);
                    let mut test_double2 = Some(1.2f64);
                    let mut alias_str = Some(::pilota::FastStr::from_static_str(A_S));
                    let mut empty = ::pilota::Bytes::from_static("".as_bytes());
                    let mut test_map = None;
                    let mut test_set = None;
                    let mut a2 = Some(true);

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
                                    faststr = __protocol.read_faststr().await?;
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    string = Some(__protocol.read_string().await?);
                                }
                                Some(3)
                                    if field_ident.field_type == ::pilota::thrift::TType::Bool =>
                                {
                                    a = Some(__protocol.read_bool().await?);
                                }
                                Some(4)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    test_b = Some(
                                        <B as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    );
                                }
                                Some(5)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    test_b2 = Some(
                                        <B as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    );
                                }
                                Some(5)
                                    if field_ident.field_type == ::pilota::thrift::TType::I8 =>
                                {
                                    test_b3 = Some(__protocol.read_i8().await?);
                                }
                                Some(6)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    map = Some({
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
                                Some(7)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Double =>
                                {
                                    test_double = Some(__protocol.read_double().await?);
                                }
                                Some(8)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Double =>
                                {
                                    test_double2 = Some(__protocol.read_double().await?);
                                }
                                Some(9)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    alias_str = Some(__protocol.read_faststr().await?);
                                }
                                Some(10)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    empty = __protocol.read_bytes().await?;
                                }
                                Some(11)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    test_map = Some({
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
                                Some(12)
                                    if field_ident.field_type == ::pilota::thrift::TType::Set =>
                                {
                                    test_set = Some({
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
                                Some(13)
                                    if field_ident.field_type == ::pilota::thrift::TType::Bool =>
                                {
                                    a2 = Some(__protocol.read_bool().await?);
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

                    let string = string.unwrap_or_else(|| "test".to_string());
                    if map.is_none() {
                        map = Some({
                            let mut map = ::pilota::AHashMap::with_capacity(1);
                            map.insert(
                                ::pilota::FastStr::from_static_str("hello"),
                                ::pilota::FastStr::from_static_str("world"),
                            );
                            map
                        });
                    }
                    let test_map = test_map.unwrap_or_else(|| {
                        let mut map = ::pilota::AHashMap::with_capacity(1);
                        map.insert(::pilota::OrderedFloat(1f64), 2f64);
                        map
                    });
                    let test_set = test_set.unwrap_or_else(|| {
                        ::pilota::AHashSet::from([::pilota::OrderedFloat(1f64)])
                    });

                    let data = Self {
                        faststr,
                        string,
                        a,
                        test_b,
                        test_b2,
                        test_b3,
                        map,
                        test_double,
                        test_double2,
                        alias_str,
                        empty,
                        test_map,
                        test_set,
                        a2,
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
                        .map_or(0, |value| __protocol.i8_field_len(Some(5), *value))
                    + self.map.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            Some(6),
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
                        .map_or(0, |value| __protocol.double_field_len(Some(7), *value))
                    + self
                        .test_double2
                        .as_ref()
                        .map_or(0, |value| __protocol.double_field_len(Some(8), *value))
                    + self
                        .alias_str
                        .as_ref()
                        .map_or(0, |value| __protocol.faststr_field_len(Some(9), value))
                    + __protocol.bytes_field_len(Some(10), &self.empty)
                    + __protocol.map_field_len(
                        Some(11),
                        ::pilota::thrift::TType::Double,
                        ::pilota::thrift::TType::Double,
                        &self.test_map,
                        |__protocol, key| __protocol.double_len(key.0),
                        |__protocol, val| __protocol.double_len(*val),
                    )
                    + __protocol.set_field_len(
                        Some(12),
                        ::pilota::thrift::TType::Double,
                        &self.test_set,
                        |__protocol, el| __protocol.double_len(el.0),
                    )
                    + self
                        .a2
                        .as_ref()
                        .map_or(0, |value| __protocol.bool_field_len(Some(13), *value))
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub const A_S: &'static str = "string";
    }
}
