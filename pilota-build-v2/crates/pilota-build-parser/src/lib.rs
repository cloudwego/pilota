//! Parser module for pilota-build.
//!
//! This module provides parsers for Thrift and Protobuf IDL files.

pub mod lexer;
pub mod parser;
pub mod thrift;
pub mod protobuf;
pub mod error;

pub use parser::{Parser, ParseResult};
pub use error::{ParseError, ParseErrorKind};

use pilota_build_common::FileId;
use pilota_build_hir::HirCrate;

/// Parse a file based on its extension.
pub fn parse_file(
    file_id: FileId,
    content: &str,
    file_name: &str,
) -> ParseResult<HirCrate> {
    if file_name.ends_with(".thrift") {
        thrift::parse(file_id, content)
    } else if file_name.ends_with(".proto") {
        protobuf::parse(file_id, content)
    } else {
        Err(ParseError::new(
            ParseErrorKind::UnsupportedFileType(file_name.to_string()),
            pilota_build_common::DUMMY_SPAN,
        ))
    }
}