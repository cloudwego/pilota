use syn::{Meta, Token, parse::Parser, punctuated::Punctuated};

use crate::tags::SerdeAttribute;

#[derive(Clone, Copy)]
pub struct SerdePlugin;

/// Emits `#[serde(rename = "..")]` on every field, so serialized output keeps
/// the field names as spelled in the IDL rather than the snake_case idents
/// generated for Rust.
///
/// Only meaningful next to [`SerdePlugin`], which emits the derives these
/// attributes attach to; this plugin does not imply it.
#[derive(Clone, Copy)]
pub struct SerdePreserveIdlNamesPlugin;

impl crate::Plugin for SerdePreserveIdlNamesPlugin {
    fn on_field(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        f: std::sync::Arc<crate::rir::Field>,
    ) {
        // Avoid duplicating `#[serde(rename = "...")]` if the user has already
        // specified one
        if has_explicit_rename(cx, &f) {
            return;
        }

        let original = f.name.raw_str();
        cx.with_adjust_mut(def_id, |adj| {
            adj.add_attrs(&[format!("#[serde(rename = {:?})]", &*original).into()]);
        });
    }
}

/// Returns true if the field has an explicit `#[serde(rename = ..)]`.
fn has_explicit_rename(cx: &crate::Context, f: &crate::rir::Field) -> bool {
    cx.tags(f.tags_id).is_some_and(|tags| {
        tags.get::<SerdeAttribute>()
            .is_some_and(|attr| is_serde_rename(&attr.0.replace('\\', "")))
    })
}

/// Returns true if `s` holds an attribute of the form `#[serde(.., rename =
/// ..)]`.
fn is_serde_rename(s: &str) -> bool {
    let Ok(attrs) = syn::Attribute::parse_outer.parse_str(s) else {
        return false;
    };
    attrs.iter().any(|attr| {
        attr.path().is_ident("serde")
            && attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .is_ok_and(|metas| metas.iter().any(|meta| meta.path().is_ident("rename")))
    })
}

impl crate::Plugin for SerdePlugin {
    fn on_item(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        item: std::sync::Arc<crate::rir::Item>,
    ) {
        let attribute = cx
            .node_tags(def_id)
            .and_then(|tags| tags.get::<SerdeAttribute>().cloned());

        match &*item {
            crate::rir::Item::Message(_)
            | crate::rir::Item::Enum(_)
            | crate::rir::Item::NewType(_) => {
                cx.with_adjust_mut(def_id, |adj| {
                    adj.add_attrs(&[
                        "#[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize)]"
                            .into(),
                    ]);
                    if let Some(attribute) = attribute {
                        let attr = attribute.0.to_string().replace('\\', "");
                        adj.add_attrs(&[attr.into()]);
                    }
                });
            }
            _ => {}
        };

        if let crate::rir::Item::Enum(e) = &*item {
            if e.repr.is_some() {
                cx.with_adjust_mut(def_id, |adj| {
                    adj.add_attrs(&["#[serde(transparent)]".into()]);
                })
            }
        }

        crate::plugin::walk_item(self, cx, def_id, item)
    }

    fn on_field(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        f: std::sync::Arc<crate::rir::Field>,
    ) {
        if let Some(attribute) = cx
            .tags(f.tags_id)
            .and_then(|tags| tags.get::<SerdeAttribute>().cloned())
        {
            let attr = attribute.0.replace('\\', "");
            cx.with_adjust_mut(def_id, |adj| adj.add_attrs(&[attr.into()]))
        }
    }

    fn on_variant(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        variant: std::sync::Arc<crate::rir::EnumVariant>,
    ) {
        if let Some(attribute) = cx
            .node_tags(variant.did)
            .and_then(|tags| tags.get::<SerdeAttribute>().cloned())
        {
            let attr = attribute.0.replace('\\', "");
            cx.with_adjust_mut(def_id, |adj| adj.add_attrs(&[attr.into()]))
        }
    }
}
