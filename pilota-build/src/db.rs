use std::{fmt, path::PathBuf, sync::Arc};

use rustc_hash::{FxHashMap, FxHashSet};
use salsa::Durability;

use crate::{
    TagId,
    middle::{
        context::{CrateId, DefLocation},
        rir,
        ty::{AdtDef, AdtKind, CodegenTy, TyKind},
        type_graph::TypeGraph,
        workspace_graph::WorkspaceGraph,
    },
    symbol::{DefId, FileId},
    tags::Tags,
};

#[salsa::query_group(RirDatabaseStorage)]
pub trait RirDatabase: salsa::Database {
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

    fn codegen_ty(&self, did: DefId) -> CodegenTy;

    fn service_methods(&self, def_id: DefId) -> Arc<[Arc<rir::Method>]>;

    fn is_arg(&self, def_id: DefId) -> bool;
}

fn node(db: &dyn RirDatabase, def_id: DefId) -> Option<rir::Node> {
    db.nodes().get(&def_id).cloned()
}

fn file(db: &dyn RirDatabase, file_id: FileId) -> Option<Arc<rir::File>> {
    db.files().get(&file_id).cloned()
}

fn file_id(db: &dyn RirDatabase, path: PathBuf) -> Option<FileId> {
    db.file_ids_map().get(&path).cloned()
}

fn item(db: &dyn RirDatabase, def_id: DefId) -> Option<Arc<rir::Item>> {
    let node = db.node(def_id);
    match node {
        Some(rir::Node {
            kind: rir::NodeKind::Item(i),
            ..
        }) => Some(i),
        None => None,
        _ => panic!("{def_id:?} is not an item"),
    }
}

fn expect_item(db: &dyn RirDatabase, def_id: DefId) -> Arc<rir::Item> {
    db.item(def_id).unwrap()
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

#[salsa::database(RirDatabaseStorage)]
pub struct RootDatabase {
    storage: salsa::Storage<Self>,
}

impl Default for RootDatabase {
    fn default() -> Self {
        let storage = salsa::Storage::default();
        Self { storage }
    }
}

impl fmt::Debug for RootDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RootDatabase").finish()
    }
}

impl salsa::Database for RootDatabase {}

impl salsa::ParallelDatabase for RootDatabase {
    fn snapshot(&self) -> salsa::Snapshot<RootDatabase> {
        salsa::Snapshot::new(RootDatabase {
            storage: self.storage.snapshot(),
        })
    }
}

impl RootDatabase {

    pub fn collect_def_ids(
        &self,
        input: &[DefId],
        locations: Option<&FxHashMap<DefId, DefLocation>>,
    ) -> FxHashMap<DefId, DefLocation> {
        use crate::middle::ty::Visitor;
        struct PathCollector<'a> {
            map: &'a mut FxHashMap<DefId, DefLocation>,
            visiting: &'a mut FxHashSet<DefId>,
            db: &'a RootDatabase,
            locations: Option<&'a FxHashMap<DefId, DefLocation>>,
        }

        impl crate::ty::Visitor for PathCollector<'_> {
            fn visit_path(&mut self, path: &crate::rir::Path) {
                collect(self.db, path.did, self.map, self.visiting, self.locations)
            }
        }

        fn collect(
            db: &RootDatabase,
            def_id: DefId,
            map: &mut FxHashMap<DefId, DefLocation>,
            visiting: &mut FxHashSet<DefId>,
            locations: Option<&FxHashMap<DefId, DefLocation>>,
        ) {
            if map.contains_key(&def_id) {
                return;
            }
            if let Some(locations) = locations {
                map.insert(def_id, locations[&def_id].clone());
            } else if !matches!(&*db.item(def_id).unwrap(), rir::Item::Mod(_)) {
                let file_id = db.node(def_id).unwrap().file_id;

                if db.input_files().contains(&file_id) {
                    let type_graph = db.workspace_graph();
                    let node = type_graph.node_map[&def_id];
                    for from in type_graph
                        .graph
                        .neighbors_directed(node, petgraph::Direction::Incoming)
                    {
                        let from_def_id = type_graph.id_map[&from];
                        let from_file_id = db.node(from_def_id).unwrap().file_id;

                        if from_file_id != file_id {
                            map.insert(def_id, DefLocation::Dynamic);
                            break;
                        } else {
                            if !map.contains_key(&from_def_id) && !visiting.contains(&from_def_id) {
                                visiting.insert(from_def_id);
                                collect(db, from_def_id, map, visiting, locations);
                                visiting.remove(&from_def_id);
                            }
                            if map
                                .get(&from_def_id)
                                .map(|v| match v {
                                    DefLocation::Fixed(_, _) => false,
                                    DefLocation::Dynamic => true,
                                })
                                .unwrap_or(true)
                            {
                                map.insert(def_id, DefLocation::Dynamic);
                                break;
                            }
                        }
                    }
                    map.entry(def_id).or_insert_with(|| {
                        let file = db.file(file_id).unwrap();
                        DefLocation::Fixed(CrateId { main_file: file_id }, file.package.clone())
                    });
                } else {
                    map.insert(def_id, DefLocation::Dynamic);
                }
            }

            let node = db.node(def_id).unwrap();
            tracing::trace!("collecting {:?}", node.expect_item().symbol_name());

            node.related_nodes
                .iter()
                .for_each(|def_id| collect(db, *def_id, map, visiting, locations));

            let item = node.expect_item();

            match item {
                rir::Item::Message(m) => m.fields.iter().for_each(|f| {
                    PathCollector {
                        db,
                        map,
                        visiting,
                        locations,
                    }
                    .visit(&f.ty)
                }),
                rir::Item::Enum(e) => e.variants.iter().flat_map(|v| &v.fields).for_each(|ty| {
                    PathCollector {
                        db,
                        map,
                        visiting,
                        locations,
                    }
                    .visit(ty)
                }),
                rir::Item::Service(s) => {
                    s.extend
                        .iter()
                        .for_each(|p| collect(db, p.did, map, visiting, locations));
                    s.methods
                        .iter()
                        .flat_map(|m| m.args.iter().map(|f| &f.ty).chain(std::iter::once(&m.ret)))
                        .for_each(|ty| {
                            PathCollector {
                                db,
                                map,
                                visiting,
                                locations,
                            }
                            .visit(ty)
                        });
                }
                rir::Item::NewType(n) => PathCollector {
                    db,
                    map,
                    visiting,
                    locations,
                }
                .visit(&n.ty),
                rir::Item::Const(c) => {
                    PathCollector {
                        db,
                        map,
                        visiting,
                        locations,
                    }
                    .visit(&c.ty);
                }
                rir::Item::Mod(m) => {
                    m.items
                        .iter()
                        .for_each(|i| collect(db, *i, map, visiting, locations));
                }
            }
        }
        let mut map = FxHashMap::default();
        let mut visiting = FxHashSet::default();

        input.iter().for_each(|def_id| {
            visiting.insert(*def_id);
            collect(self, *def_id, &mut map, &mut visiting, locations);
            visiting.remove(def_id);
        });

        map
    }
}
