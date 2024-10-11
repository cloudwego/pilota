pub mod gen {
    #![allow(warnings, clippy::all)]

    pub mod article {

        pub mod image {
            include!("article/image/message_Image.rs");

            pub mod cdn {
                include!("article/image/cdn/message_CDN.rs");
            }
        }
    }

    pub mod author {
        include!("author/service_AuthorService.rs");
        include!("author/enum_AuthorServiceGetAuthorResultRecv.rs");
        include!("author/message_AuthorServiceGetAuthorArgsRecv.rs");
        include!("author/enum_AuthorServiceGetAuthorResultSend.rs");
        include!("author/message_GetAuthorResponse.rs");
        include!("author/message_AuthorServiceGetAuthorArgsSend.rs");
        include!("author/message_GetAuthorRequest.rs");
        include!("author/message_Author.rs");
    }

    pub mod common {
        include!("common/message_CommonData.rs");
    }
    pub use author::*;
}
