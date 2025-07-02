//! Cached query functions using Salsa's tracked mechanism

use std::sync::Arc;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    db::{RootDatabase, SalsaDefId, SalsaFileId, SalsaTyKind},
    middle::ty::{AdtDef, AdtKind, CodegenTy},
    rir::{self, File, Item, Node},
    symbol::{DefId, FileId},
};

// Define a trait for basic database access without circular dependency
pub trait DatabaseStorage: salsa::Database {
    fn nodes(&self) -> &Arc<FxHashMap<DefId, rir::Node>>;
    fn files(&self) -> &Arc<FxHashMap<FileId, Arc<rir::File>>>;
    fn args(&self) -> &Arc<FxHashSet<DefId>>;
}

// We create a separate trait that includes the tracked functions
#[salsa::db]
pub trait CachedQueries: DatabaseStorage + crate::db::RirDatabase + salsa::Database {}

// Implement DatabaseStorage for RootDatabase
impl DatabaseStorage for RootDatabase {
    fn nodes(&self) -> &Arc<FxHashMap<DefId, rir::Node>> {
        &self.nodes
    }

    fn files(&self) -> &Arc<FxHashMap<FileId, Arc<rir::File>>> {
        &self.files
    }

    fn args(&self) -> &Arc<FxHashSet<DefId>> {
        &self.args
    }
}

// Implement for RootDatabase
#[salsa::db]
impl CachedQueries for RootDatabase {}

/// Get a node by DefId - cached version
#[salsa::tracked]
pub fn get_node<'db>(db: &'db dyn CachedQueries, def_id: SalsaDefId<'db>) -> Option<Node> {
    let real_id = def_id.id(db);
    DatabaseStorage::nodes(db).get(&real_id).cloned()
}

/// Get a file by FileId - cached version
#[salsa::tracked]
pub fn get_file<'db>(db: &'db dyn CachedQueries, file_id: SalsaFileId<'db>) -> Option<Arc<File>> {
    let real_id = file_id.id(db);
    DatabaseStorage::files(db).get(&real_id).cloned()
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
pub fn get_service_methods<'db>(
    db: &'db dyn CachedQueries,
    def_id: SalsaDefId<'db>,
) -> Arc<[Arc<rir::Method>]> {
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
    DatabaseStorage::args(db).contains(&real_id)
}

/// Get codegen type for item - cached version
#[salsa::tracked]
pub fn codegen_item_ty_cached<'db>(
    db: &'db dyn CachedQueries,
    ty_kind: SalsaTyKind<'db>,
) -> CodegenTy {
    let ty = ty_kind.ty(db);
    ty.to_codegen_item_ty(db)
}

/// Get codegen type for const - cached version
#[salsa::tracked]
pub fn codegen_const_ty_cached<'db>(
    db: &'db dyn CachedQueries,
    ty_kind: SalsaTyKind<'db>,
) -> CodegenTy {
    let ty = ty_kind.ty(db);
    ty.to_codegen_const_ty(db)
}

/// Get codegen type for DefId - cached version
#[salsa::tracked]
pub fn codegen_ty_cached<'db>(db: &'db dyn CachedQueries, def_id: SalsaDefId<'db>) -> CodegenTy {
    use crate::db::IntoSalsa;

    let real_id = def_id.id(db);
    let node = get_node(db, def_id).unwrap();

    match &node.kind {
        rir::NodeKind::Item(item) => {
            let kind = match &**item {
                rir::Item::Message(_) => AdtKind::Struct,
                rir::Item::Enum(_) => AdtKind::Enum,
                rir::Item::Service(_) => unimplemented!(),
                rir::Item::NewType(t) => {
                    let ty_salsa = t.ty.kind.clone().into_salsa(db);
                    AdtKind::NewType(Arc::from(codegen_item_ty_cached(db, ty_salsa)))
                }
                rir::Item::Const(c) => {
                    let ty_salsa = c.ty.kind.clone().into_salsa(db);
                    let mut ty = codegen_const_ty_cached(db, ty_salsa);
                    if let CodegenTy::StaticRef(inner) = ty {
                        ty = CodegenTy::LazyStaticRef(inner)
                    }
                    return ty;
                }
                rir::Item::Mod(_) => unreachable!(),
            };
            CodegenTy::Adt(AdtDef { did: real_id, kind })
        }
        rir::NodeKind::Variant(_) => CodegenTy::Adt(AdtDef {
            did: node.parent.unwrap(),
            kind: AdtKind::Enum,
        }),
        rir::NodeKind::Field(_) => todo!(),
        rir::NodeKind::Method(_) => todo!(),
        rir::NodeKind::Arg(_) => todo!(),
    }
}
