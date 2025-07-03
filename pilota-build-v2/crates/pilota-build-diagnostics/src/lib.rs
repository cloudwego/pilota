//! Diagnostic and error reporting system for pilota-build.

pub mod diagnostic;
pub mod error_code;
pub mod emitter;
pub mod handler;
pub mod snippet;

pub use diagnostic::{Diagnostic, DiagnosticBuilder, Level, Severity};
pub use error_code::{ErrorCode, ERROR_CODES};
pub use handler::{DiagnosticHandler, ErrorReported};

use pilota_build_common::{Span, SourceMap};
use std::sync::Arc;

/// Create a new diagnostic handler with terminal emitter.
pub fn create_handler(source_map: Arc<SourceMap>) -> DiagnosticHandler {
    let emitter = Box::new(emitter::terminal::TerminalEmitter::new(
        source_map,
        emitter::terminal::Config::default(),
    ));
    DiagnosticHandler::with_emitter(emitter)
}