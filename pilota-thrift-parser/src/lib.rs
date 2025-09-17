#![doc(
    html_logo_url = "https://github.com/cloudwego/pilota/raw/main/.github/assets/logo.png?sanitize=true"
)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod descriptor;
pub mod parser;

pub use descriptor::*;
pub use parser::{
    error::Error,
    thrift::{FileParser, FileSource},
};
