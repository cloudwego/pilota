use std::sync::Arc;

use petgraph::{graph::NodeIndex, Graph};
use rustc_hash::FxHashMap;

use super::{
    rir::Item,
    ty::{self},
};
use crate::symbol::DefId;

#[derive(Debug)]
pub struct WorkspaceGraph {
    pub(crate) graph: Graph<DefId, ()>,
    pub(crate) node_map: FxHashMap<DefId, NodeIndex>,
    pub(crate) id_map: FxHashMap<NodeIndex, DefId>,
}

impl WorkspaceGraph {
    pub fn from_items(items: impl Iterator<Item = (DefId, Arc<Item>)> + Clone) -> Self {
        let mut graph: Graph<DefId, ()> = Graph::new();
        let mut node_map = FxHashMap::default();
        let mut id_map = FxHashMap::default();
        items.clone().for_each(|(def_id, _)| {
            let node_index = graph.add_node(def_id);
            node_map.insert(def_id, node_index);
            id_map.insert(node_index, def_id);
        });

        fn visit(
            graph: &mut Graph<DefId, ()>,
            idx: NodeIndex,
            node_map: &FxHashMap<DefId, NodeIndex>,
            ty: &ty::Ty,
        ) {
            match &ty.kind {
                ty::Path(p) => {
                    graph.add_edge(idx, node_map[&p.did], ());
                }
                ty::Vec(ty) | ty::Set(ty) => {
                    if let ty::Path(p) = &ty.kind {
                        graph.add_edge(idx, node_map[&p.did], ());
                    }
                }
                ty::Map(ty1, ty2) => {
                    if let ty::Path(p) = &ty1.kind {
                        graph.add_edge(idx, node_map[&p.did], ());
                    }
                    if let ty::Path(p) = &ty2.kind {
                        graph.add_edge(idx, node_map[&p.did], ());
                    }
                }
                _ => {}
            }
        }

        items.for_each(|(def_id, item)| {
            let idx = node_map[&def_id];
            match &*item {
                Item::Message(s) => s.fields.iter().for_each(|f| {
                    visit(&mut graph, idx, &node_map, &f.ty);
                }),
                Item::Enum(e) => {
                    e.variants.iter().flat_map(|v| &v.fields).for_each(|ty| {
                        visit(&mut graph, idx, &node_map, ty);
                    });
                }
                Item::NewType(t) => {
                    visit(&mut graph, idx, &node_map, &t.ty);
                }
                _ => {}
            };
        });
        Self {
            graph,
            node_map,
            id_map,
        }
    }
}
