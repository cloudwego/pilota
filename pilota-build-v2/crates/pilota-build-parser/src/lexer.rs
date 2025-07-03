//! Lexer utilities for tokenization.

use logos::Logos;
use pilota_build_common::{BytePos, Span, FileId};

/// Token with span information.
#[derive(Debug, Clone)]
pub struct Token<T> {
    pub kind: T,
    pub span: Span,
}

impl<T> Token<T> {
    pub fn new(kind: T, span: Span) -> Self {
        Token { kind, span }
    }
}

/// Lexer wrapper for Logos-based lexers.
pub struct Lexer<'a, T: Logos<'a>> {
    inner: logos::Lexer<'a, T>,
    file_id: FileId,
    position: usize,
}

impl<'a, T: Logos<'a>> Lexer<'a, T> {
    pub fn new(input: &'a str, file_id: FileId) -> Self {
        Lexer {
            inner: T::lexer(input),
            file_id,
            position: 0,
        }
    }

    /// Get the next token.
    pub fn next_token(&mut self) -> Option<Token<T>> {
        let kind = self.inner.next()?;
        let span = self.inner.span();
        let token_span = Span::new(
            BytePos::from(span.start),
            BytePos::from(span.end),
            self.file_id,
        );
        self.position = span.end;
        Some(Token::new(kind, token_span))
    }

    /// Peek at the next token without consuming it.
    pub fn peek(&self) -> Option<&T> {
        self.inner.clone().next().as_ref()
    }

    /// Get the current position.
    pub fn position(&self) -> BytePos {
        BytePos::from(self.position)
    }

    /// Get the remaining input.
    pub fn remainder(&self) -> &'a str {
        self.inner.remainder()
    }
}

/// Common keywords used in IDL files.
pub mod keywords {
    pub const NAMESPACE: &str = "namespace";
    pub const STRUCT: &str = "struct";
    pub const SERVICE: &str = "service";
    pub const ENUM: &str = "enum";
    pub const CONST: &str = "const";
    pub const TYPEDEF: &str = "typedef";
    pub const INCLUDE: &str = "include";
    pub const IMPORT: &str = "import";
    pub const REQUIRED: &str = "required";
    pub const OPTIONAL: &str = "optional";
    pub const VOID: &str = "void";
    pub const BOOL: &str = "bool";
    pub const BYTE: &str = "byte";
    pub const I8: &str = "i8";
    pub const I16: &str = "i16";
    pub const I32: &str = "i32";
    pub const I64: &str = "i64";
    pub const DOUBLE: &str = "double";
    pub const STRING: &str = "string";
    pub const BINARY: &str = "binary";
    pub const LIST: &str = "list";
    pub const SET: &str = "set";
    pub const MAP: &str = "map";
    pub const ONEWAY: &str = "oneway";
    pub const EXTENDS: &str = "extends";
    pub const THROWS: &str = "throws";
    pub const EXCEPTION: &str = "exception";
}