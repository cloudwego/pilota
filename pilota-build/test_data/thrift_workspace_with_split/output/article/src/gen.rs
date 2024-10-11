pub mod gen {
    #![allow(warnings, clippy::all)]

    pub mod article {
        include!("article/enum_Status.rs");
        include!("article/message_ArticleServiceGetArticleArgsRecv.rs");
        include!("article/enum_ArticleServiceGetArticleResultSend.rs");
        include!("article/message_GetArticleResponse.rs");
        include!("article/message_ArticleServiceGetArticleArgsSend.rs");
        include!("article/message_GetArticleRequest.rs");
        include!("article/service_ArticleService.rs");
        include!("article/enum_ArticleServiceGetArticleResultRecv.rs");
        include!("article/message_Article.rs");

        pub mod image {
            include!("article/image/message_Image.rs");

            pub mod cdn {
                include!("article/image/cdn/message_CDN.rs");
            }
        }
    }

    pub mod author {
        include!("author/message_Author.rs");
    }

    pub mod common {
        include!("common/message_CommonData.rs");
    }
    pub use article::*;
}
