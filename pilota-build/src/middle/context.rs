use core::panic;
use std::{ops::Deref, sync::Arc};

use fxhash::FxHashMap;
use proc_macro2::Span;
use quote::format_ident;
use syn::PathSegment;

use self::tls::with_cur_item;
use super::{adjust::Adjust, rir::NodeKind};
use crate::{
    db::{RirDatabase, RootDatabase},
    symbol::{DefId, Symbol},
    tags::{TagId, Tags},
    Plugin,
};

type Segments = Vec<Symbol>;

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

    pub fn node_tags(&self, def_id: DefId) -> Arc<Tags> {
        let tag_id = self.node(def_id).unwrap().tags;
        self.tags(tag_id).unwrap()
    }

    pub fn contains_tag<T: 'static>(&self, tags_id: TagId) -> bool {
        self.tags(tags_id)
            .and_then(|tags| tags.contains::<T>().then_some(true))
            .is_some()
    }

    pub fn node_contains_tag<T: 'static>(&self, def_id: DefId) -> bool {
        self.node_tags(def_id).contains::<T>()
    }

    pub fn symbol_name(&self, def_id: DefId) -> Symbol {
        let item = self.item(def_id).unwrap();
        item.symbol_name()
    }

    fn related_path(&self, p1: &Segments, p2: &Segments) -> syn::Path {
        if p1 == p2 {
            return syn::Path::from(format_ident!("{}", p2.last().unwrap()));
        }
        let mut i = 0;
        while i < p1.len() && i < p2.len() && p1[i] == p2[i] {
            i += 1
        }
        let mut segs = syn::punctuated::Punctuated::new();

        enum Kind {
            Super,
            Ident(Symbol),
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
                    let ident = format_ident!("{}", ident);
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

    fn item_path(&self, def_id: DefId) -> Segments {
        fn calc_item_path(cx: &Context, def_id: DefId, segs: &mut Vec<Symbol>) {
            let node = cx.node(def_id).unwrap();
            if let Some(parent) = node.parent {
                calc_item_path(cx, parent, segs)
            } else {
                let file = cx.file(node.file_id).unwrap();
                let package = &file.package;
                segs.extend_from_slice(package)
            }

            let name = match node.kind {
                NodeKind::Item(item) => item.symbol_name().to_upper_camel_case(),
                NodeKind::Variant(v) => (*v.name).to_upper_camel_case(),
                _ => panic!(),
            };
            segs.push(name);
        }

        let mut segs = Default::default();

        calc_item_path(self, def_id, &mut segs);

        segs
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
