pub mod types {
    #![allow(warnings, clippy::all)]

    pub mod rust_types {

        impl ::std::default::Default for Event {
            fn default() -> Self {
                Event::Value(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum Event {
            Value(i64),

            Label(::pilota::FastStr),
        }

        impl ::pilota::thrift::Message for Event {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol
                    .write_struct_begin(&::pilota::thrift::TStructIdentifier { name: "Event" })?;
                match self {
                    Event::Value(value) => {
                        __protocol.write_i64_field(1, *value)?;
                    }
                    Event::Label(value) => {
                        __protocol.write_faststr_field(2, (value).clone())?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                let field_ident = __protocol.read_i64()?;
                                __protocol.i64_len(*&field_ident);
                                ret = ::std::option::Option::Some(Event::Value(field_ident));
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
                                let field_ident = __protocol.read_faststr()?;
                                __protocol.faststr_len(&field_ident);
                                ret = ::std::option::Option::Some(Event::Label(field_ident));
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

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    let field_ident = __protocol.read_i64().await?;

                                    ret = ::std::option::Option::Some(Event::Value(field_ident));
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
                                    let field_ident = __protocol.read_faststr().await?;

                                    ret = ::std::option::Option::Some(Event::Label(field_ident));
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Event" })
                    + match self {
                        Event::Value(value) => {
                            __protocol.i64_field_len(::std::option::Option::Some(1), *value)
                        }
                        Event::Label(value) => {
                            __protocol.faststr_field_len(::std::option::Option::Some(2), value)
                        }
                    }
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub trait Types {}

        impl ::std::default::Default for TypesValuesResultRecv {
            fn default() -> Self {
                TypesValuesResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesValuesResultRecv {
            Ok(::std::vec::Vec<i64>),
        }

        impl ::pilota::thrift::Message for TypesValuesResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesValuesResultRecv",
                })?;
                match self {
                    TypesValuesResultRecv::Ok(value) => {
                        __protocol.write_list_field(
                            0,
                            ::pilota::thrift::TType::I64,
                            &value,
                            |__protocol, val| {
                                __protocol.write_i64(*val)?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                let field_ident = unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<i64> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(__protocol.read_i64()?);
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                };
                                __protocol.list_len(
                                    ::pilota::thrift::TType::I64,
                                    &field_ident,
                                    |__protocol, el| __protocol.i64_len(*el),
                                );
                                ret = ::std::option::Option::Some(TypesValuesResultRecv::Ok(
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

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    let field_ident = {
                                        let list_ident = __protocol.read_list_begin().await?;
                                        let mut val =
                                            ::std::vec::Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(__protocol.read_i64().await?);
                                        }
                                        __protocol.read_list_end().await?;
                                        val
                                    };

                                    ret = ::std::option::Option::Some(TypesValuesResultRecv::Ok(
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesValuesResultRecv",
                }) + match self {
                    TypesValuesResultRecv::Ok(value) => __protocol.list_field_len(
                        ::std::option::Option::Some(0),
                        ::pilota::thrift::TType::I64,
                        value,
                        |__protocol, el| __protocol.i64_len(*el),
                    ),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesShadowResultRecv {
            fn default() -> Self {
                TypesShadowResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesShadowResultRecv {
            Ok(::std::sync::Arc<Send>),
        }

        impl ::pilota::thrift::Message for TypesShadowResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesShadowResultRecv",
                })?;
                match self {
                    TypesShadowResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(TypesShadowResultRecv::Ok(
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

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <Send as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );

                                    ret = ::std::option::Option::Some(TypesShadowResultRecv::Ok(
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesShadowResultRecv",
                }) + match self {
                    TypesShadowResultRecv::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesProtocolNameResultRecv {
            fn default() -> Self {
                TypesProtocolNameResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesProtocolNameResultRecv {
            Ok(::std::sync::Arc<T>),
        }

        impl ::pilota::thrift::Message for TypesProtocolNameResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesProtocolNameResultRecv",
                })?;
                match self {
                    TypesProtocolNameResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(TypesProtocolNameResultRecv::Ok(
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

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <T as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    );

                                    ret = ::std::option::Option::Some(
                                        TypesProtocolNameResultRecv::Ok(field_ident),
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesProtocolNameResultRecv",
                }) + match self {
                    TypesProtocolNameResultRecv::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesScalarResultRecv {
            fn default() -> Self {
                TypesScalarResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesScalarResultRecv {
            Ok(i32),
        }

        impl ::pilota::thrift::Message for TypesScalarResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesScalarResultRecv",
                })?;
                match self {
                    TypesScalarResultRecv::Ok(value) => {
                        __protocol.write_i32_field(0, *value)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                let field_ident = __protocol.read_i32()?;
                                __protocol.i32_len(*&field_ident);
                                ret = ::std::option::Option::Some(TypesScalarResultRecv::Ok(
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

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    let field_ident = __protocol.read_i32().await?;

                                    ret = ::std::option::Option::Some(TypesScalarResultRecv::Ok(
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesScalarResultRecv",
                }) + match self {
                    TypesScalarResultRecv::Ok(value) => {
                        __protocol.i32_field_len(::std::option::Option::Some(0), *value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesZeroResultRecv {
            fn default() -> Self {
                TypesZeroResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesZeroResultRecv {
            Ok(()),
        }

        impl ::pilota::thrift::Message for TypesZeroResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesZeroResultRecv",
                })?;
                match self {
                    TypesZeroResultRecv::Ok(value) => {}
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                    ::std::result::Result::Ok(TypesZeroResultRecv::Ok(()))
                }
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                        ::std::result::Result::Ok(TypesZeroResultRecv::Ok(()))
                    }
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesZeroResultRecv",
                }) + match self {
                    TypesZeroResultRecv::Ok(value) => 0,
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesNotifyResultRecv {
            fn default() -> Self {
                TypesNotifyResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesNotifyResultRecv {
            Ok(()),
        }

        impl ::pilota::thrift::Message for TypesNotifyResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesNotifyResultRecv",
                })?;
                match self {
                    TypesNotifyResultRecv::Ok(value) => {}
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                    ::std::result::Result::Ok(TypesNotifyResultRecv::Ok(()))
                }
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                        ::std::result::Result::Ok(TypesNotifyResultRecv::Ok(()))
                    }
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesNotifyResultRecv",
                }) + match self {
                    TypesNotifyResultRecv::Ok(value) => 0,
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesValuesResultSend {
            fn default() -> Self {
                TypesValuesResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesValuesResultSend {
            Ok(::std::vec::Vec<i64>),
        }

        impl ::pilota::thrift::Message for TypesValuesResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesValuesResultSend",
                })?;
                match self {
                    TypesValuesResultSend::Ok(value) => {
                        __protocol.write_list_field(
                            0,
                            ::pilota::thrift::TType::I64,
                            &value,
                            |__protocol, val| {
                                __protocol.write_i64(*val)?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                let field_ident = unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<i64> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(__protocol.read_i64()?);
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                };
                                __protocol.list_len(
                                    ::pilota::thrift::TType::I64,
                                    &field_ident,
                                    |__protocol, el| __protocol.i64_len(*el),
                                );
                                ret = ::std::option::Option::Some(TypesValuesResultSend::Ok(
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

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    let field_ident = {
                                        let list_ident = __protocol.read_list_begin().await?;
                                        let mut val =
                                            ::std::vec::Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(__protocol.read_i64().await?);
                                        }
                                        __protocol.read_list_end().await?;
                                        val
                                    };

                                    ret = ::std::option::Option::Some(TypesValuesResultSend::Ok(
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesValuesResultSend",
                }) + match self {
                    TypesValuesResultSend::Ok(value) => __protocol.list_field_len(
                        ::std::option::Option::Some(0),
                        ::pilota::thrift::TType::I64,
                        value,
                        |__protocol, el| __protocol.i64_len(*el),
                    ),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesShadowResultSend {
            fn default() -> Self {
                TypesShadowResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesShadowResultSend {
            Ok(::std::sync::Arc<Send>),
        }

        impl ::pilota::thrift::Message for TypesShadowResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesShadowResultSend",
                })?;
                match self {
                    TypesShadowResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(TypesShadowResultSend::Ok(
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

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <Send as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );

                                    ret = ::std::option::Option::Some(TypesShadowResultSend::Ok(
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesShadowResultSend",
                }) + match self {
                    TypesShadowResultSend::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesProtocolNameResultSend {
            fn default() -> Self {
                TypesProtocolNameResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesProtocolNameResultSend {
            Ok(::std::sync::Arc<T>),
        }

        impl ::pilota::thrift::Message for TypesProtocolNameResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesProtocolNameResultSend",
                })?;
                match self {
                    TypesProtocolNameResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                let field_ident = ::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                );
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(TypesProtocolNameResultSend::Ok(
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

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    let field_ident = ::std::sync::Arc::new(
                                        <T as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    );

                                    ret = ::std::option::Option::Some(
                                        TypesProtocolNameResultSend::Ok(field_ident),
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesProtocolNameResultSend",
                }) + match self {
                    TypesProtocolNameResultSend::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesScalarResultSend {
            fn default() -> Self {
                TypesScalarResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesScalarResultSend {
            Ok(i32),
        }

        impl ::pilota::thrift::Message for TypesScalarResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesScalarResultSend",
                })?;
                match self {
                    TypesScalarResultSend::Ok(value) => {
                        __protocol.write_i32_field(0, *value)?;
                    }
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                let field_ident = __protocol.read_i32()?;
                                __protocol.i32_len(*&field_ident);
                                ret = ::std::option::Option::Some(TypesScalarResultSend::Ok(
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

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    let field_ident = __protocol.read_i32().await?;

                                    ret = ::std::option::Option::Some(TypesScalarResultSend::Ok(
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesScalarResultSend",
                }) + match self {
                    TypesScalarResultSend::Ok(value) => {
                        __protocol.i32_field_len(::std::option::Option::Some(0), *value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesZeroResultSend {
            fn default() -> Self {
                TypesZeroResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesZeroResultSend {
            Ok(()),
        }

        impl ::pilota::thrift::Message for TypesZeroResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesZeroResultSend",
                })?;
                match self {
                    TypesZeroResultSend::Ok(value) => {}
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                    ::std::result::Result::Ok(TypesZeroResultSend::Ok(()))
                }
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                        ::std::result::Result::Ok(TypesZeroResultSend::Ok(()))
                    }
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesZeroResultSend",
                }) + match self {
                    TypesZeroResultSend::Ok(value) => 0,
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TypesNotifyResultSend {
            fn default() -> Self {
                TypesNotifyResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TypesNotifyResultSend {
            Ok(()),
        }

        impl ::pilota::thrift::Message for TypesNotifyResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TypesNotifyResultSend",
                })?;
                match self {
                    TypesNotifyResultSend::Ok(value) => {}
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                    ::std::result::Result::Ok(TypesNotifyResultSend::Ok(()))
                }
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                        ::std::result::Result::Ok(TypesNotifyResultSend::Ok(()))
                    }
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesNotifyResultSend",
                }) + match self {
                    TypesNotifyResultSend::Ok(value) => 0,
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Alias(pub Event);

        impl ::std::ops::Deref for Alias {
            type Target = Event;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<Event> for Alias {
            fn from(v: Event) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for Alias {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct((&**self))?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};
                ::std::result::Result::Ok(Alias(::pilota::thrift::Message::decode(__protocol)?))
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    ::std::result::Result::Ok(Alias(
                        <Event as ::pilota::thrift::Message>::decode_async(__protocol).await?,
                    ))
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_len(&**self)
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Node {
            pub next: ::std::option::Option<::std::boxed::Box<Node>>,
        }
        impl ::pilota::thrift::Message for Node {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Node" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let ::std::option::Option::Some(value) = self.next.as_ref() {
                    __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                var_1 = ::std::option::Option::Some(::std::boxed::Box::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `Node` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self { next: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    var_1 = ::std::option::Option::Some(::std::boxed::Box::new(
                                        <Node as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
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
                                "decode struct `Node` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self { next: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Node" })
                    + self.next.as_ref().map_or(0, |value| {
                        __protocol.struct_field_len(::std::option::Option::Some(1), value)
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesValuesArgsSend {
            pub values: ::std::vec::Vec<i64>,
        }
        impl ::pilota::thrift::Message for TypesValuesArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesValuesArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_list_field(
                    1,
                    ::pilota::thrift::TType::I64,
                    &&self.values,
                    |__protocol, val| {
                        __protocol.write_i64(*val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                if field_ident.field_type == ::pilota::thrift::TType::List =>
                            {
                                var_1 = ::std::option::Option::Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<i64> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(__protocol.read_i64()?);
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
                            "decode struct `TypesValuesArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field values is required".to_string(),
                    ));
                };

                let data = Self { values: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_1 = ::std::option::Option::Some({
                                        let list_ident = __protocol.read_list_begin().await?;
                                        let mut val =
                                            ::std::vec::Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(__protocol.read_i64().await?);
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
                            err.prepend_msg(&format!("decode struct `TypesValuesArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field values is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { values: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesValuesArgsSend",
                }) + __protocol.list_field_len(
                    ::std::option::Option::Some(1),
                    ::pilota::thrift::TType::I64,
                    &self.values,
                    |__protocol, el| __protocol.i64_len(*el),
                ) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesShadowArgsSend {
            pub value: ::std::sync::Arc<Send>,
        }
        impl ::pilota::thrift::Message for TypesShadowArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesShadowArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.value, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `TypesShadowArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };

                let data = Self { value: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                        <Send as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
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
                            err.prepend_msg(&format!("decode struct `TypesShadowArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { value: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesShadowArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.value)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesProtocolNameArgsSend {
            pub value: ::std::sync::Arc<T>,
        }
        impl ::pilota::thrift::Message for TypesProtocolNameArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesProtocolNameArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.value, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                        err.prepend_msg(&format!("decode struct `TypesProtocolNameArgsSend` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };

                let data = Self { value: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                        <T as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    ));
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
                            err.prepend_msg(&format!("decode struct `TypesProtocolNameArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { value: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesProtocolNameArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.value)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesScalarArgsSend {
            pub value: i32,
        }
        impl ::pilota::thrift::Message for TypesScalarArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesScalarArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_i32_field(1, *&self.value)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                            "decode struct `TypesScalarArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };

                let data = Self { value: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                            err.prepend_msg(&format!("decode struct `TypesScalarArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { value: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesScalarArgsSend",
                }) + __protocol.i32_field_len(::std::option::Option::Some(1), *&self.value)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesZeroArgsSend {}
        impl ::pilota::thrift::Message for TypesZeroArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesZeroArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;

                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

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
                            "decode struct `TypesZeroArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self {};
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
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
                                "decode struct `TypesZeroArgsSend` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self {};
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesZeroArgsSend",
                }) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesNotifyArgsSend {
            pub node: ::std::sync::Arc<Node>,
        }
        impl ::pilota::thrift::Message for TypesNotifyArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesNotifyArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.node, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `TypesNotifyArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field node is required".to_string(),
                    ));
                };

                let data = Self { node: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                        <Node as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
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
                            err.prepend_msg(&format!("decode struct `TypesNotifyArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field node is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { node: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesNotifyArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.node)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Send {}
        impl ::pilota::thrift::Message for Send {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Send" };

                __protocol.write_struct_begin(&struct_ident)?;

                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

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
                            "decode struct `Send` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self {};
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
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
                                "decode struct `Send` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self {};
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Send" })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesValuesArgsRecv {
            pub values: ::std::vec::Vec<i64>,
        }
        impl ::pilota::thrift::Message for TypesValuesArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesValuesArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_list_field(
                    1,
                    ::pilota::thrift::TType::I64,
                    &&self.values,
                    |__protocol, val| {
                        __protocol.write_i64(*val)?;
                        ::std::result::Result::Ok(())
                    },
                )?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                if field_ident.field_type == ::pilota::thrift::TType::List =>
                            {
                                var_1 = ::std::option::Option::Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<i64> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(__protocol.read_i64()?);
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
                            "decode struct `TypesValuesArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field values is required".to_string(),
                    ));
                };

                let data = Self { values: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_1 = ::std::option::Option::Some({
                                        let list_ident = __protocol.read_list_begin().await?;
                                        let mut val =
                                            ::std::vec::Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push(__protocol.read_i64().await?);
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
                            err.prepend_msg(&format!("decode struct `TypesValuesArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field values is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { values: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesValuesArgsRecv",
                }) + __protocol.list_field_len(
                    ::std::option::Option::Some(1),
                    ::pilota::thrift::TType::I64,
                    &self.values,
                    |__protocol, el| __protocol.i64_len(*el),
                ) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesShadowArgsRecv {
            pub value: ::std::sync::Arc<Send>,
        }
        impl ::pilota::thrift::Message for TypesShadowArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesShadowArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.value, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `TypesShadowArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };

                let data = Self { value: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                        <Send as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
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
                            err.prepend_msg(&format!("decode struct `TypesShadowArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { value: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesShadowArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.value)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesProtocolNameArgsRecv {
            pub value: ::std::sync::Arc<T>,
        }
        impl ::pilota::thrift::Message for TypesProtocolNameArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesProtocolNameArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.value, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                        err.prepend_msg(&format!("decode struct `TypesProtocolNameArgsRecv` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };

                let data = Self { value: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                        <T as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    ));
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
                            err.prepend_msg(&format!("decode struct `TypesProtocolNameArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { value: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesProtocolNameArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.value)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesScalarArgsRecv {
            pub value: i32,
        }
        impl ::pilota::thrift::Message for TypesScalarArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesScalarArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_i32_field(1, *&self.value)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                            "decode struct `TypesScalarArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field value is required".to_string(),
                    ));
                };

                let data = Self { value: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                            err.prepend_msg(&format!("decode struct `TypesScalarArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field value is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { value: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesScalarArgsRecv",
                }) + __protocol.i32_field_len(::std::option::Option::Some(1), *&self.value)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesZeroArgsRecv {}
        impl ::pilota::thrift::Message for TypesZeroArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesZeroArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;

                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

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
                            "decode struct `TypesZeroArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self {};
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
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
                                "decode struct `TypesZeroArgsRecv` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self {};
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesZeroArgsRecv",
                }) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TypesNotifyArgsRecv {
            pub node: ::std::sync::Arc<Node>,
        }
        impl ::pilota::thrift::Message for TypesNotifyArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TypesNotifyArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.node, ::pilota::thrift::TType::Struct)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `TypesNotifyArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field node is required".to_string(),
                    ));
                };

                let data = Self { node: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    var_1 = ::std::option::Option::Some(::std::sync::Arc::new(
                                        <Node as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    ));
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
                            err.prepend_msg(&format!("decode struct `TypesNotifyArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field node is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { node: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TypesNotifyArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.node)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct T {
            pub next: ::std::option::Option<::std::boxed::Box<T>>,
        }
        impl ::pilota::thrift::Message for T {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "T" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let ::std::option::Option::Some(value) = self.next.as_ref() {
                    __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                }
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
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
                                var_1 = ::std::option::Option::Some(::std::boxed::Box::new(
                                    ::pilota::thrift::Message::decode(__protocol)?,
                                ));
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
                            "decode struct `T` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self { next: var_1 };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
                __protocol: &'a mut __PilotaProtocol,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + ::std::marker::Send
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
                                    var_1 = ::std::option::Option::Some(::std::boxed::Box::new(
                                        <T as ::pilota::thrift::Message>::decode_async(__protocol)
                                            .await?,
                                    ));
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
                                "decode struct `T` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self { next: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "T" })
                    + self.next.as_ref().map_or(0, |value| {
                        __protocol.struct_field_len(::std::option::Option::Some(1), value)
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
