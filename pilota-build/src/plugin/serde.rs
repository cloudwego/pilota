use syn::{Meta, Token, parse::Parser, punctuated::Punctuated};

use crate::tags::SerdeAttribute;

#[derive(Clone, Copy, Default)]
pub struct SerdePlugin {
    preserve_idl_field_names: bool,
}

/// Lets `SerdePlugin` keep being used as a value, the way it could when it was
/// a unit struct.
#[allow(non_upper_case_globals)]
pub const SerdePlugin: SerdePlugin = SerdePlugin::new();

impl SerdePlugin {
    pub const fn new() -> Self {
        SerdePlugin {
            preserve_idl_field_names: false,
        }
    }

    /// Preserve the original IDL field names in the generated serde
    /// attributes instead of pilota's Rust-cased identifiers.
    ///
    /// When enabled, the emitted `#[serde(rename = "...")]` attributes use
    /// each field's name exactly as written in the IDL rather than the
    /// snake_case Rust names. Disabled by default.
    pub fn preserve_idl_field_names(mut self, enable: bool) -> Self {
        self.preserve_idl_field_names = enable;
        self
    }
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
        let idl_attr = cx
            .tags(f.tags_id)
            .and_then(|tags| tags.get::<SerdeAttribute>().cloned())
            .map(|attribute| attribute.0.replace('\\', ""));

        if let Some(attr) = idl_attr.clone() {
            cx.with_adjust_mut(def_id, |adj| adj.add_attrs(&[attr.into()]))
        }

        // A `rename` from the IDL is the more specific one, so it wins over the
        // one this option would derive from the field name.
        if self.preserve_idl_field_names && !idl_attr.as_deref().is_some_and(is_serde_rename) {
            let original = f.name.raw_str();
            cx.with_adjust_mut(def_id, |adj| {
                adj.add_attrs(&[format!("#[serde(rename = {:?})]", &*original).into()]);
            });
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

#[cfg(test)]
mod tests {
    use super::is_serde_rename;

    #[test]
    fn detects_rename() {
        assert!(is_serde_rename(r#"#[serde(rename = "AA")]"#));
        assert!(is_serde_rename(
            r#"#[serde(rename(serialize = "a", deserialize = "b"))]"#
        ));
        assert!(is_serde_rename(r#"#[serde(default, rename = "AA")]"#));
    }

    #[test]
    fn ignores_attrs_that_merely_embed_the_word() {
        assert!(!is_serde_rename(r#"#[serde(alias = "renamed_value")]"#));
        assert!(!is_serde_rename(r#"#[serde(rename_all = "camelCase")]"#));
        assert!(!is_serde_rename(r#"#[serde(untagged)]"#));
        assert!(!is_serde_rename(
            r#"#[serde(skip_serializing_if = "rename")]"#
        ));
    }

    #[test]
    fn ignores_non_serde_and_unparsable_attrs() {
        assert!(!is_serde_rename(r#"#[other(rename = "AA")]"#));
        assert!(!is_serde_rename("not an attribute"));
        assert!(!is_serde_rename(""));
    }
}
