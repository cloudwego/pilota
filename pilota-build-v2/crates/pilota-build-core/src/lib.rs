//! Core compiler infrastructure for pilota-build.

pub mod builder;
pub mod context;
pub mod db;
pub mod error;
pub mod session;

pub use builder::Builder;
pub use context::CompilerContext;
pub use error::{CompileError, CompileResult};
pub use session::CompilerSession;

use pilota_build_common::SourceMap;
use pilota_build_diagnostics::DiagnosticHandler;
use std::sync::Arc;

/// Create a new compiler session.
pub fn create_session() -> CompilerSession {
    let source_map = Arc::new(SourceMap::new());
    let diagnostics = pilota_build_diagnostics::create_handler(source_map.clone());
    
    CompilerSession::new(source_map, diagnostics)
}