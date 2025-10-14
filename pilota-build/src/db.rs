pub mod cached_queries;
mod salsa_ids;

use std::{fmt::Debug, path::PathBuf, sync::Arc};

pub use cached_queries::CachedQueries;
use faststr::FastStr;
use rustc_hash::{FxHashMap, FxHashSet};
pub use salsa_ids::{IntoSalsa, SalsaDefId, SalsaFileId, SalsaTyKind};

use crate::{
    middle::{
        context::{CrateId, DefLocation},
        ext::pb::{Extendee, ExtendeeIndex},
        ty::{CodegenTy, TyKind},
    },
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
    input_files: Arc<Vec<FileId>>,
    nodes: Arc<FxHashMap<DefId, rir::Node>>,
    files: Arc<FxHashMap<FileId, Arc<rir::File>>>,
    file_ids_map: Arc<FxHashMap<Arc<PathBuf>, FileId>>,
    file_paths: Arc<FxHashMap<FileId, Arc<PathBuf>>>,
    file_names: Arc<FxHashMap<FileId, FastStr>>,
    type_graph: Arc<TypeGraph>,
    args: Arc<FxHashSet<DefId>>,
    tags_map: Arc<FxHashMap<TagId, Arc<Tags>>>,
    workspace_graph: Arc<WorkspaceGraph>,
    pb_ext_indexes: Arc<FxHashMap<ExtendeeIndex, Arc<Extendee>>>,
    pb_exts_used: Arc<FxHashSet<ExtendeeIndex>>,
}

