pub mod r#gen {
    #![allow(warnings, clippy::all)]

    pub fn find_mod_file_descriptor(
        path: &str,
    ) -> Option<&'static ::pilota_thrift_reflect::thrift_reflection::FileDescriptor> {
        match path {

                "/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/cdn.thrift" => Some(
            article::image::cdn::get_file_descriptor()),

                "/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/common.thrift" => Some(
            common::get_file_descriptor()),

                "/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift" => Some(
            article::image::get_file_descriptor()),

                _ => None,
            }
    }

    pub mod article {

        pub mod image {

            static FILE_DESCRIPTOR_BYTES: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\r\0\x02\x0b\x0b\0\0\0\x02\0\0\0\x06common\0\0\0\rcommon.thrift\0\0\0\x03cdn\0\0\0\ncdn.thrift\r\0\x03\x0b\x0b\0\0\0\x01\0\0\0\x02rs\0\0\0\rarticle.image\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x0cImageService\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x08GetImage\x0c\0\x03\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x10GetImageResponse\0\x0f\0\x04\x0c\0\0\0\x01\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x03req\x0c\0\x03\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x0fGetImageRequest\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x05\x0b\x0f\0\0\0\0\x0b\0\x06\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x02\0\x08\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\x0b\0\x07\0\0\0\0\0\x0f\0\x05\x0c\0\0\0\x03\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x05Image\x0f\0\x03\x0c\0\0\0\x04\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x02id\x0c\0\x03\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x03i64\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x03url\x0c\0\x03\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x06string\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x02\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x03cdn\x0c\0\x03\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x07cdn.CDN\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x03\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x0bcommon_data\x0c\0\x03\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x11common.CommonData\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x04\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x0fGetImageRequest\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x02id\x0c\0\x03\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x03i64\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x10GetImageResponse\x0f\0\x03\x0c\0\0\0\x01\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x05image\x0c\0\x03\x0b\0\x01\0\0\0i/data02/home/giggle/projects/pilota/pilota-build/test_data/thrift_workspace_with_split/input/image.thrift\x0b\0\x02\0\0\0\x05Image\0\x0b\0\x04\0\0\0\x08required\x08\0\x05\0\0\0\x01\r\0\x07\x0b\x0f\0\0\0\0\x0b\0\x08\0\0\0\0\0\r\0\x04\x0b\x0f\0\0\0\0\x0b\0\x05\0\0\0\0\0\x0f\0\x06\x0c\0\0\0\0\x0f\0\x07\x0c\0\0\0\0\x0f\0\x08\x0c\0\0\0\0\x0f\0\t\x0c\0\0\0\0\x0f\0\n\x0c\0\0\0\0\0");

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
            include!("article/image/mod.rs");

            pub mod cdn {

                pub use ::common::article::image::cdn::get_file_descriptor;
                include!("article/image/cdn/mod.rs");
            }
        }
    }

    pub mod common {

        pub use ::common::common::get_file_descriptor;
        include!("common/mod.rs");
    }
    pub use article::image::*;
}
