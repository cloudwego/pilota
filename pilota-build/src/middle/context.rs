use std::{collections::HashMap, ops::Deref, path::PathBuf, sync::Arc};

use ahash::{AHashMap, HashSet};
use anyhow::Context as _;
use dashmap::DashMap;
use faststr::FastStr;
use itertools::Itertools;
use normpath::PathExt;
use rustc_hash::{FxHashMap, FxHashSet};

use self::tls::with_cur_item;
use super::{
    adjust::Adjust,
    resolver::{DefaultPathResolver, PathResolver, WorkspacePathResolver},
    rir::NodeKind,
};
use crate::{
    Plugin,
    db::{RirDatabase, RootDatabase},
    rir::{self, Field, Item, ItemPath, Literal},
    symbol::{DefId, FileId, IdentName, ModPath, SPECIAL_NAMINGS, Symbol},
    tags::{TagId, Tags},
    ty::{AdtDef, AdtKind, CodegenTy, Visitor},
};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct CrateId {
    pub(crate) main_file: FileId,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum DefLocation {
    Fixed(CrateId, ItemPath),
    Dynamic,
}

pub enum CollectMode {
    All,
    OnlyUsed {
        touches: Vec<(std::path::PathBuf, Vec<String>)>,
    },
}

#[derive(Debug)]
pub struct WorkspaceInfo {
    pub dir: PathBuf,
    pub(crate) location_map: FxHashMap<DefId, DefLocation>,
}

#[derive(Debug)]
pub enum Mode {
    Workspace(WorkspaceInfo),
    SingleFile { file_path: std::path::PathBuf },
}

pub struct Context {
    pub db: RootDatabase,
    pub source: Source,
    pub config: Config,
    pub cache: Cache,
}

#[derive(Clone)]
pub struct Source {
    pub source_type: SourceType,
    pub services: Arc<[crate::IdlService]>,
    pub mode: Arc<Mode>,
    pub path_resolver: Arc<dyn PathResolver>,
}

#[derive(Clone)]
pub struct Config {
    pub change_case: bool,
    pub split: bool,
    pub with_descriptor: bool,
    pub with_field_mask: bool,
    pub touch_all: bool,
    pub common_crate_name: FastStr,
}

#[derive(Clone)]
pub struct Cache {
    pub adjusts: Arc<DashMap<DefId, Adjust>>,
    pub mod_idxes: AHashMap<ModPath, DefId>, // mod kind index
    pub codegen_items: Vec<DefId>,
    pub mod_items: HashMap<ModPath, Vec<DefId>>,
    pub def_mod: HashMap<DefId, ModPath>,
    pub mod_files: HashMap<ModPath, Vec<FileId>>,
    pub keep_unknown_fields: Arc<FxHashSet<DefId>>,
    pub location_map: Arc<FxHashMap<DefId, DefLocation>>,
    pub entry_map: Arc<HashMap<DefLocation, Vec<(DefId, DefLocation)>>>,
    pub plugin_gen: Arc<DashMap<DefLocation, String>>,
    pub dedups: Vec<FastStr>,
    pub names: FxHashMap<DefId, usize>,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            source: self.source.clone(),
            config: self.config.clone(),
            cache: self.cache.clone(),
        }
    }
}

pub(crate) struct ContextBuilder {
    db: RootDatabase,
    pub(crate) codegen_items: Vec<DefId>,
    input_items: Vec<DefId>,
    mode: Mode,
    keep_unknown_fields: FxHashSet<DefId>,
    pub location_map: FxHashMap<DefId, DefLocation>,
    entry_map: HashMap<DefLocation, Vec<(DefId, DefLocation)>>,
}

