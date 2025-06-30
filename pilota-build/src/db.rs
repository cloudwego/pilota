use std::{fmt, path::PathBuf, sync::Arc};

use rustc_hash::{FxHashMap, FxHashSet};

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

// Input structs for salsa
#[salsa::input]
pub struct NodesInput {
    pub nodes: Arc<FxHashMap<DefId, rir::Node>>,
}

#[salsa::input]
pub struct FilesInput {
    pub files: Arc<FxHashMap<FileId, Arc<rir::File>>>,
}

#[salsa::input]
pub struct FileIdsMapInput {
    pub file_ids_map: Arc<FxHashMap<Arc<PathBuf>, FileId>>,
}

#[salsa::input]
pub struct TypeGraphInput {
    pub type_graph: Arc<TypeGraph>,
}

#[salsa::input]
pub struct TagsMapInput {
    pub tags_map: Arc<FxHashMap<TagId, Arc<Tags>>>,
}

#[salsa::input]
pub struct InputFilesInput {
    pub input_files: Arc<Vec<FileId>>,
}

#[salsa::input]
pub struct ArgsInput {
    pub args: Arc<FxHashSet<DefId>>,
}

#[salsa::input]
pub struct WorkspaceGraphInput {
    pub workspace_graph: Arc<WorkspaceGraph>,
}

// Database trait
#[salsa::db]
pub trait RirDatabase: salsa::Database {
    // Accessor methods for inputs
    fn nodes(&self) -> Arc<FxHashMap<DefId, rir::Node>> {
        NodesInput::new(self, Default::default()).nodes(self)
    }
    
    fn files(&self) -> Arc<FxHashMap<FileId, Arc<rir::File>>> {
        FilesInput::new(self, Default::default()).files(self)
    }
    
    fn file_ids_map(&self) -> Arc<FxHashMap<Arc<PathBuf>, FileId>> {
        FileIdsMapInput::new(self, Default::default()).file_ids_map(self)
    }
    
    fn type_graph(&self) -> Arc<TypeGraph> {
        TypeGraphInput::new(self, Default::default()).type_graph(self)
    }
    
    fn tags_map(&self) -> Arc<FxHashMap<TagId, Arc<Tags>>> {
        TagsMapInput::new(self, Default::default()).tags_map(self)
    }
    
    fn input_files(&self) -> Arc<Vec<FileId>> {
        InputFilesInput::new(self, Default::default()).input_files(self)
    }
    
    fn args(&self) -> Arc<FxHashSet<DefId>> {
        ArgsInput::new(self, Default::default()).args(self)
    }
    
    fn workspace_graph(&self) -> Arc<WorkspaceGraph> {
        WorkspaceGraphInput::new(self, Default::default()).workspace_graph(self)
    }
    
    // Setter methods for inputs
    fn set_nodes(&mut self, nodes: Arc<FxHashMap<DefId, rir::Node>>) {
        NodesInput::new(self, nodes).store_in_db(self);
    }
    
    fn set_files(&mut self, files: Arc<FxHashMap<FileId, Arc<rir::File>>>) {
        FilesInput::new(self, files).store_in_db(self);
    }
    
    fn set_file_ids_map(&mut self, file_ids_map: Arc<FxHashMap<Arc<PathBuf>, FileId>>) {
        FileIdsMapInput::new(self, file_ids_map).store_in_db(self);
    }
    
    fn set_type_graph(&mut self, type_graph: Arc<TypeGraph>) {
        TypeGraphInput::new(self, type_graph).store_in_db(self);
    }
    
    fn set_tags_map(&mut self, tags_map: Arc<FxHashMap<TagId, Arc<Tags>>>) {
        TagsMapInput::new(self, tags_map).store_in_db(self);
    }
    
    fn set_input_files(&mut self, input_files: Arc<Vec<FileId>>) {
        InputFilesInput::new(self, input_files).store_in_db(self);
    }
    
    fn set_args(&mut self, args: Arc<FxHashSet<DefId>>) {
        ArgsInput::new(self, args).store_in_db(self);
    }
    
    fn set_workspace_graph(&mut self, workspace_graph: Arc<WorkspaceGraph>) {
        WorkspaceGraphInput::new(self, workspace_graph).store_in_db(self);
    }
}

