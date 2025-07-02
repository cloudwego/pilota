pub mod cached_queries;
mod salsa_ids;

use std::{fmt::Debug, path::PathBuf, sync::Arc};

pub use cached_queries::CachedQueries;
use faststr::FastStr;
use rustc_hash::{FxHashMap, FxHashSet};
pub use salsa_ids::{IntoSalsa, SalsaDefId, SalsaFileId, SalsaTyKind};

use crate::{
    middle::context::{CrateId, DefLocation},
    middle::ty::{CodegenTy, TyKind},
    rir::{self, File, Item, Node},
    symbol::{DefId, FileId},
    tags::{TagId, Tags},
};

pub type ItemPath = Arc<Vec<FastStr>>;
pub type TypeGraph = crate::middle::type_graph::TypeGraph;
pub type WorkspaceGraph = crate::middle::workspace_graph::WorkspaceGraph;

fn empty_type_graph() -> TypeGraph {
    TypeGraph::from_items(std::iter::empty::<(DefId, Arc<Item>)>())
}

fn empty_workspace_graph() -> WorkspaceGraph {
    WorkspaceGraph::from_items(std::iter::empty::<(DefId, Arc<Item>)>())
}

// 数据库定义 - 使用新的 #[salsa::db] 宏
#[salsa::db]
#[derive(Clone)]
pub struct RootDatabase {
    storage: salsa::Storage<Self>,
    // 直接在数据库中存储数据
    nodes: Arc<FxHashMap<DefId, rir::Node>>,
    files: Arc<FxHashMap<FileId, Arc<rir::File>>>,
    file_ids_map: Arc<FxHashMap<Arc<PathBuf>, FileId>>,
    type_graph: Arc<TypeGraph>,
    tags_map: Arc<FxHashMap<TagId, Arc<Tags>>>,
    input_files: Arc<Vec<FileId>>,
    args: Arc<FxHashSet<DefId>>,
    workspace_graph: Arc<WorkspaceGraph>,
}

impl Default for RootDatabase {
    fn default() -> Self {
        RootDatabase {
            storage: salsa::Storage::new(None),
            nodes: Arc::new(FxHashMap::default()),
            files: Arc::new(FxHashMap::default()),
            file_ids_map: Arc::new(FxHashMap::default()),
            type_graph: Arc::new(empty_type_graph()),
            tags_map: Arc::new(FxHashMap::default()),
            input_files: Arc::new(Vec::new()),
            args: Arc::new(FxHashSet::default()),
            workspace_graph: Arc::new(empty_workspace_graph()),
        }
    }
}

impl RootDatabase {
    pub fn with_nodes(mut self, nodes: FxHashMap<DefId, rir::Node>) -> Self {
        self.nodes = Arc::new(nodes);
        self
    }

    pub fn with_workspace_graph(mut self, g: WorkspaceGraph) -> Self {
        self.workspace_graph = Arc::new(g);
        self
    }

    pub fn with_input_files(mut self, input_files: Vec<FileId>) -> Self {
        self.input_files = Arc::new(input_files);
        self
    }

    pub fn with_files(mut self, files: impl Iterator<Item = (FileId, Arc<File>)>) -> Self {
        self.files = Arc::new(files.collect());
        self
    }

    pub fn with_file_ids_map(mut self, file_ids_map: FxHashMap<Arc<PathBuf>, FileId>) -> Self {
        self.file_ids_map = Arc::new(file_ids_map);
        self
    }

    pub fn with_tags(
        mut self,
        tags_map: FxHashMap<TagId, Arc<Tags>>,
        type_graph: TypeGraph,
    ) -> Self {
        self.tags_map = Arc::new(tags_map);
        self.type_graph = Arc::new(type_graph);
        self
    }

    pub fn with_args(mut self, args: FxHashSet<DefId>) -> Self {
        self.args = Arc::new(args);
        self
    }

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

// 实现 salsa::Database trait
#[salsa::db]
impl salsa::Database for RootDatabase {}

// 定义 RirDatabase trait
pub trait RirDatabase: salsa::Database {
    // 访问数据库中的数据
    fn nodes(&self) -> &Arc<FxHashMap<DefId, rir::Node>>;
    fn files(&self) -> &Arc<FxHashMap<FileId, Arc<rir::File>>>;
    fn file_ids_map(&self) -> &Arc<FxHashMap<Arc<PathBuf>, FileId>>;
    fn type_graph(&self) -> &Arc<TypeGraph>;
    fn tags_map(&self) -> &Arc<FxHashMap<TagId, Arc<Tags>>>;
    fn input_files(&self) -> &Arc<Vec<FileId>>;
    fn args(&self) -> &Arc<FxHashSet<DefId>>;
    fn workspace_graph(&self) -> &Arc<WorkspaceGraph>;

