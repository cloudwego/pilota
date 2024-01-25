use std::{fmt, path::PathBuf, sync::Arc};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    middle::{
        rir::{self},
        ty::{AdtDef, AdtKind, CodegenTy, TyKind},
        type_graph::TypeGraph,
        workspace_graph::WorkspaceGraph,
    },
    symbol::{DefId, FileId},
    tags::Tags,
    TagId,
};

#[derive(Default)]
#[salsa::database(RirDatabaseStorage)]
pub struct RootDatabase {
    storage: salsa::Storage<RootDatabase>,
}

impl fmt::Debug for RootDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RootDatabase").finish()
    }
}

impl salsa::ParallelDatabase for RootDatabase {
    fn snapshot(&self) -> salsa::Snapshot<RootDatabase> {
        salsa::Snapshot::new(RootDatabase {
            storage: self.storage.snapshot(),
        })
    }
}

#[salsa::query_group(RirDatabaseStorage)]
pub trait RirDatabase {
    #[salsa::input]
    fn nodes(&self) -> Arc<FxHashMap<DefId, rir::Node>>;
    #[salsa::input]
    fn files(&self) -> Arc<FxHashMap<FileId, Arc<rir::File>>>;
    #[salsa::input]
    fn file_ids_map(&self) -> Arc<FxHashMap<Arc<PathBuf>, FileId>>;
    #[salsa::input]
    fn type_graph(&self) -> Arc<TypeGraph>;
    #[salsa::input]
    fn tags_map(&self) -> Arc<FxHashMap<TagId, Arc<Tags>>>;
    #[salsa::input]
    fn input_files(&self) -> Arc<Vec<FileId>>;
    #[salsa::input]
    fn args(&self) -> Arc<FxHashSet<DefId>>;
    #[salsa::input]
    fn workspace_graph(&self) -> Arc<WorkspaceGraph>;

    fn node(&self, def_id: DefId) -> Option<rir::Node>;
    fn file(&self, file_id: FileId) -> Option<Arc<rir::File>>;
    fn file_id(&self, path: PathBuf) -> Option<FileId>;
    fn item(&self, def_id: DefId) -> Option<Arc<rir::Item>>;
    fn expect_item(&self, def_id: DefId) -> Arc<rir::Item>;
    fn codegen_item_ty(&self, ty: TyKind) -> CodegenTy;
    fn codegen_const_ty(&self, ty: TyKind) -> CodegenTy;
    fn codegen_ty(&self, def_id: DefId) -> CodegenTy;
    fn service_methods(&self, def_id: DefId) -> Arc<[Arc<rir::Method>]>;
    fn is_arg(&self, def_id: DefId) -> bool;
}

fn node(db: &dyn RirDatabase, def_id: DefId) -> Option<rir::Node> {
    db.nodes().get(&def_id).cloned()
}

fn item(db: &dyn RirDatabase, def_id: DefId) -> Option<Arc<rir::Item>> {
    let node = db.node(def_id);
    match node {
        Some(rir::Node {
            kind: rir::NodeKind::Item(i),
            ..
        }) => Some(i),
        None => None,
        _ => panic!("{:?} is not an item", def_id),
    }
}

fn expect_item(db: &dyn RirDatabase, def_id: DefId) -> Arc<rir::Item> {
    db.item(def_id).unwrap()
}

fn file(db: &dyn RirDatabase, file_id: FileId) -> Option<Arc<rir::File>> {
    db.files().get(&file_id).cloned()
}

fn file_id(db: &dyn RirDatabase, path: PathBuf) -> Option<FileId> {
    db.file_ids_map().get(&path).cloned()
}

fn codegen_item_ty(db: &dyn RirDatabase, ty: TyKind) -> CodegenTy {
    ty.to_codegen_item_ty(db)
}

fn codegen_const_ty(db: &dyn RirDatabase, ty: TyKind) -> CodegenTy {
    ty.to_codegen_const_ty(db)
}

fn codegen_ty(db: &dyn RirDatabase, did: DefId) -> CodegenTy {
    let node = db.node(did).unwrap();
    match &node.kind {
        rir::NodeKind::Item(item) => {
            let kind = match &**item {
                rir::Item::Message(_) => AdtKind::Struct,
                rir::Item::Enum(_) => AdtKind::Enum,
                rir::Item::Service(_) => unimplemented!(),
                rir::Item::NewType(t) => {
                    AdtKind::NewType(Arc::from(db.codegen_item_ty(t.ty.kind.clone())))
                }
                rir::Item::Const(c) => {
                    let mut ty = db.codegen_const_ty(c.ty.kind.clone());
                    if let CodegenTy::StaticRef(inner) = ty {
                        ty = CodegenTy::LazyStaticRef(inner)
                    }

                    return ty;
                }
                rir::Item::Mod(_) => unreachable!(),
            };
            CodegenTy::Adt(AdtDef { did, kind })
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

fn service_methods(db: &dyn RirDatabase, def_id: DefId) -> Arc<[Arc<rir::Method>]> {
    let item = db.expect_item(def_id);
    let service = match &*item {
        rir::Item::Service(s) => s,
        _ => panic!(),
    };
    let methods = service
        .extend
        .iter()
        .flat_map(|p| {
            db.service_methods(p.did)
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

fn is_arg(db: &dyn RirDatabase, def_id: DefId) -> bool {
    db.args().contains(&def_id)
}

impl salsa::Database for RootDatabase {}
