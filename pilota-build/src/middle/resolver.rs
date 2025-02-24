use std::sync::Arc;

use faststr::FastStr;
use itertools::Itertools;

use crate::{db::RirDatabase, rir::NodeKind, symbol::Symbol, Context, DefId, IdentName};

pub trait PathResolver: Sync + Send {
    fn path_for_def_id(&self, cx: &Context, def_id: DefId) -> Arc<[Symbol]> {
        fn calc_node_path(cx: &Context, def_id: DefId, segs: &mut Vec<Symbol>) {
            let node = cx.node(def_id).unwrap();

            match node.kind {
                NodeKind::Item(_) => {}
                _ => calc_node_path(cx, node.parent.unwrap(), segs),
            }

            let name = match node.kind {
                NodeKind::Item(item) => match &*item {
                    crate::rir::Item::Mod(_) => return,
                    _ => cx.rust_name(def_id),
                },
                _ => cx.rust_name(def_id),
            };

            segs.push(name);
        }

        let mut segs = Vec::from(&*self.mod_prefix(cx, def_id));

        calc_node_path(cx, def_id, &mut segs);

        Arc::from(segs)
    }
    fn mod_prefix(&self, cx: &Context, def_id: DefId) -> Arc<[Symbol]>;

    fn related_path(&self, p1: &[Symbol], p2: &[Symbol]) -> FastStr;
}

pub struct DefaultPathResolver;

impl PathResolver for DefaultPathResolver {
    fn mod_prefix(&self, cx: &Context, def_id: DefId) -> Arc<[Symbol]> {
        fn calc_item_path(cx: &Context, def_id: DefId, segs: &mut Vec<Symbol>) {
            let node = cx.node(def_id).unwrap();
            match node.parent { Some(parent) => {
                tracing::debug!("the parent of {:?} is {:?} ", def_id, parent);
                calc_item_path(cx, parent, segs)
            } _ => {
                let file = cx.file(node.file_id).unwrap();
                let package = &file.package;
                if package.len() != 1 || !package.first().unwrap().0.is_empty() {
                    segs.extend(package.iter().map(|s| (&*s.0).mod_ident().into()))
                }
            }}

            if let NodeKind::Item(item) = node.kind {
                if let crate::rir::Item::Mod(_) = &*item {
                    segs.push(cx.rust_name(def_id));
                }
            }
        }

        let mut segs = Default::default();

        calc_item_path(cx, def_id, &mut segs);

        Arc::from(segs)
    }

    fn related_path(&self, p1: &[Symbol], p2: &[Symbol]) -> FastStr {
        if p1 == p2 {
            return p2.last().unwrap().clone().0;
        }
        let mut i = 0;
        while i < p1.len() && i < p2.len() && p1[i] == p2[i] {
            i += 1
        }
        let mut segs = vec![];

        #[derive(Debug)]
        enum Kind {
            Super,
            Ident(Symbol),
        }

        let path = (0..p1.len() - i)
            .map(|_| Kind::Super)
            .chain((i..p2.len()).map(|i| Kind::Ident(p2[i].clone())))
            .collect::<Vec<_>>();

        let _length = path.len();

        for k in path.into_iter() {
            segs.push(match k {
                Kind::Super => "super".into(),
                Kind::Ident(ident) => ident.to_string(),
            });
        }
        segs.into_iter().join("::").into()
    }
}

pub struct WorkspacePathResolver;

impl PathResolver for WorkspacePathResolver {
    fn mod_prefix(&self, cx: &Context, def_id: DefId) -> Arc<[Symbol]> {
        let mut item_def_id = def_id;
        while !matches!(cx.node(item_def_id).unwrap().kind, NodeKind::Item(_)) {
            item_def_id = cx.node(item_def_id).unwrap().parent.unwrap()
        }

        let info = cx.workspace_info();
        let prefix = match info.location_map.get(&item_def_id) {
            location @ Some(super::context::DefLocation::Fixed(_, prefix)) => {
                let mut path = Vec::with_capacity(prefix.len() + 1);
                path.push(cx.crate_name(location.unwrap()).into());
                path.extend(prefix.iter().cloned());
                path
            }
            Some(super::context::DefLocation::Dynamic) => [cx.common_crate_name.clone().into()]
                .iter()
                .chain(DefaultPathResolver.mod_prefix(cx, def_id).iter())
                .cloned()
                .collect_vec(),
            None => {
                panic!(
                    "no location found for \"{}\" in file \"{}\"",
                    cx.rust_name(item_def_id),
                    cx.file(cx.node(item_def_id).unwrap().file_id)
                        .unwrap()
                        .package
                        .join("/")
                )
            }
        };

        Arc::from(prefix)
    }

    fn related_path(&self, p1: &[Symbol], p2: &[Symbol]) -> FastStr {
        if p2[0] == p1[0] {
            DefaultPathResolver.related_path(p1, p2)
        } else {
            format!("::{}", p2.iter().map(|s| s.to_string()).join("::")).into()
        }
    }
}
