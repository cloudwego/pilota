//! Error types for the compiler.

use pilota_build_diagnostics::ErrorReported;
use std::path::PathBuf;
use thiserror::Error;

/// Result type for compilation.
pub type CompileResult<T> = Result<T, CompileError>;

/// Compilation error.
#[derive(Debug, Error)]
pub enum CompileError {
    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Parse error.
    #[error("parse error")]
    Parse(ErrorReported),

    /// Resolution error.
    #[error("resolution error")]
    Resolution(ErrorReported),

    /// Type check error.
    #[error("type check error")]
    TypeCheck(ErrorReported),

    /// Codegen error.
    #[error("code generation error")]
    Codegen(ErrorReported),

    /// File not found.
    #[error("file not found: {0}")]
    FileNotFound(PathBuf),

    /// Unsupported feature.
    #[error("unsupported feature: {0}")]
    UnsupportedFeature(String),

    /// Internal compiler error.
    #[error("internal compiler error: {0}")]
    Internal(String),
}

impl From<ErrorReported> for CompileError {
    fn from(err: ErrorReported) -> Self {
        // The specific error has already been reported through diagnostics
        CompileError::Parse(err)
    }
}