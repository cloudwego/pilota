pub mod r#gen {
    #![allow(warnings, clippy::all)]

    pub mod article {

        pub mod image {

            pub use ::common::article::image::Image;
            pub mod cdn {

                pub use ::common::article::image::cdn::Cdn;
            }
        }
    }

    pub mod common {

        pub use ::common::common::CommonData;
    }
    pub use article::image::*;
}
