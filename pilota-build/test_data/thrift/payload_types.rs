pub mod payload_types {
    #![allow(warnings, clippy::all)]

    pub mod payload_types {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Message {
            pub payload: ::std::option::Option<::pilota::Bytes>,

            pub sequence: ::std::option::Option<i64>,
        }
        impl ::pilota::thrift::Message for Message {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
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

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
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
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Failure {
            pub message: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::thrift::Message for Failure {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
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
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
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

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
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
    }
}
