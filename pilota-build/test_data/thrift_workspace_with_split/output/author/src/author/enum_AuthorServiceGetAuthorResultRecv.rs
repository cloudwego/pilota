
impl ::std::default::Default for AuthorServiceGetAuthorResultRecv {
    fn default() -> Self {
        AuthorServiceGetAuthorResultRecv::Ok(::std::default::Default::default())
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
pub enum AuthorServiceGetAuthorResultRecv {
    Ok(GetAuthorResponse),
}

impl ::pilota::thrift::Message for AuthorServiceGetAuthorResultRecv {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
            name: "AuthorServiceGetAuthorResultRecv",
        })?;
        match self {
            AuthorServiceGetAuthorResultRecv::Ok(value) => {
                __protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
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
        let mut ret = None;
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
                Some(0) => {
                    if ret.is_none() {
                        let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                        __protocol.struct_len(&field_ident);
                        ret = Some(AuthorServiceGetAuthorResultRecv::Ok(field_ident));
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
        if let Some(ret) = ret {
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
            let mut ret = None;
            __protocol.read_struct_begin().await?;
            loop {
                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    break;
                } else {
                }
                match field_ident.id {
                    Some(0) => {
                        if ret.is_none() {
                            let field_ident =
                                <GetAuthorResponse as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(AuthorServiceGetAuthorResultRecv::Ok(field_ident));
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
            if let Some(ret) = ret {
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
            name: "AuthorServiceGetAuthorResultRecv",
        }) + match self {
            AuthorServiceGetAuthorResultRecv::Ok(value) => {
                __protocol.struct_field_len(Some(0), value)
            }
        } + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
