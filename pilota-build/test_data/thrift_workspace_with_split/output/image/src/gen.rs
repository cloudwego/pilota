pub mod gen {
    #![allow(warnings, clippy::all)]

    pub mod article {

        pub mod image {
            include!("message_ImageServiceGetImageArgsRecv.rs");
            include!("enum_ImageServiceGetImageResultSend.rs");
            include!("message_GetImageResponse.rs");
            include!("message_ImageServiceGetImageArgsSend.rs");
            include!("message_GetImageRequest.rs");
            include!("service_ImageService.rs");
            include!("enum_ImageServiceGetImageResultRecv.rs");
            include!("message_Image.rs");

            pub mod cdn {
                include!("message_CDN.rs");
            }
        }
    }

    pub mod common {
        include!("message_CommonData.rs");
    }
    pub use article::image::*;
}
