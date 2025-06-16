pub mod fieldmask {
    #![allow(warnings, clippy::all)]

    pub fn find_mod_file_descriptor(
        path: &str,
    ) -> Option<&'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor> {
        match path {
            r"/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift" => {
                Some(fieldmask::get_file_descriptor())
            }
            _ => None,
        }
    }

    pub mod fieldmask {

        static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\r\0\x02\x0b\x0b\0\0\0\0\r\0\x03\x0b\x0b\0\0\0\x01\0\0\0\x04rust\0\0\0\x03std\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x04Test\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x04test\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x08Response\0\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03req\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x07Request\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x02\0\x08\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\x0b\0\x07\0\0\0\0\0\x0f\0\x05\x0c\0\0\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x01A\x0f\0\x03\x0c\0\0\0\x02\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x01a\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03i32\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x01b\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x07Request\x0f\0\x03\x0c\0\0\0\x11\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02f1\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x04bool\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02f2\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02i8\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02f3\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03i16\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x03\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02f4\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03i32\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x04\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02f5\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03i64\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x05\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02f6\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x06double\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x06\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02f7\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x07\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02f8\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x06binary\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x08\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x02f9\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03i32\0\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\t\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03f10\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03set\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x06string\0\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\n\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03f11\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x01A\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x0b\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03f12\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03i32\0\0\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x0c\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03f13\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x01A\0\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\r\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03f14\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03map\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03i32\0\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x06string\0\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x0e\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03f15\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03map\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x06string\0\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x01A\0\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x0f\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03f16\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03map\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x06string\0\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x01A\0\0\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x10\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03f17\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03map\x0c\0\x03\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x06string\0\x0c\0\x04\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x03i32\0\0\0\x0b\0\x04\0\0\0\x07default\x08\0\x05\0\0\0\x11\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0A/data02/home/giggle/projects/pilota/examples/idl/fieldmask.thrift\x0b\0\x02\0\0\0\x08Response\x0f\0\x03\x0c\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x0f\0\x08\x0c\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\0\0");

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

        pub fn get_file_descriptor()
        -> &'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor {
            &*FILE_DESCRIPTOR
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub a: ::std::option::Option<i32>,

            pub b: ::std::option::Option<::pilota::FastStr>,
            pub _field_mask: ::std::option::Option<::pilota_thrift_fieldmask::FieldMask>,
        }
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.all() {
                        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };
                        __protocol.write_struct_begin(&struct_ident)?;
                        if let Some(value) = self.a.as_ref() {
                            if struct_fm.field(1).is_none() {
                                __protocol.write_i32_field(1, *value)?;
                            }
                        }
                        if let Some(value) = self.b.as_ref() {
                            if struct_fm.field(2).is_none() {
                                __protocol.write_faststr_field(2, (value).clone())?;
                            }
                        }
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    } else {
                        ::std::result::Result::Ok(())
                    }
                } else {
                    let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                    __protocol.write_struct_begin(&struct_ident)?;
                    if let Some(value) = self.a.as_ref() {
                        __protocol.write_i32_field(1, *value)?;
                    }
                    if let Some(value) = self.b.as_ref() {
                        __protocol.write_faststr_field(2, (value).clone())?;
                    }
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = None;
                let mut var_2 = None;

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
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_2 = Some(__protocol.read_faststr()?);
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

                let data = Self {
                    a: var_1,
                    b: var_2,
                    _field_mask: ::std::option::Option::None,
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
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    var_2 = Some(__protocol.read_faststr().await?);
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

                    let data = Self {
                        a: var_1,
                        b: var_2,
                        _field_mask: ::std::option::Option::None,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.all() {
                        __protocol
                            .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                            + self.a.as_ref().map_or(0, |value| {
                                if struct_fm.field(1).is_none() {
                                    __protocol.i32_field_len(Some(1), *value)
                                } else {
                                    0
                                }
                            })
                            + self.b.as_ref().map_or(0, |value| {
                                if struct_fm.field(2).is_none() {
                                    __protocol.faststr_field_len(Some(2), value)
                                } else {
                                    0
                                }
                            })
                            + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    } else {
                        0
                    }
                } else {
                    __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                        + self
                            .a
                            .as_ref()
                            .map_or(0, |value| __protocol.i32_field_len(Some(1), *value))
                        + self
                            .b
                            .as_ref()
                            .map_or(0, |value| __protocol.faststr_field_len(Some(2), value))
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
        }
        impl A {
            pub fn get_descriptor()
            -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor {
                let file_descriptor = get_file_descriptor();
                file_descriptor.find_struct_by_name("A").unwrap()
            }

            pub fn set_field_mask(&mut self, field_mask: ::pilota_thrift_fieldmask::FieldMask) {
                self._field_mask = Some(field_mask.clone());
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestTestArgsSend {
            pub req: Request,
        }
        impl ::pilota::thrift::Message for TestTestArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTestArgsSend",
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
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `TestTestArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
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
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(
                                        <Request as ::pilota::thrift::Message>::decode_async(
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
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `TestTestArgsSend` field(#{}) failed, caused by: ",
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
                    name: "TestTestArgsSend",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct Request {
            pub f1: ::std::option::Option<bool>,

            pub f2: ::std::option::Option<i8>,

            pub f3: ::std::option::Option<i16>,

            pub f4: ::std::option::Option<i32>,

            pub f5: ::std::option::Option<i64>,

            pub f6: ::std::option::Option<f64>,

            pub f7: ::std::option::Option<::pilota::FastStr>,

            pub f8: ::std::option::Option<::pilota::Bytes>,

            pub f9: ::std::option::Option<::std::vec::Vec<i32>>,

            pub f10: ::std::option::Option<::pilota::AHashSet<::pilota::FastStr>>,

            pub f11: ::std::option::Option<A>,

            pub f12: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<i32>>>,

            pub f13: ::std::option::Option<::std::vec::Vec<A>>,

            pub f14: ::std::option::Option<::pilota::AHashMap<i32, ::pilota::FastStr>>,

            pub f15: ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, A>>,

            pub f16:
                ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<A>>>,

            pub f17:
                ::std::option::Option<::std::vec::Vec<::pilota::AHashMap<::pilota::FastStr, i32>>>,
            pub _field_mask: ::std::option::Option<::pilota_thrift_fieldmask::FieldMask>,
        }
        impl ::pilota::thrift::Message for Request {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.all() {
                        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Request" };
                        __protocol.write_struct_begin(&struct_ident)?;
                        if let Some(value) = self.f1.as_ref() {
                            if struct_fm.field(1).is_none() {
                                __protocol.write_bool_field(1, *value)?;
                            }
                        }
                        if let Some(value) = self.f2.as_ref() {
                            if struct_fm.field(2).is_none() {
                                __protocol.write_i8_field(2, *value)?;
                            }
                        }
                        if let Some(value) = self.f3.as_ref() {
                            if struct_fm.field(3).is_none() {
                                __protocol.write_i16_field(3, *value)?;
                            }
                        }
                        if let Some(value) = self.f4.as_ref() {
                            if struct_fm.field(4).is_none() {
                                __protocol.write_i32_field(4, *value)?;
                            }
                        }
                        if let Some(value) = self.f5.as_ref() {
                            if struct_fm.field(5).is_none() {
                                __protocol.write_i64_field(5, *value)?;
                            }
                        }
                        if let Some(value) = self.f6.as_ref() {
                            if struct_fm.field(6).is_none() {
                                __protocol.write_double_field(6, *value)?;
                            }
                        }
                        if let Some(value) = self.f7.as_ref() {
                            if struct_fm.field(7).is_none() {
                                __protocol.write_faststr_field(7, (value).clone())?;
                            }
                        }
                        if let Some(value) = self.f8.as_ref() {
                            if struct_fm.field(8).is_none() {
                                __protocol.write_bytes_field(8, (value).clone())?;
                            }
                        }
                        if let Some(value) = self.f9.as_ref() {
                            if let Some(list_fm) = struct_fm.field(9) {
                                if !list_fm.all() {
                                    __protocol
                                        .write_field_begin(::pilota::thrift::TType::List, 9)?;
                                    __protocol.write_list_begin(
                                        ::pilota::thrift::TListIdentifier {
                                            element_type: ::pilota::thrift::TType::I32,
                                            size: {
                                                let mut count = (*value).len() as i32;
                                                for idx in 0..(*value).len() {
                                                    if let Some(item_fm) = list_fm.int(idx as i32) {
                                                        if item_fm.all() {
                                                            count -= 1;
                                                        }
                                                    }
                                                }
                                                count as usize
                                            },
                                        },
                                    )?;
                                    let mut idx = 0;
                                    for val in value {
                                        let item_fm = list_fm.int(idx as i32);
                                        if item_fm.is_none() {
                                            __protocol.write_i32(*val)?;
                                        }
                                        idx += 1;
                                    }
                                    __protocol.write_list_end()?;
                                    __protocol.write_field_end()?;
                                }
                            } else {
                                __protocol.write_list_field(
                                    9,
                                    ::pilota::thrift::TType::I32,
                                    &value,
                                    |__protocol, val| {
                                        __protocol.write_i32(*val)?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                            }
                        }
                        if let Some(value) = self.f10.as_ref() {
                            __protocol.write_set_field(
                                10,
                                ::pilota::thrift::TType::Binary,
                                &value,
                                |__protocol, val| {
                                    __protocol.write_faststr((val).clone())?;
                                    ::std::result::Result::Ok(())
                                },
                            )?;
                        }
                        if let Some(value) = self.f11.as_ref() {
                            if let Some(struct_fm) = struct_fm.field(11) {
                                if !struct_fm.all() {
                                    __protocol.write_struct_field(
                                        11,
                                        value,
                                        ::pilota::thrift::TType::Struct,
                                    )?;
                                }
                            } else {
                                __protocol.write_struct_field(
                                    11,
                                    value,
                                    ::pilota::thrift::TType::Struct,
                                )?;
                            }
                        }
                        if let Some(value) = self.f12.as_ref() {
                            if let Some(list_fm) = struct_fm.field(12) {
                                if !list_fm.all() {
                                    __protocol
                                        .write_field_begin(::pilota::thrift::TType::List, 12)?;
                                    __protocol.write_list_begin(
                                        ::pilota::thrift::TListIdentifier {
                                            element_type: ::pilota::thrift::TType::List,
                                            size: {
                                                let mut count = (*value).len() as i32;
                                                for idx in 0..(*value).len() {
                                                    if let Some(item_fm) = list_fm.int(idx as i32) {
                                                        if item_fm.all() {
                                                            count -= 1;
                                                        }
                                                    }
                                                }
                                                count as usize
                                            },
                                        },
                                    )?;
                                    let mut idx = 0;
                                    for val in value {
                                        let item_fm = list_fm.int(idx as i32);
                                        if let Some(list_fm) = item_fm {
                                            if !list_fm.all() {
                                                __protocol.write_list_begin(
                                                    ::pilota::thrift::TListIdentifier {
                                                        element_type: ::pilota::thrift::TType::I32,
                                                        size: {
                                                            let mut count = val.len() as i32;
                                                            let mut idx = 0;
                                                            for _ in val {
                                                                if let Some(item_fm) =
                                                                    list_fm.int(idx as i32)
                                                                {
                                                                    if item_fm.all() {
                                                                        count -= 1;
                                                                    }
                                                                }
                                                                idx += 1;
                                                            }
                                                            count as usize
                                                        },
                                                    },
                                                )?;
                                                let mut idx = 0;
                                                for val in val {
                                                    let item_fm = list_fm.int(idx as i32);
                                                    if item_fm.is_none() {
                                                        __protocol.write_i32(*val)?;
                                                    }
                                                    idx += 1;
                                                }
                                                __protocol.write_list_end()?;
                                            }
                                        } else {
                                            __protocol.write_list(
                                                ::pilota::thrift::TType::I32,
                                                &val,
                                                |__protocol, val| {
                                                    __protocol.write_i32(*val)?;
                                                    ::std::result::Result::Ok(())
                                                },
                                            )?;
                                        }
                                        idx += 1;
                                    }
                                    __protocol.write_list_end()?;
                                    __protocol.write_field_end()?;
                                }
                            } else {
                                __protocol.write_list_field(
                                    12,
                                    ::pilota::thrift::TType::List,
                                    &value,
                                    |__protocol, val| {
                                        __protocol.write_list(
                                            ::pilota::thrift::TType::I32,
                                            &val,
                                            |__protocol, val| {
                                                __protocol.write_i32(*val)?;
                                                ::std::result::Result::Ok(())
                                            },
                                        )?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                            }
                        }
                        if let Some(value) = self.f13.as_ref() {
                            if let Some(list_fm) = struct_fm.field(13) {
                                if !list_fm.all() {
                                    __protocol
                                        .write_field_begin(::pilota::thrift::TType::List, 13)?;
                                    __protocol.write_list_begin(
                                        ::pilota::thrift::TListIdentifier {
                                            element_type: ::pilota::thrift::TType::Struct,
                                            size: {
                                                let mut count = (*value).len() as i32;
                                                for idx in 0..(*value).len() {
                                                    if let Some(item_fm) = list_fm.int(idx as i32) {
                                                        if item_fm.all() {
                                                            count -= 1;
                                                        }
                                                    }
                                                }
                                                count as usize
                                            },
                                        },
                                    )?;
                                    let mut idx = 0;
                                    for val in value {
                                        let item_fm = list_fm.int(idx as i32);
                                        if let Some(fm) = item_fm {
                                            if !fm.all() {
                                                __protocol.write_struct(val)?;
                                            }
                                        } else {
                                            __protocol.write_struct(val)?;
                                        }
                                        idx += 1;
                                    }
                                    __protocol.write_list_end()?;
                                    __protocol.write_field_end()?;
                                }
                            } else {
                                __protocol.write_list_field(
                                    13,
                                    ::pilota::thrift::TType::Struct,
                                    &value,
                                    |__protocol, val| {
                                        __protocol.write_struct(val)?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                            }
                        }
                        if let Some(value) = self.f14.as_ref() {
                            if let Some(map_fm) = struct_fm.field(14) {
                                if !map_fm.all() {
                                    __protocol
                                        .write_field_begin(::pilota::thrift::TType::Map, 14)?;
                                    __protocol.write_map_begin(
                                        ::pilota::thrift::TMapIdentifier {
                                            key_type: ::pilota::thrift::TType::I32,
                                            value_type: ::pilota::thrift::TType::Binary,
                                            size: {
                                                let mut count = (*value).len() as i32;
                                                for key in value.keys() {
                                                    if let Some(item_fm) = map_fm.int(*key as i32) {
                                                        if item_fm.all() {
                                                            count -= 1;
                                                        }
                                                    }
                                                }
                                                count as usize
                                            },
                                        },
                                    )?;
                                    for (key, val) in value {
                                        let item_fm = map_fm.int(*key as i32);
                                        if let Some(fm) = item_fm {
                                            if !fm.all() {
                                                __protocol.write_i32(*key)?;
                                                let item_fm = Some(fm);
                                                if item_fm.is_none() {
                                                    __protocol.write_faststr((val).clone())?;
                                                }
                                            }
                                        } else {
                                            __protocol.write_i32(*key)?;
                                            __protocol.write_faststr((val).clone())?;
                                        }
                                    }
                                    __protocol.write_map_end()?;
                                    __protocol.write_field_end()?;
                                }
                            } else {
                                __protocol.write_map_field(
                                    14,
                                    ::pilota::thrift::TType::I32,
                                    ::pilota::thrift::TType::Binary,
                                    &value,
                                    |__protocol, key| {
                                        __protocol.write_i32(*key)?;
                                        ::std::result::Result::Ok(())
                                    },
                                    |__protocol, val| {
                                        __protocol.write_faststr((val).clone())?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                            }
                        }
                        if let Some(value) = self.f15.as_ref() {
                            if let Some(map_fm) = struct_fm.field(15) {
                                if !map_fm.all() {
                                    __protocol
                                        .write_field_begin(::pilota::thrift::TType::Map, 15)?;
                                    __protocol.write_map_begin(
                                        ::pilota::thrift::TMapIdentifier {
                                            key_type: ::pilota::thrift::TType::Binary,
                                            value_type: ::pilota::thrift::TType::Struct,
                                            size: {
                                                let mut count = value.len() as i32;
                                                for key in (*value).keys() {
                                                    if let Some(item_fm) = map_fm.str(key.as_str())
                                                    {
                                                        if item_fm.all() {
                                                            count -= 1;
                                                        }
                                                    }
                                                }
                                                count as usize
                                            },
                                        },
                                    )?;
                                    for (key, val) in value {
                                        let item_fm = map_fm.str(key);
                                        if let Some(fm) = item_fm {
                                            if !fm.all() {
                                                __protocol.write_faststr((key).clone())?;
                                                let item_fm = Some(fm);
                                                if let Some(fm) = item_fm {
                                                    if !fm.all() {
                                                        __protocol.write_struct(val)?;
                                                    }
                                                } else {
                                                    __protocol.write_struct(val)?;
                                                }
                                            }
                                        } else {
                                            __protocol.write_faststr((key).clone())?;
                                            __protocol.write_struct(val)?;
                                        }
                                    }
                                    __protocol.write_map_end()?;
                                    __protocol.write_field_end()?;
                                }
                            } else {
                                __protocol.write_map_field(
                                    15,
                                    ::pilota::thrift::TType::Binary,
                                    ::pilota::thrift::TType::Struct,
                                    &value,
                                    |__protocol, key| {
                                        __protocol.write_faststr((key).clone())?;
                                        ::std::result::Result::Ok(())
                                    },
                                    |__protocol, val| {
                                        __protocol.write_struct(val)?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                            }
                        }
                        if let Some(value) = self.f16.as_ref() {
                            if let Some(map_fm) = struct_fm.field(16) {
                                if !map_fm.all() {
                                    __protocol
                                        .write_field_begin(::pilota::thrift::TType::Map, 16)?;
                                    __protocol.write_map_begin(
                                        ::pilota::thrift::TMapIdentifier {
                                            key_type: ::pilota::thrift::TType::Binary,
                                            value_type: ::pilota::thrift::TType::List,
                                            size: {
                                                let mut count = value.len() as i32;
                                                for key in (*value).keys() {
                                                    if let Some(item_fm) = map_fm.str(key.as_str())
                                                    {
                                                        if item_fm.all() {
                                                            count -= 1;
                                                        }
                                                    }
                                                }
                                                count as usize
                                            },
                                        },
                                    )?;
                                    for (key, val) in value {
                                        let item_fm = map_fm.str(key);
                                        if let Some(fm) = item_fm {
                                            if !fm.all() {
                                                __protocol.write_faststr((key).clone())?;
                                                let item_fm = Some(fm);
                                                if let Some(list_fm) = item_fm {
                                                    if !list_fm.all() {
                                                        __protocol.write_list_begin(
                                                            ::pilota::thrift::TListIdentifier {
                                                                element_type:
                                                                    ::pilota::thrift::TType::Struct,
                                                                size: {
                                                                    let mut count =
                                                                        val.len() as i32;
                                                                    let mut idx = 0;
                                                                    for _ in val {
                                                                        if let Some(item_fm) =
                                                                            list_fm.int(idx as i32)
                                                                        {
                                                                            if item_fm.all() {
                                                                                count -= 1;
                                                                            }
                                                                        }
                                                                        idx += 1;
                                                                    }
                                                                    count as usize
                                                                },
                                                            },
                                                        )?;
                                                        let mut idx = 0;
                                                        for val in val {
                                                            let item_fm = list_fm.int(idx as i32);
                                                            if let Some(fm) = item_fm {
                                                                if !fm.all() {
                                                                    __protocol.write_struct(val)?;
                                                                }
                                                            } else {
                                                                __protocol.write_struct(val)?;
                                                            }
                                                            idx += 1;
                                                        }
                                                        __protocol.write_list_end()?;
                                                    }
                                                } else {
                                                    __protocol.write_list(
                                                        ::pilota::thrift::TType::Struct,
                                                        &val,
                                                        |__protocol, val| {
                                                            __protocol.write_struct(val)?;
                                                            ::std::result::Result::Ok(())
                                                        },
                                                    )?;
                                                }
                                            }
                                        } else {
                                            __protocol.write_faststr((key).clone())?;
                                            __protocol.write_list(
                                                ::pilota::thrift::TType::Struct,
                                                &val,
                                                |__protocol, val| {
                                                    __protocol.write_struct(val)?;
                                                    ::std::result::Result::Ok(())
                                                },
                                            )?;
                                        }
                                    }
                                    __protocol.write_map_end()?;
                                    __protocol.write_field_end()?;
                                }
                            } else {
                                __protocol.write_map_field(
                                    16,
                                    ::pilota::thrift::TType::Binary,
                                    ::pilota::thrift::TType::List,
                                    &value,
                                    |__protocol, key| {
                                        __protocol.write_faststr((key).clone())?;
                                        ::std::result::Result::Ok(())
                                    },
                                    |__protocol, val| {
                                        __protocol.write_list(
                                            ::pilota::thrift::TType::Struct,
                                            &val,
                                            |__protocol, val| {
                                                __protocol.write_struct(val)?;
                                                ::std::result::Result::Ok(())
                                            },
                                        )?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                            }
                        }
                        if let Some(value) = self.f17.as_ref() {
                            if let Some(list_fm) = struct_fm.field(17) {
                                if !list_fm.all() {
                                    __protocol
                                        .write_field_begin(::pilota::thrift::TType::List, 17)?;
                                    __protocol.write_list_begin(
                                        ::pilota::thrift::TListIdentifier {
                                            element_type: ::pilota::thrift::TType::Map,
                                            size: {
                                                let mut count = (*value).len() as i32;
                                                for idx in 0..(*value).len() {
                                                    if let Some(item_fm) = list_fm.int(idx as i32) {
                                                        if item_fm.all() {
                                                            count -= 1;
                                                        }
                                                    }
                                                }
                                                count as usize
                                            },
                                        },
                                    )?;
                                    let mut idx = 0;
                                    for val in value {
                                        let item_fm = list_fm.int(idx as i32);
                                        if let Some(map_fm) = item_fm {
                                            if !map_fm.all() {
                                                __protocol.write_map_begin(
                                                    ::pilota::thrift::TMapIdentifier {
                                                        key_type: ::pilota::thrift::TType::Binary,
                                                        value_type: ::pilota::thrift::TType::I32,
                                                        size: {
                                                            let mut count = val.len() as i32;
                                                            for (key, item) in val {
                                                                if let Some(item_fm) =
                                                                    map_fm.str(key.as_str())
                                                                {
                                                                    if item_fm.all() {
                                                                        count -= 1;
                                                                    }
                                                                }
                                                            }
                                                            count as usize
                                                        },
                                                    },
                                                )?;
                                                for (key, val) in val {
                                                    let item_fm = map_fm.str(key.as_str());
                                                    if let Some(fm) = item_fm {
                                                        if !fm.all() {
                                                            __protocol
                                                                .write_faststr((key).clone())?;
                                                            let item_fm = Some(fm);
                                                            if item_fm.is_none() {
                                                                __protocol.write_i32(*val)?;
                                                            }
                                                        }
                                                    } else {
                                                        __protocol.write_faststr((key).clone())?;
                                                        __protocol.write_i32(*val)?;
                                                    }
                                                }
                                                __protocol.write_map_end()?;
                                            }
                                        } else {
                                            __protocol.write_map(
                                                ::pilota::thrift::TType::Binary,
                                                ::pilota::thrift::TType::I32,
                                                &val,
                                                |__protocol, key| {
                                                    __protocol.write_faststr((key).clone())?;
                                                    ::std::result::Result::Ok(())
                                                },
                                                |__protocol, val| {
                                                    __protocol.write_i32(*val)?;
                                                    ::std::result::Result::Ok(())
                                                },
                                            )?;
                                        }
                                        idx += 1;
                                    }
                                    __protocol.write_list_end()?;
                                    __protocol.write_field_end()?;
                                }
                            } else {
                                __protocol.write_list_field(
                                    17,
                                    ::pilota::thrift::TType::Map,
                                    &value,
                                    |__protocol, val| {
                                        __protocol.write_map(
                                            ::pilota::thrift::TType::Binary,
                                            ::pilota::thrift::TType::I32,
                                            &val,
                                            |__protocol, key| {
                                                __protocol.write_faststr((key).clone())?;
                                                ::std::result::Result::Ok(())
                                            },
                                            |__protocol, val| {
                                                __protocol.write_i32(*val)?;
                                                ::std::result::Result::Ok(())
                                            },
                                        )?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                            }
                        }
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    } else {
                        ::std::result::Result::Ok(())
                    }
                } else {
                    let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Request" };

                    __protocol.write_struct_begin(&struct_ident)?;
                    if let Some(value) = self.f1.as_ref() {
                        __protocol.write_bool_field(1, *value)?;
                    }
                    if let Some(value) = self.f2.as_ref() {
                        __protocol.write_i8_field(2, *value)?;
                    }
                    if let Some(value) = self.f3.as_ref() {
                        __protocol.write_i16_field(3, *value)?;
                    }
                    if let Some(value) = self.f4.as_ref() {
                        __protocol.write_i32_field(4, *value)?;
                    }
                    if let Some(value) = self.f5.as_ref() {
                        __protocol.write_i64_field(5, *value)?;
                    }
                    if let Some(value) = self.f6.as_ref() {
                        __protocol.write_double_field(6, *value)?;
                    }
                    if let Some(value) = self.f7.as_ref() {
                        __protocol.write_faststr_field(7, (value).clone())?;
                    }
                    if let Some(value) = self.f8.as_ref() {
                        __protocol.write_bytes_field(8, (value).clone())?;
                    }
                    if let Some(value) = self.f9.as_ref() {
                        __protocol.write_list_field(
                            9,
                            ::pilota::thrift::TType::I32,
                            &value,
                            |__protocol, val| {
                                __protocol.write_i32(*val)?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                    if let Some(value) = self.f10.as_ref() {
                        __protocol.write_set_field(
                            10,
                            ::pilota::thrift::TType::Binary,
                            &value,
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                    if let Some(value) = self.f11.as_ref() {
                        __protocol.write_struct_field(
                            11,
                            value,
                            ::pilota::thrift::TType::Struct,
                        )?;
                    }
                    if let Some(value) = self.f12.as_ref() {
                        __protocol.write_list_field(
                            12,
                            ::pilota::thrift::TType::List,
                            &value,
                            |__protocol, val| {
                                __protocol.write_list(
                                    ::pilota::thrift::TType::I32,
                                    &val,
                                    |__protocol, val| {
                                        __protocol.write_i32(*val)?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                    if let Some(value) = self.f13.as_ref() {
                        __protocol.write_list_field(
                            13,
                            ::pilota::thrift::TType::Struct,
                            &value,
                            |__protocol, val| {
                                __protocol.write_struct(val)?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                    if let Some(value) = self.f14.as_ref() {
                        __protocol.write_map_field(
                            14,
                            ::pilota::thrift::TType::I32,
                            ::pilota::thrift::TType::Binary,
                            &value,
                            |__protocol, key| {
                                __protocol.write_i32(*key)?;
                                ::std::result::Result::Ok(())
                            },
                            |__protocol, val| {
                                __protocol.write_faststr((val).clone())?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                    if let Some(value) = self.f15.as_ref() {
                        __protocol.write_map_field(
                            15,
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::Struct,
                            &value,
                            |__protocol, key| {
                                __protocol.write_faststr((key).clone())?;
                                ::std::result::Result::Ok(())
                            },
                            |__protocol, val| {
                                __protocol.write_struct(val)?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                    if let Some(value) = self.f16.as_ref() {
                        __protocol.write_map_field(
                            16,
                            ::pilota::thrift::TType::Binary,
                            ::pilota::thrift::TType::List,
                            &value,
                            |__protocol, key| {
                                __protocol.write_faststr((key).clone())?;
                                ::std::result::Result::Ok(())
                            },
                            |__protocol, val| {
                                __protocol.write_list(
                                    ::pilota::thrift::TType::Struct,
                                    &val,
                                    |__protocol, val| {
                                        __protocol.write_struct(val)?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                    if let Some(value) = self.f17.as_ref() {
                        __protocol.write_list_field(
                            17,
                            ::pilota::thrift::TType::Map,
                            &value,
                            |__protocol, val| {
                                __protocol.write_map(
                                    ::pilota::thrift::TType::Binary,
                                    ::pilota::thrift::TType::I32,
                                    &val,
                                    |__protocol, key| {
                                        __protocol.write_faststr((key).clone())?;
                                        ::std::result::Result::Ok(())
                                    },
                                    |__protocol, val| {
                                        __protocol.write_i32(*val)?;
                                        ::std::result::Result::Ok(())
                                    },
                                )?;
                                ::std::result::Result::Ok(())
                            },
                        )?;
                    }
                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

                let mut var_1 = None;
                let mut var_2 = None;
                let mut var_3 = None;
                let mut var_4 = None;
                let mut var_5 = None;
                let mut var_6 = None;
                let mut var_7 = None;
                let mut var_8 = None;
                let mut var_9 = None;
                let mut var_10 = None;
                let mut var_11 = None;
                let mut var_12 = None;
                let mut var_13 = None;
                let mut var_14 = None;
                let mut var_15 = None;
                let mut var_16 = None;
                let mut var_17 = None;

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
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                                var_1 = Some(__protocol.read_bool()?);
                            }
                            Some(2) if field_ident.field_type == ::pilota::thrift::TType::I8 => {
                                var_2 = Some(__protocol.read_i8()?);
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::I16 => {
                                var_3 = Some(__protocol.read_i16()?);
                            }
                            Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                var_4 = Some(__protocol.read_i32()?);
                            }
                            Some(5) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                                var_5 = Some(__protocol.read_i64()?);
                            }
                            Some(6)
                                if field_ident.field_type == ::pilota::thrift::TType::Double =>
                            {
                                var_6 = Some(__protocol.read_double()?);
                            }
                            Some(7)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_7 = Some(__protocol.read_faststr()?);
                            }
                            Some(8)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                var_8 = Some(__protocol.read_bytes()?);
                            }
                            Some(9) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_9 = Some(unsafe {
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
                            Some(10) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                var_10 = Some({
                                    let list_ident = __protocol.read_set_begin()?;
                                    let mut val =
                                        ::pilota::AHashSet::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.insert(__protocol.read_faststr()?);
                                    }
                                    __protocol.read_set_end()?;
                                    val
                                });
                            }
                            Some(11)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_11 = Some(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            Some(12) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_12 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<::std::vec::Vec<i32>> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr().offset(i as isize).write(unsafe {
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
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                });
                            }
                            Some(13) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_13 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<A> =
                                        ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr()
                                            .offset(i as isize)
                                            .write(::pilota::thrift::Message::decode(__protocol)?);
                                    }
                                    val.set_len(list_ident.size);
                                    __protocol.read_list_end()?;
                                    val
                                });
                            }
                            Some(14) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_14 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_i32()?,
                                            __protocol.read_faststr()?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(15) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_15 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(
                                            __protocol.read_faststr()?,
                                            ::pilota::thrift::Message::decode(__protocol)?,
                                        );
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(16) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                                var_16 = Some({
                                    let map_ident = __protocol.read_map_begin()?;
                                    let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        val.insert(__protocol.read_faststr()?, unsafe {
                                            let list_ident = __protocol.read_list_begin()?;
                                            let mut val: ::std::vec::Vec<A> =
                                                ::std::vec::Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr().offset(i as isize).write(
                                                    ::pilota::thrift::Message::decode(__protocol)?,
                                                );
                                            }
                                            val.set_len(list_ident.size);
                                            __protocol.read_list_end()?;
                                            val
                                        });
                                    }
                                    __protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(17) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                var_17 = Some(unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<
                                        ::pilota::AHashMap<::pilota::FastStr, i32>,
                                    > = ::std::vec::Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr().offset(i as isize).write({
                                            let map_ident = __protocol.read_map_begin()?;
                                            let mut val =
                                                ::pilota::AHashMap::with_capacity(map_ident.size);
                                            for _ in 0..map_ident.size {
                                                val.insert(
                                                    __protocol.read_faststr()?,
                                                    __protocol.read_i32()?,
                                                );
                                            }
                                            __protocol.read_map_end()?;
                                            val
                                        });
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
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `Request` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self {
                    f1: var_1,
                    f2: var_2,
                    f3: var_3,
                    f4: var_4,
                    f5: var_5,
                    f6: var_6,
                    f7: var_7,
                    f8: var_8,
                    f9: var_9,
                    f10: var_10,
                    f11: var_11,
                    f12: var_12,
                    f13: var_13,
                    f14: var_14,
                    f15: var_15,
                    f16: var_16,
                    f17: var_17,
                    _field_mask: ::std::option::Option::None,
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
                    let mut var_6 = None;
                    let mut var_7 = None;
                    let mut var_8 = None;
                    let mut var_9 = None;
                    let mut var_10 = None;
                    let mut var_11 = None;
                    let mut var_12 = None;
                    let mut var_13 = None;
                    let mut var_14 = None;
                    let mut var_15 = None;
                    let mut var_16 = None;
                    let mut var_17 = None;

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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Bool  => {
                    var_1 = Some(__protocol.read_bool().await?);

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::I8  => {
                    var_2 = Some(__protocol.read_i8().await?);

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::I16  => {
                    var_3 = Some(__protocol.read_i16().await?);

                },Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32  => {
                    var_4 = Some(__protocol.read_i32().await?);

                },Some(5) if field_ident.field_type == ::pilota::thrift::TType::I64  => {
                    var_5 = Some(__protocol.read_i64().await?);

                },Some(6) if field_ident.field_type == ::pilota::thrift::TType::Double  => {
                    var_6 = Some(__protocol.read_double().await?);

                },Some(7) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_7 = Some(__protocol.read_faststr().await?);

                },Some(8) if field_ident.field_type == ::pilota::thrift::TType::Binary  => {
                    var_8 = Some(__protocol.read_bytes().await?);

                },Some(9) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_9 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_i32().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(10) if field_ident.field_type == ::pilota::thrift::TType::Set  => {
                    var_10 = Some({let list_ident = __protocol.read_set_begin().await?;
                    let mut val = ::pilota::AHashSet::with_capacity(list_ident.size);
                    for _ in 0..list_ident.size {
                        val.insert(__protocol.read_faststr().await?);
                    };
                    __protocol.read_set_end().await?;
                    val});

                },Some(11) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    var_11 = Some(<A as ::pilota::thrift::Message>::decode_async(__protocol).await?);

                },Some(12) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_12 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(__protocol.read_i32().await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(13) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_13 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<A as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

                },Some(14) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_14 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_i32().await?, __protocol.read_faststr().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(15) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_15 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, <A as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(16) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_16 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<A as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(17) if field_ident.field_type == ::pilota::thrift::TType::List  => {
                    var_17 = Some({
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, __protocol.read_i32().await?);
                        }
                        __protocol.read_map_end().await?;
                        val
                    });
                            };
                            __protocol.read_list_end().await?;
                            val
                        });

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
                    err.prepend_msg(&format!("decode struct `Request` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
                    __protocol.read_struct_end().await?;

                    let data = Self {
                        f1: var_1,
                        f2: var_2,
                        f3: var_3,
                        f4: var_4,
                        f5: var_5,
                        f6: var_6,
                        f7: var_7,
                        f8: var_8,
                        f9: var_9,
                        f10: var_10,
                        f11: var_11,
                        f12: var_12,
                        f13: var_13,
                        f14: var_14,
                        f15: var_15,
                        f16: var_16,
                        f17: var_17,
                        _field_mask: ::std::option::Option::None,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.all() {
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "Request",
                        }) + self.f1.as_ref().map_or(0, |value| {
                            if struct_fm.field(1).is_none() {
                                __protocol.bool_field_len(Some(1), *value)
                            } else {
                                0
                            }
                        }) + self.f2.as_ref().map_or(0, |value| {
                            if struct_fm.field(2).is_none() {
                                __protocol.i8_field_len(Some(2), *value)
                            } else {
                                0
                            }
                        }) + self.f3.as_ref().map_or(0, |value| {
                            if struct_fm.field(3).is_none() {
                                __protocol.i16_field_len(Some(3), *value)
                            } else {
                                0
                            }
                        }) + self.f4.as_ref().map_or(0, |value| {
                            if struct_fm.field(4).is_none() {
                                __protocol.i32_field_len(Some(4), *value)
                            } else {
                                0
                            }
                        }) + self.f5.as_ref().map_or(0, |value| {
                            if struct_fm.field(5).is_none() {
                                __protocol.i64_field_len(Some(5), *value)
                            } else {
                                0
                            }
                        }) + self.f6.as_ref().map_or(0, |value| {
                            if struct_fm.field(6).is_none() {
                                __protocol.double_field_len(Some(6), *value)
                            } else {
                                0
                            }
                        }) + self.f7.as_ref().map_or(0, |value| {
                            if struct_fm.field(7).is_none() {
                                __protocol.faststr_field_len(Some(7), value)
                            } else {
                                0
                            }
                        }) + self.f8.as_ref().map_or(0, |value| {
                            if struct_fm.field(8).is_none() {
                                __protocol.bytes_field_len(Some(8), value)
                            } else {
                                0
                            }
                        }) + self.f9.as_ref().map_or(0, |value| {
                            if let Some(list_fm) = struct_fm.field(9) {
                                if list_fm.all() {
                                    0
                                } else {
                                    let mut idx = 0;
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::List, None)
                                        + __protocol.field_end_len()
                                        + __protocol.list_begin_len(
                                            ::pilota::thrift::TListIdentifier {
                                                element_type: ::pilota::thrift::TType::I32,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.list_end_len();
                                    for el in value {
                                        let item_fm = list_fm.int(idx as i32);
                                        size += if item_fm.is_none() {
                                            __protocol.i32_len(*el)
                                        } else {
                                            0
                                        };
                                        idx += 1;
                                    }
                                    size
                                }
                            } else {
                                __protocol.list_field_len(
                                    Some(9),
                                    ::pilota::thrift::TType::I32,
                                    value,
                                    |__protocol, el| __protocol.i32_len(*el),
                                )
                            }
                        }) + self.f10.as_ref().map_or(0, |value| {
                            __protocol.set_field_len(
                                Some(10),
                                ::pilota::thrift::TType::Binary,
                                value,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        }) + self.f11.as_ref().map_or(0, |value| {
                            if let Some(struct_fm) = struct_fm.field(11) {
                                if !struct_fm.all() {
                                    __protocol.struct_field_len(Some(11), value)
                                } else {
                                    0
                                }
                            } else {
                                __protocol.struct_field_len(Some(11), value)
                            }
                        }) + self.f12.as_ref().map_or(0, |value| {
                            if let Some(list_fm) = struct_fm.field(12) {
                                if list_fm.all() {
                                    0
                                } else {
                                    let mut idx = 0;
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::List, None)
                                        + __protocol.field_end_len()
                                        + __protocol.list_begin_len(
                                            ::pilota::thrift::TListIdentifier {
                                                element_type: ::pilota::thrift::TType::List,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.list_end_len();
                                    for el in value {
                                        let item_fm = list_fm.int(idx as i32);
                                        size += if let Some(list_fm) = item_fm {
                                            if list_fm.all() {
                                                0
                                            } else {
                                                let mut idx = 0;
                                                let mut size = __protocol.list_begin_len(
                                                    ::pilota::thrift::TListIdentifier {
                                                        element_type: ::pilota::thrift::TType::I32,
                                                        size: 0,
                                                    },
                                                ) + __protocol.list_end_len();
                                                for el in el {
                                                    let item_fm = list_fm.int(idx as i32);
                                                    size += if item_fm.is_none() {
                                                        __protocol.i32_len(*el)
                                                    } else {
                                                        0
                                                    };
                                                    idx += 1;
                                                }
                                                size
                                            }
                                        } else {
                                            __protocol.list_len(
                                                ::pilota::thrift::TType::I32,
                                                el,
                                                |__protocol, el| __protocol.i32_len(*el),
                                            )
                                        };
                                        idx += 1;
                                    }
                                    size
                                }
                            } else {
                                __protocol.list_field_len(
                                    Some(12),
                                    ::pilota::thrift::TType::List,
                                    value,
                                    |__protocol, el| {
                                        __protocol.list_len(
                                            ::pilota::thrift::TType::I32,
                                            el,
                                            |__protocol, el| __protocol.i32_len(*el),
                                        )
                                    },
                                )
                            }
                        }) + self.f13.as_ref().map_or(0, |value| {
                            if let Some(list_fm) = struct_fm.field(13) {
                                if list_fm.all() {
                                    0
                                } else {
                                    let mut idx = 0;
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::List, None)
                                        + __protocol.field_end_len()
                                        + __protocol.list_begin_len(
                                            ::pilota::thrift::TListIdentifier {
                                                element_type: ::pilota::thrift::TType::Struct,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.list_end_len();
                                    for el in value {
                                        let item_fm = list_fm.int(idx as i32);
                                        size += if let Some(struct_fm) = item_fm {
                                            if !struct_fm.all() {
                                                __protocol.struct_len(el)
                                            } else {
                                                0
                                            }
                                        } else {
                                            __protocol.struct_len(el)
                                        };
                                        idx += 1;
                                    }
                                    size
                                }
                            } else {
                                __protocol.list_field_len(
                                    Some(13),
                                    ::pilota::thrift::TType::Struct,
                                    value,
                                    |__protocol, el| __protocol.struct_len(el),
                                )
                            }
                        }) + self.f14.as_ref().map_or(0, |value| {
                            if let Some(map_fm) = struct_fm.field(14) {
                                if map_fm.all() {
                                    0
                                } else {
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::Map, None)
                                        + __protocol.field_end_len()
                                        + __protocol.map_begin_len(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::I32,
                                                value_type: ::pilota::thrift::TType::Binary,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in value {
                                        let item_fm = map_fm.int(*key as i32);
                                        if let Some(fm) = item_fm {
                                            if !fm.all() {
                                                size += __protocol.i32_len(*key);
                                                let item_fm = Some(fm);
                                                size += if item_fm.is_none() {
                                                    __protocol.faststr_len(val)
                                                } else {
                                                    0
                                                };
                                            }
                                        } else {
                                            size += __protocol.i32_len(*key);
                                            size += __protocol.faststr_len(val);
                                        }
                                    }
                                    size
                                }
                            } else {
                                __protocol.map_field_len(
                                    Some(14),
                                    ::pilota::thrift::TType::I32,
                                    ::pilota::thrift::TType::Binary,
                                    value,
                                    |__protocol, key| __protocol.i32_len(*key),
                                    |__protocol, val| __protocol.faststr_len(val),
                                )
                            }
                        }) + self.f15.as_ref().map_or(0, |value| {
                            if let Some(map_fm) = struct_fm.field(15) {
                                if map_fm.all() {
                                    0
                                } else {
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::Map, None)
                                        + __protocol.field_end_len()
                                        + __protocol.map_begin_len(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::Binary,
                                                value_type: ::pilota::thrift::TType::Struct,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in value {
                                        let item_fm = map_fm.str(key);
                                        if let Some(fm) = item_fm {
                                            if !fm.all() {
                                                size += __protocol.faststr_len(key);
                                                let item_fm = Some(fm);
                                                size += if let Some(struct_fm) = item_fm {
                                                    if !struct_fm.all() {
                                                        __protocol.struct_len(val)
                                                    } else {
                                                        0
                                                    }
                                                } else {
                                                    __protocol.struct_len(val)
                                                };
                                            }
                                        } else {
                                            size += __protocol.faststr_len(key);
                                            size += __protocol.struct_len(val);
                                        }
                                    }
                                    size
                                }
                            } else {
                                __protocol.map_field_len(
                                    Some(15),
                                    ::pilota::thrift::TType::Binary,
                                    ::pilota::thrift::TType::Struct,
                                    value,
                                    |__protocol, key| __protocol.faststr_len(key),
                                    |__protocol, val| __protocol.struct_len(val),
                                )
                            }
                        }) + self.f16.as_ref().map_or(0, |value| {
                            if let Some(map_fm) = struct_fm.field(16) {
                                if map_fm.all() {
                                    0
                                } else {
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::Map, None)
                                        + __protocol.field_end_len()
                                        + __protocol.map_begin_len(
                                            ::pilota::thrift::TMapIdentifier {
                                                key_type: ::pilota::thrift::TType::Binary,
                                                value_type: ::pilota::thrift::TType::List,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.map_end_len();
                                    for (key, val) in value {
                                        let item_fm = map_fm.str(key);
                                        if let Some(fm) = item_fm {
                                            if !fm.all() {
                                                size += __protocol.faststr_len(key);
                                                let item_fm = Some(fm);
                                                size += if let Some(list_fm) = item_fm {
                                                    if list_fm.all() {
                                                        0
                                                    } else {
                                                        let mut idx = 0;
                                                        let mut size = __protocol.list_begin_len(
                                                            ::pilota::thrift::TListIdentifier {
                                                                element_type:
                                                                    ::pilota::thrift::TType::Struct,
                                                                size: 0,
                                                            },
                                                        ) + __protocol
                                                            .list_end_len();
                                                        for el in val {
                                                            let item_fm = list_fm.int(idx as i32);
                                                            size += if let Some(struct_fm) = item_fm
                                                            {
                                                                if !struct_fm.all() {
                                                                    __protocol.struct_len(el)
                                                                } else {
                                                                    0
                                                                }
                                                            } else {
                                                                __protocol.struct_len(el)
                                                            };
                                                            idx += 1;
                                                        }
                                                        size
                                                    }
                                                } else {
                                                    __protocol.list_len(
                                                        ::pilota::thrift::TType::Struct,
                                                        val,
                                                        |__protocol, el| __protocol.struct_len(el),
                                                    )
                                                };
                                            }
                                        } else {
                                            size += __protocol.faststr_len(key);
                                            size += __protocol.list_len(
                                                ::pilota::thrift::TType::Struct,
                                                val,
                                                |__protocol, el| __protocol.struct_len(el),
                                            );
                                        }
                                    }
                                    size
                                }
                            } else {
                                __protocol.map_field_len(
                                    Some(16),
                                    ::pilota::thrift::TType::Binary,
                                    ::pilota::thrift::TType::List,
                                    value,
                                    |__protocol, key| __protocol.faststr_len(key),
                                    |__protocol, val| {
                                        __protocol.list_len(
                                            ::pilota::thrift::TType::Struct,
                                            val,
                                            |__protocol, el| __protocol.struct_len(el),
                                        )
                                    },
                                )
                            }
                        }) + self.f17.as_ref().map_or(0, |value| {
                            if let Some(list_fm) = struct_fm.field(17) {
                                if list_fm.all() {
                                    0
                                } else {
                                    let mut idx = 0;
                                    let mut size = __protocol
                                        .field_begin_len(::pilota::thrift::TType::List, None)
                                        + __protocol.field_end_len()
                                        + __protocol.list_begin_len(
                                            ::pilota::thrift::TListIdentifier {
                                                element_type: ::pilota::thrift::TType::Map,
                                                size: 0,
                                            },
                                        )
                                        + __protocol.list_end_len();
                                    for el in value {
                                        let item_fm = list_fm.int(idx as i32);
                                        size += if let Some(map_fm) = item_fm {
                                            if map_fm.all() {
                                                0
                                            } else {
                                                let mut size = __protocol.map_begin_len(
                                                    ::pilota::thrift::TMapIdentifier {
                                                        key_type: ::pilota::thrift::TType::Binary,
                                                        value_type: ::pilota::thrift::TType::I32,
                                                        size: 0,
                                                    },
                                                ) + __protocol.map_end_len();
                                                for (key, val) in el {
                                                    let item_fm = map_fm.str(key);
                                                    if let Some(fm) = item_fm {
                                                        if !fm.all() {
                                                            size += __protocol.faststr_len(key);
                                                            let item_fm = Some(fm);
                                                            size += if item_fm.is_none() {
                                                                __protocol.i32_len(*val)
                                                            } else {
                                                                0
                                                            };
                                                        }
                                                    } else {
                                                        size += __protocol.faststr_len(key);
                                                        size += __protocol.i32_len(*val);
                                                    }
                                                }
                                                size
                                            }
                                        } else {
                                            __protocol.map_len(
                                                ::pilota::thrift::TType::Binary,
                                                ::pilota::thrift::TType::I32,
                                                el,
                                                |__protocol, key| __protocol.faststr_len(key),
                                                |__protocol, val| __protocol.i32_len(*val),
                                            )
                                        };
                                        idx += 1;
                                    }
                                    size
                                }
                            } else {
                                __protocol.list_field_len(
                                    Some(17),
                                    ::pilota::thrift::TType::Map,
                                    value,
                                    |__protocol, el| {
                                        __protocol.map_len(
                                            ::pilota::thrift::TType::Binary,
                                            ::pilota::thrift::TType::I32,
                                            el,
                                            |__protocol, key| __protocol.faststr_len(key),
                                            |__protocol, val| __protocol.i32_len(*val),
                                        )
                                    },
                                )
                            }
                        }) + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    } else {
                        0
                    }
                } else {
                    __protocol
                        .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Request" })
                        + self
                            .f1
                            .as_ref()
                            .map_or(0, |value| __protocol.bool_field_len(Some(1), *value))
                        + self
                            .f2
                            .as_ref()
                            .map_or(0, |value| __protocol.i8_field_len(Some(2), *value))
                        + self
                            .f3
                            .as_ref()
                            .map_or(0, |value| __protocol.i16_field_len(Some(3), *value))
                        + self
                            .f4
                            .as_ref()
                            .map_or(0, |value| __protocol.i32_field_len(Some(4), *value))
                        + self
                            .f5
                            .as_ref()
                            .map_or(0, |value| __protocol.i64_field_len(Some(5), *value))
                        + self
                            .f6
                            .as_ref()
                            .map_or(0, |value| __protocol.double_field_len(Some(6), *value))
                        + self
                            .f7
                            .as_ref()
                            .map_or(0, |value| __protocol.faststr_field_len(Some(7), value))
                        + self
                            .f8
                            .as_ref()
                            .map_or(0, |value| __protocol.bytes_field_len(Some(8), value))
                        + self.f9.as_ref().map_or(0, |value| {
                            __protocol.list_field_len(
                                Some(9),
                                ::pilota::thrift::TType::I32,
                                value,
                                |__protocol, el| __protocol.i32_len(*el),
                            )
                        })
                        + self.f10.as_ref().map_or(0, |value| {
                            __protocol.set_field_len(
                                Some(10),
                                ::pilota::thrift::TType::Binary,
                                value,
                                |__protocol, el| __protocol.faststr_len(el),
                            )
                        })
                        + self
                            .f11
                            .as_ref()
                            .map_or(0, |value| __protocol.struct_field_len(Some(11), value))
                        + self.f12.as_ref().map_or(0, |value| {
                            __protocol.list_field_len(
                                Some(12),
                                ::pilota::thrift::TType::List,
                                value,
                                |__protocol, el| {
                                    __protocol.list_len(
                                        ::pilota::thrift::TType::I32,
                                        el,
                                        |__protocol, el| __protocol.i32_len(*el),
                                    )
                                },
                            )
                        })
                        + self.f13.as_ref().map_or(0, |value| {
                            __protocol.list_field_len(
                                Some(13),
                                ::pilota::thrift::TType::Struct,
                                value,
                                |__protocol, el| __protocol.struct_len(el),
                            )
                        })
                        + self.f14.as_ref().map_or(0, |value| {
                            __protocol.map_field_len(
                                Some(14),
                                ::pilota::thrift::TType::I32,
                                ::pilota::thrift::TType::Binary,
                                value,
                                |__protocol, key| __protocol.i32_len(*key),
                                |__protocol, val| __protocol.faststr_len(val),
                            )
                        })
                        + self.f15.as_ref().map_or(0, |value| {
                            __protocol.map_field_len(
                                Some(15),
                                ::pilota::thrift::TType::Binary,
                                ::pilota::thrift::TType::Struct,
                                value,
                                |__protocol, key| __protocol.faststr_len(key),
                                |__protocol, val| __protocol.struct_len(val),
                            )
                        })
                        + self.f16.as_ref().map_or(0, |value| {
                            __protocol.map_field_len(
                                Some(16),
                                ::pilota::thrift::TType::Binary,
                                ::pilota::thrift::TType::List,
                                value,
                                |__protocol, key| __protocol.faststr_len(key),
                                |__protocol, val| {
                                    __protocol.list_len(
                                        ::pilota::thrift::TType::Struct,
                                        val,
                                        |__protocol, el| __protocol.struct_len(el),
                                    )
                                },
                            )
                        })
                        + self.f17.as_ref().map_or(0, |value| {
                            __protocol.list_field_len(
                                Some(17),
                                ::pilota::thrift::TType::Map,
                                value,
                                |__protocol, el| {
                                    __protocol.map_len(
                                        ::pilota::thrift::TType::Binary,
                                        ::pilota::thrift::TType::I32,
                                        el,
                                        |__protocol, key| __protocol.faststr_len(key),
                                        |__protocol, val| __protocol.i32_len(*val),
                                    )
                                },
                            )
                        })
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
        }
        impl Request {
            pub fn get_descriptor()
            -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor {
                let file_descriptor = get_file_descriptor();
                file_descriptor.find_struct_by_name("Request").unwrap()
            }

            pub fn set_field_mask(&mut self, field_mask: ::pilota_thrift_fieldmask::FieldMask) {
                self._field_mask = Some(field_mask.clone());
                if let Some(value) = &mut self.f11 {
                    if let Some(fm) = field_mask.field(11) {
                        value.set_field_mask(fm.clone());
                    }
                }
                if let Some(value) = &mut self.f13 {
                    if let Some(list_fm) = field_mask.field(13) {
                        if !list_fm.all() {
                            for (idx, item) in value.iter_mut().enumerate() {
                                if let Some(item_fm) = list_fm.int(idx as i32) {
                                    item.set_field_mask(item_fm.clone());
                                }
                            }
                        }
                    }
                }
                if let Some(value) = &mut self.f15 {
                    if let Some(map_mask) = field_mask.field(15) {
                        if !map_mask.all() {
                            for (key, item) in value.iter_mut() {
                                if let Some(item_fm) = map_mask.str(key) {
                                    item.set_field_mask(item_fm.clone());
                                }
                            }
                        }
                    }
                }
                if let Some(value) = &mut self.f16 {
                    if let Some(map_mask) = field_mask.field(16) {
                        if !map_mask.all() {
                            for (key, item) in value.iter_mut() {
                                if let Some(item_fm) = map_mask.str(key) {
                                    if !item_fm.all() {
                                        for (idx, item) in item.iter_mut().enumerate() {
                                            if let Some(item_fm) = item_fm.int(idx as i32) {
                                                item.set_field_mask(item_fm.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct TestTestArgsRecv {
            pub req: Request,
        }
        impl ::pilota::thrift::Message for TestTestArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTestArgsRecv",
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
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                            "decode struct `TestTestArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let Some(var_1) = var_1 else {
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
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    var_1 = Some(
                                        <Request as ::pilota::thrift::Message>::decode_async(
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
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `TestTestArgsRecv` field(#{}) failed, caused by: ",
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
                    name: "TestTestArgsRecv",
                }) + __protocol.struct_field_len(Some(1), &self.req)
                    + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        impl ::std::default::Default for TestTestResultSend {
            fn default() -> Self {
                TestTestResultSend::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TestTestResultSend {
            Ok(Response),
        }

        impl ::pilota::thrift::Message for TestTestResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestResultSend",
                })?;
                match self {
                    TestTestResultSend::Ok(value) => {
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
                                ret = Some(TestTestResultSend::Ok(field_ident));
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
                                        <Response as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(TestTestResultSend::Ok(field_ident));
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
                    name: "TestTestResultSend",
                }) + match self {
                    TestTestResultSend::Ok(value) => __protocol.struct_field_len(Some(0), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Response {
            pub _field_mask: ::std::option::Option<::pilota_thrift_fieldmask::FieldMask>,
        }
        impl ::pilota::thrift::Message for Response {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.all() {
                        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Response" };
                        __protocol.write_struct_begin(&struct_ident)?;

                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())
                    } else {
                        ::std::result::Result::Ok(())
                    }
                } else {
                    let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Response" };

                    __protocol.write_struct_begin(&struct_ident)?;

                    __protocol.write_field_stop()?;
                    __protocol.write_struct_end()?;
                    ::std::result::Result::Ok(())
                }
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                __protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{Buf, thrift::TLengthProtocolExt};

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
                            "decode struct `Response` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                __protocol.read_struct_end()?;

                let data = Self {
                    _field_mask: ::std::option::Option::None,
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
                                "decode struct `Response` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    __protocol.read_struct_end().await?;

                    let data = Self {
                        _field_mask: ::std::option::Option::None,
                    };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                if let Some(struct_fm) = self._field_mask.as_ref() {
                    if !struct_fm.all() {
                        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: "Response",
                        }) + __protocol.field_stop_len()
                            + __protocol.struct_end_len()
                    } else {
                        0
                    }
                } else {
                    __protocol
                        .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Response" })
                        + __protocol.field_stop_len()
                        + __protocol.struct_end_len()
                }
            }
        }
        impl Response {
            pub fn get_descriptor()
            -> &'static ::pilota_thrift_reflect::thrift_reflection::StructDescriptor {
                let file_descriptor = get_file_descriptor();
                file_descriptor.find_struct_by_name("Response").unwrap()
            }

            pub fn set_field_mask(&mut self, field_mask: ::pilota_thrift_fieldmask::FieldMask) {
                self._field_mask = Some(field_mask.clone());
            }
        }
        pub trait Test {}

        impl ::std::default::Default for TestTestResultRecv {
            fn default() -> Self {
                TestTestResultRecv::Ok(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum TestTestResultRecv {
            Ok(Response),
        }

        impl ::pilota::thrift::Message for TestTestResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                __protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestResultRecv",
                })?;
                match self {
                    TestTestResultRecv::Ok(value) => {
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
                                ret = Some(TestTestResultRecv::Ok(field_ident));
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
                                        <Response as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?;

                                    ret = Some(TestTestResultRecv::Ok(field_ident));
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
                    name: "TestTestResultRecv",
                }) + match self {
                    TestTestResultRecv::Ok(value) => __protocol.struct_field_len(Some(0), value),
                } + __protocol.field_stop_len()
                    + __protocol.struct_end_len()
            }
        }
    }
}