impl ContextBuilder {
    pub fn new(db: RootDatabase, mode: Mode, input_items: Vec<DefId>) -> Self {
        ContextBuilder {
            db,
            mode,
            input_items,
            codegen_items: Default::default(),
            keep_unknown_fields: Default::default(),
            location_map: Default::default(),
            entry_map: Default::default(),
        }
    }
    pub(crate) fn collect(&mut self, mode: CollectMode) {
        match mode {
            CollectMode::All => {
                let nodes = self.db.nodes();
                self.codegen_items
                    .extend(nodes.iter().filter_map(|(k, v)| match &v.kind {
                        NodeKind::Item(i) => {
                            if !matches!(&**i, Item::Mod(_)) {
                                Some(k)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }));
            }
            CollectMode::OnlyUsed { touches } => {
                let extra_def_ids = touches
                    .into_iter()
                    .flat_map(|s| {
                        let path = s.0.normalize().unwrap().into_path_buf();
                        let file_id = *self.db.file_ids_map().get(&path).unwrap();
                        s.1.into_iter()
                            .filter_map(|item_name| {
                                let def_id = self
                                    .db
                                    .files()
                                    .get(&file_id)
                                    .unwrap()
                                    .items
                                    .iter()
                                    .find(|def_id| {
                                        *self.db.item(**def_id).unwrap().symbol_name() == item_name
                                    })
                                    .cloned();
                                if let Some(def_id) = def_id {
                                    Some(def_id)
                                } else {
                                    println!(
                                        "cargo:warning=item `{item_name}` of `{}` not exists",
                                        path.display(),
                                    );
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                self.input_items.extend(extra_def_ids);

                let def_ids = self.collect_items(&self.input_items);
                self.codegen_items.extend(def_ids.iter());
            }
        }
        if matches!(self.mode, Mode::Workspace(_)) {
            let location_map = self.workspace_collect_def_ids(&self.codegen_items);
            self.location_map = location_map.clone();
            self.entry_map = location_map
                .clone()
                .into_iter()
                .into_group_map_by(|item| item.1.clone());
            if let Mode::Workspace(info) = &mut self.mode {
                info.location_map = location_map
            }
        }
    }

    pub(crate) fn collect_items(&self, input: &[DefId]) -> FxHashSet<DefId> {
        struct PathCollector<'a> {
            set: &'a mut FxHashSet<DefId>,
            cx: &'a ContextBuilder,
            file_ids: &'a mut FxHashSet<FileId>,
        }

        impl super::ty::Visitor for PathCollector<'_> {
            fn visit_path(&mut self, path: &crate::rir::Path) {
                collect(self.cx, path.did, self.set, self.file_ids)
            }
        }

        fn collect(
            cx: &ContextBuilder,
            def_id: DefId,
            set: &mut FxHashSet<DefId>,
            file_ids: &mut FxHashSet<FileId>,
        ) {
            if set.contains(&def_id) {
                return;
            }

            let node = cx.db.node(def_id).unwrap();

            if !file_ids.contains(&node.file_id) {
                file_ids.insert(node.file_id);

                let file = cx.db.file(node.file_id).unwrap();
                if file.extensions.has_used_options() {
                    file.extensions
                        .unwrap_as_pb()
                        .used_options
                        .0
                        .iter()
                        .for_each(|option| {
                            let extendee = cx.db.pb_ext(option).unwrap();
                            PathCollector { cx, set, file_ids }
                                .visit(&extendee.extendee_ty.item_ty);
                        });
                }
            }

            match node.kind {
                NodeKind::Item(_) => {}
                _ => return collect(cx, node.parent.unwrap(), set, file_ids),
            }

            if !matches!(&*cx.db.item(def_id).unwrap(), rir::Item::Mod(_)) {
                set.insert(def_id);
            }

            let node = cx.db.node(def_id).unwrap();
            tracing::trace!("collecting {:?}", node.expect_item().symbol_name());

            node.related_nodes
                .iter()
                .for_each(|def_id| collect(cx, *def_id, set, file_ids));

            let item = node.expect_item();

            match item {
                rir::Item::Message(m) => {
                    // collect fields
                    m.fields.iter().for_each(|f| {
                        PathCollector { cx, set, file_ids }.visit(&f.ty);
                        if let Some(Literal::Path(p)) = &f.default {
                            PathCollector { cx, set, file_ids }.visit_path(p);
                        }
                        // collect extensions
                        if f.item_exts.has_used_options() {
                            f.item_exts
                                .unwrap_as_pb()
                                .used_options
                                .0
                                .iter()
                                .for_each(|index| {
                                    let extendee = cx.db.pb_ext(index).unwrap();
                                    PathCollector { cx, set, file_ids }
                                        .visit(&extendee.extendee_ty.item_ty);
                                });
                        }
                    });
                    // collect extensions
                    if m.item_exts.has_used_options() {
                        m.item_exts
                            .unwrap_as_pb()
                            .used_options
                            .0
                            .iter()
                            .for_each(|index| {
                                let extendee = cx.db.pb_ext(index).unwrap();
                                PathCollector { cx, set, file_ids }
                                    .visit(&extendee.extendee_ty.item_ty);
                            });
                    }
                }
                rir::Item::Enum(e) => {
                    e.variants.iter().for_each(|v| {
                        for ty in &v.fields {
                            PathCollector { cx, set, file_ids }.visit(ty);
                        }
                        if v.item_exts.has_used_options() {
                            v.item_exts
                                .unwrap_as_pb()
                                .used_options
                                .0
                                .iter()
                                .for_each(|index| {
                                    let extendee = cx.db.pb_ext(index).unwrap();
                                    PathCollector { cx, set, file_ids }
                                        .visit(&extendee.extendee_ty.item_ty);
                                });
                        }
                    });
                    if e.item_exts.has_used_options() {
                        e.item_exts
                            .unwrap_as_pb()
                            .used_options
                            .0
                            .iter()
                            .for_each(|index| {
                                let extendee = cx.db.pb_ext(index).unwrap();
                                PathCollector { cx, set, file_ids }
                                    .visit(&extendee.extendee_ty.item_ty);
                            });
                    }
                }
                rir::Item::Service(s) => {
                    s.extend
                        .iter()
                        .for_each(|p| collect(cx, p.did, set, file_ids));
                    s.methods.iter().for_each(|m| {
                        // collect args
                        m.args
                            .iter()
                            .for_each(|f| PathCollector { cx, set, file_ids }.visit(&f.ty));
                        // collect ret
                        PathCollector { cx, set, file_ids }.visit(&m.ret);
                        // collect exceptions
                        if let Some(exceptions) = &m.exceptions {
                            PathCollector { cx, set, file_ids }.visit_path(exceptions);
                        }
                        // collect extensions
                        if m.item_exts.has_used_options() {
                            m.item_exts
                                .unwrap_as_pb()
                                .used_options
                                .0
                                .iter()
                                .for_each(|index| {
                                    let extendee = cx.db.pb_ext(index).unwrap();
                                    PathCollector { cx, set, file_ids }
                                        .visit(&extendee.extendee_ty.item_ty);
                                });
                        }
                    });

                    // collect extensions
                    if s.item_exts.has_used_options() {
                        s.item_exts
                            .unwrap_as_pb()
                            .used_options
                            .0
                            .iter()
                            .for_each(|index| {
                                let extendee = cx.db.pb_ext(index).unwrap();
                                PathCollector { cx, set, file_ids }
                                    .visit(&extendee.extendee_ty.item_ty);
                            });
                    }
                }
                rir::Item::NewType(n) => PathCollector { cx, set, file_ids }.visit(&n.ty),
                rir::Item::Const(c) => {
                    PathCollector { cx, set, file_ids }.visit(&c.ty);
                }
                rir::Item::Mod(m) => {
                    m.items.iter().for_each(|i| collect(cx, *i, set, file_ids));
                }
            }
        }
        let mut set = FxHashSet::default();

        let mut file_ids = FxHashSet::default();

        input.iter().for_each(|def_id| {
            collect(self, *def_id, &mut set, &mut file_ids);
        });

        self.db.nodes().iter().for_each(|(def_id, node)| {
            if let NodeKind::Item(item) = &node.kind {
                if let rir::Item::Const(_) = &**item {
                    collect(self, *def_id, &mut set, &mut file_ids);
                }
            }
        });

        set
    }

    pub(crate) fn workspace_collect_def_ids(
        &self,
        input: &[DefId],
    ) -> FxHashMap<DefId, DefLocation> {
        self.db.collect_def_ids(input, None)
    }

    pub(crate) fn keep(&mut self, keep_unknown_fields: Vec<PathBuf>) {
        let mut file_ids = FxHashSet::default();
        keep_unknown_fields.into_iter().for_each(|p| {
            let path = p.normalize().unwrap().into_path_buf();
            let file_id = {
                let file_ids_map = self.db.file_ids_map();
                *file_ids_map.get(&path).unwrap()
            };
            keep_files(self, &file_id, &mut file_ids);

            fn keep_files(
                cx: &mut ContextBuilder,
                file_id: &FileId,
                file_ids: &mut FxHashSet<FileId>,
            ) {
                if !file_ids.insert(*file_id) {
                    return;
                }
                let (uses, items_to_keep) = {
                    let files = cx.db.files();
                    let file = files.get(file_id).unwrap();
                    let uses = file.uses.clone();
                    let items_to_keep = file
                        .items
                        .iter()
                        .filter(|&&def_id| match cx.db.node(def_id) {
                            Some(rir::Node {
                                kind: rir::NodeKind::Item(_),
                                tags,
                                ..
                            }) => !matches!(
                                cx.db.tags_map().get(&tags).and_then(|tags| {
                                    tags.get::<crate::tags::KeepUnknownFields>()
                                }),
                                Some(crate::tags::KeepUnknownFields(false))
                            ),
                            _ => true,
                        })
                        .cloned()
                        .collect::<Vec<_>>();
                    (uses, items_to_keep)
                };

                for f in &uses {
                    keep_files(cx, f, file_ids);
                }

                cx.keep_unknown_fields.extend(items_to_keep);
            }
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn build(
        self,
        services: Arc<[crate::IdlService]>,
        source_type: SourceType,
        change_case: bool,
        dedups: Vec<FastStr>,
        special_namings: Vec<FastStr>,
        common_crate_name: FastStr,
        split: bool,
        with_descriptor: bool,
        with_field_mask: bool,
        touch_all: bool,
    ) -> Context {
        let mode = Arc::new(self.mode);
        SPECIAL_NAMINGS.get_or_init(|| special_namings);
        let mut cx = Context {
            db: self.db.clone(),
            source: Source {
                source_type,
                services,
                mode: mode.clone(),
                path_resolver: match &*mode {
                    Mode::Workspace(_) => Arc::new(WorkspacePathResolver),
                    Mode::SingleFile { .. } => Arc::new(DefaultPathResolver),
                },
            },
            config: Config {
                change_case,
                split,
                with_descriptor,
                with_field_mask,
                touch_all,
                common_crate_name,
            },
            cache: Cache {
                adjusts: Default::default(),
                codegen_items: self.codegen_items,
                keep_unknown_fields: Arc::new(self.keep_unknown_fields),
                location_map: Arc::new(self.location_map),
                entry_map: Arc::new(self.entry_map),
                plugin_gen: Default::default(),
                dedups,
                names: Default::default(),
                mod_idxes: Default::default(),
                mod_items: Default::default(),
                mod_files: Default::default(),
                def_mod: Default::default(),
            },
        };
        let mut map: FxHashMap<(Vec<DefId>, String), Vec<DefId>> = FxHashMap::default();
        let mut mod_idxes = AHashMap::default();
        cx.nodes()
            .iter()
            .for_each(|(def_id, node)| match &node.kind {
                NodeKind::Item(item) => {
                    if let crate::rir::Item::Mod(_) = &**item {
                        mod_idxes.insert(
                            ModPath::from(Arc::from_iter(
                                cx.mod_path(*def_id).iter().map(|s| s.0.clone()),
                            )),
                            *def_id,
                        );
                        return;
                    }
                    if let Mode::Workspace(_) = &*cx.source.mode {
                        if !cx.cache.location_map.contains_key(def_id) {
                            return;
                        }
                    }
                    let rust_name = cx.item_path(*def_id).join("::");
                    map.entry((vec![], rust_name)).or_default().push(*def_id);
                }
                _ => {
                    let mut item_def_ids = vec![];
                    let mut item_def_id = *def_id;
                    while !matches!(cx.node(item_def_id).unwrap().kind, NodeKind::Item(_)) {
                        item_def_id = cx.node(item_def_id).unwrap().parent.unwrap();
                        item_def_ids.push(item_def_id);
                    }
                    let rust_name = cx.rust_name(*def_id).to_string();
                    map.entry((item_def_ids, rust_name))
                        .or_default()
                        .push(*def_id);
                }
            });
        cx.cache.names.extend(
            map.into_iter()
                .filter(|(_, v)| v.len() > 1)
                .map(|(_, v)| v)
                .flat_map(|v| v.into_iter().enumerate().map(|(i, def_id)| (def_id, i)))
                .collect::<HashMap<DefId, usize>>(),
        );
        cx.cache.mod_idxes.extend(mod_idxes);

        let mut mod_files = HashMap::<ModPath, HashSet<FileId>>::default();
        let mod_items = cx
            .cache
            .codegen_items
            .clone()
            .into_iter()
            .into_group_map_by(|def_id| {
                let file_id = cx.node(*def_id).unwrap().file_id;

                let path = Arc::from_iter(cx.mod_path(*def_id).iter().map(|s| s.0.clone()));
                let mod_path = match &*cx.source.mode {
                    Mode::SingleFile { .. } => path,
                    Mode::Workspace(_) => Arc::from(&path[1..]),
                };
                let mod_path = ModPath::from(mod_path);
                let set = mod_files.entry(mod_path.clone()).or_default();
                if !set.contains(&file_id) {
                    set.insert(file_id);
                }
                cx.cache.def_mod.insert(*def_id, mod_path.clone());
                mod_path
            });
        cx.cache.mod_items.extend(mod_items);
        cx.cache.mod_files.extend(
            mod_files
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().collect())),
        );
        cx
    }
}

impl Deref for Context {
    type Target = RootDatabase;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

#[derive(Clone, Copy)]
pub enum SourceType {
    Thrift,
    Protobuf,
}

impl Context {
    pub fn config_data(&self) -> &Config {
        &self.config
    }

    pub fn cache_data(&self) -> &Cache {
        &self.cache
    }

    pub fn source_data(&self) -> &Source {
        &self.source
    }

    pub fn with_adjust<T, F>(&self, def_id: DefId, f: F) -> T
    where
        F: FnOnce(Option<&Adjust>) -> T,
    {
        match self.cache.adjusts.get(&def_id) {
            Some(adj) => f(Some(&*adj)),
            None => f(None),
        }
    }

    pub fn with_adjust_mut<T, F>(&self, def_id: DefId, f: F) -> T
    where
        F: FnOnce(&mut Adjust) -> T,
    {
        let adjust = &mut *self.cache.adjusts.entry(def_id).or_default();
        f(adjust)
    }

    pub fn tags(&self, tags_id: TagId) -> Option<Arc<Tags>> {
        self.db.tags_map().get(&tags_id).cloned()
    }

    pub fn node_tags(&self, def_id: DefId) -> Option<Arc<Tags>> {
        let tags_id = self.node(def_id).unwrap().tags;
        self.tags(tags_id)
    }

    pub fn contains_tag<T: 'static>(&self, tags_id: TagId) -> bool {
        self.tags(tags_id)
            .and_then(|tags| tags.contains::<T>().then_some(true))
            .is_some()
    }

    pub fn node_contains_tag<T: 'static>(&self, def_id: DefId) -> bool {
        self.node_tags(def_id)
            .and_then(|tags| tags.contains::<T>().then_some(true))
            .is_some()
    }

    pub fn symbol_name(&self, def_id: DefId) -> Symbol {
        let item = self.item(def_id).unwrap();
        item.symbol_name()
    }

    fn get_codegen_ty_for_path(&self, def_id: DefId) -> CodegenTy {
        let node = self.node(def_id).unwrap();
        match &node.kind {
            NodeKind::Item(item) => match &**item {
                Item::Const(c) => self.codegen_const_ty(c.ty.kind.clone()),
                Item::Enum(_) => CodegenTy::Adt(AdtDef {
                    did: def_id,
                    kind: AdtKind::Enum,
                }),
                Item::NewType(t) => CodegenTy::Adt(AdtDef {
                    did: def_id,
                    kind: AdtKind::NewType(Arc::new(self.codegen_item_ty(t.ty.kind.clone()))),
                }),
                Item::Message(_) => CodegenTy::Adt(AdtDef {
                    did: def_id,
                    kind: AdtKind::Struct,
                }),
                _ => panic!("Unexpected item type for path: {:?}", item),
            },
            NodeKind::Variant(_v) => {
                // For enum variants, get the parent enum's type
                let parent_def_id = node.parent.unwrap();
                CodegenTy::Adt(AdtDef {
                    did: parent_def_id,
                    kind: AdtKind::Enum,
                })
            }
            NodeKind::Field(_) | NodeKind::Method(_) | NodeKind::Arg(_) => {
                panic!("Unexpected node kind for path: {:?}", node.kind)
            }
        }
    }

    pub fn default_val(&self, f: &Field) -> Option<(FastStr, bool /* const? */)> {
        f.default.as_ref().map(|d| {
            let ty = self.codegen_item_ty(f.ty.kind.clone());
            match self
                .lit_as_rvalue(d, &ty)
                .with_context(|| format!("calc the default value for field {}", f.name))
            {
                Ok(v) => v,
                Err(err) => {
                    panic!("{err}")
                }
            }
        })
    }

    fn lit_as_rvalue(
        &self,
        lit: &Literal,
        ty: &CodegenTy,
    ) -> anyhow::Result<(FastStr, bool /* const? */)> {
        let mk_map = |m: &Vec<(Literal, Literal)>,
                      k_ty: &Arc<CodegenTy>,
                      v_ty: &Arc<CodegenTy>,
                      btree: bool| {
            let k_ty = &**k_ty;
            let v_ty = &**v_ty;
            let len = m.len();
            let kvs = m
                .iter()
                .map(|(k, v)| {
                    let k = self.lit_into_ty(k, k_ty)?.0;
                    let v = self.lit_into_ty(v, v_ty)?.0;
                    anyhow::Ok(format!("map.insert({k}, {v});"))
                })
                .try_collect::<_, Vec<_>, _>()?
                .join("");
            let new = if btree {
                "::std::collections::BTreeMap::new()".to_string()
            } else {
                format!("::pilota::AHashMap::with_capacity({len})")
            };
            anyhow::Ok(
                format! {r#"{{
                    let mut map = {new};
                    {kvs}
                    map
                }}"#}
                .into(),
            )
        };

        anyhow::Ok(match (lit, ty) {
            (Literal::Map(m), CodegenTy::LazyStaticRef(map)) => match &**map {
                CodegenTy::Map(k_ty, v_ty) => (mk_map(m, k_ty, v_ty, false)?, false),
                CodegenTy::BTreeMap(k_ty, v_ty) => (mk_map(m, k_ty, v_ty, true)?, false),
                _ => panic!("invalid map type {map:?}"),
            },
            (Literal::Map(m), CodegenTy::Map(k_ty, v_ty)) => (mk_map(m, k_ty, v_ty, false)?, false),
            (Literal::Map(m), CodegenTy::BTreeMap(k_ty, v_ty)) => {
                (mk_map(m, k_ty, v_ty, true)?, false)
            }
            (Literal::List(l), CodegenTy::LazyStaticRef(map)) => {
                assert!(l.is_empty());
                match &**map {
                    CodegenTy::Map(_, _) => ("::pilota::AHashMap::new()".into(), false),
                    CodegenTy::BTreeMap(_, _) => {
                        ("::std::collections::BTreeMap::new()".into(), false)
                    }
                    _ => panic!("invalid map type {map:?}"),
                }
            }
            (Literal::List(l), CodegenTy::Map(_, _)) => {
                assert!(l.is_empty());
                ("::pilota::AHashMap::new()".into(), false)
            }
            (Literal::List(l), CodegenTy::BTreeMap(_, _)) => {
                assert!(l.is_empty());
                ("::std::collections::BTreeMap::new()".into(), false)
            }
            _ => self.lit_into_ty(lit, ty)?,
        })
    }

    fn ident_into_ty(
        &self,
        did: DefId,
        ident_ty: &CodegenTy,
        target: &CodegenTy,
    ) -> (FastStr, bool /* const? */) {
        if ident_ty == target {
            let stream = self.cur_related_item_path(did);
            return (stream, true);
        }
        match (ident_ty, target) {
            (CodegenTy::Str, CodegenTy::FastStr) => {
                let stream = self.cur_related_item_path(did);
                (
                    format!("::pilota::FastStr::from_static_str({stream})").into(),
                    true,
                )
            }
            (
                CodegenTy::Adt(AdtDef {
                    did: _,
                    kind: AdtKind::Enum,
                }),
                CodegenTy::I64,
            )
            | (
                CodegenTy::Adt(AdtDef {
                    did: _,
                    kind: AdtKind::Enum,
                }),
                CodegenTy::I32,
            )
            | (
                CodegenTy::Adt(AdtDef {
                    did: _,
                    kind: AdtKind::Enum,
                }),
                CodegenTy::I16,
            )
            | (
                CodegenTy::Adt(AdtDef {
                    did: _,
                    kind: AdtKind::Enum,
                }),
                CodegenTy::I8,
            ) => {
                let stream = self.cur_related_item_path(did);
                let target = match target {
                    CodegenTy::I64 => "i64",
                    CodegenTy::I32 => "i32",
                    CodegenTy::I16 => "i16",
                    CodegenTy::I8 => "i8",
                    _ => unreachable!(),
                };
                (format!("({stream}.inner() as {target})").into(), true)
            }
            _ => panic!("invalid convert {ident_ty:?} to {target:?}"),
        }
    }

    fn lit_into_ty(
        &self,
        lit: &Literal,
        ty: &CodegenTy,
    ) -> anyhow::Result<(FastStr, bool /* const? */)> {
        Ok(match (lit, ty) {
            (Literal::Path(p), ty) => {
                let ident_ty = self.get_codegen_ty_for_path(p.did);

                self.ident_into_ty(p.did, &ident_ty, ty)
            }
            (Literal::String(s), CodegenTy::Str) => (format!("\"{s}\"").into(), true),
            (Literal::String(s), CodegenTy::String) => {
                (format! {"\"{s}\".to_string()"}.into(), false)
            }
            (Literal::String(s), CodegenTy::FastStr) => (
                format! { "::pilota::FastStr::from_static_str(\"{s}\")" }.into(),
                true,
            ),
            (Literal::Int(i), CodegenTy::I8) => (format! { "{i}i8" }.into(), true),
            (Literal::Int(i), CodegenTy::I16) => (format! { "{i}i16" }.into(), true),
            (Literal::Int(i), CodegenTy::I32) => (format! { "{i}i32" }.into(), true),
            (Literal::Int(i), CodegenTy::I64) => (format! { "{i}i64" }.into(), true),
            (Literal::Int(i), CodegenTy::F32) => {
                let f = (*i) as f32;
                (format!("{f}f32").into(), true)
            }
            (Literal::Int(i), CodegenTy::F64) => {
                let f = (*i) as f64;
                (format!("{f}f64").into(), true)
            }
            (
                Literal::Int(i),
                CodegenTy::Adt(AdtDef {
                    did,
                    kind: AdtKind::Enum,
                }),
            ) => {
                let item = self.item(*did).unwrap();
                let e = match &*item {
                    Item::Enum(e) => e,
                    _ => panic!("invalid enum"),
                };

                (
                    e.variants.iter().find(|v| v.discr == Some(*i)).map_or_else(
                        || panic!("invalid enum value"),
                        |v| self.cur_related_item_path(v.did),
                    ),
                    true,
                )
            }
            (Literal::Float(f), CodegenTy::F64) => {
                let f = f.parse::<f64>().unwrap();
                (format! { "{f}f64" }.into(), true)
            }
            (Literal::Float(f), CodegenTy::OrderedF64) => {
                let f = f.parse::<f64>().unwrap();
                (format! { "::pilota::OrderedFloat({f}f64)" }.into(), true)
            }
            (
                l,
                CodegenTy::Adt(AdtDef {
                    kind: AdtKind::NewType(inner_ty),
                    did,
                }),
            ) => {
                let ident = self.cur_related_item_path(*did);
                let (stream, is_const) = self.lit_into_ty(l, inner_ty)?;
                (format! { "{ident}({stream})" }.into(), is_const)
            }
            (Literal::Map(_), CodegenTy::StaticRef(map)) => match &**map {
                CodegenTy::Map(_, _) | CodegenTy::BTreeMap(_, _) => {
                    let lazy_map =
                        self.def_lit("INNER_MAP", lit, &mut CodegenTy::LazyStaticRef(map.clone()))?;
                    let stream = format! {
                        r#"{{
                            {lazy_map}
                            &*INNER_MAP
                        }}"#
                    }
                    .into();
                    (stream, false)
                }
                _ => panic!("invalid map type {map:?}"),
            },
            (Literal::List(els), CodegenTy::Array(inner, _)) => {
                let stream = els
                    .iter()
                    .map(|el| self.lit_into_ty(el, inner))
                    .try_collect::<_, Vec<_>, _>()?;
                let is_const = stream.iter().all(|(_, is_const)| *is_const);
                let stream = stream.into_iter().map(|(s, _)| s).join(",");

                (format! {"[{stream}]" }.into(), is_const)
            }
            (Literal::List(els), CodegenTy::Vec(inner)) => {
                let stream = self.list_stream(els, inner)?;
                (format! { "::std::vec![{stream}]" }.into(), false)
            }
            (Literal::List(els), CodegenTy::Set(inner)) => {
                let stream = self.list_stream(els, inner)?;
                (
                    format! { "::pilota::AHashSet::from([{stream}])" }.into(),
                    false,
                )
            }
            (Literal::List(els), CodegenTy::BTreeSet(inner)) => {
                let stream = self.list_stream(els, inner)?;
                (
                    format! { "::std::collections::BTreeSet::from([{stream}])" }.into(),
                    false,
                )
            }
            (Literal::Bool(b), CodegenTy::Bool) => (format! { "{b}" }.into(), true),
            (Literal::Int(i), CodegenTy::Bool) => {
                let b = *i != 0;
                (format! { "{b}" }.into(), true)
            }
            (Literal::String(s), CodegenTy::Bytes) => {
                let s = &**s;
                (
                    format! { "::pilota::Bytes::from_static(\"{s}\".as_bytes())" }.into(),
                    true,
                )
            }
            (
                Literal::Map(m),
                CodegenTy::Adt(AdtDef {
                    did,
                    kind: AdtKind::Struct,
                }),
            ) => {
                let def = self.item(*did).unwrap();
                let def = match &*def {
                    Item::Message(m) => m,
                    _ => panic!(),
                };

                let fields: Vec<_> = def
                    .fields
                    .iter()
                    .map(|f| {
                        let v = m.iter().find_map(|(k, v)| {
                            let k = match k {
                                Literal::String(s) => s,
                                _ => panic!(),
                            };
                            if **k == **f.name { Some(v) } else { None }
                        });

                        let name = self.rust_name(f.did);

                        if let Some(v) = v {
                            let (mut v, is_const) =
                                self.lit_into_ty(v, &self.codegen_item_ty(f.ty.kind.clone()))?;

                            if f.is_optional() {
                                v = format!("Some({v})").into()
                            }
                            anyhow::Ok((format!("{name}: {v}"), is_const))
                        } else if f.is_optional() {
                            anyhow::Ok((format!("{name}: None"), true))
                        } else {
                            anyhow::Ok((format!("{name}: Default::default()"), false))
                        }
                    })
                    .try_collect()?;
                let is_const = fields.iter().all(|(_, is_const)| *is_const);
                let fields = fields.into_iter().map(|f| f.0).join(",");

                let name = self.cur_related_item_path(*did);

                (
                    format! {
                        r#"{name} {{
                            {fields}
                        }}"#
                    }
                    .into(),
                    is_const,
                )
            }
            _ => panic!("unexpected literal {lit:?} with ty {ty:?}"),
        })
    }

