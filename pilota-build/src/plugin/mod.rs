use std::{collections::HashSet, ops::DerefMut, sync::Arc};

use faststr::FastStr;
use fxhash::FxHashMap;
use itertools::Itertools;
use quote::quote;

use crate::{
    db::RirDatabase,
    rir::{EnumVariant, Field, Item},
    symbol::{DefId, EnumRepr},
    tags::EnumMode,
    ty::{self, Ty, Visitor},
    Context,
};

mod serde;

pub use self::serde::SerdePlugin;

pub trait Plugin: Sync + Send {
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        walk_item(self, cx, def_id, item)
    }

    fn on_field(&mut self, cx: &Context, def_id: DefId, f: Arc<Field>) {
        walk_field(self, cx, def_id, f)
    }

    fn on_variant(&mut self, cx: &Context, def_id: DefId, variant: Arc<EnumVariant>) {
        walk_variant(self, cx, def_id, variant)
    }

    fn on_emit(&mut self, _cx: &Context) {}
}

pub trait ClonePlugin: Plugin {
    fn clone_box(&self) -> Box<dyn ClonePlugin>;
}

pub struct BoxClonePlugin(Box<dyn ClonePlugin>);

impl BoxClonePlugin {
    pub fn new<P: ClonePlugin + 'static>(p: P) -> Self {
        Self(Box::new(p))
    }
}

impl Clone for BoxClonePlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

impl Plugin for BoxClonePlugin {
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        self.0.on_item(cx, def_id, item)
    }

    fn on_field(&mut self, cx: &Context, def_id: DefId, f: Arc<Field>) {
        self.0.on_field(cx, def_id, f)
    }

    fn on_variant(&mut self, cx: &Context, def_id: DefId, variant: Arc<EnumVariant>) {
        self.0.on_variant(cx, def_id, variant)
    }

    fn on_emit(&mut self, cx: &Context) {
        self.0.on_emit(cx)
    }
}

impl<T> ClonePlugin for T
where
    T: Plugin + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn ClonePlugin> {
        Box::new(self.clone())
    }
}

impl<T> Plugin for &mut T
where
    T: Plugin,
{
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        (*self).on_item(cx, def_id, item)
    }

    fn on_field(&mut self, cx: &Context, def_id: DefId, f: Arc<Field>) {
        (*self).on_field(cx, def_id, f)
    }

    fn on_variant(&mut self, cx: &Context, def_id: DefId, variant: Arc<EnumVariant>) {
        (*self).on_variant(cx, def_id, variant)
    }

    fn on_emit(&mut self, cx: &Context) {
        (*self).on_emit(cx)
    }
}

#[allow(clippy::single_match)]
pub fn walk_item<P: Plugin + ?Sized>(p: &mut P, cx: &Context, _def_id: DefId, item: Arc<Item>) {
    match &*item {
        Item::Message(s) => s
            .fields
            .iter()
            .for_each(|f| p.on_field(cx, f.did, f.clone())),
        Item::Enum(e) => e
            .variants
            .iter()
            .for_each(|v| p.on_variant(cx, v.did, v.clone())),
        _ => {}
    }
}

pub fn walk_field<P: Plugin + ?Sized>(
    _p: &mut P,
    _cx: &Context,
    _def_id: DefId,
    _field: Arc<Field>,
) {
}

pub fn walk_variant<P: Plugin + ?Sized>(
    _p: &mut P,
    _cx: &Context,
    _def_id: DefId,
    _variant: Arc<EnumVariant>,
) {
}

pub struct BoxedPlugin;

impl Plugin for BoxedPlugin {
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        if let Item::Message(s) = &*item {
            s.fields.iter().for_each(|f| {
                if let ty::Path(p) = &f.ty.kind {
                    if cx.type_graph().is_nested(p.did, def_id) {
                        cx.with_adjust_mut(f.did, |adj| adj.set_boxed())
                    }
                }
            })
        }
        walk_item(self, cx, def_id, item)
    }
}

pub struct AutoDerivePlugin<F> {
    can_derive: FxHashMap<DefId, CanDerive>,
    predicate: F,
    attrs: Arc<[FastStr]>,
}

