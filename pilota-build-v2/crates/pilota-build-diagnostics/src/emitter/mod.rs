//! Diagnostic emitters for different output formats.

pub mod terminal;

use crate::diagnostic::Diagnostic;

/// Trait for emitting diagnostics.
pub trait Emitter: Send + Sync {
    /// Emit a diagnostic.
    fn emit_diagnostic(&mut self, diagnostic: &Diagnostic);
}

/// A buffer emitter for testing.
#[cfg(test)]
pub struct BufferEmitter {
    buffer: std::sync::Arc<parking_lot::Mutex<Vec<Diagnostic>>>,
}

#[cfg(test)]
impl BufferEmitter {
    pub fn new() -> Self {
        BufferEmitter {
            buffer: std::sync::Arc::new(parking_lot::Mutex::new(Vec::new())),
        }
    }

    pub fn get_diagnostics(&self) -> Vec<Diagnostic> {
        self.buffer.lock().clone()
    }
}

#[cfg(test)]
impl Emitter for BufferEmitter {
    fn emit_diagnostic(&mut self, diagnostic: &Diagnostic) {
        self.buffer.lock().push(diagnostic.clone());
    }
}