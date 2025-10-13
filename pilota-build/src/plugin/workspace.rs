use crate::{
    Plugin,
    db::RirDatabase,
    middle::context::tls::CUR_ITEM,
    rir::{Item, NodeKind},
};

#[derive(Clone, Copy)]
pub struct _WorkspacePlugin;

impl Plugin for _WorkspacePlugin {
    fn on_codegen_uint(&mut self, cx: &crate::Context, _items: &[crate::DefId]) {
        cx.cache.entry_map.iter().for_each(|(k, v)| {
            cx.cache.plugin_gen.insert(k.clone(), "".to_string());
            v.iter().for_each(|(def_id, _)| {
                CUR_ITEM.set(def_id, || {
                    let node = cx.node(*def_id).unwrap();
                    if let NodeKind::Item(item) = &node.kind {
                        self.on_item(cx, *def_id, item.clone());
                    }
                });
            })
        });
    }

    fn on_item(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        item: std::sync::Arc<crate::rir::Item>,
    ) {
        if let Item::Service(s) = &*item {
            if let Some(loc) = cx.cache.location_map.get(&def_id) {
                if let Some(mut r#gen) = cx.cache.plugin_gen.get_mut(loc) {
                    r#gen.push_str(&format!("pub struct {};", s.name.sym));
                }
            };
        }
    }
}
