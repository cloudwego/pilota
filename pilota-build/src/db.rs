mod salsa_ids;
pub mod cached_queries;

use std::{fmt::Debug, path::PathBuf, sync::Arc};

use faststr::FastStr;
use rustc_hash::{FxHashMap, FxHashSet};

pub use salsa_ids::{SalsaDefId, SalsaFileId, IntoSalsa};
pub use cached_queries::CachedQueries;

use crate::{
    middle::context::{CrateId, DefLocation},
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

    pub fn with_files(
        mut self,
        files: impl Iterator<Item = (FileId, Arc<File>)>,
    ) -> Self {
        self.files = Arc::new(files.collect());
        self
    }

    pub fn with_file_ids_map(
        mut self,
        file_ids_map: FxHashMap<Arc<PathBuf>, FileId>,
    ) -> Self {
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
            } else if let Some(item) = db.item(def_id) {
                if matches!(&*item, rir::Item::Mod(_)) {
                    return;
                }
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
    
    // 查询方法 - 直接实现，不使用 tracked
    fn node(&self, def_id: DefId) -> Option<Node> {
        self.nodes().get(&def_id).cloned()
    }
    
    fn file(&self, file_id: FileId) -> Option<Arc<File>> {
        self.files().get(&file_id).cloned()
    }
    
    fn file_id(&self, path: PathBuf) -> Option<FileId> {
        self.file_ids_map().get(&path).cloned()
    }
    
    fn item(&self, def_id: DefId) -> Option<Arc<Item>> {
        let node = self.node(def_id)?;
        match node.kind {
            rir::NodeKind::Item(i) => Some(i),
            _ => panic!("{def_id:?} is not an item"),
        }
    }
    
    fn expect_item(&self, def_id: DefId) -> Arc<Item> {
        self.item(def_id).unwrap()
    }
    
    fn service_methods(&self, def_id: DefId) -> Arc<[Arc<rir::Method>]> {
        let item = self.expect_item(def_id);
        let service = match &*item {
            rir::Item::Service(s) => s,
            _ => panic!(),
        };
        let methods = service
            .extend
            .iter()
            .flat_map(|p| {
                self.service_methods(p.did)
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
    
    fn is_arg(&self, def_id: DefId) -> bool {
        self.args().contains(&def_id)
    }
}

// Extension trait for cached queries
pub trait RirDatabaseExt: RirDatabase + CachedQueries + Sized {
    /// Get a node using the cached version
    fn node_cached(&self, def_id: DefId) -> Option<Node> {
        let salsa_id = def_id.into_salsa(self);
        cached_queries::get_node(self, salsa_id)
    }
    
    /// Get a file using the cached version
    fn file_cached(&self, file_id: FileId) -> Option<Arc<File>> {
        let salsa_id = file_id.into_salsa(self);
        cached_queries::get_file(self, salsa_id)
    }
    
    /// Get an item using the cached version
    fn item_cached(&self, def_id: DefId) -> Option<Arc<Item>> {
        let salsa_id = def_id.into_salsa(self);
        cached_queries::get_item(self, salsa_id)
    }
    
    /// Get service methods using the cached version
    fn service_methods_cached(&self, def_id: DefId) -> Arc<[Arc<rir::Method>]> {
        let salsa_id = def_id.into_salsa(self);
        cached_queries::get_service_methods(self, salsa_id)
    }
    
    /// Check if DefId is an argument using the cached version
    fn is_arg_cached_ext(&self, def_id: DefId) -> bool {
        let salsa_id = def_id.into_salsa(self);
        cached_queries::is_arg_cached(self, salsa_id)
    }
}

// Implement the extension trait for any type that implements both traits
impl<T: RirDatabase + CachedQueries> RirDatabaseExt for T {}

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

    fn node(&self, def_id: DefId) -> Option<Node> {
        // Use cached version for better performance
        self.node_cached(def_id)
    }

    fn item(&self, def_id: DefId) -> Option<Arc<Item>> {
        // Use cached version for better performance
        self.item_cached(def_id)
    }

    fn file(&self, file_id: FileId) -> Option<Arc<File>> {
        // Use cached version for better performance
        self.file_cached(file_id)
    }

    fn service_methods(&self, service_def_id: DefId) -> Arc<[Arc<rir::Method>]> {
        // Use cached version for better performance
        self.service_methods_cached(service_def_id)
    }

    fn is_arg(&self, def_id: DefId) -> bool {
        // Use cached version for better performance
        self.is_arg_cached_ext(def_id)
    }
}

impl Debug for RootDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RootDatabase {{ .. }}")
    }
}


