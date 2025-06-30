//! Cached query functions using Salsa's tracked mechanism

use std::sync::Arc;

use crate::{
    db::{RirDatabase, RootDatabase, SalsaDefId, SalsaFileId},
    rir::{self, File, Item, Node},
};

// We create a separate trait that includes the tracked functions
#[salsa::db]
pub trait CachedQueries: RirDatabase + salsa::Database {}

// Implement for RootDatabase
#[salsa::db]
impl CachedQueries for RootDatabase {}

/// Get a node by DefId - cached version
#[salsa::tracked]
pub fn get_node<'db>(db: &'db dyn CachedQueries, def_id: SalsaDefId<'db>) -> Option<Node> {
    let real_id = def_id.id(db);
    db.nodes().get(&real_id).cloned()
}

/// Get a file by FileId - cached version
#[salsa::tracked]
pub fn get_file<'db>(db: &'db dyn CachedQueries, file_id: SalsaFileId<'db>) -> Option<Arc<File>> {
    let real_id = file_id.id(db);
    db.files().get(&real_id).cloned()
}

/// Get an item by DefId - cached version
#[salsa::tracked]
pub fn get_item<'db>(db: &'db dyn CachedQueries, def_id: SalsaDefId<'db>) -> Option<Arc<Item>> {
    let node = get_node(db, def_id)?;
    match node.kind {
        rir::NodeKind::Item(i) => Some(i),
        _ => None,
    }
}

/// Get service methods - cached version
/// This is especially beneficial as it involves recursive computation
#[salsa::tracked]
pub fn get_service_methods<'db>(db: &'db dyn CachedQueries, def_id: SalsaDefId<'db>) -> Arc<[Arc<rir::Method>]> {
    let item = match get_item(db, def_id) {
        Some(item) => item,
        None => return Arc::new([]),
    };
    
    let service = match &*item {
        rir::Item::Service(s) => s,
        _ => return Arc::new([]),
    };
    
    let methods = service
        .extend
        .iter()
        .flat_map(|p| {
            use crate::db::IntoSalsa;
            let extend_id = p.did.into_salsa(db);
            get_service_methods(db, extend_id)
                .iter()
                .map(|m| match m.source {
                    rir::MethodSource::Extend(_) => m.clone(),
                    rir::MethodSource::Own => Arc::from(rir::Method {
                        source: rir::MethodSource::Extend(p.did),
                        ..(**m).clone()
                    }),
                })
                .collect::<Vec<_>>()
        })
        .chain(service.methods.iter().cloned());

    Arc::from_iter(methods)
}

/// Check if a DefId is an argument - cached version
#[salsa::tracked]
pub fn is_arg_cached<'db>(db: &'db dyn CachedQueries, def_id: SalsaDefId<'db>) -> bool {
    let real_id = def_id.id(db);
    db.args().contains(&real_id)
}