    #[inline]
    fn list_stream(&self, els: &[Literal], inner: &Arc<CodegenTy>) -> anyhow::Result<String> {
        Ok(els
            .iter()
            .map(|el| self.lit_into_ty(el, inner))
            .try_collect::<_, Vec<_>, _>()?
            .into_iter()
            .map(|(s, _)| s)
            .join(","))
    }

    pub(crate) fn def_lit(
        &self,
        name: &str,
        lit: &Literal,
        ty: &mut CodegenTy,
    ) -> anyhow::Result<String> {
        let should_lazy_static = ty.should_lazy_static();
        if let (Literal::List(lit), CodegenTy::Array(_, size)) = (lit, &mut *ty) {
            *size = lit.len()
        }
        Ok(if should_lazy_static {
            let lit = self.lit_as_rvalue(lit, ty)?.0;
            format! {r#"
                pub static {name}: ::std::sync::LazyLock<{ty}> = ::std::sync::LazyLock::new(|| {{
                    {lit}
                }});
            "#}
        } else {
            let (lit, is_const) = self.lit_into_ty(lit, ty)?;
            if is_const {
                format!(r#"pub const {name}: {ty} = {lit};"#)
            } else {
                format! {r#"
                pub static {name}: ::std::sync::LazyLock<{ty}> = ::std::sync::LazyLock::new(|| {{
                    {lit}
                }});
            "#}
            }
        })
    }

    pub fn rust_name(&self, def_id: DefId) -> Symbol {
        let node = self.node(def_id).unwrap();

        if let Some(name) = self
            .tags(node.tags)
            .and_then(|tags| tags.get::<crate::tags::PilotaName>().cloned())
        {
            return name.0.into();
        }

        if !self.config.change_case || self.cache.names.contains_key(&def_id) {
            return node.name();
        }

        match self.node(def_id).unwrap().kind {
            NodeKind::Item(item) => match &*item {
                crate::rir::Item::Message(m) => (&**m.name).struct_ident(),
                crate::rir::Item::Enum(e) => (&**e.name).enum_ident(),
                crate::rir::Item::Service(s) => (&**s.name).trait_ident(),
                crate::rir::Item::NewType(t) => (&**t.name).newtype_ident(),
                crate::rir::Item::Const(c) => (&**c.name).const_ident(),
                crate::rir::Item::Mod(m) => (&**m.name).mod_ident(),
            },
            NodeKind::Variant(v) => {
                let parent = self.node(def_id).unwrap().parent.unwrap();
                let item = self.expect_item(parent);
                match &*item {
                    rir::Item::Enum(e) => {
                        if e.repr.is_some() {
                            (&**v.name).const_ident()
                        } else {
                            (&**v.name).variant_ident()
                        }
                    }
                    _ => unreachable!(),
                }
            }
            NodeKind::Field(f) => (&**f.name).field_ident(),
            NodeKind::Method(m) => (&**m.name).fn_ident(),
            NodeKind::Arg(a) => (&**a.name).field_ident(),
        }
        .into()
    }

    pub fn mod_path(&self, def_id: DefId) -> Arc<[Symbol]> {
        self.source.path_resolver.mod_prefix(self, def_id)
    }

    pub fn item_path(&self, def_id: DefId) -> Arc<[Symbol]> {
        self.source.path_resolver.path_for_def_id(self, def_id)
    }

    fn related_path(&self, p1: &[Symbol], p2: &[Symbol]) -> FastStr {
        self.source.path_resolver.related_path(p1, p2)
    }

    pub fn cur_related_item_path(&self, did: DefId) -> FastStr {
        let a = with_cur_item(|def_id| def_id);
        self.related_item_path(a, did)
    }

    pub fn related_item_path(&self, a: DefId, b: DefId) -> FastStr {
        let cur_item_path = self.item_path(a);
        let mut mod_segs = vec![];

        cur_item_path[..cur_item_path.len() - 1]
            .iter()
            .for_each(|p| {
                mod_segs.push(p.clone());
            });

        let other_item_path = self.item_path(b);
        self.related_path(&mod_segs, &other_item_path)
    }

    #[allow(clippy::single_match)]
    pub fn exec_plugin<P: Plugin>(&self, mut p: P) {
        p.on_codegen_uint(self, &self.cache.codegen_items);

        p.on_emit(self)
    }

    pub(crate) fn workspace_info(&self) -> &WorkspaceInfo {
        let Mode::Workspace(info) = &*self.source.mode else {
            panic!(
                "can not access workspace info in mode `{:?}`",
                self.source.mode
            )
        };
        info
    }

    // pub fn def_id_info(&self, def_id: DefId) -> FastStr {
    //     let file_path = self
    //         .file(self.node(def_id).unwrap().file_id)
    //         .unwrap()
    //         .package
    //         .clone();
    //     file_path
    //         .iter()
    //         .chain(&[self.node(def_id).unwrap().name()])
    //         .join("::")
    //         .into()
    // }

    pub fn config(&self, crate_id: &CrateId) -> &serde_yaml::Value {
        &self.find_service(crate_id.main_file).config
    }

    pub(crate) fn crate_name(&self, location: &DefLocation) -> FastStr {
        match location {
            DefLocation::Fixed(crate_id, _) => {
                let main_file = crate_id.main_file;
                let service = self.find_service(main_file);
                self.config(crate_id)
                    .get("crate_name")
                    .and_then(|s| s.as_str().map(FastStr::new))
                    .unwrap_or_else(|| {
                        service
                            .path
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .replace('.', "_")
                            .into()
                    })
            }
            DefLocation::Dynamic => self.config.common_crate_name.clone(),
        }
    }

    fn find_service(&self, file_id: FileId) -> &crate::IdlService {
        self.source
            .services
            .iter()
            .find(|s| {
                let path = s
                    .path
                    .normalize()
                    .unwrap_or_else(|err| {
                        panic!("normalize path {} failed: {:?}", s.path.display(), err)
                    })
                    .into_path_buf();
                self.file_id(path.clone()).unwrap_or_else(|| {
                    panic!(
                        "file_id not found for path {} in file_ids_map {:?}",
                        path.display(),
                        self.file_ids_map()
                    )
                }) == file_id
            })
            .unwrap()
    }
}

pub mod tls {

    use scoped_tls::scoped_thread_local;

    use super::Context;
    use crate::DefId;

    scoped_thread_local!(pub static CONTEXT: Context);
    scoped_thread_local!(pub static CUR_ITEM: DefId);

    pub fn with_cx<T, F>(f: F) -> T
    where
        F: FnOnce(&Context) -> T,
    {
        CONTEXT.with(|cx| f(cx))
    }

    pub fn with_cur_item<T, F>(f: F) -> T
    where
        F: FnOnce(DefId) -> T,
    {
        CUR_ITEM.with(|def_id| f(*def_id))
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf, sync::Arc};

    use anyhow::Result;
    use faststr::FastStr;
    use pilota::Bytes;
    use rustc_hash::{FxHashMap, FxHashSet};

    use super::*;
    use crate::{
        middle::{
            ext::{FileExts, ItemExts},
            rir::{self, FieldKind, Message},
            ty::{CodegenTy, Ty, TyKind},
        },
        symbol::{Ident, Symbol},
    };

    fn make_test_context() -> Context {
        let mode = Arc::new(Mode::SingleFile {
            file_path: PathBuf::from("dummy.rs"),
        });
        let services: Arc<[crate::IdlService]> =
            Arc::from(Vec::<crate::IdlService>::new().into_boxed_slice());
        Context {
            db: RootDatabase::default(),
            source: Source {
                source_type: SourceType::Thrift,
                services,
                mode: mode.clone(),
                path_resolver: Arc::new(DefaultPathResolver),
            },
            config: Config {
                change_case: false,
                split: false,
                with_descriptor: false,
                with_field_mask: false,
                touch_all: false,
                common_crate_name: "common".into(),
            },
            cache: Cache {
                adjusts: Arc::new(DashMap::default()),
                mod_idxes: AHashMap::new(),
                codegen_items: Vec::new(),
                mod_items: HashMap::new(),
                def_mod: HashMap::new(),
                mod_files: HashMap::new(),
                keep_unknown_fields: Arc::new(FxHashSet::default()),
                location_map: Arc::new(FxHashMap::default()),
                entry_map: Arc::new(HashMap::default()),
                plugin_gen: Arc::new(DashMap::default()),
                dedups: Vec::new(),
                names: FxHashMap::default(),
            },
        }
    }

    #[test]
    fn collect_items_traverses_field_dependencies() {
        let file_id = FileId::from_u32(0);
        let root_id = DefId::from_u32(0);
        let dep_id = DefId::from_u32(1);
        let field_id = DefId::from_u32(2);
        let tag_id = TagId::from_u32(0);

        let dep_message = Arc::new(rir::Item::Message(Message {
            name: Ident::from("Dep"),
            fields: Vec::new(),
            is_wrapper: false,
            item_exts: ItemExts::Thrift,
            leading_comments: FastStr::new(""),
            trailing_comments: FastStr::new(""),
        }));

        let dep_node = rir::Node {
            file_id,
            kind: rir::NodeKind::Item(dep_message.clone()),
            parent: None,
            tags: tag_id,
            related_nodes: Vec::new(),
        };

        let field_ty = Ty {
            kind: TyKind::Path(rir::Path {
                kind: rir::DefKind::Type,
                did: dep_id,
            }),
            tags_id: tag_id,
        };

        let field = Arc::new(rir::Field {
            did: field_id,
            name: Ident::from("dep"),
            id: 1,
            ty: field_ty,
            kind: FieldKind::Required,
            tags_id: tag_id,
            default: None,
            item_exts: ItemExts::Thrift,
            leading_comments: FastStr::new(""),
            trailing_comments: FastStr::new(""),
        });

        let root_message = Arc::new(rir::Item::Message(Message {
            name: Ident::from("Root"),
            fields: vec![field],
            is_wrapper: false,
            item_exts: ItemExts::Thrift,
            leading_comments: FastStr::new(""),
            trailing_comments: FastStr::new(""),
        }));

        let root_node = rir::Node {
            file_id,
            kind: rir::NodeKind::Item(root_message.clone()),
            parent: None,
            tags: tag_id,
            related_nodes: Vec::new(),
        };

        let mut nodes = FxHashMap::default();
        nodes.insert(root_id, root_node);
        nodes.insert(dep_id, dep_node);

        let package = ItemPath::from(Arc::<[Symbol]>::from(
            Vec::<Symbol>::new().into_boxed_slice(),
        ));
        let file = rir::File {
            package,
            items: vec![root_id, dep_id],
            file_id,
            uses: Vec::new(),
            descriptor: Bytes::new(),
            extensions: FileExts::Thrift,
            comments: FastStr::new(""),
        };

        let file_arc = Arc::new(file);

        let mut file_ids_map = FxHashMap::default();
        let normalized = Arc::new(PathBuf::from("/tmp/test.thrift"));
        file_ids_map.insert(normalized.clone(), file_id);

        let mut file_paths = FxHashMap::default();
        file_paths.insert(file_id, normalized);

        let mut file_names = FxHashMap::default();
        file_names.insert(file_id, FastStr::from_static_str("test.thrift"));

        let db = RootDatabase::default()
            .with_nodes(nodes)
            .with_files(vec![(file_id, file_arc)].into_iter())
            .with_file_ids_map(file_ids_map)
            .with_file_paths(file_paths)
            .with_file_names(file_names)
            .with_input_files(vec![file_id]);

        let builder = ContextBuilder::new(
            db,
            Mode::SingleFile {
                file_path: PathBuf::from("/tmp/output.rs"),
            },
            vec![root_id],
        );

        let collected = builder.collect_items(&[root_id]);

        assert!(collected.contains(&root_id));
        assert!(collected.contains(&dep_id));
        assert_eq!(collected.len(), 2);
    }

    #[test]
    fn lit_into_ty_handles_basic_literals() -> Result<()> {
        let cx = make_test_context();

        let (expr, is_const) =
            cx.lit_into_ty(&Literal::String(Arc::from("hello")), &CodegenTy::FastStr)?;
        assert_eq!(&*expr, "::pilota::FastStr::from_static_str(\"hello\")");
        assert!(is_const);

        let list_lit = Literal::List(vec![Literal::Int(1), Literal::Int(2)]);
        let vec_ty = CodegenTy::Vec(Arc::new(CodegenTy::I32));
        let (expr, is_const) = cx.lit_into_ty(&list_lit, &vec_ty)?;
        assert_eq!(&*expr, "::std::vec![1i32,2i32]");
        assert!(!is_const);

        Ok(())
    }
}
