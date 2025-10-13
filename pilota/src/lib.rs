#![doc(
    html_logo_url = "https://github.com/cloudwego/pilota/raw/main/.github/assets/logo.png?sanitize=true"
)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod pb;
pub mod thrift;

// reexport
pub use ahash::{AHashMap, AHashSet};
pub use async_recursion;
pub use bytes::*;
pub use faststr::FastStr;
pub use lazy_static;
pub use linkedbytes::LinkedBytes;
pub use ordered_float::OrderedFloat;
pub use serde;
pub use thiserror::Error as ThisError;
pub use tokio::io::AsyncRead;

pub use crate::thrift::unknown::BytesVec;

#[derive(thiserror::Error, Debug)]
pub enum EnumConvertError<Num> {
    #[error("invalid value `{0}` for enum `{1}`")]
    InvalidNum(Num, &'static str),
}