impl Default for RootDatabase {
    fn default() -> Self {
        RootDatabase {
            storage: salsa::Storage::new(None),
            nodes: Arc::new(FxHashMap::default()),
            files: Arc::new(FxHashMap::default()),
            file_ids_map: Arc::new(FxHashMap::default()),
            file_names: Arc::new(FxHashMap::default()),
            type_graph: Arc::new(empty_type_graph()),
            tags_map: Arc::new(FxHashMap::default()),
            input_files: Arc::new(Vec::new()),
            args: Arc::new(FxHashSet::default()),
            workspace_graph: Arc::new(empty_workspace_graph()),
            file_paths: Arc::new(FxHashMap::default()),
            pb_ext_indexes: Arc::new(FxHashMap::default()),
            pb_exts_used: Arc::new(FxHashSet::default()),
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

    pub fn with_file_paths(mut self, file_paths: FxHashMap<FileId, Arc<PathBuf>>) -> Self {
        self.file_paths = Arc::new(file_paths);
        self
    }

    pub fn with_file_names(mut self, file_names: FxHashMap<FileId, FastStr>) -> Self {
        self.file_names = Arc::new(file_names);
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

    pub fn with_pb_ext_indexes(
        mut self,
        pb_ext_indexes: FxHashMap<ExtendeeIndex, Arc<Extendee>>,
    ) -> Self {
        self.pb_ext_indexes = Arc::new(pb_ext_indexes);
        self
    }

    pub fn with_pb_exts_used(mut self, pb_exts_used: FxHashSet<ExtendeeIndex>) -> Self {
        self.pb_exts_used = Arc::new(pb_exts_used);
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
    fn file_paths(&self) -> &Arc<FxHashMap<FileId, Arc<PathBuf>>>;
    fn type_graph(&self) -> &Arc<TypeGraph>;
    fn tags_map(&self) -> &Arc<FxHashMap<TagId, Arc<Tags>>>;
    fn input_files(&self) -> &Arc<Vec<FileId>>;
    fn args(&self) -> &Arc<FxHashSet<DefId>>;
    fn workspace_graph(&self) -> &Arc<WorkspaceGraph>;
    fn pb_ext_indexes(&self) -> &Arc<FxHashMap<ExtendeeIndex, Arc<Extendee>>>;
    fn pb_exts_used(&self) -> &Arc<FxHashSet<ExtendeeIndex>>;

    // 查询方法
    fn node(&self, def_id: DefId) -> Option<Node>;

    fn file(&self, file_id: FileId) -> Option<Arc<File>>;

    fn file_id(&self, path: PathBuf) -> Option<FileId> {
        self.file_ids_map().get(&path).cloned()
    }

    fn file_name(&self, file_id: FileId) -> Option<FastStr>;

    fn item(&self, def_id: DefId) -> Option<Arc<Item>>;

    fn expect_item(&self, def_id: DefId) -> Arc<Item> {
        self.item(def_id).unwrap()
    }

    fn codegen_item_ty(&self, ty: TyKind) -> CodegenTy;

    fn codegen_const_ty(&self, ty: TyKind) -> CodegenTy;

    fn codegen_ty(&self, def_id: DefId) -> CodegenTy;

    fn service_methods(&self, def_id: DefId) -> Arc<[Arc<rir::Method>]>;

    fn is_arg(&self, def_id: DefId) -> bool;

    fn pb_ext(&self, index: &ExtendeeIndex) -> Option<Arc<Extendee>>;

    fn pb_ext_used(&self, index: &ExtendeeIndex) -> bool;
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

    fn file_paths(&self) -> &Arc<FxHashMap<FileId, Arc<PathBuf>>> {
        &self.file_paths
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

    fn pb_ext_indexes(&self) -> &Arc<FxHashMap<ExtendeeIndex, Arc<Extendee>>> {
        &self.pb_ext_indexes
    }

    fn pb_exts_used(&self) -> &Arc<FxHashSet<ExtendeeIndex>> {
        &self.pb_exts_used
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

    fn file_name(&self, file_id: FileId) -> Option<FastStr> {
        self.file_names.get(&file_id).cloned()
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

    fn pb_ext(&self, index: &ExtendeeIndex) -> Option<Arc<Extendee>> {
        self.pb_ext_indexes().get(index).cloned()
    }

    fn pb_ext_used(&self, index: &ExtendeeIndex) -> bool {
        self.pb_exts_used().contains(index)
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use pilota::Bytes;
    use rustc_hash::FxHashMap;

    use crate::{
        middle::{
            context::{CrateId, DefLocation},
            ext::{FileExts, ItemExts},
            rir::{self, FieldKind},
            ty::{Ty, TyKind},
        },
        symbol::{DefId, FileId, Ident, Symbol},
        tags::TagId,
    };

    fn make_item_path(parts: &[&str]) -> rir::ItemPath {
        let symbols: Vec<Symbol> = parts
            .iter()
            .map(|p| Symbol::from(FastStr::new((*p).to_string())))
            .collect();
        let boxed: Box<[Symbol]> = symbols.into_boxed_slice();
        rir::ItemPath::from(boxed)
    }

    fn make_message_item(name: &str, fields: Vec<Arc<rir::Field>>) -> Arc<rir::Item> {
        Arc::new(rir::Item::Message(rir::Message {
            name: Ident::from(FastStr::new(name.to_string())),
            fields,
            is_wrapper: false,
            item_exts: ItemExts::Thrift,
        }))
    }

    fn make_node(file_id: FileId, item: Arc<rir::Item>, related_nodes: Vec<DefId>) -> rir::Node {
        rir::Node {
            file_id,
            kind: rir::NodeKind::Item(item),
            parent: None,
            tags: TagId::from_u32(0),
            related_nodes,
        }
    }

    fn make_file(file_id: FileId, package: rir::ItemPath, items: Vec<DefId>) -> Arc<rir::File> {
        Arc::new(rir::File {
            package,
            items,
            file_id,
            uses: vec![],
            descriptor: Bytes::new(),
            extensions: FileExts::Thrift,
        })
    }

    #[test]
    fn collect_def_ids_uses_provided_locations() {
        let root = DefId::from_u32(1);
        let child = DefId::from_u32(2);
        let file_id = FileId::from_u32(10);

        let root_item = make_message_item("Root", Vec::new());
        let child_item = make_message_item("Child", Vec::new());

        let mut nodes = FxHashMap::default();
        nodes.insert(root, make_node(file_id, root_item.clone(), vec![child]));
        nodes.insert(child, make_node(file_id, child_item.clone(), vec![]));

        let package = make_item_path(&["pkg", "root"]);
        let files = vec![(
            file_id,
            make_file(file_id, package.clone(), vec![root, child]),
        )];

        let workspace_graph = WorkspaceGraph::from_items(
            vec![(root, root_item.clone()), (child, child_item.clone())].into_iter(),
        );

        let db = RootDatabase::default()
            .with_nodes(nodes)
            .with_files(files.into_iter())
            .with_workspace_graph(workspace_graph)
            .with_input_files(vec![file_id]);

        let mut provided = FxHashMap::default();
        provided.insert(
            root,
            DefLocation::Fixed(CrateId { main_file: file_id }, package.clone()),
        );
        provided.insert(child, DefLocation::Dynamic);

        let result = db.collect_def_ids(&[root], Some(&provided));

        assert_eq!(result.get(&root), provided.get(&root));
        assert_eq!(result.get(&child), provided.get(&child));
    }

    #[test]
    fn collect_def_ids_infers_locations_from_workspace() {
        let file_main = FileId::from_u32(20);
        let file_other = FileId::from_u32(30);

        let def_main = DefId::from_u32(100);
        let def_standalone = DefId::from_u32(200);
        let def_referrer = DefId::from_u32(300);

        let main_item = make_message_item("Main", Vec::new());
        let standalone_item = make_message_item("Standalone", Vec::new());

        let dep_ty = Ty {
            kind: TyKind::Path(rir::Path {
                kind: rir::DefKind::Type,
                did: def_main,
            }),
            tags_id: TagId::from_u32(0),
        };
        let dep_field = Arc::new(rir::Field {
            did: DefId::from_u32(400),
            name: Ident::from(FastStr::new("dep".to_string())),
            id: 1,
            ty: dep_ty,
            kind: FieldKind::Required,
            tags_id: TagId::from_u32(0),
            default: None,
            item_exts: ItemExts::Thrift,
        });
        let referrer_item = make_message_item("Ref", vec![dep_field]);

        let mut nodes = FxHashMap::default();
        nodes.insert(def_main, make_node(file_main, main_item.clone(), vec![]));
        nodes.insert(
            def_standalone,
            make_node(file_main, standalone_item.clone(), vec![]),
        );
        nodes.insert(
            def_referrer,
            make_node(file_other, referrer_item.clone(), vec![]),
        );

        let main_package = make_item_path(&["pkg", "main"]);
        let other_package = make_item_path(&["pkg", "other"]);
        let files = vec![
            (
                file_main,
                make_file(
                    file_main,
                    main_package.clone(),
                    vec![def_main, def_standalone],
                ),
            ),
            (
                file_other,
                make_file(file_other, other_package, vec![def_referrer]),
            ),
        ];

        let workspace_graph = WorkspaceGraph::from_items(
            vec![
                (def_main, main_item.clone()),
                (def_standalone, standalone_item.clone()),
                (def_referrer, referrer_item.clone()),
            ]
            .into_iter(),
        );

        let db = RootDatabase::default()
            .with_nodes(nodes)
            .with_files(files.into_iter())
            .with_workspace_graph(workspace_graph)
            .with_input_files(vec![file_main]);

        let result = db.collect_def_ids(&[def_main, def_standalone], None);

        assert_eq!(result.get(&def_main), Some(&DefLocation::Dynamic));

        let expected_fixed = DefLocation::Fixed(
            CrateId {
                main_file: file_main,
            },
            main_package,
        );
        assert_eq!(result.get(&def_standalone), Some(&expected_fixed));
    }
}
