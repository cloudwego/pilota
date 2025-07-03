//! Diagnostic handler for managing error reporting.

use crate::{
    diagnostic::{Diagnostic, DiagnosticBuilder, Level},
    emitter::Emitter,
};
use parking_lot::Mutex;
use rustc_hash::{FxHashSet, FxHasher};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A handler for diagnostics.
pub struct DiagnosticHandler {
    emitter: Mutex<Box<dyn Emitter>>,
    err_count: AtomicUsize,
    warn_count: AtomicUsize,
    deduplicated: Mutex<FxHashSet<u64>>,
}

impl DiagnosticHandler {
    /// Create a new diagnostic handler with the given emitter.
    pub fn with_emitter(emitter: Box<dyn Emitter>) -> Self {
        DiagnosticHandler {
            emitter: Mutex::new(emitter),
            err_count: AtomicUsize::new(0),
            warn_count: AtomicUsize::new(0),
            deduplicated: Mutex::new(FxHashSet::default()),
        }
    }

    /// Emit a diagnostic.
    pub fn emit_diagnostic(&self, diagnostic: Diagnostic) {
        // Update counters
        match diagnostic.level {
            Level::Error => {
                self.err_count.fetch_add(1, Ordering::Relaxed);
            }
            Level::Warning => {
                self.warn_count.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }

        // Check for deduplication
        let hash = self.hash_diagnostic(&diagnostic);
        if !self.deduplicated.lock().insert(hash) {
            return; // Already emitted
        }

        // Emit the diagnostic
        self.emitter.lock().emit_diagnostic(&diagnostic);
    }

    /// Create an error diagnostic builder.
    pub fn struct_err(&self, message: impl Into<String>) -> DiagnosticBuilder<'_> {
        DiagnosticBuilder::new(self, Level::Error, message.into())
    }

    /// Create an error diagnostic builder with a span.
    pub fn struct_span_err(
        &self,
        span: pilota_build_common::Span,
        message: impl Into<String>,
    ) -> DiagnosticBuilder<'_> {
        let mut builder = self.struct_err(message);
        builder.span_label(span, "");
        builder
    }

    /// Create a warning diagnostic builder.
    pub fn struct_warn(&self, message: impl Into<String>) -> DiagnosticBuilder<'_> {
        DiagnosticBuilder::new(self, Level::Warning, message.into())
    }

    /// Create a warning diagnostic builder with a span.
    pub fn struct_span_warn(
        &self,
        span: pilota_build_common::Span,
        message: impl Into<String>,
    ) -> DiagnosticBuilder<'_> {
        let mut builder = self.struct_warn(message);
        builder.span_label(span, "");
        builder
    }

    /// Emit an error.
    pub fn error(&self, message: impl Into<String>) -> ErrorReported {
        let mut builder = self.struct_err(message);
        builder.emit();
        ErrorReported
    }

    /// Emit an error with a span.
    pub fn span_error(
        &self,
        span: pilota_build_common::Span,
        message: impl Into<String>,
    ) -> ErrorReported {
        let mut builder = self.struct_span_err(span, message);
        builder.emit();
        ErrorReported
    }

    /// Emit a warning.
    pub fn warning(&self, message: impl Into<String>) {
        let mut builder = self.struct_warn(message);
        builder.emit();
    }

    /// Emit a warning with a span.
    pub fn span_warning(&self, span: pilota_build_common::Span, message: impl Into<String>) {
        let mut builder = self.struct_span_warn(span, message);
        builder.emit();
    }

    /// Get the number of errors emitted.
    pub fn err_count(&self) -> usize {
        self.err_count.load(Ordering::Relaxed)
    }

    /// Get the number of warnings emitted.
    pub fn warn_count(&self) -> usize {
        self.warn_count.load(Ordering::Relaxed)
    }

    /// Check if any errors have been emitted.
    pub fn has_errors(&self) -> bool {
        self.err_count() > 0
    }

    /// Abort if any errors have been emitted.
    pub fn abort_if_errors(&self) {
        if self.has_errors() {
            self.emitter.lock().emit_diagnostic(&Diagnostic::error(
                format!("aborting due to {} previous error(s)", self.err_count()),
            ));
            std::process::exit(1);
        }
    }

    fn hash_diagnostic(&self, diagnostic: &Diagnostic) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = FxHasher::default();
        
        diagnostic.message.hash(&mut hasher);
        diagnostic.code.hash(&mut hasher);
        for span_label in &diagnostic.spans {
            span_label.span.hash(&mut hasher);
            span_label.label.hash(&mut hasher);
        }
        
        hasher.finish()
    }
}

/// A marker type indicating that an error has been reported.
#[derive(Copy, Clone, Debug)]
pub struct ErrorReported;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emitter::BufferEmitter;
    use pilota_build_common::{BytePos, FileId, Span};

    #[test]
    fn test_error_counting() {
        let emitter = Box::new(BufferEmitter::new());
        let handler = DiagnosticHandler::with_emitter(emitter);

        assert_eq!(handler.err_count(), 0);
        assert!(!handler.has_errors());

        handler.error("test error");
        assert_eq!(handler.err_count(), 1);
        assert!(handler.has_errors());

        handler.warning("test warning");
        assert_eq!(handler.err_count(), 1);
        assert_eq!(handler.warn_count(), 1);
    }

    #[test]
    fn test_deduplication() {
        let emitter = Box::new(BufferEmitter::new());
        let handler = DiagnosticHandler::with_emitter(emitter);

        let span = Span::new(BytePos(0), BytePos(10), FileId::DUMMY);

        handler.span_error(span, "duplicate error");
        handler.span_error(span, "duplicate error");

        assert_eq!(handler.err_count(), 2); // Count is incremented
        // But only one unique diagnostic should be emitted
    }
}