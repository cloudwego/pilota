//! Database for incremental compilation.

use pilota_build_common::{DefId, FileId};
use pilota_build_hir::HirCrate;
use std::sync::Arc;

// For now, we will use a simpler approach without Salsa
// TODO: Integrate Salsa properly when we have more time to study the new API

/// Simple database for storing compilation data.
pub struct CompilerDatabase {
    context: Arc<crate::CompilerContext>,
}

impl CompilerDatabase {
    pub fn new(context: Arc<crate::CompilerContext>) -> Self {
        CompilerDatabase { context }
    }

    pub fn context(&self) -> Arc<crate::CompilerContext> {
        self.context.clone()
    }

    /// Set source text for a file.
    pub fn set_source_text(&self, _file_id: FileId, _text: Arc<String>) {
        // TODO: Store in context
    }

    /// Parse a file into HIR.
    pub fn parse_file(&self, _file_id: FileId) -> Arc<HirCrate> {
        // TODO: Implement parsing
        Arc::new(HirCrate {
            items: Vec::new(),
            span: pilota_build_common::DUMMY_SPAN,
        })
    }
}
