use std::{collections::HashMap, ops::Deref, path::PathBuf, sync::Arc};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use normpath::PathExt;
use proc_macro2::Span;
use quote::format_ident;
use syn::PathSegment;

use self::tls::with_cur_item;
use super::{adjust::Adjust, rir::NodeKind};
use crate::{
    db::{RirDatabase, RootDatabase},
    rir,
    symbol::{DefId, IdentName, Symbol},
    tags::{TagId, Tags},
    ty::Visitor,
    Plugin,
};

pub enum CollectMode {
    All,
    OnlyUsed {
        touches: Vec<(std::path::PathBuf, Vec<String>)>,
        input: Vec<DefId>,
    },
}

pub struct Context {
    pub db: salsa::Snapshot<RootDatabase>,
    adjusts: FxHashMap<DefId, Adjust>,
    tags_map: FxHashMap<TagId, Arc<Tags>>,
}

impl Deref for Context {
    type Target = salsa::Snapshot<RootDatabase>;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl Context {
    pub fn new(db: salsa::Snapshot<RootDatabase>) -> Context {
        Context {
            db,
            adjusts: Default::default(),
            tags_map: Default::default(),
        }
    }

    pub fn set_tags_map(&mut self, tags_map: FxHashMap<TagId, Arc<Tags>>) {
        self.tags_map = tags_map
    }

    pub fn adjust(&self, def_id: DefId) -> Option<&Adjust> {
        self.adjusts.get(&def_id)
    }

    pub fn with_adjust<T, F>(&mut self, def_id: DefId, f: F) -> T
    where
        F: FnOnce(&mut Adjust) -> T,
    {
        let adjust = self.adjusts.entry(def_id).or_insert_with(Default::default);
        f(adjust)
    }

    pub fn tags(&self, tags_id: TagId) -> Option<Arc<Tags>> {
        self.tags_map.get(&tags_id).cloned()
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

    pub fn rust_name(&self, def_id: DefId) -> smol_str::SmolStr {
        let node = self.node(def_id).unwrap();

        if let Some(name) = self
            .tags(node.tags)
            .and_then(|tags| tags.get::<crate::tags::PilotaName>().cloned())
        {
            return name.0;
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
            NodeKind::Variant(v) => (&**v.name).variant_ident(),
            NodeKind::Field(f) => (&**f.name).field_ident(),
            NodeKind::Method(m) => (&**m.name).fn_ident(),
        }
    }

    pub fn mod_path(&self, def_id: DefId) -> Arc<[smol_str::SmolStr]> {
        fn calc_item_path(cx: &Context, def_id: DefId, segs: &mut Vec<smol_str::SmolStr>) {
            let node = cx.node(def_id).unwrap();
            if let Some(parent) = node.parent {
                calc_item_path(cx, parent, segs)
            } else {
                let file = cx.file(node.file_id).unwrap();
                let package = &file.package;
                segs.extend(package.iter().map(|s| (&*s.0).mod_ident()))
            }

            if let NodeKind::Item(item) = node.kind {
                if let crate::rir::Item::Mod(_) = &*item {
                    segs.push(cx.rust_name(def_id));
                }
            }
        }

        let mut segs = Default::default();

        calc_item_path(self, def_id, &mut segs);

        Arc::from(segs)
    }

    pub fn item_path(&self, def_id: DefId) -> Arc<[smol_str::SmolStr]> {
        fn calc_item_path(db: &Context, def_id: DefId, segs: &mut Vec<smol_str::SmolStr>) {
            let node = db.node(def_id).unwrap();

            match node.kind {
                NodeKind::Item(_) => {}
                _ => calc_item_path(db, node.parent.unwrap(), segs),
            }

            let name = match node.kind {
                NodeKind::Item(item) => match &*item {
                    crate::rir::Item::Const(_) => (&*item.symbol_name()).const_ident(),
                    crate::rir::Item::Mod(_) => return,
                    _ => (&*item.symbol_name()).upper_camel_ident(),
                },
                NodeKind::Variant(v) => (&**v.name).variant_ident(),
                _ => panic!(),
            };
            segs.push(name);
        }

        let mut segs = Vec::from(&*self.mod_path(def_id));

        calc_item_path(&self, def_id, &mut segs);

        Arc::from(segs)
    }

    fn related_path(&self, p1: &[smol_str::SmolStr], p2: &[smol_str::SmolStr]) -> syn::Path {
        if p1 == p2 {
            return syn::Path::from(format_ident!("{}", p2.last().unwrap().as_syn_ident()));
        }
        let mut i = 0;
        while i < p1.len() && i < p2.len() && p1[i] == p2[i] {
            i += 1
        }
        let mut segs = syn::punctuated::Punctuated::new();

        enum Kind {
            Super,
            Ident(smol_str::SmolStr),
        }
        let path = (0..p1.len() - i)
            .map(|_| Kind::Super)
            .chain((i..p2.len()).map(|i| Kind::Ident(p2[i].clone())))
            .collect::<Vec<_>>();

        let _length = path.len();

        for (_idx, k) in path.into_iter().enumerate() {
            segs.push(match k {
                Kind::Super => PathSegment::from(syn::token::Super(Span::call_site())),
                Kind::Ident(ident) => {
                    let ident = ident.as_syn_ident();
                    PathSegment::from(ident)
                }
            });
        }

        syn::Path {
            leading_colon: None,
            segments: segs,
        }
    }

    pub fn cur_related_item_path(&self, did: DefId) -> syn::Path {
        let a = with_cur_item(|def_id| def_id);
        self.related_item_path(a, did)
    }

    pub fn related_item_path(&self, a: DefId, b: DefId) -> syn::Path {
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
    pub fn exec_plugin<P: Plugin>(&mut self, mut p: P) {
        self.nodes()
            .iter()
            .for_each(|(def_id, node)| match &node.kind {
                NodeKind::Item(item) => p.on_item(self, *def_id, item.clone()),
                _ => {}
            });
        p.on_emit(self)
    }

    pub(crate) fn collect_items(
        &self,
        touches: Vec<(std::path::PathBuf, Vec<String>)>,
        input: Vec<DefId>,
    ) -> FxHashSet<DefId> {
        struct PathCollector<'a> {
            set: &'a mut FxHashSet<DefId>,
            cx: &'a Context,
        }

        impl super::ty::Visitor for PathCollector<'_> {
            fn visit_path(&mut self, path: &crate::rir::Path) {
                collect(self.cx, path.did, self.set)
            }
        }

        fn collect(cx: &Context, def_id: DefId, set: &mut FxHashSet<DefId>) {
            if set.contains(&def_id) {
                return;
            }
            if !matches!(&*cx.item(def_id).unwrap(), rir::Item::Mod(_)) {
                set.insert(def_id);
            }

            let node = cx.node(def_id).unwrap();

            node.related_nodes
                .iter()
                .for_each(|def_id| collect(cx, *def_id, set));

            let item = node.expect_item();

            match item {
                rir::Item::Message(m) => m
                    .fields
                    .iter()
                    .for_each(|f| PathCollector { cx, set }.visit(&f.ty)),
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

        touches.iter().for_each(|s| {
            let path = &s.0;
            s.1.iter().for_each(|item_name| {
                let file_id = *self
                    .file_ids_map()
                    .get(&PathBuf::from(path).normalize().unwrap().into_path_buf())
                    .unwrap();
                let def_id = self
                    .files()
                    .get(&file_id)
                    .unwrap()
                    .items
                    .iter()
                    .find(|def_id| &*self.item(**def_id).unwrap().symbol_name() == item_name)
                    .cloned();
                if let Some(def_id) = def_id {
                    collect(&self, def_id, &mut set);
                } else {
                    println!(
                        "cargo:warning=item `{}` of `{}` not exists",
                        item_name,
                        path.display(),
                    );
                }
            });
        });

        input.iter().for_each(|def_id| {
            collect(&self, *def_id, &mut set);
        });

        self.nodes().iter().for_each(|(def_id, node)| {
            if let NodeKind::Item(item) = &node.kind {
                if let rir::Item::Const(_) = &**item {
                    collect(&self, *def_id, &mut set);
                }
            }
        });

        set
    }

    pub(crate) fn collect_pkgs(
        &mut self,
        mode: CollectMode,
    ) -> HashMap<Arc<[smol_str::SmolStr]>, Vec<DefId>> {
        match mode {
            CollectMode::All => {
                let files = self.files();
                let mut map: HashMap<_, Vec<DefId>> = HashMap::with_capacity(files.len());
                for file in files.values() {
                    map.entry(Arc::from_iter(
                        file.package.iter().map(|s| (&**s).mod_ident()),
                    ))
                    .or_default()
                    .extend_from_slice(&file.items);
                }

                map
            }
            CollectMode::OnlyUsed { touches, input } => {
                let def_ids = self.collect_items(touches, input);
                def_ids
                    .into_iter()
                    .into_group_map_by(|def_id| self.mod_path(*def_id))
            }
        }
    }
}

pub mod tls {
    use std::sync::Arc;

    use scoped_tls::scoped_thread_local;

    use super::Context;
    use crate::DefId;

    scoped_thread_local!(pub static CONTEXT: Arc<Context>);
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
