pub mod payload {
    #![allow(warnings, clippy::all)]

    pub mod grpc_parent {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Failure {
            pub message: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::thrift::Message for Failure {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Failure" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let ::std::option::Option::Some(value) = self.message.as_ref() {
                    __protocol.write_faststr_field(1, (value).clone())?;
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
                            "decode struct `Failure` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self { message: var_1 };
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
                                "decode struct `Failure` field(#{}) failed, caused by: ",
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

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol
                    .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Failure" })
                    + self.message.as_ref().map_or(0, |value| {
                        __protocol.faststr_field_len(::std::option::Option::Some(1), value)
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Other {
            pub code: ::std::option::Option<i32>,
        }
        impl ::pilota::thrift::Message for Other {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Other" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let ::std::option::Option::Some(value) = self.code.as_ref() {
                    __protocol.write_i32_field(1, *value)?;
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
                            "decode struct `Other` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self { code: var_1 };
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
                            err.prepend_msg(&format!(
                                "decode struct `Other` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self { code: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Other" })
                    + self.code.as_ref().map_or(0, |value| {
                        __protocol.i32_field_len(::std::option::Option::Some(1), *value)
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub trait Parent {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Message {
            pub payload: ::std::option::Option<::pilota::Bytes>,

            pub sequence: ::std::option::Option<i64>,
        }
        impl ::pilota::thrift::Message for Message {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Message" };

                __protocol.write_struct_begin(&struct_ident)?;
                if let ::std::option::Option::Some(value) = self.payload.as_ref() {
                    __protocol.write_bytes_field(1, (value).clone())?;
                }
                if let ::std::option::Option::Some(value) = self.sequence.as_ref() {
                    __protocol.write_i64_field(2, *value)?;
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
                let mut var_2 = ::std::option::Option::None;

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
                                var_1 = ::std::option::Option::Some(__protocol.read_bytes()?);
                            }
                            ::std::option::Option::Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                            {
                                var_2 = ::std::option::Option::Some(__protocol.read_i64()?);
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
                            "decode struct `Message` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self {
                    payload: var_1,
                    sequence: var_2,
                };
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
                    let mut var_2 = ::std::option::Option::None;

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
                                    var_1 =
                                        ::std::option::Option::Some(__protocol.read_bytes().await?);
                                }
                                ::std::option::Option::Some(2)
                                    if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                                {
                                    var_2 =
                                        ::std::option::Option::Some(__protocol.read_i64().await?);
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
                                "decode struct `Message` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self {
                        payload: var_1,
                        sequence: var_2,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol
                    .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Message" })
                    + self.payload.as_ref().map_or(0, |value| {
                        __protocol.bytes_field_len(::std::option::Option::Some(1), value)
                    })
                    + self.sequence.as_ref().map_or(0, |value| {
                        __protocol.i64_field_len(::std::option::Option::Some(2), *value)
                    })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }

    pub mod grpc_thrift {

        pub trait PureUnary {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Empty {}
        impl ::pilota::thrift::Message for Empty {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Empty" };

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
                            "decode struct `Empty` field(#{}) failed, caused by: ",
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
                                "decode struct `Empty` field(#{}) failed, caused by: ",
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
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Empty" })
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for PureUnaryEchoResultRecv {
            fn default() -> Self {
                PureUnaryEchoResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum PureUnaryEchoResultRecv {
            Ok(Empty),
        }

        impl ::pilota::thrift::Message for PureUnaryEchoResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "PureUnaryEchoResultRecv",
                })?;
                match self {
                    PureUnaryEchoResultRecv::Ok(value) => {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(PureUnaryEchoResultRecv::Ok(
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
                                    let field_ident =
                                        <Empty as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(PureUnaryEchoResultRecv::Ok(
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
                    name: "PureUnaryEchoResultRecv",
                }) + match self {
                    PureUnaryEchoResultRecv::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropVoidThrowArgsSend {}
        impl ::pilota::thrift::Message for InteropVoidThrowArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropVoidThrowArgsSend",
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
                        err.prepend_msg(&format!("decode struct `InteropVoidThrowArgsSend` field(#{}) failed, caused by: ", field_id));
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
                            err.prepend_msg(&format!("decode struct `InteropVoidThrowArgsSend` field(#{}) failed, caused by: ", field_id));
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
                    name: "InteropVoidThrowArgsSend",
                }) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropThrowArgsRecv {
            pub request: super::grpc_parent::Message,
        }
        impl ::pilota::thrift::Message for InteropThrowArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropThrowArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
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
                            "decode struct `InteropThrowArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    ::std::option::Option::Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = ::std::option::Option::Some(<super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `InteropThrowArgsRecv` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
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
                    name: "InteropThrowArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropThrowResultRecv {
            fn default() -> Self {
                InteropThrowResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropThrowResultRecv {
            Ok(super::grpc_parent::Message),

            Failure(super::grpc_parent::Failure),

            Other(super::grpc_parent::Other),
        }

        impl ::pilota::thrift::Message for InteropThrowResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropThrowResultRecv",
                })?;
                match self {
                    InteropThrowResultRecv::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    InteropThrowResultRecv::Failure(value) => {
                        __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
                    }
                    InteropThrowResultRecv::Other(value) => {
                        __protocol.write_struct_field(7, value, ::pilota::thrift::TType::Struct)?;
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropThrowResultRecv::Ok(
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
                        ::std::option::Option::Some(2) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropThrowResultRecv::Failure(
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
                        ::std::option::Option::Some(7) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropThrowResultRecv::Other(
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
                                    let field_ident = <super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(InteropThrowResultRecv::Ok(
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
                            ::std::option::Option::Some(2) => {
                                if ret.is_none() {
                                    let field_ident = <super::grpc_parent::Failure as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropThrowResultRecv::Failure(field_ident),
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
                            ::std::option::Option::Some(7) => {
                                if ret.is_none() {
                                    let field_ident = <super::grpc_parent::Other as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropThrowResultRecv::Other(field_ident),
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
                    name: "InteropThrowResultRecv",
                }) + match self {
                    InteropThrowResultRecv::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                    InteropThrowResultRecv::Failure(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(2), value)
                    }
                    InteropThrowResultRecv::Other(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(7), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropContainerResultRecv {
            fn default() -> Self {
                InteropContainerResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropContainerResultRecv {
            Ok(::std::vec::Vec<i64>),
        }

        impl ::pilota::thrift::Message for InteropContainerResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropContainerResultRecv",
                })?;
                match self {
                    InteropContainerResultRecv::Ok(value) => {
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
                                ret = ::std::option::Option::Some(InteropContainerResultRecv::Ok(
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

                                    ret = ::std::option::Option::Some(
                                        InteropContainerResultRecv::Ok(field_ident),
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
                    name: "InteropContainerResultRecv",
                }) + match self {
                    InteropContainerResultRecv::Ok(value) => __protocol.list_field_len(
                        ::std::option::Option::Some(0),
                        ::pilota::thrift::TType::I64,
                        value,
                        |__protocol, el| __protocol.i64_len(*el),
                    ),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropMultiResultRecv {
            fn default() -> Self {
                InteropMultiResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropMultiResultRecv {
            Ok(i64),
        }

        impl ::pilota::thrift::Message for InteropMultiResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropMultiResultRecv",
                })?;
                match self {
                    InteropMultiResultRecv::Ok(value) => {
                        __protocol.write_i64_field(0, *value)?;
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
                                let field_ident = __protocol.read_i64()?;
                                __protocol.i64_len(*&field_ident);
                                ret = ::std::option::Option::Some(InteropMultiResultRecv::Ok(
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
                                    let field_ident = __protocol.read_i64().await?;

                                    ret = ::std::option::Option::Some(InteropMultiResultRecv::Ok(
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
                    name: "InteropMultiResultRecv",
                }) + match self {
                    InteropMultiResultRecv::Ok(value) => {
                        __protocol.i64_field_len(::std::option::Option::Some(0), *value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropZeroResultRecv {
            fn default() -> Self {
                InteropZeroResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropZeroResultRecv {
            Ok(()),
        }

        impl ::pilota::thrift::Message for InteropZeroResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropZeroResultRecv",
                })?;
                match self {
                    InteropZeroResultRecv::Ok(value) => {}
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
                    ::std::result::Result::Ok(InteropZeroResultRecv::Ok(()))
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
                        ::std::result::Result::Ok(InteropZeroResultRecv::Ok(()))
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
                    name: "InteropZeroResultRecv",
                }) + match self {
                    InteropZeroResultRecv::Ok(value) => 0,
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropExplicitUnaryResultRecv {
            fn default() -> Self {
                InteropExplicitUnaryResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropExplicitUnaryResultRecv {
            Ok(super::grpc_parent::Message),
        }

        impl ::pilota::thrift::Message for InteropExplicitUnaryResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropExplicitUnaryResultRecv",
                })?;
                match self {
                    InteropExplicitUnaryResultRecv::Ok(value) => {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(
                                    InteropExplicitUnaryResultRecv::Ok(field_ident),
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
                                    let field_ident = <super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropExplicitUnaryResultRecv::Ok(field_ident),
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
                    name: "InteropExplicitUnaryResultRecv",
                }) + match self {
                    InteropExplicitUnaryResultRecv::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropUnaryResultRecv {
            fn default() -> Self {
                InteropUnaryResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropUnaryResultRecv {
            Ok(super::grpc_parent::Message),
        }

        impl ::pilota::thrift::Message for InteropUnaryResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropUnaryResultRecv",
                })?;
                match self {
                    InteropUnaryResultRecv::Ok(value) => {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropUnaryResultRecv::Ok(
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
                                    let field_ident = <super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(InteropUnaryResultRecv::Ok(
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
                    name: "InteropUnaryResultRecv",
                }) + match self {
                    InteropUnaryResultRecv::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct PureUnaryEchoArgsSend {
            pub request: Empty,
        }
        impl ::pilota::thrift::Message for PureUnaryEchoArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "PureUnaryEchoArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
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
                            "decode struct `PureUnaryEchoArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                                    var_1 = ::std::option::Option::Some(
                                        <Empty as ::pilota::thrift::Message>::decode_async(
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
                            err.prepend_msg(&format!("decode struct `PureUnaryEchoArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
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
                    name: "PureUnaryEchoArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub trait PureStreaming {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropVoidThrowArgsRecv {}
        impl ::pilota::thrift::Message for InteropVoidThrowArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropVoidThrowArgsRecv",
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
                        err.prepend_msg(&format!("decode struct `InteropVoidThrowArgsRecv` field(#{}) failed, caused by: ", field_id));
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
                            err.prepend_msg(&format!("decode struct `InteropVoidThrowArgsRecv` field(#{}) failed, caused by: ", field_id));
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
                    name: "InteropVoidThrowArgsRecv",
                }) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropVoidThrowException {
            fn default() -> Self {
                InteropVoidThrowException::Failure(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropVoidThrowException {
            Failure(super::grpc_parent::Failure),
        }

        impl ::pilota::thrift::Message for InteropVoidThrowException {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropVoidThrowException",
                })?;
                match self {
                    InteropVoidThrowException::Failure(value) => {
                        __protocol.write_struct_field(4, value, ::pilota::thrift::TType::Struct)?;
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
                        ::std::option::Option::Some(4) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(
                                    InteropVoidThrowException::Failure(field_ident),
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
                            ::std::option::Option::Some(4) => {
                                if ret.is_none() {
                                    let field_ident = <super::grpc_parent::Failure as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropVoidThrowException::Failure(field_ident),
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
                    name: "InteropVoidThrowException",
                }) + match self {
                    InteropVoidThrowException::Failure(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(4), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropVoidThrowResultSend {
            fn default() -> Self {
                InteropVoidThrowResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropVoidThrowResultSend {
            Ok(()),

            Failure(super::grpc_parent::Failure),
        }

        impl ::pilota::thrift::Message for InteropVoidThrowResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropVoidThrowResultSend",
                })?;
                match self {
                    InteropVoidThrowResultSend::Ok(value) => {}
                    InteropVoidThrowResultSend::Failure(value) => {
                        __protocol.write_struct_field(4, value, ::pilota::thrift::TType::Struct)?;
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
                        ::std::option::Option::Some(4) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(
                                    InteropVoidThrowResultSend::Failure(field_ident),
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
                            __protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                __protocol.read_field_end()?;
                __protocol.read_struct_end()?;
                if let ::std::option::Option::Some(ret) = ret {
                    ::std::result::Result::Ok(ret)
                } else {
                    ::std::result::Result::Ok(InteropVoidThrowResultSend::Ok(()))
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
                            ::std::option::Option::Some(4) => {
                                if ret.is_none() {
                                    let field_ident = <super::grpc_parent::Failure as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropVoidThrowResultSend::Failure(field_ident),
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
                        ::std::result::Result::Ok(InteropVoidThrowResultSend::Ok(()))
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
                    name: "InteropVoidThrowResultSend",
                }) + match self {
                    InteropVoidThrowResultSend::Ok(value) => 0,
                    InteropVoidThrowResultSend::Failure(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(4), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropVoidThrowResultRecv {
            fn default() -> Self {
                InteropVoidThrowResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropVoidThrowResultRecv {
            Ok(()),

            Failure(super::grpc_parent::Failure),
        }

        impl ::pilota::thrift::Message for InteropVoidThrowResultRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropVoidThrowResultRecv",
                })?;
                match self {
                    InteropVoidThrowResultRecv::Ok(value) => {}
                    InteropVoidThrowResultRecv::Failure(value) => {
                        __protocol.write_struct_field(4, value, ::pilota::thrift::TType::Struct)?;
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
                        ::std::option::Option::Some(4) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(
                                    InteropVoidThrowResultRecv::Failure(field_ident),
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
                            __protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                __protocol.read_field_end()?;
                __protocol.read_struct_end()?;
                if let ::std::option::Option::Some(ret) = ret {
                    ::std::result::Result::Ok(ret)
                } else {
                    ::std::result::Result::Ok(InteropVoidThrowResultRecv::Ok(()))
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
                            ::std::option::Option::Some(4) => {
                                if ret.is_none() {
                                    let field_ident = <super::grpc_parent::Failure as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropVoidThrowResultRecv::Failure(field_ident),
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
                        ::std::result::Result::Ok(InteropVoidThrowResultRecv::Ok(()))
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
                    name: "InteropVoidThrowResultRecv",
                }) + match self {
                    InteropVoidThrowResultRecv::Ok(value) => 0,
                    InteropVoidThrowResultRecv::Failure(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(4), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropThrowResultSend {
            fn default() -> Self {
                InteropThrowResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropThrowResultSend {
            Ok(super::grpc_parent::Message),

            Failure(super::grpc_parent::Failure),

            Other(super::grpc_parent::Other),
        }

        impl ::pilota::thrift::Message for InteropThrowResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropThrowResultSend",
                })?;
                match self {
                    InteropThrowResultSend::Ok(value) => {
                        __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    InteropThrowResultSend::Failure(value) => {
                        __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
                    }
                    InteropThrowResultSend::Other(value) => {
                        __protocol.write_struct_field(7, value, ::pilota::thrift::TType::Struct)?;
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropThrowResultSend::Ok(
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
                        ::std::option::Option::Some(2) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropThrowResultSend::Failure(
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
                        ::std::option::Option::Some(7) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropThrowResultSend::Other(
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
                                    let field_ident = <super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(InteropThrowResultSend::Ok(
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
                            ::std::option::Option::Some(2) => {
                                if ret.is_none() {
                                    let field_ident = <super::grpc_parent::Failure as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropThrowResultSend::Failure(field_ident),
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
                            ::std::option::Option::Some(7) => {
                                if ret.is_none() {
                                    let field_ident = <super::grpc_parent::Other as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropThrowResultSend::Other(field_ident),
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
                    name: "InteropThrowResultSend",
                }) + match self {
                    InteropThrowResultSend::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                    InteropThrowResultSend::Failure(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(2), value)
                    }
                    InteropThrowResultSend::Other(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(7), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropContainerArgsSend {
            pub values: ::std::vec::Vec<i64>,
        }
        impl ::pilota::thrift::Message for InteropContainerArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropContainerArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_list_field(
                    7,
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

                let mut var_7 = ::std::option::Option::None;

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
                            ::std::option::Option::Some(7)
                                if field_ident.field_type == ::pilota::thrift::TType::List =>
                            {
                                var_7 = ::std::option::Option::Some(unsafe {
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
                        err.prepend_msg(&format!("decode struct `InteropContainerArgsSend` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_7) = var_7 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field values is required".to_string(),
                    ));
                };

                let data = Self { values: var_7 };
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
                    let mut var_7 = ::std::option::Option::None;

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
                                ::std::option::Option::Some(7)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_7 = ::std::option::Option::Some({
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
                            err.prepend_msg(&format!("decode struct `InteropContainerArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_7) = var_7 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field values is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { values: var_7 };
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
                    name: "InteropContainerArgsSend",
                }) + __protocol.list_field_len(
                    ::std::option::Option::Some(7),
                    ::pilota::thrift::TType::I64,
                    &self.values,
                    |__protocol, el| __protocol.i64_len(*el),
                ) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropMultiArgsSend {
            pub left: i32,

            pub right: i64,
        }
        impl ::pilota::thrift::Message for InteropMultiArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropMultiArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_i32_field(2, *&self.left)?;
                __protocol.write_i64_field(9, *&self.right)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_2 = ::std::option::Option::None;
                let mut var_9 = ::std::option::Option::None;

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
                            ::std::option::Option::Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                            {
                                var_2 = ::std::option::Option::Some(__protocol.read_i32()?);
                            }
                            ::std::option::Option::Some(9)
                                if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                            {
                                var_9 = ::std::option::Option::Some(__protocol.read_i64()?);
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
                            "decode struct `InteropMultiArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field left is required".to_string(),
                    ));
                };
                let ::std::option::Option::Some(var_9) = var_9 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field right is required".to_string(),
                    ));
                };

                let data = Self {
                    left: var_2,
                    right: var_9,
                };
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
                    let mut var_2 = ::std::option::Option::None;
                    let mut var_9 = ::std::option::Option::None;

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
                                ::std::option::Option::Some(2)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_2 =
                                        ::std::option::Option::Some(__protocol.read_i32().await?);
                                }
                                ::std::option::Option::Some(9)
                                    if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                                {
                                    var_9 =
                                        ::std::option::Option::Some(__protocol.read_i64().await?);
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
                            err.prepend_msg(&format!("decode struct `InteropMultiArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field left is required".to_string(),
                            ),
                        );
                    };
                    let ::std::option::Option::Some(var_9) = var_9 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field right is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        left: var_2,
                        right: var_9,
                    };
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
                    name: "InteropMultiArgsSend",
                }) + __protocol.i32_field_len(::std::option::Option::Some(2), *&self.left)
                    + __protocol.i64_field_len(::std::option::Option::Some(9), *&self.right)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropZeroArgsSend {}
        impl ::pilota::thrift::Message for InteropZeroArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropZeroArgsSend",
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
                            "decode struct `InteropZeroArgsSend` field(#{}) failed, caused by: ",
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
                            err.prepend_msg(&format!("decode struct `InteropZeroArgsSend` field(#{}) failed, caused by: ", field_id));
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
                    name: "InteropZeroArgsSend",
                }) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropExplicitUnaryArgsSend {
            pub request: super::grpc_parent::Message,
        }
        impl ::pilota::thrift::Message for InteropExplicitUnaryArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropExplicitUnaryArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
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
                        err.prepend_msg(&format!("decode struct `InteropExplicitUnaryArgsSend` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    ::std::option::Option::Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = ::std::option::Option::Some(<super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `InteropExplicitUnaryArgsSend` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
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
                    name: "InteropExplicitUnaryArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropUnaryArgsSend {
            pub request: super::grpc_parent::Message,
        }
        impl ::pilota::thrift::Message for InteropUnaryArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropUnaryArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
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
                            "decode struct `InteropUnaryArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    ::std::option::Option::Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = ::std::option::Option::Some(<super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `InteropUnaryArgsSend` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
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
                    name: "InteropUnaryArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct PureUnaryEchoArgsRecv {
            pub request: Empty,
        }
        impl ::pilota::thrift::Message for PureUnaryEchoArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "PureUnaryEchoArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
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
                            "decode struct `PureUnaryEchoArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                                    var_1 = ::std::option::Option::Some(
                                        <Empty as ::pilota::thrift::Message>::decode_async(
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
                            err.prepend_msg(&format!("decode struct `PureUnaryEchoArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
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
                    name: "PureUnaryEchoArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for PureUnaryEchoResultSend {
            fn default() -> Self {
                PureUnaryEchoResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum PureUnaryEchoResultSend {
            Ok(Empty),
        }

        impl ::pilota::thrift::Message for PureUnaryEchoResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "PureUnaryEchoResultSend",
                })?;
                match self {
                    PureUnaryEchoResultSend::Ok(value) => {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(PureUnaryEchoResultSend::Ok(
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
                                    let field_ident =
                                        <Empty as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = ::std::option::Option::Some(PureUnaryEchoResultSend::Ok(
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
                    name: "PureUnaryEchoResultSend",
                }) + match self {
                    PureUnaryEchoResultSend::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        pub trait Interop {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropThrowArgsSend {
            pub request: super::grpc_parent::Message,
        }
        impl ::pilota::thrift::Message for InteropThrowArgsSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropThrowArgsSend",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
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
                            "decode struct `InteropThrowArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    ::std::option::Option::Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = ::std::option::Option::Some(<super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `InteropThrowArgsSend` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
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
                    name: "InteropThrowArgsSend",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropThrowException {
            fn default() -> Self {
                InteropThrowException::Failure(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropThrowException {
            Failure(super::grpc_parent::Failure),

            Other(super::grpc_parent::Other),
        }

        impl ::pilota::thrift::Message for InteropThrowException {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropThrowException",
                })?;
                match self {
                    InteropThrowException::Failure(value) => {
                        __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
                    }
                    InteropThrowException::Other(value) => {
                        __protocol.write_struct_field(7, value, ::pilota::thrift::TType::Struct)?;
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
                        ::std::option::Option::Some(2) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropThrowException::Failure(
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
                        ::std::option::Option::Some(7) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropThrowException::Other(
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
                            ::std::option::Option::Some(2) => {
                                if ret.is_none() {
                                    let field_ident = <super::grpc_parent::Failure as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropThrowException::Failure(field_ident),
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
                            ::std::option::Option::Some(7) => {
                                if ret.is_none() {
                                    let field_ident = <super::grpc_parent::Other as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropThrowException::Other(field_ident),
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
                    name: "InteropThrowException",
                }) + match self {
                    InteropThrowException::Failure(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(2), value)
                    }
                    InteropThrowException::Other(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(7), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropContainerArgsRecv {
            pub values: ::std::vec::Vec<i64>,
        }
        impl ::pilota::thrift::Message for InteropContainerArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropContainerArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_list_field(
                    7,
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

                let mut var_7 = ::std::option::Option::None;

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
                            ::std::option::Option::Some(7)
                                if field_ident.field_type == ::pilota::thrift::TType::List =>
                            {
                                var_7 = ::std::option::Option::Some(unsafe {
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
                        err.prepend_msg(&format!("decode struct `InteropContainerArgsRecv` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_7) = var_7 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field values is required".to_string(),
                    ));
                };

                let data = Self { values: var_7 };
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
                    let mut var_7 = ::std::option::Option::None;

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
                                ::std::option::Option::Some(7)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    var_7 = ::std::option::Option::Some({
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
                            err.prepend_msg(&format!("decode struct `InteropContainerArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_7) = var_7 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field values is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { values: var_7 };
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
                    name: "InteropContainerArgsRecv",
                }) + __protocol.list_field_len(
                    ::std::option::Option::Some(7),
                    ::pilota::thrift::TType::I64,
                    &self.values,
                    |__protocol, el| __protocol.i64_len(*el),
                ) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropContainerResultSend {
            fn default() -> Self {
                InteropContainerResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropContainerResultSend {
            Ok(::std::vec::Vec<i64>),
        }

        impl ::pilota::thrift::Message for InteropContainerResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropContainerResultSend",
                })?;
                match self {
                    InteropContainerResultSend::Ok(value) => {
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
                                ret = ::std::option::Option::Some(InteropContainerResultSend::Ok(
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

                                    ret = ::std::option::Option::Some(
                                        InteropContainerResultSend::Ok(field_ident),
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
                    name: "InteropContainerResultSend",
                }) + match self {
                    InteropContainerResultSend::Ok(value) => __protocol.list_field_len(
                        ::std::option::Option::Some(0),
                        ::pilota::thrift::TType::I64,
                        value,
                        |__protocol, el| __protocol.i64_len(*el),
                    ),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropMultiArgsRecv {
            pub left: i32,

            pub right: i64,
        }
        impl ::pilota::thrift::Message for InteropMultiArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropMultiArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_i32_field(2, *&self.left)?;
                __protocol.write_i64_field(9, *&self.right)?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_2 = ::std::option::Option::None;
                let mut var_9 = ::std::option::Option::None;

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
                            ::std::option::Option::Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                            {
                                var_2 = ::std::option::Option::Some(__protocol.read_i32()?);
                            }
                            ::std::option::Option::Some(9)
                                if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                            {
                                var_9 = ::std::option::Option::Some(__protocol.read_i64()?);
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
                            "decode struct `InteropMultiArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field left is required".to_string(),
                    ));
                };
                let ::std::option::Option::Some(var_9) = var_9 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field right is required".to_string(),
                    ));
                };

                let data = Self {
                    left: var_2,
                    right: var_9,
                };
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
                    let mut var_2 = ::std::option::Option::None;
                    let mut var_9 = ::std::option::Option::None;

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
                                ::std::option::Option::Some(2)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_2 =
                                        ::std::option::Option::Some(__protocol.read_i32().await?);
                                }
                                ::std::option::Option::Some(9)
                                    if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                                {
                                    var_9 =
                                        ::std::option::Option::Some(__protocol.read_i64().await?);
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
                            err.prepend_msg(&format!("decode struct `InteropMultiArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field left is required".to_string(),
                            ),
                        );
                    };
                    let ::std::option::Option::Some(var_9) = var_9 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field right is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        left: var_2,
                        right: var_9,
                    };
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
                    name: "InteropMultiArgsRecv",
                }) + __protocol.i32_field_len(::std::option::Option::Some(2), *&self.left)
                    + __protocol.i64_field_len(::std::option::Option::Some(9), *&self.right)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropMultiResultSend {
            fn default() -> Self {
                InteropMultiResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropMultiResultSend {
            Ok(i64),
        }

        impl ::pilota::thrift::Message for InteropMultiResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropMultiResultSend",
                })?;
                match self {
                    InteropMultiResultSend::Ok(value) => {
                        __protocol.write_i64_field(0, *value)?;
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
                                let field_ident = __protocol.read_i64()?;
                                __protocol.i64_len(*&field_ident);
                                ret = ::std::option::Option::Some(InteropMultiResultSend::Ok(
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
                                    let field_ident = __protocol.read_i64().await?;

                                    ret = ::std::option::Option::Some(InteropMultiResultSend::Ok(
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
                    name: "InteropMultiResultSend",
                }) + match self {
                    InteropMultiResultSend::Ok(value) => {
                        __protocol.i64_field_len(::std::option::Option::Some(0), *value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropZeroArgsRecv {}
        impl ::pilota::thrift::Message for InteropZeroArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropZeroArgsRecv",
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
                            "decode struct `InteropZeroArgsRecv` field(#{}) failed, caused by: ",
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
                            err.prepend_msg(&format!("decode struct `InteropZeroArgsRecv` field(#{}) failed, caused by: ", field_id));
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
                    name: "InteropZeroArgsRecv",
                }) + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropZeroResultSend {
            fn default() -> Self {
                InteropZeroResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropZeroResultSend {
            Ok(()),
        }

        impl ::pilota::thrift::Message for InteropZeroResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropZeroResultSend",
                })?;
                match self {
                    InteropZeroResultSend::Ok(value) => {}
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
                    ::std::result::Result::Ok(InteropZeroResultSend::Ok(()))
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
                        ::std::result::Result::Ok(InteropZeroResultSend::Ok(()))
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
                    name: "InteropZeroResultSend",
                }) + match self {
                    InteropZeroResultSend::Ok(value) => 0,
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropExplicitUnaryArgsRecv {
            pub request: super::grpc_parent::Message,
        }
        impl ::pilota::thrift::Message for InteropExplicitUnaryArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropExplicitUnaryArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
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
                        err.prepend_msg(&format!("decode struct `InteropExplicitUnaryArgsRecv` field(#{}) failed, caused by: ", field_id));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    ::std::option::Option::Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = ::std::option::Option::Some(<super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `InteropExplicitUnaryArgsRecv` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
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
                    name: "InteropExplicitUnaryArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropExplicitUnaryResultSend {
            fn default() -> Self {
                InteropExplicitUnaryResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropExplicitUnaryResultSend {
            Ok(super::grpc_parent::Message),
        }

        impl ::pilota::thrift::Message for InteropExplicitUnaryResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropExplicitUnaryResultSend",
                })?;
                match self {
                    InteropExplicitUnaryResultSend::Ok(value) => {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(
                                    InteropExplicitUnaryResultSend::Ok(field_ident),
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
                                    let field_ident = <super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(
                                        InteropExplicitUnaryResultSend::Ok(field_ident),
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
                    name: "InteropExplicitUnaryResultSend",
                }) + match self {
                    InteropExplicitUnaryResultSend::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct InteropUnaryArgsRecv {
            pub request: super::grpc_parent::Message,
        }
        impl ::pilota::thrift::Message for InteropUnaryArgsRecv {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "InteropUnaryArgsRecv",
                };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_struct_field(1, &self.request, ::pilota::thrift::TType::Struct)?;
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
                            "decode struct `InteropUnaryArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let ::std::option::Option::Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field request is required".to_string(),
                    ));
                };

                let data = Self { request: var_1 };
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
                    ::std::option::Option::Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_1 = ::std::option::Option::Some(<super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let ::std::option::Option::Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `InteropUnaryArgsRecv` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let ::std::option::Option::Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field request is required".to_string(),
                            ),
                        );
                    };

                    let data = Self { request: var_1 };
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
                    name: "InteropUnaryArgsRecv",
                }) + __protocol.struct_field_len(::std::option::Option::Some(1), &self.request)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for InteropUnaryResultSend {
            fn default() -> Self {
                InteropUnaryResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum InteropUnaryResultSend {
            Ok(super::grpc_parent::Message),
        }

        impl ::pilota::thrift::Message for InteropUnaryResultSend {
            fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut __PilotaProtocol,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "InteropUnaryResultSend",
                })?;
                match self {
                    InteropUnaryResultSend::Ok(value) => {
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
                                let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                                __protocol.struct_len(&field_ident);
                                ret = ::std::option::Option::Some(InteropUnaryResultSend::Ok(
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
                                    let field_ident = <super::grpc_parent::Message as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                                    ret = ::std::option::Option::Some(InteropUnaryResultSend::Ok(
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
                    name: "InteropUnaryResultSend",
                }) + match self {
                    InteropUnaryResultSend::Ok(value) => {
                        __protocol.struct_field_len(::std::option::Option::Some(0), value)
                    }
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
