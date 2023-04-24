use crate::tags::{EnumMode, SerdeAttribute};

#[derive(Clone, Copy)]
pub struct SerdePlugin;

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

                if cx.node_tags(def_id).unwrap().get::<EnumMode>().copied()
                    == Some(EnumMode::NewType)
                {
                    cx.with_adjust_mut(def_id, |adj| {
                        adj.add_attrs(&["#[serde(transparent)]".into()]);
                    })
                }
            }
            _ => {}
        };
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
