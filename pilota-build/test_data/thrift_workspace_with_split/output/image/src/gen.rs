pub mod gen {
    #![allow(warnings, clippy::all)]

    pub mod article {

        pub mod image {
            include!("article/image/message_ImageServiceGetImageArgsRecv.rs");
            include!("article/image/enum_ImageServiceGetImageResultSend.rs");
            include!("article/image/message_GetImageResponse.rs");
            include!("article/image/message_ImageServiceGetImageArgsSend.rs");
            include!("article/image/message_GetImageRequest.rs");
            include!("article/image/service_ImageService.rs");
            include!("article/image/enum_ImageServiceGetImageResultRecv.rs");
            include!("article/image/message_Image.rs");

            pub mod cdn {
                include!("article/image/cdn/message_CDN.rs");
            }
        }
    }

    pub mod common {
        include!("common/message_CommonData.rs");
    }
    pub use article::image::*;
}
