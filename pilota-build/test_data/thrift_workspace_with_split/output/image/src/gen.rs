pub mod r#gen {
    #![allow(warnings, clippy::all)]

    pub mod article {
        use ::pilota::{Buf as _, BufMut as _};

        pub mod image {
            use ::pilota::{Buf as _, BufMut as _};
            include!("article/image/mod.rs");

            pub mod cdn {
                use ::pilota::{Buf as _, BufMut as _};
                include!("article/image/cdn/mod.rs");
            }
        }
    }

    pub mod common {
        use ::pilota::{Buf as _, BufMut as _};
        include!("common/mod.rs");
    }
    pub use article::image::*;
}
