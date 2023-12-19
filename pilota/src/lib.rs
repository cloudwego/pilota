#![doc(
    html_logo_url = "https://github.com/cloudwego/pilota/raw/main/.github/assets/logo.png?sanitize=true"
)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod prost;
pub mod thrift;

// reexport
pub use async_recursion;
pub use bytes::*;
pub use derivative;
pub use faststr::FastStr;
pub use lazy_static;
pub use serde;
pub use thiserror::Error as ThisError;
pub use tokio::io::AsyncRead;

pub use crate::thrift::unknown::LinkedBytes;

#[derive(thiserror::Error, Debug)]
pub enum EnumConvertError<Num> {
    #[error("invalid value `{0}` for enum `{1}`")]
    InvalidNum(Num, &'static str),
}

#[cfg(feature = "ahash")]
pub type Hasher = ahash::random_state::RandomState;
#[cfg(not(feature = "ahash"))]
pub type Hasher = std::collections::hash_map::RandomState;
