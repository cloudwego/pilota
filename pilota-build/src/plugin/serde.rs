use std::str::FromStr;

use syn::parse_quote;

use crate::tags::SerdeAttribute;

#[derive(Clone, Copy)]
pub struct SerdePlugin;

impl crate::Plugin for SerdePlugin {
    fn on_item(
        &mut self,
        cx: &mut crate::Context,
        def_id: crate::DefId,
        item: std::sync::Arc<crate::rir::Item>,
    ) {
        let attribute = cx
            .node_tags(def_id)
            .and_then(|tags| tags.get::<SerdeAttribute>().cloned());

        match &*item {
            crate::rir::Item::Message(_)
            | crate::rir::Item::Enum(_)
            | crate::rir::Item::NewType(_) => cx.with_adjust(def_id, |adj| {
                adj.add_attrs(&[parse_quote!(#[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize)])]);
                if let Some(attribute) = attribute {
                    let attr = attribute.0.to_string().replace('\\', "");
                    let tokens = proc_macro2::TokenStream::from_str(&attr).unwrap();
                    adj.add_attrs(&[parse_quote!(#tokens)]);
                }
            }),
            _ => {}
        };
        crate::plugin::walk_item(self, cx, def_id, item)
    }

    fn on_field(
        &mut self,
        cx: &mut crate::Context,
        def_id: crate::DefId,
        f: std::sync::Arc<crate::rir::Field>,
    ) {
        if let Some(attribute) = cx
            .tags(f.tags_id)
            .and_then(|tags| tags.get::<SerdeAttribute>().cloned())
        {
            let attr = attribute.0.replace('\\', "");
            let tokens = proc_macro2::TokenStream::from_str(&attr).unwrap();
            cx.with_adjust(def_id, |adj| adj.add_attrs(&[parse_quote!(#tokens)]))
        }
    }

    fn on_variant(
        &mut self,
        cx: &mut crate::Context,
        def_id: crate::DefId,
        variant: std::sync::Arc<crate::rir::EnumVariant>,
    ) {
        if let Some(attribute) = cx
            .node_tags(variant.did)
            .and_then(|tags| tags.get::<SerdeAttribute>().cloned())
        {
            let attr = attribute.0.replace('\\', "");
            let tokens = proc_macro2::TokenStream::from_str(&attr).unwrap();
            cx.with_adjust(def_id, |adj| adj.add_attrs(&[parse_quote!(#tokens)]))
        }
    }
}
