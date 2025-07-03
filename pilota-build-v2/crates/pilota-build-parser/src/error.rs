//! Parser error types.

use pilota_build_common::Span;
use thiserror::Error;

/// Result type for parsing operations.
pub type ParseResult<T> = Result<T, ParseError>;

/// Parse error.
#[derive(Debug, Clone, Error)]
#[error("{kind}")]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, span: Span) -> Self {
        ParseError { kind, span }
    }

    /// Create a syntax error.
    pub fn syntax(message: impl Into<String>, span: Span) -> Self {
        ParseError::new(ParseErrorKind::SyntaxError(message.into()), span)
    }

    /// Create an unexpected token error.
    pub fn unexpected_token(expected: &str, found: &str, span: Span) -> Self {
        ParseError::new(
            ParseErrorKind::UnexpectedToken {
                expected: expected.to_string(),
                found: found.to_string(),
            },
            span,
        )
    }

    /// Create an unexpected EOF error.
    pub fn unexpected_eof(span: Span) -> Self {
        ParseError::new(ParseErrorKind::UnexpectedEof, span)
    }
}

/// Parse error kind.
#[derive(Debug, Clone, Error)]
pub enum ParseErrorKind {
    #[error("syntax error: {0}")]
    SyntaxError(String),

    #[error("unexpected token: expected {expected}, found {found}")]
    UnexpectedToken { expected: String, found: String },

    #[error("unexpected end of file")]
    UnexpectedEof,

    #[error("invalid number literal: {0}")]
    InvalidNumber(String),

    #[error("invalid string literal: {0}")]
    InvalidString(String),

    #[error("duplicate field ID: {0}")]
    DuplicateFieldId(i32),

    #[error("invalid field ID: {0}")]
    InvalidFieldId(i32),

    #[error("duplicate definition: {0}")]
    DuplicateDefinition(String),

    #[error("unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}