use std::{collections::HashMap, ops::Deref, path::PathBuf, sync::Arc};

use anyhow::Context as _;
use dashmap::DashMap;
use faststr::FastStr;
use fxhash::{FxHashMap, FxHashSet};
use heck::ToShoutySnakeCase;
use itertools::Itertools;
use normpath::PathExt;
use quote::format_ident;
use salsa::ParallelDatabase;

use self::tls::with_cur_item;
use super::{
    adjust::Adjust,
    resolver::{DefaultPathResolver, PathResolver, WorkspacePathResolver},
    rir::NodeKind,
};
use crate::{
    db::{RirDatabase, RootDatabase},
    rir::{self, Field, Item, ItemPath, Literal},
    symbol::{DefId, FileId, IdentName, Symbol},
    tags::{EnumMode, TagId, Tags},
    ty::{AdtDef, AdtKind, CodegenTy, Visitor},
    Plugin,
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
    pub(crate) dir: PathBuf,
    pub(crate) location_map: FxHashMap<DefId, DefLocation>,
}

#[derive(Debug)]
pub enum Mode {
    Workspace(WorkspaceInfo),
    SingleFile { file_path: std::path::PathBuf },
}

pub struct Context {
    pub source_type: SourceType,
    pub db: salsa::Snapshot<RootDatabase>,
    pub adjusts: Arc<DashMap<DefId, Adjust>>,
    pub services: Arc<[crate::IdlService]>,
    pub(crate) change_case: bool,
    pub(crate) codegen_items: Arc<[DefId]>,
    pub(crate) path_resolver: Arc<dyn PathResolver>,
    pub(crate) mode: Arc<Mode>,
    pub(crate) keep_unknown_fields: FxHashSet<DefId>,
    pub location_map: FxHashMap<DefId, DefLocation>,
    pub entry_map: HashMap<DefLocation, Vec<(DefId, DefLocation)>>,
    pub plugin_gen: DashMap<DefLocation, String>,
    pub(crate) dedups: Vec<FastStr>,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            source_type: self.source_type,
            db: self.db.snapshot(),
            adjusts: self.adjusts.clone(),
            change_case: self.change_case,
            codegen_items: self.codegen_items.clone(),
            path_resolver: self.path_resolver.clone(),
            mode: self.mode.clone(),
            services: self.services.clone(),
            keep_unknown_fields: self.keep_unknown_fields.clone(),
            location_map: self.location_map.clone(),
            entry_map: self.entry_map.clone(),
            plugin_gen: self.plugin_gen.clone(),
            dedups: self.dedups.clone(),
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
                self.codegen_items.extend(nodes.iter().filter_map(|(k, v)| {
                    if let NodeKind::Item(i) = &v.kind {
                        if !matches!(&**i, Item::Mod(_)) {
                            Some(k)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
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
                                        "cargo:warning=item `{}` of `{}` not exists",
                                        item_name,
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
        }

        impl super::ty::Visitor for PathCollector<'_> {
            fn visit_path(&mut self, path: &crate::rir::Path) {
                collect(self.cx, path.did, self.set)
            }
        }

        fn collect(cx: &ContextBuilder, def_id: DefId, set: &mut FxHashSet<DefId>) {
            if set.contains(&def_id) {
                return;
            }

            let node = cx.db.node(def_id).unwrap();

            match node.kind {
                NodeKind::Item(_) => {}
                _ => return collect(cx, node.parent.unwrap(), set),
            }

            if !matches!(&*cx.db.item(def_id).unwrap(), rir::Item::Mod(_)) {
                set.insert(def_id);
            }

            let node = cx.db.node(def_id).unwrap();
            tracing::trace!("collecting {:?}", node.expect_item().symbol_name());

            node.related_nodes
                .iter()
                .for_each(|def_id| collect(cx, *def_id, set));

            let item = node.expect_item();

            match item {
                rir::Item::Message(m) => m.fields.iter().for_each(|f| {
                    PathCollector { cx, set }.visit(&f.ty);
                    if let Some(Literal::Path(p)) = &f.default {
                        PathCollector { cx, set }.visit_path(&p);
                    }
                }),
                rir::Item::Enum(e) => e
                    .variants
                    .iter()
                    .flat_map(|v| &v.fields)
                    .for_each(|ty| PathCollector { cx, set }.visit(ty)),
                rir::Item::Service(s) => {
                    s.extend.iter().for_each(|p| collect(cx, p.did, set));
                    s.methods
                        .iter()
                        .flat_map(|m| m.args.iter().map(|f| &f.ty).chain(std::iter::once(&m.ret)))
                        .for_each(|ty| PathCollector { cx, set }.visit(ty));
                }
                rir::Item::NewType(n) => PathCollector { cx, set }.visit(&n.ty),
                rir::Item::Const(c) => {
                    PathCollector { cx, set }.visit(&c.ty);
                }
                rir::Item::Mod(m) => {
                    m.items.iter().for_each(|i| collect(cx, *i, set));
                }
            }
        }
        let mut set = FxHashSet::default();

        input.iter().for_each(|def_id| {
            collect(self, *def_id, &mut set);
        });

        self.db.nodes().iter().for_each(|(def_id, node)| {
            if let NodeKind::Item(item) = &node.kind {
                if let rir::Item::Const(_) = &**item {
                    collect(self, *def_id, &mut set);
                }
            }
        });

        set
    }

    pub(crate) fn workspace_collect_def_ids(
        &self,
        input: &[DefId],
    ) -> FxHashMap<DefId, DefLocation> {
        const MAX_RECURSION_DEPTH: usize = 64;
        struct PathCollector<'a> {
            map: &'a mut FxHashMap<DefId, DefLocation>,
            cx: &'a ContextBuilder,
            depth: usize,
        }

        impl crate::ty::Visitor for PathCollector<'_> {
            fn visit_path(&mut self, path: &crate::rir::Path) {
                collect(self.cx, path.did, self.map, self.depth)
            }
        }

        fn collect(
            cx: &ContextBuilder,
            def_id: DefId,
            map: &mut FxHashMap<DefId, DefLocation>,
            mut depth: usize,
        ) {
            if map.contains_key(&def_id) || depth > MAX_RECURSION_DEPTH {
                return;
            }
            depth += 1;
            if !matches!(&*cx.db.item(def_id).unwrap(), rir::Item::Mod(_)) {
                let file_id = cx.db.node(def_id).unwrap().file_id;

                if cx.db.input_files().contains(&file_id) {
                    let type_graph = cx.db.workspace_graph();
                    let node = type_graph.node_map[&def_id];
                    for from in type_graph
                        .graph
                        .neighbors_directed(node, petgraph::Direction::Incoming)
                    {
                        let from_def_id = type_graph.id_map[&from];
                        let from_file_id = cx.db.node(from_def_id).unwrap().file_id;
                        if from_file_id != file_id {
                            map.insert(def_id, DefLocation::Dynamic);
                            break;
                        } else {
                            if !map.contains_key(&from_def_id) {
                                collect(cx, from_def_id, map, depth);
                            }
                            if map
                                .get(&from_def_id)
                                .map(|v| match v {
                                    DefLocation::Fixed(_, _) => false,
                                    DefLocation::Dynamic => true,
                                })
                                .unwrap_or_default()
                            {
                                map.insert(def_id, DefLocation::Dynamic);
                                break;
                            }
                        }
                    }
                    map.entry(def_id).or_insert_with(|| {
                        let file = cx.db.file(file_id).unwrap();
                        DefLocation::Fixed(CrateId { main_file: file_id }, file.package.clone())
                    });
                } else {
                    map.insert(def_id, DefLocation::Dynamic);
                }
            }

            let node = cx.db.node(def_id).unwrap();
            tracing::trace!("collecting {:?}", node.expect_item().symbol_name());

            node.related_nodes
                .iter()
                .for_each(|def_id| collect(cx, *def_id, map, depth));

            let item = node.expect_item();

            match item {
                rir::Item::Message(m) => m
                    .fields
                    .iter()
                    .for_each(|f| PathCollector { cx, map, depth }.visit(&f.ty)),
                rir::Item::Enum(e) => e
                    .variants
                    .iter()
                    .flat_map(|v| &v.fields)
                    .for_each(|ty| PathCollector { cx, map, depth }.visit(ty)),
                rir::Item::Service(s) => {
                    s.extend.iter().for_each(|p| collect(cx, p.did, map, depth));
                    s.methods
                        .iter()
                        .flat_map(|m| m.args.iter().map(|f| &f.ty).chain(std::iter::once(&m.ret)))
                        .for_each(|ty| PathCollector { cx, map, depth }.visit(ty));
                }
                rir::Item::NewType(n) => PathCollector { cx, map, depth }.visit(&n.ty),
                rir::Item::Const(c) => {
                    PathCollector { cx, map, depth }.visit(&c.ty);
                }
                rir::Item::Mod(m) => {
                    m.items.iter().for_each(|i| collect(cx, *i, map, depth));
                }
            }
        }
        let mut map = FxHashMap::default();

        input.iter().for_each(|def_id| {
            collect(self, *def_id, &mut map, 0);
        });

        map
    }