    // 查询方法
    fn node(&self, def_id: DefId) -> Option<Node>;

    fn file(&self, file_id: FileId) -> Option<Arc<File>>;

    fn file_id(&self, path: PathBuf) -> Option<FileId> {
        self.file_ids_map().get(&path).cloned()
    }

    fn item(&self, def_id: DefId) -> Option<Arc<Item>>;

    fn expect_item(&self, def_id: DefId) -> Arc<Item> {
        self.item(def_id).unwrap()
    }

    fn codegen_item_ty(&self, ty: TyKind) -> CodegenTy;
    
    fn codegen_const_ty(&self, ty: TyKind) -> CodegenTy;
    
    fn codegen_ty(&self, def_id: DefId) -> CodegenTy;

    fn service_methods(&self, def_id: DefId) -> Arc<[Arc<rir::Method>]>;

    fn is_arg(&self, def_id: DefId) -> bool;
}

// 为 RootDatabase 实现 RirDatabase trait
impl RirDatabase for RootDatabase {
    fn nodes(&self) -> &Arc<FxHashMap<DefId, rir::Node>> {
        &self.nodes
    }

    fn files(&self) -> &Arc<FxHashMap<FileId, Arc<rir::File>>> {
        &self.files
    }

    fn file_ids_map(&self) -> &Arc<FxHashMap<Arc<PathBuf>, FileId>> {
        &self.file_ids_map
    }

    fn type_graph(&self) -> &Arc<TypeGraph> {
        &self.type_graph
    }

    fn tags_map(&self) -> &Arc<FxHashMap<TagId, Arc<Tags>>> {
        &self.tags_map
    }

    fn input_files(&self) -> &Arc<Vec<FileId>> {
        &self.input_files
    }

    fn args(&self) -> &Arc<FxHashSet<DefId>> {
        &self.args
    }

    fn workspace_graph(&self) -> &Arc<WorkspaceGraph> {
        &self.workspace_graph
    }

    // 使用缓存实现查询方法
    fn node(&self, def_id: DefId) -> Option<Node> {
        use cached_queries::{CachedQueries, get_node};
        let salsa_id = def_id.into_salsa(self as &dyn CachedQueries);
        get_node(self as &dyn CachedQueries, salsa_id)
    }

    fn file(&self, file_id: FileId) -> Option<Arc<File>> {
        use cached_queries::{CachedQueries, get_file};
        let salsa_id = file_id.into_salsa(self as &dyn CachedQueries);
        get_file(self as &dyn CachedQueries, salsa_id)
    }

    fn item(&self, def_id: DefId) -> Option<Arc<Item>> {
        use cached_queries::{CachedQueries, get_item};
        let salsa_id = def_id.into_salsa(self as &dyn CachedQueries);
        get_item(self as &dyn CachedQueries, salsa_id)
    }

    fn service_methods(&self, def_id: DefId) -> Arc<[Arc<rir::Method>]> {
        use cached_queries::{CachedQueries, get_service_methods};
        let salsa_id = def_id.into_salsa(self as &dyn CachedQueries);
        get_service_methods(self as &dyn CachedQueries, salsa_id)
    }

    fn is_arg(&self, def_id: DefId) -> bool {
        use cached_queries::{CachedQueries, is_arg_cached};
        let salsa_id = def_id.into_salsa(self as &dyn CachedQueries);
        is_arg_cached(self as &dyn CachedQueries, salsa_id)
    }

    fn codegen_item_ty(&self, ty: TyKind) -> CodegenTy {
        use cached_queries::{CachedQueries, codegen_item_ty_cached};
        let salsa_ty = ty.into_salsa(self as &dyn CachedQueries);
        codegen_item_ty_cached(self as &dyn CachedQueries, salsa_ty)
    }
    
    fn codegen_const_ty(&self, ty: TyKind) -> CodegenTy {
        use cached_queries::{CachedQueries, codegen_const_ty_cached};
        let salsa_ty = ty.into_salsa(self as &dyn CachedQueries);
        codegen_const_ty_cached(self as &dyn CachedQueries, salsa_ty)
    }
    
    fn codegen_ty(&self, def_id: DefId) -> CodegenTy {
        use cached_queries::{CachedQueries, codegen_ty_cached};
        let salsa_id = def_id.into_salsa(self as &dyn CachedQueries);
        codegen_ty_cached(self as &dyn CachedQueries, salsa_id)
    }
}

impl Debug for RootDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RootDatabase {{ .. }}")
    }
}
