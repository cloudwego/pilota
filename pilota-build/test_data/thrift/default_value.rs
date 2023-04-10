pub mod default_value {
    #![allow(warnings, clippy::all)]
    pub mod default_value {
        impl ::std::convert::From<B> for i32 {
            fn from(e: B) -> Self {
                e as _
            }
        }
        impl ::std::convert::TryFrom<i32> for B {
            type Error = ::pilota::EnumConvertError<i32>;
            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> Result<Self, ::pilota::EnumConvertError<i32>> {
                const Read: i32 = B::Read as i32;
                const Write: i32 = B::Write as i32;
                match v {
                    Read => ::std::result::Result::Ok(B::Read),
                    Write => ::std::result::Result::Ok(B::Write),
                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(v, "B")),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, :: pilota :: derivative :: Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum B {
            #[derivative(Default)]
            Read = 1i32,
            Write = 2i32,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(*self as i32)?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let value = protocol.read_i32()?;
                Ok(Self::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for B, value: {}", value),
                    )
                })?)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let value = protocol.read_i32().await?;
                Ok(Self::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for B, value: {}", value),
                    )
                })?)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_i32_len(*self as i32)
            }
        }
        impl Default for A {
            fn default() -> Self {
                A {
                    faststr: "hello world".into(),
                    string: "test".into(),
                    a: Some(false),
                    test_b: Default::default(),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub struct A {
            pub faststr: ::pilota::FastStr,
            pub string: ::std::string::String,
            pub a: ::std::option::Option<bool>,
            pub test_b: ::std::option::Option<B>,
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
                if let Some(value) = self.a.as_ref() {
                    protocol.write_bool_field(3i16, *value)?;
                };
                if let Some(value) = self.test_b.as_ref() {
                    protocol.write_i32_field(4i16, (*value).into())?;
                };
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut faststr = "hello world".into();
                let mut string = "test".into();
                let mut a = Some(false);
                let mut test_b = None;
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
                                faststr = protocol.read_faststr()?;
                            }
                            Some(2i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                string = protocol.read_string()?;
                            }
                            Some(3i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Bool =>
                            {
                                a = Some(protocol.read_bool()?);
                            }
                            Some(4i16)
                                if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                            {
                                test_b = Some(::pilota::thrift::Message::decode(protocol)?);
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
                let data = Self {
                    faststr,
                    string,
                    a,
                    test_b,
                };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut faststr = "hello world".into();
                let mut string = "test".into();
                let mut a = Some(false);
                let mut test_b = None;
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
                                faststr = protocol.read_faststr().await?;
                            }
                            Some(2i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                string = protocol.read_string().await?;
                            }
                            Some(3i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Bool =>
                            {
                                a = Some(protocol.read_bool().await?);
                            }
                            Some(4i16)
                                if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                            {
                                test_b =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
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
                let data = Self {
                    faststr,
                    string,
                    a,
                    test_b,
                };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + protocol.write_faststr_field_len(Some(1i16), &self.faststr)
                    + protocol.write_string_field_len(Some(2i16), &&self.string)
                    + self
                        .a
                        .as_ref()
                        .map_or(0, |value| protocol.write_bool_field_len(Some(3i16), *value))
                    + self.test_b.as_ref().map_or(0, |value| {
                        protocol.write_i32_field_len(Some(4i16), (*value).into())
                    })
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
    }
}
