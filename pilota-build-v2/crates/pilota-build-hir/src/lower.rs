//! HIR lowering utilities.

use crate::{HirId, LocalId};
use pilota_build_common::{DefId, DefIndex};

/// Context for lowering AST to HIR.
pub struct LoweringContext {
    current_def_id: DefId,
    local_id_counter: u32,
}

impl LoweringContext {
    pub fn new() -> Self {
        LoweringContext {
            current_def_id: DefId::local(DefIndex::from_u32(0)),
            local_id_counter: 0,
        }
    }

    /// Get the next HIR ID.
    pub fn next_hir_id(&mut self) -> HirId {
        let local_id = LocalId(self.local_id_counter);
        self.local_id_counter += 1;
        HirId {
            owner: self.current_def_id,
            local: local_id,
        }
    }

    /// Reset the local ID counter.
    pub fn reset_local_id_counter(&mut self) {
        self.local_id_counter = 0;
    }

    /// Set the current definition ID.
    pub fn set_current_def_id(&mut self, def_id: DefId) {
        self.current_def_id = def_id;
    }
}

impl Default for LoweringContext {
    fn default() -> Self {
        Self::new()
    }
}