    pub(crate) fn keep(&mut self, keep_unknown_fields: Vec<PathBuf>) {
        let mut file_ids = FxHashSet::default();
        keep_unknown_fields.into_iter().for_each(|p| {
            let path = p.normalize().unwrap().into_path_buf();
            let file_ids_map = self.db.file_ids_map();
            let file_id = file_ids_map.get(&path).unwrap();
            keep_files(self, file_id, &mut file_ids);

            fn keep_files(
                cx: &mut ContextBuilder,
                file_id: &FileId,
                file_ids: &mut FxHashSet<FileId>,
            ) {
                if !file_ids.insert(*file_id) {
                    return;
                }
                let files = cx.db.files();
                let file = files.get(&file_id).unwrap();
                file.uses.iter().for_each(|f| keep_files(cx, f, file_ids));
                cx.keep_unknown_fields.extend(
                    file.items
                        .iter()
                        .filter(|&&def_id| match cx.db.node(def_id) {
                            Some(rir::Node {
                                kind: rir::NodeKind::Item(_),
                                tags,
                                ..
                            }) => {
                                if let Some(crate::tags::KeepUnknownFields(false)) =
                                    cx.db.tags_map().get(&tags).and_then(|tags| {
                                        tags.get::<crate::tags::KeepUnknownFields>()
                                    })
                                {
                                    false
                                } else {
                                    true
                                }
                            }
                            _ => true,
                        })
                        .cloned(),
                )
            }
        });
    }

