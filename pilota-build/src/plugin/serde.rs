use syn::{Meta, Token, parse::Parser, punctuated::Punctuated};

use crate::{Plugin, tags::SerdeAttribute};

#[derive(Clone, Copy, Default)]
pub struct SerdePlugin;

#[derive(Clone, Copy)]
pub struct ConfiguredSerdePlugin {
    preserve_idl_field_names: bool,
}

impl SerdePlugin {
    pub const fn new() -> Self {
        Self
    }

    /// Preserve the original IDL field names in the generated serde
    /// attributes instead of pilota's Rust-cased identifiers.
    ///
    /// When enabled, the emitted `#[serde(rename = "...")]` attributes use
    /// each field's name exactly as written in the IDL rather than the
    /// generated Rust names. Disabled by default.
    pub const fn preserve_idl_field_names(self, enable: bool) -> ConfiguredSerdePlugin {
        ConfiguredSerdePlugin {
            preserve_idl_field_names: enable,
        }
    }
}

/// Which directions the serde attributes in `s` already rename, as
/// `(serialize, deserialize)`.
///
/// `rename = "x"` renames both, while `rename(serialize = "x")` renames only
/// one and leaves the other still spelled as the Rust ident.
fn serde_renames(s: &str) -> (bool, bool) {
    let Ok(attrs) = syn::Attribute::parse_outer.parse_str(s) else {
        return (false, false);
    };
    let renames = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("serde"))
        .filter_map(|attr| {
            attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .ok()
        })
        .flatten()
        .filter(|meta| meta.path().is_ident("rename"));

    let (mut ser, mut de) = (false, false);
    for rename in renames {
        match rename {
            Meta::NameValue(_) => return (true, true),
            Meta::List(list) => {
                if let Ok(dirs) =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                {
                    ser |= dirs.iter().any(|d| d.path().is_ident("serialize"));
                    de |= dirs.iter().any(|d| d.path().is_ident("deserialize"));
                }
            }
            Meta::Path(_) => {}
        }
    }
    (ser, de)
}

/// Emits the `#[serde(rename = ..)]` that keeps `name`, the IDL spelling, for
/// whichever directions `idl_attr` has not already renamed itself.
fn preserve_idl_name(
    cx: &crate::Context,
    def_id: crate::DefId,
    name: &str,
    idl_attr: Option<&str>,
) {
    // A `rename` from the IDL is the more specific one, so it wins over the one
    // we would derive from the name.
    let attr = match idl_attr.map(serde_renames).unwrap_or((false, false)) {
        (true, true) => return,
        (false, false) => format!("#[serde(rename = {name:?})]"),
        (true, false) => format!("#[serde(rename(deserialize = {name:?}))]"),
        (false, true) => format!("#[serde(rename(serialize = {name:?}))]"),
    };
    cx.with_adjust_mut(def_id, |adj| adj.add_attrs(&[attr.into()]));
}

/// Plain `SerdePlugin` is just the configured one with every option off.
impl Plugin for SerdePlugin {
    fn on_item(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        item: std::sync::Arc<crate::rir::Item>,
    ) {
        self.preserve_idl_field_names(false)
            .on_item(cx, def_id, item)
    }

    fn on_field(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        f: std::sync::Arc<crate::rir::Field>,
    ) {
        self.preserve_idl_field_names(false).on_field(cx, def_id, f)
    }

    fn on_variant(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        variant: std::sync::Arc<crate::rir::EnumVariant>,
    ) {
        self.preserve_idl_field_names(false)
            .on_variant(cx, def_id, variant)
    }
}

impl Plugin for ConfiguredSerdePlugin {
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

        if self.preserve_idl_field_names {
            preserve_idl_name(cx, def_id, &f.name.raw_str(), idl_attr.as_deref());
        }
    }

    fn on_variant(
        &mut self,
        cx: &crate::Context,
        def_id: crate::DefId,
        variant: std::sync::Arc<crate::rir::EnumVariant>,
    ) {
        let idl_attr = cx
            .node_tags(variant.did)
            .and_then(|tags| tags.get::<SerdeAttribute>().cloned())
            .map(|attribute| attribute.0.replace('\\', ""));

        if let Some(attr) = idl_attr.clone() {
            cx.with_adjust_mut(def_id, |adj| adj.add_attrs(&[attr.into()]))
        }

        if self.preserve_idl_field_names {
            preserve_idl_name(cx, def_id, &variant.name.raw_str(), idl_attr.as_deref());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::serde_renames;

    #[test]
    fn detects_rename_of_both_directions() {
        assert_eq!(serde_renames(r#"#[serde(rename = "AA")]"#), (true, true));
        assert_eq!(
            serde_renames(r#"#[serde(default, rename = "AA")]"#),
            (true, true)
        );
        assert_eq!(
            serde_renames(r#"#[serde(rename(serialize = "a", deserialize = "b"))]"#),
            (true, true)
        );
    }

    #[test]
    fn detects_rename_of_one_direction() {
        assert_eq!(
            serde_renames(r#"#[serde(rename(serialize = "a"))]"#),
            (true, false)
        );
        assert_eq!(
            serde_renames(r#"#[serde(rename(deserialize = "b"))]"#),
            (false, true)
        );
    }

    #[test]
    fn ignores_attrs_that_merely_embed_the_word() {
        for s in [
            r#"#[serde(alias = "renamed_value")]"#,
            r#"#[serde(rename_all = "camelCase")]"#,
            r#"#[serde(untagged)]"#,
            r#"#[serde(skip_serializing_if = "rename")]"#,
        ] {
            assert_eq!(serde_renames(s), (false, false), "{s}");
        }
    }

    #[test]
    fn ignores_non_serde_and_unparsable_attrs() {
        for s in [r#"#[other(rename = "AA")]"#, "not an attribute", ""] {
            assert_eq!(serde_renames(s), (false, false), "{s}");
        }
    }
}
