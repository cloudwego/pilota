pub mod gen {
    #![allow(warnings, clippy::all)]

    pub mod article {

        pub mod image {
            include!("message_Image.rs");

            pub mod cdn {
                include!("message_CDN.rs");
            }
        }
    }

    pub mod author {
        include!("service_AuthorService.rs");
        include!("enum_AuthorServiceGetAuthorResultRecv.rs");
        include!("message_AuthorServiceGetAuthorArgsRecv.rs");
        include!("enum_AuthorServiceGetAuthorResultSend.rs");
        include!("message_GetAuthorResponse.rs");
        include!("message_AuthorServiceGetAuthorArgsSend.rs");
        include!("message_GetAuthorRequest.rs");
        include!("message_Author.rs");
    }

    pub mod common {
        include!("message_CommonData.rs");
    }
    pub use author::*;
}
