//! Protobuf parser implementation (placeholder).

use super::lexer::ProtobufToken;
use crate::{
    error::{ParseError, ParseResult},
    parser::Parser as ParserTrait,
};
use pilota_build_common::{BytePos, FileId, Span};
use pilota_build_hir::HirCrate;

/// Protobuf parser.
pub struct ProtobufParser;

impl ProtobufParser {
    pub fn new() -> Self {
        ProtobufParser
    }
}

impl ParserTrait for ProtobufParser {
    fn parse(&self, file_id: FileId, content: &str) -> ParseResult<HirCrate> {
        // TODO: Implement Protobuf parsing
        Err(ParseError::new(
            crate::error::ParseErrorKind::UnsupportedFileType("Protobuf parsing not yet implemented".to_string()),
            Span::new(BytePos(0), BytePos(0), file_id),
        ))
    }
}