//! Builder API for compiling IDL files.

use crate::{
    context::CompilerContext,
    db::CompilerDatabase,
    error::{CompileError, CompileResult},
    session::{CompilerOptions, CompilerSession},
};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Builder for compiling IDL files.
pub struct Builder {
    session: CompilerSession,
    context: Arc<CompilerContext>,
    database: CompilerDatabase,
    source_files: Vec<PathBuf>,
}

impl Builder {
    /// Create a new builder.
    pub fn new() -> Self {
        let session = crate::create_session();
        let context = Arc::new(CompilerContext::new());
        let database = CompilerDatabase::new(context.clone());

        Builder {
            session,
            context,
            database,
            source_files: Vec::new(),
        }
    }

    /// Set compiler options.
    pub fn with_options(mut self, options: CompilerOptions) -> Self {
        self.session.set_options(options);
        self
    }

    /// Add a source file to compile.
    pub fn add_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.source_files.push(path.into());
        self
    }

    /// Compile the source files.
    pub fn compile(self) -> CompileResult<()> {
        // Load source files
        for path in &self.source_files {
            if !path.exists() {
                return Err(CompileError::FileNotFound(path.clone()));
            }

            let file_id = self.session.load_file(path)?;
            let source = std::fs::read_to_string(path)?;
            self.database.set_source_text(file_id, Arc::new(source));
        }

        // TODO: Implement compilation phases
        Ok(())
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
