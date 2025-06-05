pub mod wrapper_arc {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};

    pub fn find_mod_file_descriptor(
        path: &str,
    ) -> Option<&'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor> {
        match path {

                "/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift" => Some(
            wrapper_arc::get_file_descriptor()),

                _ => None,
            }
    }

    pub mod wrapper_arc {
<<<<<<< HEAD
        use ::pilota::{Buf as _, BufMut as _};
=======

        static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\r\0\x02\x0b\x0b\0\0\0\0\r\0\x03\x0b\x0b\0\0\0\0\x0f\0\x04\x0c\0\0\0\x02\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x0bTestService\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04test\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04TEST\0\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x03req\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04TEST\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\x01\0\0\0\x17pilota.rust_wrapper_arc\x0b\0\0\0\x01\0\0\0\x04true\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x02\0\x08\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\x0b\0\x07\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x0btestService\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04test\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04TEST\0\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x03req\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04TEST\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\x01\0\0\0\x17pilota.rust_wrapper_arc\x0b\0\0\0\x01\0\0\0\x04true\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x02\0\x08\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\x0b\0\x07\0\0\0\0\0\x0f\0\x05\x0c\0\0\0\x02\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x01A\x0f\0\x03\x0c\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04TEST\x0f\0\x03\x0c\0\0\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x02ID\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x05Name2\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x01A\0\0\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\x01\0\0\0\x17pilota.rust_wrapper_arc\x0b\0\0\0\x01\0\0\0\x04true\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x05Name3\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x03map\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x03i32\0\x0c\0\x04\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x04list\x0c\0\x03\x0b\0\x01\0\0\0_/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_with_split/wrapper_arc.thrift\x0b\0\x02\0\0\0\x01A\0\0\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x03\r\0\x07\x0b\x0f\0\0\0\x01\0\0\0\x17pilota.rust_wrapper_arc\x0b\0\0\0\x01\0\0\0\x04true\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x0f\0\x08\x0c\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\0\0");

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
        include!("wrapper_arc/mod.rs");
    }
}
