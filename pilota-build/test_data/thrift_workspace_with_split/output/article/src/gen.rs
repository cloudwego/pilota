pub mod gen {
    #![allow(warnings, clippy::all)]

    pub mod article {
        include!("enum_Status.rs");
        include!("message_ArticleServiceGetArticleArgsRecv.rs");
        include!("enum_ArticleServiceGetArticleResultSend.rs");
        include!("message_GetArticleResponse.rs");
        include!("message_ArticleServiceGetArticleArgsSend.rs");
        include!("message_GetArticleRequest.rs");
        include!("service_ArticleService.rs");
        include!("enum_ArticleServiceGetArticleResultRecv.rs");
        include!("message_Article.rs");

        pub mod image {
            include!("message_Image.rs");

            pub mod cdn {
                include!("message_CDN.rs");
            }
        }
    }

    pub mod author {
        include!("message_Author.rs");
    }

    pub mod common {
        include!("message_CommonData.rs");
    }
    pub use article::*;
}
