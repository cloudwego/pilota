//! Parser trait and common utilities.

use crate::error::ParseResult;
use pilota_build_common::FileId;
use pilota_build_hir::HirCrate;

/// Trait for parsers.
pub trait Parser {
    /// Parse source code into HIR.
    fn parse(&self, file_id: FileId, content: &str) -> ParseResult<HirCrate>;
}

/// Parser context for tracking state during parsing.
pub struct ParseContext {
    pub file_id: FileId,
    pub errors: Vec<crate::ParseError>,
}

impl ParseContext {
    pub fn new(file_id: FileId) -> Self {
        ParseContext {
            file_id,
            errors: Vec::new(),
        }
    }

    /// Report an error.
    pub fn error(&mut self, error: crate::ParseError) {
        self.errors.push(error);
    }

    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Take all errors.
    pub fn take_errors(&mut self) -> Vec<crate::ParseError> {
        std::mem::take(&mut self.errors)
    }
}