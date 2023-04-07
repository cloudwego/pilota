pub mod default_value {
    #![allow(warnings, clippy::all)]
    pub mod default_value {
        impl Default for A {
            fn default() -> Self {
                A {
                    faststr: "hello world".into(),
                    string: "test".into(),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub struct A {
            pub faststr: ::pilota::FastStr,
            pub string: ::std::string::String,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_faststr_field(1i16, (&self.faststr).clone())?;
                protocol.write_string_field(2i16, &&self.string)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut faststr = Some("hello world".into());
                let mut string = Some("test".into());
                let mut __pilota_decoding_field_id = None;
                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        }
                        let field_id = field_ident.id;
                        __pilota_decoding_field_id = field_id;
                        match field_id {
                            Some(1i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                faststr = Some(protocol.read_faststr()?);
                            }
                            Some(2i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                string = Some(protocol.read_string()?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                        protocol.read_field_end()?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `A` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;
                let data = Self { faststr, string };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut faststr = Some("hello world".into());
                let mut string = Some("test".into());
                let mut __pilota_decoding_field_id = None;
                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        }
                        let field_id = field_ident.id;
                        __pilota_decoding_field_id = field_id;
                        match field_id {
                            Some(1i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                faststr = Some(protocol.read_faststr().await?);
                            }
                            Some(2i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                string = Some(protocol.read_string().await?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `A` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;
                let data = Self { faststr, string };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + protocol.write_faststr_field_len(Some(1i16), &self.faststr)
                    + protocol.write_string_field_len(Some(2i16), &&self.string)
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
    }
}
