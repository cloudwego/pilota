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
        include!("author/message_Author.rs");
    }

    pub mod common {
        include!("common/message_CommonData.rs");
    }
}
