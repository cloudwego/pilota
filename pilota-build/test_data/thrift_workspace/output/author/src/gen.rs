pub mod r#gen {
    #![allow(warnings, clippy::all)]

    pub mod author {

        pub use ::common::author::Author;
    }

    pub mod common {

        pub use ::common::common::CommonData;
    }
    pub use author::*;
}