impl<F> AutoDerivePlugin<F>
where
    F: Fn(&Ty) -> PredicateResult,
{
    pub fn new(attrs: Arc<[FastStr]>, f: F) -> Self {
        Self {
            can_derive: FxHashMap::default(),
            predicate: f,
            attrs,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CanDerive {
    Yes,
    No,
    Delay, // delay to next pass
}

pub enum PredicateResult {
    No,   // can not derive,
    GoOn, // can derive, but need more pass
}

#[derive(Default)]
pub struct PathCollector {
    paths: Vec<crate::rir::Path>,
}

impl super::ty::Visitor for PathCollector {
    fn visit_path(&mut self, path: &crate::rir::Path) {
        self.paths.push(path.clone())
    }
}

impl<F> AutoDerivePlugin<F>
where
    F: Fn(&Ty) -> PredicateResult,
{
    fn can_derive(
        &mut self,
        cx: &Context,
        def_id: DefId,
        visiting: &mut HashSet<DefId>,
        delayed: &mut HashSet<DefId>,
    ) -> CanDerive {
        if let Some(b) = self.can_derive.get(&def_id) {
            return *b;
        }
        if visiting.contains(&def_id) {
            return CanDerive::Delay;
        }
        visiting.insert(def_id);
        let item = cx.expect_item(def_id);
        let deps = match &*item {
            Item::Message(s) => s.fields.iter().map(|f| &f.ty).collect::<Vec<_>>(),
            Item::Enum(e) => e
                .variants
                .iter()
                .flat_map(|v| &v.fields)
                .collect::<Vec<_>>(),
            Item::Service(_) => return CanDerive::No,
            Item::NewType(t) => vec![&t.ty],
            Item::Const(_) => return CanDerive::No,
            Item::Mod(_) => return CanDerive::No,
        };

        let can_derive = if deps
            .iter()
            .any(|t| matches!((self.predicate)(t), PredicateResult::No))
        {
            CanDerive::No
        } else {
            let paths = deps.iter().flat_map(|t| {
                let mut visitor = PathCollector::default();
                visitor.visit(t);
                visitor.paths
            });
            let paths_can_derive = paths
                .map(|p| (p.did, self.can_derive(cx, p.did, visiting, delayed)))
                .collect::<Vec<_>>();

            let delayed_count = paths_can_derive
                .iter()
                .filter(|(_, p)| *p == CanDerive::Delay)
                .count();

            if paths_can_derive.iter().any(|(_, p)| *p == CanDerive::No) {
                delayed.iter().for_each(|def_id| {
                    self.can_derive.insert(*def_id, CanDerive::No);
                });

                CanDerive::No
            } else if delayed_count > 0 {
                delayed.insert(def_id);
                CanDerive::Delay
            } else {
                CanDerive::Yes
            }
        };

        self.can_derive.insert(def_id, can_derive);
        visiting.remove(&def_id);

        can_derive
    }
}

impl<F> Plugin for AutoDerivePlugin<F>
where
    F: Fn(&Ty) -> PredicateResult + Send + Sync,
{
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        self.can_derive(cx, def_id, &mut HashSet::default(), &mut HashSet::default());
        walk_item(self, cx, def_id, item)
    }

    fn on_emit(&mut self, cx: &Context) {
        self.can_derive.iter().for_each(|(def_id, can_derive)| {
            if !matches!(can_derive, CanDerive::No) {
                cx.with_adjust_mut(*def_id, |adj| adj.add_attrs(&self.attrs));
            }
        })
    }
}

impl<T> Plugin for Box<T>
where
    T: Plugin + ?Sized,
{
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        self.deref_mut().on_item(cx, def_id, item)
    }

    fn on_field(&mut self, cx: &Context, def_id: DefId, f: Arc<Field>) {
        self.deref_mut().on_field(cx, def_id, f)
    }

    fn on_emit(&mut self, cx: &Context) {
        self.deref_mut().on_emit(cx)
    }
}

pub struct WithAttrsPlugin(pub Arc<[FastStr]>);

impl Plugin for WithAttrsPlugin {
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        match &*item {
            Item::Message(_) | Item::Enum(_) | Item::NewType(_) => {
                cx.with_adjust_mut(def_id, |adj| adj.add_attrs(&self.0))
            }
            _ => {}
        }
        walk_item(self, cx, def_id, item)
    }
}