    pub(crate) fn build(
        self,
        services: Arc<[crate::IdlService]>,
        source_type: SourceType,
        change_case: bool,
        dedups: Vec<FastStr>,
    ) -> Context {
        Context {
            adjusts: Default::default(),
            source_type,
            db: self.db.snapshot(),
            change_case,
            services,
            codegen_items: Arc::from(self.codegen_items),
            path_resolver: match &self.mode {
                Mode::Workspace(_) => Arc::new(WorkspacePathResolver),
                Mode::SingleFile { .. } => Arc::new(DefaultPathResolver),
            },
            mode: Arc::new(self.mode),
            keep_unknown_fields: self.keep_unknown_fields,
            location_map: self.location_map,
            entry_map: self.entry_map,
            plugin_gen: Default::default(),
            dedups,
        }
    }
}

impl Deref for Context {
    type Target = salsa::Snapshot<RootDatabase>;

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
    pub fn with_adjust<T, F>(&self, def_id: DefId, f: F) -> T
    where
        F: FnOnce(Option<&Adjust>) -> T,
    {
        match self.adjusts.get(&def_id) {
            Some(adj) => f(Some(&*adj)),
            None => f(None),
        }
    }

    pub fn with_adjust_mut<T, F>(&self, def_id: DefId, f: F) -> T
    where
        F: FnOnce(&mut Adjust) -> T,
    {
        let adjust = &mut *self.adjusts.entry(def_id).or_insert_with(Default::default);
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

    pub fn default_val(&self, f: &Field) -> Option<(FastStr, bool /* const? */)> {
        f.default.as_ref().map(|d| {
            let ty = self.codegen_item_ty(f.ty.kind.clone());
            match self
                .lit_as_rvalue(d, &ty)
                .with_context(|| format!("calc the default value for field {}", f.name))
            {
                Ok(v) => v,
                Err(err) => {
                    panic!("{:?}", err)
                }
            }
        })
    }

    fn lit_as_rvalue(
        &self,
        lit: &Literal,
        ty: &CodegenTy,
    ) -> anyhow::Result<(FastStr, bool /* const? */)> {
        let mk_map = |m: &Vec<(Literal, Literal)>, k_ty: &Arc<CodegenTy>, v_ty: &Arc<CodegenTy>| {
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
            anyhow::Ok(
                format! {r#"{{
                    let mut map = ::pilota::AHashMap::with_capacity({len});
                    {kvs}
                    map
                }}"#}
                .into(),
            )
        };

        anyhow::Ok(match (lit, ty) {
            (Literal::Map(m), CodegenTy::LazyStaticRef(map)) => match &**map {
                CodegenTy::Map(k_ty, v_ty) => (mk_map(m, k_ty, v_ty)?, false),
                _ => panic!("invalid map type {:?}", map),
            },
            (Literal::Map(m), CodegenTy::Map(k_ty, v_ty)) => (mk_map(m, k_ty, v_ty)?, false),
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
                (format!("({stream} as {target})").into(), true)
            }
            _ => panic!("invalid convert {:?} to {:?}", ident_ty, target),
        }
    }

