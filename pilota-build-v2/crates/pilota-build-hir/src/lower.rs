//! Lowering from parser AST to HIR.

use crate::*;
use pilota_build_common::{DefIdGenerator, LocalId};

/// Context for lowering AST to HIR.
pub struct LoweringContext {
    def_id_gen: DefIdGenerator,
    local_id_counter: u32,
}

impl LoweringContext {
    pub fn new() -> Self {
        LoweringContext {
            def_id_gen: DefIdGenerator::new(),
            local_id_counter: 0,
        }
    }

    /// Generate a new HIR ID.
    pub fn next_hir_id(&mut self) -> HirId {
        let owner = self.def_id_gen.next_def_id();
        let local_id = LocalId(self.local_id_counter);
        self.local_id_counter += 1;
        HirId { owner, local_id }
    }

    /// Reset local ID counter for a new item.
    pub fn reset_local_id_counter(&mut self) {
        self.local_id_counter = 0;
    }

    // Lowering methods would be implemented here based on the specific parser AST
}

impl Default for LoweringContext {
    fn default() -> Self {
        Self::new()
    }
}