pub struct ImplDefaultPlugin;

impl Plugin for ImplDefaultPlugin {
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        match &*item {
            Item::Message(m) => {
                let name = cx.rust_name(def_id);

                if m.fields.iter().all(|f| cx.default_val(f).is_none()) {
                    cx.with_adjust_mut(def_id, |adj| adj.add_attrs(&["#[derive(Default)]".into()]));
                } else {
                    let fields = m
                        .fields
                        .iter()
                        .map(|f| {
                            let name = cx.rust_name(f.did);
                            let default = cx.default_val(f).map(|v| v.0);

                            if let Some(default) = default {
                                let mut val = default;
                                if f.is_optional() {
                                    val = format!("Some({val})").into()
                                }
                                format!("{name}: {val}")
                            } else {
                                format!("{name}: Default::default()")
                            }
                        })
                        .join(",\n");

                    cx.with_adjust_mut(def_id, |adj| {
                        adj.add_nested_item(
                            format!(
                                r#"
                                impl Default for {name} {{
                                    fn default() -> Self {{
                                        {name} {{
                                            {fields}
                                        }}
                                    }}
                                }}
                            "#
                            )
                            .into(),
                        )
                    });
                };
            }
            Item::NewType(_) => {
                cx.with_adjust_mut(def_id, |adj| adj.add_attrs(&["#[derive(Default)]".into()]))
            }
            Item::Enum(e) => {
                if !e.variants.is_empty() {
                    cx.with_adjust_mut(def_id, |adj| {
                        adj.add_attrs(&[
                            "#[derive(::pilota::derivative::Derivative)]".into(),
                            "#[derivative(Default)]".into(),
                        ]);
                    });

                    if let Some(v) = e.variants.first() {
                        cx.with_adjust_mut(v.did, |adj| {
                            adj.add_attrs(&["#[derivative(Default)]".into()]);
                        })
                    }
                }
            }
            _ => {}
        }
        walk_item(self, cx, def_id, item)
    }
}

pub struct EnumNumPlugin;

impl Plugin for EnumNumPlugin {
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        match &*item {
            Item::Enum(e)
                if e.repr.is_some()
                    && cx
                        .node_tags(def_id)
                        .unwrap()
                        .get::<EnumMode>()
                        .copied()
                        .unwrap_or(EnumMode::Enum)
                        == EnumMode::Enum =>
            {
                let name_str = &*cx.rust_name(def_id);
                let name = name_str;
                let num_ty = match e.repr {
                    Some(EnumRepr::I32) => quote!(i32),
                    None => return,
                };
                let variants = e
                    .variants
                    .iter()
                    .map(|v| {
                        let variant_name_str = cx.rust_name(v.did);
                        let variant_name = variant_name_str;
                        format!(
                            "{variant_name} => ::std::result::Result::Ok({name}::{variant_name}), \n"
                        )
                    })
                    .join("");

                let nums = e
                    .variants
                    .iter()
                    .map(|v| {
                        let variant_name_str = cx.rust_name(v.did);
                        let variant_name = variant_name_str;
                        format!(
                            "const {variant_name}: {num_ty} = {name}::{variant_name} as {num_ty};"
                        )
                    })
                    .join("\n");

                cx.with_adjust_mut(def_id, |adj| {
                    adj.add_nested_item(format!(r#"
                        impl ::std::convert::From<{name}> for {num_ty} {{
                            fn from(e: {name}) -> Self {{
                                e as _
                            }}
                        }}

                        impl ::std::convert::TryFrom<{num_ty}> for {name} {{
                            type Error = ::pilota::EnumConvertError<{num_ty}>;

                            #[allow(non_upper_case_globals)]
                            fn try_from(v: i32) -> ::std::result::Result<Self, ::pilota::EnumConvertError<{num_ty}>> {{
                                {nums}
                                match v {{
                                    {variants}
                                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(v, "{name_str}")),
                                }}
                            }}
                        }}"#).into(),
                    )
                });
            }
            _ => {}
        }
        walk_item(self, cx, def_id, item)
    }
}
