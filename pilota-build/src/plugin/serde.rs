use syn::parse_quote;

#[derive(Clone, Copy)]
pub struct SerdePlugin;

impl crate::Plugin for SerdePlugin {
    fn on_item(
        &mut self,
        cx: &mut crate::Context,
        def_id: crate::DefId,
        item: std::sync::Arc<crate::rir::Item>,
    ) {
        match &*item {
            crate::rir::Item::Message(_)
            | crate::rir::Item::Enum(_)
            | crate::rir::Item::NewType(_) => cx.with_adjust(def_id, |adj| {
                adj.add_attrs(&[parse_quote!(#[derive(::serde::Serialize, ::serde::Deserialize)])])
            }),
            _ => {}
        };
        crate::plugin::walk_item(self, cx, def_id, item)
    }
}
