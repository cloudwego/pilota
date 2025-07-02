//! Salsa wrapper types for DefId and FileId
//!
//! Since DefId and FileId are not Salsa structs, we need to wrap them
//! to use them as parameters for tracked functions.

use crate::symbol::{DefId, FileId};
use crate::middle::ty::TyKind;

/// Salsa wrapper for DefId
#[salsa::interned]
pub struct SalsaDefId<'db> {
    pub id: DefId,
}

/// Salsa wrapper for FileId  
#[salsa::interned]
pub struct SalsaFileId<'db> {
    pub id: FileId,
}

/// Salsa wrapper for TyKind
#[salsa::interned]
pub struct SalsaTyKind<'db> {
    pub ty: TyKind,
}

// Helper trait to convert between regular IDs and Salsa IDs
pub trait IntoSalsa {
    type SalsaType<'db>;
    fn into_salsa<'db>(
        self,
        db: &'db dyn crate::db::cached_queries::CachedQueries,
    ) -> Self::SalsaType<'db>;
}

impl IntoSalsa for DefId {
    type SalsaType<'db> = SalsaDefId<'db>;

    fn into_salsa<'db>(
        self,
        db: &'db dyn crate::db::cached_queries::CachedQueries,
    ) -> SalsaDefId<'db> {
        SalsaDefId::new(db, self)
    }
}

impl IntoSalsa for FileId {
    type SalsaType<'db> = SalsaFileId<'db>;

    fn into_salsa<'db>(
        self,
        db: &'db dyn crate::db::cached_queries::CachedQueries,
    ) -> SalsaFileId<'db> {
        SalsaFileId::new(db, self)
    }
}

impl IntoSalsa for TyKind {
    type SalsaType<'db> = SalsaTyKind<'db>;

    fn into_salsa<'db>(
        self,
        db: &'db dyn crate::db::cached_queries::CachedQueries,
    ) -> SalsaTyKind<'db> {
        SalsaTyKind::new(db, self)
    }
}