// Tracked functions
#[salsa::tracked]
pub fn node(db: &dyn RirDatabase, def_id: DefId) -> Option<rir::Node> {
    db.nodes().get(&def_id).cloned()
}

#[salsa::tracked]
pub fn file(db: &dyn RirDatabase, file_id: FileId) -> Option<Arc<rir::File>> {
    db.files().get(&file_id).cloned()
}

#[salsa::tracked]
pub fn file_id(db: &dyn RirDatabase, path: PathBuf) -> Option<FileId> {
    db.file_ids_map().get(&path).cloned()
}

#[salsa::tracked]
pub fn item(db: &dyn RirDatabase, def_id: DefId) -> Option<Arc<rir::Item>> {
    let node = node(db, def_id);
    match node {
        Some(rir::Node {
            kind: rir::NodeKind::Item(i),
            ..
        }) => Some(i),
        None => None,
        _ => panic!("{def_id:?} is not an item"),
    }
}

#[salsa::tracked]
pub fn expect_item(db: &dyn RirDatabase, def_id: DefId) -> Arc<rir::Item> {
    item(db, def_id).unwrap()
}

#[salsa::tracked]
pub fn codegen_item_ty(db: &dyn RirDatabase, ty: TyKind) -> CodegenTy {
    ty.to_codegen_item_ty(db)
}

#[salsa::tracked]
pub fn codegen_const_ty(db: &dyn RirDatabase, ty: TyKind) -> CodegenTy {
    ty.to_codegen_const_ty(db)
}

#[salsa::tracked]
pub fn codegen_ty(db: &dyn RirDatabase, did: DefId) -> CodegenTy {
    let node = node(db, did).unwrap();
    match &node.kind {
        rir::NodeKind::Item(item) => {
            let kind = match &**item {
                rir::Item::Message(_) => AdtKind::Struct,
                rir::Item::Enum(_) => AdtKind::Enum,
                rir::Item::Service(_) => unimplemented!(),
                rir::Item::NewType(t) => {
                    AdtKind::NewType(Arc::from(codegen_item_ty(db, t.ty.kind.clone())))
                }
                rir::Item::Const(c) => {
                    let mut ty = codegen_const_ty(db, c.ty.kind.clone());
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

#[salsa::tracked]
pub fn service_methods(db: &dyn RirDatabase, def_id: DefId) -> Arc<[Arc<rir::Method>]> {
    let item = expect_item(db, def_id);
    let service = match &*item {
        rir::Item::Service(s) => s,
        _ => panic!(),
    };
    let methods = service
        .extend
        .iter()
        .flat_map(|p| {
            service_methods(db, p.did)
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

#[salsa::tracked]
pub fn is_arg(db: &dyn RirDatabase, def_id: DefId) -> bool {
    db.args().contains(&def_id)
}

// Root database struct
pub struct RootDatabase {
    storage: salsa::DatabaseImpl<RootDatabase>,
}

impl Default for RootDatabase {
    fn default() -> Self {
        let storage = salsa::DatabaseImpl::new();
        Self { storage }
    }
}

impl fmt::Debug for RootDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RootDatabase").finish()
    }
}

impl salsa::Database for RootDatabase {
    fn storage(&self) -> &salsa::Storage<Self> {
        &self.storage
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
            } else if !matches!(&*item(db, def_id).unwrap(), rir::Item::Mod(_)) {
                let file_id = node(db, def_id).unwrap().file_id;

                if db.input_files().contains(&file_id) {
                    let type_graph = db.workspace_graph();
                    let node = type_graph.node_map[&def_id];
                    for from in type_graph
                        .graph
                        .neighbors_directed(node, petgraph::Direction::Incoming)
                    {
                        let from_def_id = type_graph.id_map[&from];
                        let from_file_id = node(db, from_def_id).unwrap().file_id;

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
                        let file = file(db, file_id).unwrap();
                        DefLocation::Fixed(CrateId { main_file: file_id }, file.package.clone())
                    });
                } else {
                    map.insert(def_id, DefLocation::Dynamic);
                }
            }

            let node = node(db, def_id).unwrap();
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
