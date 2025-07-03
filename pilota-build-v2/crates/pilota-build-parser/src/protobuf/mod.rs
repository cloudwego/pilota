//! Protobuf IDL parser.

mod lexer;
mod parser;

use crate::error::ParseResult;
use pilota_build_common::FileId;
use pilota_build_hir::HirCrate;

/// Parse Protobuf IDL content.
pub fn parse(file_id: FileId, content: &str) -> ParseResult<HirCrate> {
    let parser = parser::ProtobufParser::new();
    parser.parse(file_id, content)
}