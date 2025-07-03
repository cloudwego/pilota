//! Compiler session management.

use crate::error::CompileResult;
use pilota_build_common::{FileId, SourceMap};
use pilota_build_diagnostics::DiagnosticHandler;
use std::path::Path;
use std::sync::Arc;

/// A compiler session containing global state.
pub struct CompilerSession {
    /// Source map for managing source files.
    pub source_map: Arc<SourceMap>,
    /// Diagnostic handler for error reporting.
    pub diagnostics: DiagnosticHandler,
    /// Compilation options.
    pub options: CompilerOptions,
}

/// Compiler options.
#[derive(Clone, Debug)]
pub struct CompilerOptions {
    /// Enable debug output.
    pub debug: bool,
    /// Enable verbose output.
    pub verbose: bool,
    /// Number of threads to use (0 = use all cores).
    pub threads: usize,
    /// Enable incremental compilation.
    pub incremental: bool,
    /// Output directory.
    pub output_dir: Option<std::path::PathBuf>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        CompilerOptions {
            debug: false,
            verbose: false,
            threads: 0,
            incremental: true,
            output_dir: None,
        }
    }
}

impl CompilerSession {
    /// Create a new compiler session.
    pub fn new(source_map: Arc<SourceMap>, diagnostics: DiagnosticHandler) -> Self {
        CompilerSession {
            source_map,
            diagnostics,
            options: CompilerOptions::default(),
        }
    }

    /// Set compiler options.
    pub fn set_options(&mut self, options: CompilerOptions) {
        self.options = options;
    }

    /// Load a source file.
    pub fn load_file(&self, path: &Path) -> CompileResult<FileId> {
        self.source_map
            .load_file(path)
            .map_err(|e| crate::error::CompileError::Io(e))
    }

    /// Create a source file from a string.
    pub fn create_source_file(&self, name: impl Into<std::path::PathBuf>, src: String) -> FileId {
        self.source_map.new_source_file(name.into(), src)
    }

    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.has_errors()
    }

    /// Abort if there are any errors.
    pub fn abort_if_errors(&self) {
        self.diagnostics.abort_if_errors();
    }
}