pub mod r#gen {
    #![allow(warnings, clippy::all)]

    pub fn find_mod_file_descriptor(
        path: &str,
    ) -> Option<&'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor> {
        match path {

                "/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift" => Some(
            article::image::get_file_descriptor()),

                "/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift" => Some(
            author::get_file_descriptor()),

                "/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/cdn.thrift" => Some(
            article::image::cdn::get_file_descriptor()),

                "/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/common.thrift" => Some(
            common::get_file_descriptor()),

                _ => None,
            }
    }

    pub mod article {

        pub mod image {

            static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\r\0\x02\x0b\x0b\0\0\0\x02\0\0\0\x06common\0\0\0\rcommon.thrift\0\0\0\x03cdn\0\0\0\ncdn.thrift\r\0\x03\x0b\x0b\0\0\0\x01\0\0\0\x02rs\0\0\0\rarticle.image\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x0cImageService\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x08GetImage\x0c\0\x03\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x10GetImageResponse\0\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x03req\x0c\0\x03\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x0fGetImageRequest\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x02\0\x08\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\x0b\0\x07\0\0\0\0\0\x0f\0\x05\x0c\0\0\0\x03\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x05Image\x0f\0\x03\x0c\0\0\0\x04\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x02id\x0c\0\x03\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x03i64\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x03url\x0c\0\x03\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x03cdn\x0c\0\x03\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x07cdn.CDN\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x03\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x0bcommon_data\x0c\0\x03\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x11common.CommonData\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x04\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x0fGetImageRequest\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x02id\x0c\0\x03\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x03i64\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x10GetImageResponse\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x05image\x0c\0\x03\x0b\0\x01\0\0\0^/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/image.thrift\x0b\0\x02\0\0\0\x05Image\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x0f\0\x08\x0c\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\0\0");

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

                    let include_file_descriptor = crate::find_mod_file_descriptor(path)
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
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct Image {
                pub id: i64,

                pub url: ::pilota::FastStr,

                pub cdn: cdn::Cdn,

                pub common_data: super::super::common::CommonData,
            }
            impl ::pilota::thrift::Message for Image {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    __protocol: &mut T,
                ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Image" };

                    __protocol.write_struct_begin(&struct_ident)?;
                    __protocol.write_i64_field(1, *&self.id)?;
                    __protocol.write_faststr_field(2, (&self.url).clone())?;
                    __protocol.write_struct_field(3, &self.cdn, ::pilota::thrift::TType::Struct)?;
                    __protocol.write_struct_field(
                        4,
                        &self.common_data,
                        ::pilota::thrift::TType::Struct,
                    )?;
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    __protocol: &mut T,
                ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                {
                    #[allow(unused_imports)]
                    use ::pilota::{thrift::TLengthProtocolExt, Buf};

                    let mut var_1 = None;
                    let mut var_2 = None;
                    let mut var_3 = None;
                    let mut var_4 = None;

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
                                    if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                                {
                                    var_1 = Some(__protocol.read_i64()?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_2 = Some(__protocol.read_faststr()?);
                                }
                                Some(3)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_3 = Some(::pilota::thrift::Message::decode(__protocol)?);
                                }
                                Some(4)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_4 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                                "decode struct `Image` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end()?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field id is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field url is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field cdn is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field common_data is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        id: var_1,
                        url: var_2,
                        cdn: var_3,
                        common_data: var_4,
                    };
                    ::std::result::Result::Ok(data)
                }

                fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                    __protocol: &'a mut T,
                ) -> ::std::pin::Pin<
                    ::std::boxed::Box<
                        dyn ::std::future::Future<
                                Output = ::std::result::Result<
                                    Self,
                                    ::pilota::thrift::ThriftException,
                                >,
                            > + Send
                            + 'a,
                    >,
                > {
                    ::std::boxed::Box::pin(async move {
                        let mut var_1 = None;
                        let mut var_2 = None;
                        let mut var_3 = None;
                        let mut var_4 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64  => {
                    var_1 = Some(__protocol.read_i64().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_3 = Some(<cdn::Cdn as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_4 = Some(<super::super::common::CommonData as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `Image` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                        __protocol.read_struct_end().await?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field id is required".to_string(),
                                ),
                            );
                        };
                        let Some(var_2) = var_2 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field url is required".to_string(),
                                ),
                            );
                        };
                        let Some(var_3) = var_3 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field cdn is required".to_string(),
                                ),
                            );
                        };
                        let Some(var_4) = var_4 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field common_data is required".to_string(),
                                ),
                            );
                        };

                        let data = Self {
                            id: var_1,
                            url: var_2,
                            cdn: var_3,
                            common_data: var_4,
                        };
                        ::std::result::Result::Ok(data)
                    })
                }

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    __protocol
                        .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Image" })
                        + __protocol.i64_field_len(Some(1), *&self.id)
                        + __protocol.faststr_field_len(Some(2), &self.url)
                        + __protocol.struct_field_len(Some(3), &self.cdn)
                        + __protocol.struct_field_len(Some(4), &self.common_data)
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
            impl Image {
                pub fn get_descriptor(
                    &self,
                ) -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor
                {
                    let file_descriptor = get_file_descriptor();
                    file_descriptor.find_struct_by_name("Image").unwrap()
                }
            }
            pub mod cdn {

                static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0\\/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/cdn.thrift\r\0\x02\x0b\x0b\0\0\0\x01\0\0\0\x06common\0\0\0\rcommon.thrift\r\0\x03\x0b\x0b\0\0\0\x01\0\0\0\x02rs\0\0\0\x11article.image.cdn\x0f\0\x04\x0c\0\0\0\0\x0f\0\x05\x0c\0\0\0\x01\x0b\0\x01\0\0\0\\/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/cdn.thrift\x0b\0\x02\0\0\0\x03CDN\x0f\0\x03\x0c\0\0\0\x03\x0b\0\x01\0\0\0\\/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/cdn.thrift\x0b\0\x02\0\0\0\x02id\x0c\0\x03\x0b\0\x01\0\0\0\\/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/cdn.thrift\x0b\0\x02\0\0\0\x03i64\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0\\/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/cdn.thrift\x0b\0\x02\0\0\0\x03url\x0c\0\x03\x0b\0\x01\0\0\0\\/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/cdn.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0\\/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/cdn.thrift\x0b\0\x02\0\0\0\x0bcommon_data\x0c\0\x03\x0b\0\x01\0\0\0\\/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/cdn.thrift\x0b\0\x02\0\0\0\x11common.CommonData\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x03\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x0f\0\x08\x0c\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\0\0");

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

                        let include_file_descriptor = crate::find_mod_file_descriptor(path)
                            .expect("include file descriptor must exist");
                        ::pilota_thrift_reflect::service::Register::register(
                            include_file_descriptor.filepath.clone(),
                            include_file_descriptor.clone(),
                        );
                    }
                    descriptor
                });

                pub fn get_file_descriptor(
                ) -> &'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor
                {
                    &*FILE_DESCRIPTOR
                }
                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
                pub struct Cdn {
                    pub id: i64,

                    pub url: ::pilota::FastStr,

                    pub common_data: super::super::super::common::CommonData,
                }
                impl ::pilota::thrift::Message for Cdn {
                    fn encode<T: ::pilota::thrift::TOutputProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TOutputProtocolExt;
                        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Cdn" };

                        __protocol.write_struct_begin(&struct_ident)?;
                        __protocol.write_i64_field(1, *&self.id)?;
                        __protocol.write_faststr_field(2, (&self.url).clone())?;
                        __protocol.write_struct_field(
                            3,
                            &self.common_data,
                            ::pilota::thrift::TType::Struct,
                        )?;
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    }

                    fn decode<T: ::pilota::thrift::TInputProtocol>(
                        __protocol: &mut T,
                    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException>
                    {
                        #[allow(unused_imports)]
                        use ::pilota::{thrift::TLengthProtocolExt, Buf};

                        let mut var_1 = None;
                        let mut var_2 = None;
                        let mut var_3 = None;

                        let mut __pilota_decoding_field_id = None;

                        __protocol.read_struct_begin()?;
                        if let ::std::result::Result::Err(mut err) = (|| {
                            loop {
                                let field_ident = __protocol.read_field_begin()?;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    __protocol.field_stop_len();
                                    break;
                                } else {
                                    __protocol
                                        .field_begin_len(field_ident.field_type, field_ident.id);
                                }
                                __pilota_decoding_field_id = field_ident.id;
                                match field_ident.id {
                                    Some(1)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::I64 =>
                                    {
                                        var_1 = Some(__protocol.read_i64()?);
                                    }
                                    Some(2)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::Binary =>
                                    {
                                        var_2 = Some(__protocol.read_faststr()?);
                                    }
                                    Some(3)
                                        if field_ident.field_type
                                            == ::pilota::thrift::TType::Struct =>
                                    {
                                        var_3 =
                                            Some(::pilota::thrift::Message::decode(__protocol)?);
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
                                    "decode struct `Cdn` field(#{}) failed, caused by: ",
                                    field_id
                                ));
                            }
                            return ::std::result::Result::Err(err);
                        };
                        __protocol.read_struct_end()?;

                        let Some(var_1) = var_1 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field id is required".to_string(),
                                ),
                            );
                        };
                        let Some(var_2) = var_2 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field url is required".to_string(),
                                ),
                            );
                        };
                        let Some(var_3) = var_3 else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "field common_data is required".to_string(),
                                ),
                            );
                        };

                        let data = Self {
                            id: var_1,
                            url: var_2,
                            common_data: var_3,
                        };
                        ::std::result::Result::Ok(data)
                    }

                    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                        __protocol: &'a mut T,
                    ) -> ::std::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::std::future::Future<
                                    Output = ::std::result::Result<
                                        Self,
                                        ::pilota::thrift::ThriftException,
                                    >,
                                > + Send
                                + 'a,
                        >,
                    > {
                        ::std::boxed::Box::pin(async move {
                            let mut var_1 = None;
                            let mut var_2 = None;
                            let mut var_3 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64  => {
                    var_1 = Some(__protocol.read_i64().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_3 = Some(<super::super::super::common::CommonData as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `Cdn` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                            __protocol.read_struct_end().await?;

                            let Some(var_1) = var_1 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field id is required".to_string(),
                                    ),
                                );
                            };
                            let Some(var_2) = var_2 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field url is required".to_string(),
                                    ),
                                );
                            };
                            let Some(var_3) = var_3 else {
                                return ::std::result::Result::Err(
                                    ::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "field common_data is required".to_string(),
                                    ),
                                );
                            };

                            let data = Self {
                                id: var_1,
                                url: var_2,
                                common_data: var_3,
                            };
                            ::std::result::Result::Ok(data)
                        })
                    }

                    fn size<T: ::pilota::thrift::TLengthProtocol>(
                        &self,
                        __protocol: &mut T,
                    ) -> usize {
                        #[allow(unused_imports)]
                        use ::pilota::thrift::TLengthProtocolExt;
                        __protocol
                            .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Cdn" })
                            + __protocol.i64_field_len(Some(1), *&self.id)
                            + __protocol.faststr_field_len(Some(2), &self.url)
                            + __protocol.struct_field_len(Some(3), &self.common_data)
                            + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    }
                }
                impl Cdn {
                    pub fn get_descriptor(
                        &self,
                    ) -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor
                    {
                        let file_descriptor = get_file_descriptor();
                        file_descriptor.find_struct_by_name("Cdn").unwrap()
                    }
                }
            }
        }
    }

    pub mod author {

        static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\r\0\x02\x0b\x0b\0\0\0\x02\0\0\0\x06common\0\0\0\rcommon.thrift\0\0\0\x05image\0\0\0\x0cimage.thrift\r\0\x03\x0b\x0b\0\0\0\x01\0\0\0\x02rs\0\0\0\x06author\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\rAuthorService\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\tGetAuthor\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x11GetAuthorResponse\0\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x03req\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x10GetAuthorRequest\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x02\0\x08\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\x0b\0\x07\0\0\0\0\0\x0f\0\x05\x0c\0\0\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x06Author\x0f\0\x03\x0c\0\0\0\x05\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x02id\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x03i64\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x08username\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x05email\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x03\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x06avatar\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x0bimage.Image\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x04\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x0bcommon_data\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x11common.CommonData\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x05\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x10GetAuthorRequest\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x02id\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x03i64\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x11GetAuthorResponse\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x06author\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/author.thrift\x0b\0\x02\0\0\0\x06Author\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x0f\0\x08\x0c\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\0\0");

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

                let include_file_descriptor = crate::find_mod_file_descriptor(path)
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
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Author {
            pub id: i64,

            pub username: ::pilota::FastStr,

            pub email: ::pilota::FastStr,

            pub avatar: super::article::image::Image,

            pub common_data: super::common::CommonData,
        }
        impl ::pilota::thrift::Message for Author {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Author" };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_i64_field(1, *&self.id)?;
                __protocol.write_faststr_field(2, (&self.username).clone())?;
                __protocol.write_faststr_field(3, (&self.email).clone())?;
                __protocol.write_struct_field(4, &self.avatar, ::pilota::thrift::TType::Struct)?;
                __protocol.write_struct_field(
                    5,
                    &self.common_data,
                    ::pilota::thrift::TType::Struct,
                )?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;

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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                                var_1 = Some(__protocol.read_i64()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_2 = Some(__protocol.read_faststr()?);
                            }
                            Some(3)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_3 = Some(__protocol.read_faststr()?);
                            }
                            Some(4)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_4 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(5)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_5 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `Author` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field id is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field username is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field email is required".to_string(),
                    ));
                };
                let Some(var_4) = var_4 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field avatar is required".to_string(),
                    ));
                };
                let Some(var_5) = var_5 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field common_data is required".to_string(),
                    ));
                };

                let data = Self {
                    id: var_1,
                    username: var_2,
                    email: var_3,
                    avatar: var_4,
                    common_data: var_5,
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
                    let mut var_1 = None;
                    let mut var_2 = None;
                    let mut var_3 = None;
                    let mut var_4 = None;
                    let mut var_5 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64  => {
                    var_1 = Some(__protocol.read_i64().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_2 = Some(__protocol.read_faststr().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_3 = Some(__protocol.read_faststr().await?);

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_4 = Some(<super::article::image::Image as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_5 = Some(<super::common::CommonData as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `Author` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field id is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field username is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field email is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_4) = var_4 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field avatar is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_5) = var_5 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field common_data is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        id: var_1,
                        username: var_2,
                        email: var_3,
                        avatar: var_4,
                        common_data: var_5,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Author" })
                    + __protocol.i64_field_len(Some(1), *&self.id)
                    + __protocol.faststr_field_len(Some(2), &self.username)
                    + __protocol.faststr_field_len(Some(3), &self.email)
                    + __protocol.struct_field_len(Some(4), &self.avatar)
                    + __protocol.struct_field_len(Some(5), &self.common_data)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl Author {
            pub fn get_descriptor(
                &self,
            ) -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor {
                let file_descriptor = get_file_descriptor();
                file_descriptor.find_struct_by_name("Author").unwrap()
            }
        }
    }

    pub mod common {

        static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/common.thrift\r\0\x02\x0b\x0b\0\0\0\0\r\0\x03\x0b\x0b\0\0\0\x01\0\0\0\x02rs\0\0\0\x06common\x0f\0\x04\x0c\0\0\0\0\x0f\0\x05\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/common.thrift\x0b\0\x02\0\0\0\nCommonData\x0f\0\x03\x0c\0\0\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/common.thrift\x0b\0\x02\0\0\0\x02id\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/common.thrift\x0b\0\x02\0\0\0\x03i64\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/common.thrift\x0b\0\x02\0\0\0\x04name\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/common.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/common.thrift\x0b\0\x02\0\0\0\x0bdescription\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace/input/common.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x03\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x0f\0\x08\x0c\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\0\0");

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

                let include_file_descriptor = crate::find_mod_file_descriptor(path)
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
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct CommonData {
            pub id: i64,

            pub name: ::pilota::FastStr,

            pub description: ::pilota::FastStr,
        }
        impl ::pilota::thrift::Message for CommonData {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "CommonData" };

                __protocol.write_struct_begin(&struct_ident)?;
                __protocol.write_i64_field(1, *&self.id)?;
                __protocol.write_faststr_field(2, (&self.name).clone())?;
                __protocol.write_faststr_field(3, (&self.description).clone())?;
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;

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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                                var_1 = Some(__protocol.read_i64()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_2 = Some(__protocol.read_faststr()?);
                            }
                            Some(3)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_3 = Some(__protocol.read_faststr()?);
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
                            "decode struct `CommonData` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field id is required".to_string(),
                    ));
                };
                let Some(var_2) = var_2 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field name is required".to_string(),
                    ));
                };
                let Some(var_3) = var_3 else {
                    return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field description is required".to_string(),
                    ));
                };

                let data = Self {
                    id: var_1,
                    name: var_2,
                    description: var_3,
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
                    let mut var_1 = None;
                    let mut var_2 = None;
                    let mut var_3 = None;

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
                                    if field_ident.field_type == ::pilota::thrift::TType::I64 =>
                                {
                                    var_1 = Some(__protocol.read_i64().await?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_2 = Some(__protocol.read_faststr().await?);
                                }
                                Some(3)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_3 = Some(__protocol.read_faststr().await?);
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
                                "decode struct `CommonData` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let Some(var_1) = var_1 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field id is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_2) = var_2 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field name is required".to_string(),
                            ),
                        );
                    };
                    let Some(var_3) = var_3 else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "field description is required".to_string(),
                            ),
                        );
                    };

                    let data = Self {
                        id: var_1,
                        name: var_2,
                        description: var_3,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                __protocol
                    .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "CommonData" })
                    + __protocol.i64_field_len(Some(1), *&self.id)
                    + __protocol.faststr_field_len(Some(2), &self.name)
                    + __protocol.faststr_field_len(Some(3), &self.description)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl CommonData {
            pub fn get_descriptor(
                &self,
            ) -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor {
                let file_descriptor = get_file_descriptor();
                file_descriptor.find_struct_by_name("CommonData").unwrap()
            }
        }
    }
}
