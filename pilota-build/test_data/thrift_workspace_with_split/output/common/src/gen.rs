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
        include!("message_Author.rs");
    }

    pub mod common {
        include!("message_CommonData.rs");
    }
}
