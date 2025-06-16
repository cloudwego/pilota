pub mod must_gen_items {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};

    pub mod must_gen_items {
<<<<<<< HEAD
<<<<<<< HEAD
        use ::pilota::{Buf as _, BufMut as _};
=======

        static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0P/data02/home/giggle/projects/pilota/pilota-build/test_data/must_gen_items.thrift\r\0\x02\x0b\x0b\0\0\0\0\r\0\x03\x0b\x0b\0\0\0\0\x0f\0\x04\x0c\0\0\0\0\x0f\0\x05\x0c\0\0\0\x02\x0b\0\x01\0\0\0P/data02/home/giggle/projects/pilota/pilota-build/test_data/must_gen_items.thrift\x0b\0\x02\0\0\0\x01A\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0P/data02/home/giggle/projects/pilota/pilota-build/test_data/must_gen_items.thrift\x0b\0\x02\0\0\0\x01a\x0c\0\x03\x0b\0\x01\0\0\0P/data02/home/giggle/projects/pilota/pilota-build/test_data/must_gen_items.thrift\x0b\0\x02\0\0\0\x03i32\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0P/data02/home/giggle/projects/pilota/pilota-build/test_data/must_gen_items.thrift\x0b\0\x02\0\0\0\x01b\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0P/data02/home/giggle/projects/pilota/pilota-build/test_data/must_gen_items.thrift\x0b\0\x02\0\0\0\x01a\x0c\0\x03\x0b\0\x01\0\0\0P/data02/home/giggle/projects/pilota/pilota-build/test_data/must_gen_items.thrift\x0b\0\x02\0\0\0\x01A\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x0f\0\x08\x0c\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\0\0");

        pub static FILE_DESCRIPTOR: ::std::sync::LazyLock<
            ::pilota_thrift_reflect::thrift_reflection::FileDescriptor,
        > = ::std::sync::LazyLock::new(|| {
            let descriptor =
                ::pilota_thrift_reflect::thrift_reflection::FileDescriptor::deserialize(
                    FILE_DESCRIPTOR_BYTES.clone(),
                )
                .expect("Failed to decode file descriptor");
            ::pilota_thrift_reflect::service::Register::register(
                descriptor.filepath.clone(),
                descriptor.clone(),
            );

            for (key, include) in descriptor.includes.iter() {
                let path = include.as_str();
                if ::pilota_thrift_reflect::service::Register::contains(path) {
                    continue;
                }

                let include_file_descriptor = super::find_mod_file_descriptor(path)
                    .expect("include file descriptor must exist");
                ::pilota_thrift_reflect::service::Register::register(
                    include_file_descriptor.filepath.clone(),
                    include_file_descriptor.clone(),
                );
            }
            descriptor
        });

        pub fn get_file_descriptor(
        ) -> &'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor {
            &*FILE_DESCRIPTOR
        }
>>>>>>> ae87e76 (feat(pilota-build): codegen file descriptor)
=======
>>>>>>> 0314c00 (feat(pilota-build): codegen fieldmask)
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub a: ::std::option::Option<i32>,
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
                if let Some(value) = self.a.as_ref() {
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

                let mut var_1 = None;

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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                var_1 = Some(__protocol.read_i32()?);
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

                let data = Self { a: var_1 };
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
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    var_1 = Some(__protocol.read_i32().await?);
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

                    let data = Self { a: var_1 };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + self
                        .a
                        .as_ref()
                        .map_or(0, |value| __protocol.i32_field_len(Some(1), *value))
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
