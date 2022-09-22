#![doc(
    html_logo_url = "https://github.com/cloudwego/pilota/raw/main/.github/assets/logo.png?sanitize=true"
)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod thrift;

// reexport
pub use async_recursion;
pub use async_trait;
pub use derivative;
pub use lazy_static;
pub use num_enum;
pub use thiserror::Error as ThisError;
pub use tokio::io::AsyncRead;