    fn lit_into_ty(
        &self,
        lit: &Literal,
        ty: &CodegenTy,
    ) -> anyhow::Result<(FastStr, bool /* const? */)> {
        Ok(match (lit, ty) {
            (Literal::Path(p), ty) => {
                let ident_ty = self.codegen_ty(p.did);

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
                CodegenTy::Map(_, _) => {
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
                _ => panic!("invalid map type {:?}", map),
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
                let stream = els
                    .iter()
                    .map(|el| self.lit_into_ty(el, inner))
                    .try_collect::<_, Vec<_>, _>()?
                    .into_iter()
                    .map(|(s, _)| s)
                    .join(",");

                (format! { "::std::vec![{stream}]" }.into(), false)
            }
            (Literal::Bool(b), CodegenTy::Bool) => (format! { "{b}" }.into(), true),
            (Literal::String(s), CodegenTy::Bytes) => {
                let s = &**s;
                (
                    format! { "::bytes::Bytes::from_static({s}.as_bytes())" }.into(),
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
                            if **k == **f.name {
                                Some(v)
                            } else {
                                None
                            }
                        });

                        let name = self.rust_name(f.did);

                        if let Some(v) = v {
                            let (mut v, is_const) =
                                self.lit_into_ty(v, &self.codegen_item_ty(f.ty.kind.clone()))?;

                            if f.is_optional() {
                                v = format!("Some({v})").into()
                            }
                            anyhow::Ok((format!("{name}: {v}"), is_const))
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
            _ => panic!("unexpected literal {:?} with ty {:?}", lit, ty),
        })
    }

    pub(crate) fn def_lit(
        &self,
        name: &str,
        lit: &Literal,
        ty: &mut CodegenTy,
    ) -> anyhow::Result<String> {
        let should_lazy_static = ty.should_lazy_static();
        let name = format_ident!("{}", name.to_shouty_snake_case());
        if let (Literal::List(lit), CodegenTy::Array(_, size)) = (lit, &mut *ty) {
            *size = lit.len()
        }
        Ok(if should_lazy_static {
            let lit = self.lit_as_rvalue(lit, ty)?.0;
            format! {r#"
                ::pilota::lazy_static::lazy_static! {{
                    pub static ref {name}: {ty} = {lit};
                }}
            "#}
        } else {
            let lit = self.lit_into_ty(lit, ty)?.0;
            format!(r#"pub const {name}: {ty} = {lit};"#)
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

        if !self.change_case {
            return self.node(def_id).unwrap().name().0.into();
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

                if self
                    .node_tags(parent)
                    .unwrap()
                    .get::<EnumMode>()
                    .copied()
                    .unwrap_or(EnumMode::Enum)
                    == EnumMode::NewType
                {
                    (&**v.name).shouty_snake_case()
                } else {
                    (&**v.name).variant_ident()
                }
            }
            NodeKind::Field(f) => (&**f.name).field_ident(),
            NodeKind::Method(m) => (&**m.name).fn_ident(),
            NodeKind::Arg(a) => (&**a.name).field_ident(),
        }
        .into()
    }

    pub fn mod_path(&self, def_id: DefId) -> Arc<[Symbol]> {
        self.path_resolver.mod_prefix(self, def_id)
    }

    pub fn item_path(&self, def_id: DefId) -> Arc<[Symbol]> {
        self.path_resolver.path_for_def_id(self, def_id)
    }

    fn related_path(&self, p1: &[Symbol], p2: &[Symbol]) -> FastStr {
        self.path_resolver.related_path(p1, p2)
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
        p.on_codegen_uint(self, &self.codegen_items);

        p.on_emit(self)
    }

    pub(crate) fn workspace_info(&self) -> &WorkspaceInfo {
        let Mode::Workspace(info) = &*self.mode else {
            panic!("can not access workspace info in mode `{:?}`", self.mode)
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
        let main_file = crate_id.main_file;
        let service = self
            .services
            .iter()
            .find(|s| self.file_id(s.path.clone()).unwrap() == main_file)
            .unwrap();
        &service.config
    }

    pub(crate) fn crate_name(&self, location: &DefLocation) -> FastStr {
        match location {
            DefLocation::Fixed(crate_id, _) => {
                let main_file = crate_id.main_file;
                let service = self
                    .services
                    .iter()
                    .find(|s| self.file_id(s.path.clone()).unwrap() == main_file)
                    .unwrap();
                self.config(crate_id)
                    .get("crate_name")
                    .map(|s| s.as_str().map(|s| FastStr::new(s)))
                    .flatten()
                    .unwrap_or_else(|| {
                        service
                            .path
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .replace(".", "_")
                            .into()
                    })
            }
            DefLocation::Dynamic => "common".into(),
        }
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
