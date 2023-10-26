use std::sync::Arc;

use fxhash::FxHashMap;
use petgraph::{algo::has_path_connecting, graph::NodeIndex, Graph};

use super::{
    rir::Item,
    ty::{self},
};
use crate::symbol::DefId;

#[derive(Debug)]
pub struct TypeGraph {
    graph: Graph<DefId, ()>,
    node_map: FxHashMap<DefId, NodeIndex>,
}

impl TypeGraph {
    pub fn from_items(items: impl Iterator<Item = (DefId, Arc<Item>)> + Clone) -> Self {
        let mut graph: Graph<DefId, ()> = Graph::new();
        let mut node_map = FxHashMap::default();
        items.clone().for_each(|(def_id, _)| {
            node_map.insert(def_id, graph.add_node(def_id));
        });

        items.for_each(|(def_id, item)| {
            let idx = node_map[&def_id];
            match &*item {
                Item::Message(s) => s.fields.iter().for_each(|f| {
                    if let ty::Path(p) = &f.ty.kind {
                        graph.add_edge(idx, node_map[&p.did], ());
                    }
                }),
                Item::Enum(e) => {
                    e.variants.iter().flat_map(|v| &v.fields).for_each(|ty| {
                        if let ty::Path(p) = &ty.kind {
                            graph.add_edge(idx, node_map[&p.did], ());
                        }
                    });
                }
                Item::NewType(t) => {
                    if let ty::Path(p) = &t.ty.kind {
                        graph.add_edge(idx, node_map[&p.did], ());
                    }
                }
                _ => {}
            };
        });
        Self { graph, node_map }
    }

    pub fn is_nested(&self, a: DefId, b: DefId) -> bool {
        let a = self.node_map[&a];
        let b = self.node_map[&b];
        has_path_connecting(&self.graph, a, b, None)
    }

    pub fn is_cycled(&self, a: DefId) -> bool {
        let a = self.node_map[&a];
        for n in self
            .graph
            .neighbors_directed(a, petgraph::Direction::Outgoing)
        {
            if has_path_connecting(&self.graph, n, a, None) {
                return true;
            }
        }
        false
    }
}
