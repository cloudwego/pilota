pub mod option_shadow {
    #![allow(warnings, clippy::all)]

    pub mod option_shadow {

        // Types whose Rust names shadow the prelude's `Some`, `None` and `Option`.

        // The generated code must keep working with these in scope.

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Some(i32);

        impl Some {
            pub const ONE: Self = Self(1);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(1) => ::std::string::String::from("ONE"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    1 => ::std::option::Option::Some(Self::ONE),
                    _ => ::std::option::Option::None,
                }
            }
        }

        impl ::std::convert::From<i32> for Some {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Some> for i32 {
            fn from(value: Some) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for Some {
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
                            format!("invalid enum value for Some, value: {}", value),
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
                                format!("invalid enum value for Some, value: {}", value),
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
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct ItemServiceFireArgsRecv {
            pub choice: Choice,
        }
        impl ::pilota::thrift::Message for ItemServiceFireArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceFireArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.choice, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!("decode struct `ItemServiceFireArgsRecv` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field choice is required".to_string(),
                    ));
                };

                let data = Self { choice: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        <Choice as ::pilota::thrift::Message>::decode_async(
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ItemServiceFireArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field choice is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { choice: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceFireArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.choice)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ItemServiceFireResultSend {
            fn default() -> Self {
                ItemServiceFireResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ItemServiceFireResultSend {
            Ok(()),
        }

        impl ::pilota::thrift::Message for ItemServiceFireResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceFireResultSend",
                })?;
                match self {
                    ItemServiceFireResultSend::Ok(value) => {}
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
                let mut ret = ::std::option::Option::None;
                __protocol.read_struct_begin()?;
                loop {
                    let field_ident = __protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        __protocol.field_stop_len();
                        break;
                    } else {
                        __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        _ => {
                            __protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                __protocol.read_field_end()?;
                __protocol.read_struct_end()?;
                if let ::std::option::Option::Some(ret) = ret {
                    ::std::result::Result::Ok(ret)
                } else {
                    ::std::result::Result::Ok(ItemServiceFireResultSend::Ok(()))
                }
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
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            _ => {
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
                        ::std::result::Result::Ok(ret)
                    } else {
                        ::std::result::Result::Ok(ItemServiceFireResultSend::Ok(()))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceFireResultSend",
                }) + match self {
                    ItemServiceFireResultSend::Ok(value) => 0,
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct ItemServiceGetArgsRecv {
            pub req: Item,
        }
        impl ::pilota::thrift::Message for ItemServiceGetArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `ItemServiceGetArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        <Item as ::pilota::thrift::Message>::decode_async(
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ItemServiceGetArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field req is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { req: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ItemServiceGetException {
            fn default() -> Self {
                ItemServiceGetException::Err(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ItemServiceGetException {
            Err(ItemError),
        }

        impl ::pilota::thrift::Message for ItemServiceGetException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetException",
                })?;
                match self {
                    ItemServiceGetException::Err(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
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
                let mut ret = ::std::option::Option::None;
                __protocol.read_struct_begin()?;
                loop {
                    let field_ident = __protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        __protocol.field_stop_len();
                        break;
                    } else {
                        __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        ::std::option::Option::Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(ItemServiceGetException::Err(
                                    field_ident,
                                ));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        _ => {
                            __protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                __protocol.read_field_end()?;
                __protocol.read_struct_end()?;
                if let ::std::option::Option::Some(ret) = ret {
                    ::std::result::Result::Ok(ret)
                } else {
                    ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
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
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            ::std::option::Option::Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <ItemError as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(
                                        ItemServiceGetException::Err(field_ident),
                                    );
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            _ => {
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
                        ::std::result::Result::Ok(ret)
                    } else {
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetException",
                }) + match self {
                    ItemServiceGetException::Err(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(1), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ItemServiceGetResultSend {
            fn default() -> Self {
                ItemServiceGetResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum ItemServiceGetResultSend {
            Ok(Item),

            Err(ItemError),
        }

        impl ::pilota::thrift::Message for ItemServiceGetResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetResultSend",
                })?;
                match self {
                    ItemServiceGetResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    ItemServiceGetResultSend::Err(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
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
                let mut ret = ::std::option::Option::None;
                __protocol.read_struct_begin()?;
                loop {
                    let field_ident = __protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        __protocol.field_stop_len();
                        break;
                    } else {
                        __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        ::std::option::Option::Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(ItemServiceGetResultSend::Ok(
                                    field_ident,
                                ));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        ::std::option::Option::Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(ItemServiceGetResultSend::Err(
                                    field_ident,
                                ));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        _ => {
                            __protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                __protocol.read_field_end()?;
                __protocol.read_struct_end()?;
                if let ::std::option::Option::Some(ret) = ret {
                    ::std::result::Result::Ok(ret)
                } else {
                    ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
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
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            ::std::option::Option::Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <Item as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(
                                        ItemServiceGetResultSend::Ok(field_ident),
                                    );
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            ::std::option::Option::Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <ItemError as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(
                                        ItemServiceGetResultSend::Err(field_ident),
                                    );
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            _ => {
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
                        ::std::result::Result::Ok(ret)
                    } else {
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetResultSend",
                }) + match self {
                    ItemServiceGetResultSend::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                    ItemServiceGetResultSend::Err(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(1), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for ItemServiceGetResultRecv {
            fn default() -> Self {
                ItemServiceGetResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum ItemServiceGetResultRecv {
            Ok(Item),

            Err(ItemError),
        }

        impl ::pilota::thrift::Message for ItemServiceGetResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetResultRecv",
                })?;
                match self {
                    ItemServiceGetResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    ItemServiceGetResultRecv::Err(value) => {
                        __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
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
                let mut ret = ::std::option::Option::None;
                __protocol.read_struct_begin()?;
                loop {
                    let field_ident = __protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        __protocol.field_stop_len();
                        break;
                    } else {
                        __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        ::std::option::Option::Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(ItemServiceGetResultRecv::Ok(
                                    field_ident,
                                ));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        ::std::option::Option::Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(ItemServiceGetResultRecv::Err(
                                    field_ident,
                                ));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        _ => {
                            __protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                __protocol.read_field_end()?;
                __protocol.read_struct_end()?;
                if let ::std::option::Option::Some(ret) = ret {
                    ::std::result::Result::Ok(ret)
                } else {
                    ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
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
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            ::std::option::Option::Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <Item as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(
                                        ItemServiceGetResultRecv::Ok(field_ident),
                                    );
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            ::std::option::Option::Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <ItemError as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(
                                        ItemServiceGetResultRecv::Err(field_ident),
                                    );
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            _ => {
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
                        ::std::result::Result::Ok(ret)
                    } else {
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetResultRecv",
                }) + match self {
                    ItemServiceGetResultRecv::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                    ItemServiceGetResultRecv::Err(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(1), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }

        impl ::std::default::Default for Item {
            fn default() -> Self {
                Item {
                    name: ::std::default::Default::default(),
                    some_code: ::std::default::Default::default(),
                    none_code: ::std::default::Default::default(),
                    option_code: ::std::default::Default::default(),
                    nums: ::std::default::Default::default(),
                    tags: ::std::default::Default::default(),
                    ids: ::std::default::Default::default(),
                    inner: ::std::default::Default::default(),
                    with_default: ::std::option::Option::Some(::pilota::FastStr::from_static_str(
                        "default",
                    )),
                    lazy_default: ::std::option::Option::Some(::std::vec![1i32, 2i32]),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Item {
            pub name: ::std::option::Option<::pilota::FastStr>,

            pub some_code: Some,

            pub none_code: None,

            pub option_code: ::std::option::Option<Option>,

            pub nums: ::std::option::Option<::std::vec::Vec<i32>>,

            pub tags:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,

            pub ids: ::std::option::Option<::pilota::AHashSet<i32>>,

            pub inner: ::std::option::Option<Inner>,

            pub with_default: ::std::option::Option<::pilota::FastStr>,

            pub lazy_default: ::std::option::Option<::std::vec::Vec<i32>>,
        }
        impl ::pilota::thrift::Message for Item {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Item" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let ::std::option::Option::Some(value) = self.name.as_ref() {
                    __protocol.write_faststr_field(1, (value).clone())?;
                }
                __protocol.write_i32_field(2, (&self.some_code).inner())?;
                __protocol.write_i32_field(3, (&self.none_code).inner())?;
                if let ::std::option::Option::Some(value) = self.option_code.as_ref() {
                    __protocol.write_i32_field(4, (value).inner())?;
                }
                if let ::std::option::Option::Some(value) = self.nums.as_ref() {
                    __protocol.write_list_field(
                        5,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let ::std::option::Option::Some(value) = self.tags.as_ref() {
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
                if let ::std::option::Option::Some(value) = self.ids.as_ref() {
                    __protocol.write_set_field(
                        7,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                if let ::std::option::Option::Some(value) = self.inner.as_ref() {
                    __protocol.write_struct_field(8, value, ::pilota::thrift::TType::Struct)?;
                }
                if let ::std::option::Option::Some(value) = self.with_default.as_ref() {
                    __protocol.write_faststr_field(9, (value).clone())?;
                }
                if let ::std::option::Option::Some(value) = self.lazy_default.as_ref() {
                    __protocol.write_list_field(
                        10,
                        ::pilota::thrift::TType::I32,
                        &value,
                        |__protocol, val| {
                            __protocol.write_i32(*val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
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

                let mut var_1 = ::std::option::Option::None;
                let mut var_2 = ::std::option::Option::None;
                let mut var_3 = ::std::option::Option::None;
                let mut var_4 = ::std::option::Option::None;
                let mut var_5 = ::std::option::Option::None;
                let mut var_6 = ::std::option::Option::None;
                let mut var_7 = ::std::option::Option::None;
                let mut var_8 = ::std::option::Option::None;
                let mut var_9 =
                    ::std::option::Option::Some(::pilota::FastStr::from_static_str("default"));
                let mut var_10 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_1 = ::std::option::Option::Some(__protocol.read_faststr()?);
                            }
                            ::std::option::Option::Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                            {
                                var_2 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                            }
                            ::std::option::Option::Some(3)
                                if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                            {
                                var_3 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                            }
                            ::std::option::Option::Some(4)
                                if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                            {
                                var_4 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                            }
                            ::std::option::Option::Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::List =>
                            {
                                var_5 = ::std::option::Option::Some(unsafe {
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
                            ::std::option::Option::Some(6)
                                if field_ident.field_type == ::pilota::thrift::TType::Map =>
                            {
                                var_6 = ::std::option::Option::Some({
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
                            ::std::option::Option::Some(7)
                                if field_ident.field_type == ::pilota::thrift::TType::Set =>
                            {
                                var_7 = ::std::option::Option::Some({
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
                            ::std::option::Option::Some(8)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_8 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                            }
                            ::std::option::Option::Some(9)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_9 = ::std::option::Option::Some(__protocol.read_faststr()?);
                            }
                            ::std::option::Option::Some(10)
                                if field_ident.field_type == ::pilota::thrift::TType::List =>
                            {
                                var_10 = ::std::option::Option::Some(unsafe {
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
                            _ => {
                                __protocol.skip(field_ident.field_type)?;
                            }
                        }

                        __protocol.read_field_end()?;
                        __protocol.field_end_len();
                    }
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `Item` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field some_code is required".to_string(),
                    ));
                };
                let ::std::option::Option::Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field none_code is required".to_string(),
                    ));
                };

                if var_10.is_none() {
                    var_10 = ::std::option::Option::Some(::std::vec![1i32, 2i32]);
                }

                let data = Self {
                    name: var_1,
                    some_code: var_2,
                    none_code: var_3,
                    option_code: var_4,
                    nums: var_5,
                    tags: var_6,
                    ids: var_7,
                    inner: var_8,
                    with_default: var_9,
                    lazy_default: var_10,
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
                    let mut var_1 = ::std::option::Option::None;
                    let mut var_2 = ::std::option::Option::None;
                    let mut var_3 = ::std::option::Option::None;
                    let mut var_4 = ::std::option::Option::None;
                    let mut var_5 = ::std::option::Option::None;
                    let mut var_6 = ::std::option::Option::None;
                    let mut var_7 = ::std::option::Option::None;
                    let mut var_8 = ::std::option::Option::None;
                    let mut var_9 =
                        ::std::option::Option::Some(::pilota::FastStr::from_static_str("default"));
                    let mut var_10 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        __protocol.read_faststr().await?,
                                    );
                                }
                                ::std::option::Option::Some(2)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_2 = ::std::option::Option::Some(
                                        <Some as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                ::std::option::Option::Some(3)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_3 = ::std::option::Option::Some(
                                        <None as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                ::std::option::Option::Some(4)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_4 = ::std::option::Option::Some(
                                        <Option as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                ::std::option::Option::Some(5)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_5 = ::std::option::Option::Some({
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
                                ::std::option::Option::Some(6)
                                    if field_ident.field_type == ::pilota::thrift::TType::Map =>
                                {
                                    var_6 = ::std::option::Option::Some({
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
                                ::std::option::Option::Some(7)
                                    if field_ident.field_type == ::pilota::thrift::TType::Set =>
                                {
                                    var_7 = ::std::option::Option::Some({
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
                                ::std::option::Option::Some(8)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_8 = ::std::option::Option::Some(
                                        <Inner as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                ::std::option::Option::Some(9)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_9 = ::std::option::Option::Some(
                                        __protocol.read_faststr().await?,
                                    );
                                }
                                ::std::option::Option::Some(10)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_10 = ::std::option::Option::Some({
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `Item` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field some_code is required".to_string(),
                            ),
                        );
                    };
                    let ::std::option::Option::Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field none_code is required".to_string(),
                            ),
                        );
                    };

                    if var_10.is_none() {
                        var_10 = ::std::option::Option::Some(::std::vec![1i32, 2i32]);
                    }

                    let data = Self {
                        name: var_1,
                        some_code: var_2,
                        none_code: var_3,
                        option_code: var_4,
                        nums: var_5,
                        tags: var_6,
                        ids: var_7,
                        inner: var_8,
                        with_default: var_9,
                        lazy_default: var_10,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Item" })
                    + self.name.as_ref().map_or(0, |value| {
                        __protocol.faststr_field_len(::std::option::Option::Some(1), value)
                    })
                    + __protocol
                        .i32_field_len(::std::option::Option::Some(2), (&self.some_code).inner())
                    + __protocol
                        .i32_field_len(::std::option::Option::Some(3), (&self.none_code).inner())
                    + self.option_code.as_ref().map_or(0, |value| {
                        __protocol.i32_field_len(::std::option::Option::Some(4), (value).inner())
                    })
                    + self.nums.as_ref().map_or(0, |value| {
                        __protocol.list_field_len(
                            ::std::option::Option::Some(5),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + self.tags.as_ref().map_or(0, |value| {
                        __protocol.map_field_len(
                            ::std::option::Option::Some(6),
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Binary,
                            value,
                            |__protocol, key| __protocol.faststr_len(key),
                            |__protocol, val| __protocol.faststr_len(val),
                        )
                    })
                    + self.ids.as_ref().map_or(0, |value| {
                        __protocol.set_field_len(
                            ::std::option::Option::Some(7),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + self.inner.as_ref().map_or(0, |value| {
                        __protocol.struct_field_len(::std::option::Option::Some(8), value)
                    })
                    + self.with_default.as_ref().map_or(0, |value| {
                        __protocol.faststr_field_len(::std::option::Option::Some(9), value)
                    })
                    + self.lazy_default.as_ref().map_or(0, |value| {
                        __protocol.list_field_len(
                            ::std::option::Option::Some(10),
                            ::pilota::thrift::TType::I32,
                            value,
                            |__protocol, el| __protocol.i32_len(*el),
                        )
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Option(i32);

        impl Option {
            pub const ANY: Self = Self(0);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("ANY"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => ::std::option::Option::Some(Self::ANY),
                    _ => ::std::option::Option::None,
                }
            }
        }

        impl ::std::convert::From<i32> for Option {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Option> for i32 {
            fn from(value: Option) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for Option {
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
                            format!("invalid enum value for Option, value: {}", value),
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
                                format!("invalid enum value for Option, value: {}", value),
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

        pub trait ItemService {}

        impl ::std::default::Default for ItemServiceFireResultRecv {
            fn default() -> Self {
                ItemServiceFireResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum ItemServiceFireResultRecv {
            Ok(()),
        }

        impl ::pilota::thrift::Message for ItemServiceFireResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceFireResultRecv",
                })?;
                match self {
                    ItemServiceFireResultRecv::Ok(value) => {}
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
                let mut ret = ::std::option::Option::None;
                __protocol.read_struct_begin()?;
                loop {
                    let field_ident = __protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        __protocol.field_stop_len();
                        break;
                    } else {
                        __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        _ => {
                            __protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                __protocol.read_field_end()?;
                __protocol.read_struct_end()?;
                if let ::std::option::Option::Some(ret) = ret {
                    ::std::result::Result::Ok(ret)
                } else {
                    ::std::result::Result::Ok(ItemServiceFireResultRecv::Ok(()))
                }
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
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            _ => {
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
                        ::std::result::Result::Ok(ret)
                    } else {
                        ::std::result::Result::Ok(ItemServiceFireResultRecv::Ok(()))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceFireResultRecv",
                }) + match self {
                    ItemServiceFireResultRecv::Ok(value) => 0,
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub static DEFAULT_ITEM: ::std::sync::LazyLock<Item> =
            ::std::sync::LazyLock::new(|| Item {
                name: ::std::option::Option::Some(::pilota::FastStr::from_static_str("const")),
                some_code: ::std::default::Default::default(),
                none_code: ::std::default::Default::default(),
                option_code: ::std::option::Option::None,
                nums: ::std::option::Option::None,
                tags: ::std::option::Option::None,
                ids: ::std::option::Option::None,
                inner: ::std::option::Option::None,
                with_default: ::std::option::Option::None,
                lazy_default: ::std::option::Option::None,
            });

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct None(i32);

        impl None {
            pub const ZERO: Self = Self(0);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn to_string(&self) -> ::std::string::String {
                match self {
                    Self(0) => ::std::string::String::from("ZERO"),
                    Self(val) => val.to_string(),
                }
            }

            pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                match value {
                    0 => ::std::option::Option::Some(Self::ZERO),
                    _ => ::std::option::Option::None,
                }
            }
        }

        impl ::std::convert::From<i32> for None {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<None> for i32 {
            fn from(value: None) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for None {
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
                            format!("invalid enum value for None, value: {}", value),
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
                                format!("invalid enum value for None, value: {}", value),
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
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct ItemServiceFireArgsSend {
            pub choice: Choice,
        }
        impl ::pilota::thrift::Message for ItemServiceFireArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceFireArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.choice, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!("decode struct `ItemServiceFireArgsSend` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field choice is required".to_string(),
                    ));
                };

                let data = Self { choice: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        <Choice as ::pilota::thrift::Message>::decode_async(
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ItemServiceFireArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field choice is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { choice: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceFireArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.choice)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct ItemServiceGetArgsSend {
            pub req: Item,
        }
        impl ::pilota::thrift::Message for ItemServiceGetArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = ::std::option::Option::Some(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `ItemServiceGetArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        <Item as ::pilota::thrift::Message>::decode_async(
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `ItemServiceGetArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field req is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { req: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "ItemServiceGetArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct ItemError {
            pub message: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::thrift::Message for ItemError {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "ItemError" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let ::std::option::Option::Some(value) = self.message.as_ref() {
                    __protocol.write_faststr_field(1, (value).clone())?;
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

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_1 = ::std::option::Option::Some(__protocol.read_faststr()?);
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `ItemError` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self { message: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_1 = ::std::option::Option::Some(
                                        __protocol.read_faststr().await?,
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `ItemError` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self { message: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol
                    .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "ItemError" })
                    + self.message.as_ref().map_or(0, |value| {
                        __protocol.faststr_field_len(::std::option::Option::Some(1), value)
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for Choice {
            fn default() -> Self {
                Choice::Text(::std::default::Default::default())
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub enum Choice {
            Text(::pilota::FastStr),

            Item(Item),
        }

        impl ::pilota::thrift::Message for Choice {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol
                    .write_struct_begin(&::pilota::thrift::TStructIdentifier { name: "Choice" })?;
                match self {
                    Choice::Text(value) => {
                        __protocol.write_faststr_field(1, (value).clone())?;
                    }
                    Choice::Item(value) => {
                        __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
                    }
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
                let mut ret = ::std::option::Option::None;
                __protocol.read_struct_begin()?;
                loop {
                    let field_ident = __protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        __protocol.field_stop_len();
                        break;
                    } else {
                        __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        ::std::option::Option::Some(1) => {
                            if ret.is_none() {
                                let field_ident = __protocol.read_faststr()?;
                                __protocol.faststr_len(&field_ident);
                                ret = ::std::option::Option::Some(Choice::Text(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        ::std::option::Option::Some(2) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(Choice::Item(field_ident));
                            } else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ),
                                );
                            }
                        }
                        _ => {
                            __protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                __protocol.read_field_end()?;
                __protocol.read_struct_end()?;
                if let ::std::option::Option::Some(ret) = ret {
                    ::std::result::Result::Ok(ret)
                } else {
                    ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
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
                    let mut ret = ::std::option::Option::None;
                    __protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = __protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            ::std::option::Option::Some(1) => {
                                if ret.is_none() {
                                    let field_ident = __protocol.read_faststr().await?;

                                    ret = ::std::option::Option::Some(Choice::Text(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            ::std::option::Option::Some(2) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <Item as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(Choice::Item(field_ident));
                                } else {
                                    return ::std::result::Result::Err(
                                        ::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message",
                                        ),
                                    );
                                }
                            }
                            _ => {
                                __protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    __protocol.read_field_end().await?;
                    __protocol.read_struct_end().await?;
                    if let ::std::option::Option::Some(ret) = ret {
                        ::std::result::Result::Ok(ret)
                    } else {
                        ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Choice" })
                    + match self {
                        Choice::Text(value) => {
                            __protocol.faststr_field_len(::std::option::Option::Some(1), value)
                        }
                        Choice::Item(value) => {
                            __protocol.struct_field_len(::std::option::Option::Some(2), value)
                        }
                    }
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Inner {
            pub id: i32,
        }
        impl ::pilota::thrift::Message for Inner {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Inner" };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_i32_field(1, *&self.id)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = ::std::option::Option::None;

                let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                            ::std::option::Option::Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                            {
                                var_1 = ::std::option::Option::Some(__protocol.read_i32()?);
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
                    if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `Inner` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field id is required".to_string(),
                    ));
                };

                let data = Self { id: var_1 };
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
                    let mut var_1 = ::std::option::Option::None;

                    let mut __pilota_decoding_field_id = ::std::option::Option::None;

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
                                ::std::option::Option::Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_1 =
                                        ::std::option::Option::Some(__protocol.read_i32().await?);
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
                        if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `Inner` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field id is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { id: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Inner" })
                    + __protocol.i32_field_len(::std::option::Option::Some(1), *&self.id)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
