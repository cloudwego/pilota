use std::{collections::HashSet, ops::DerefMut, sync::Arc};

use fxhash::FxHashMap;
use itertools::Itertools;
use quote::quote;
use syn::{parse_quote, Attribute};

use crate::{
    db::RirDatabase,
    rir::{EnumVariant, Field, Item},
    symbol::{DefId, EnumRepr},
    ty::{self, Ty, Visitor},
    Context, IdentName,
};

mod serde;

pub use serde::SerdePlugin;

pub trait Plugin {
    fn on_item(&mut self, cx: &mut Context, def_id: DefId, item: Arc<Item>) {
        walk_item(self, cx, def_id, item)
    }

    fn on_field(&mut self, cx: &mut Context, def_id: DefId, f: Arc<Field>) {
        walk_field(self, cx, def_id, f)
    }

    fn on_variant(&mut self, cx: &mut Context, def_id: DefId, variant: Arc<EnumVariant>) {
        walk_variant(self, cx, def_id, variant)
    }

    fn on_emit(&mut self, _cx: &mut Context) {}
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
    fn on_item(&mut self, cx: &mut Context, def_id: DefId, item: Arc<Item>) {
        self.0.on_item(cx, def_id, item)
    }

    fn on_field(&mut self, cx: &mut Context, def_id: DefId, f: Arc<Field>) {
        self.0.on_field(cx, def_id, f)
    }

    fn on_variant(&mut self, cx: &mut Context, def_id: DefId, variant: Arc<EnumVariant>) {
        self.0.on_variant(cx, def_id, variant)
    }

    fn on_emit(&mut self, cx: &mut Context) {
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
    fn on_item(&mut self, cx: &mut Context, def_id: DefId, item: Arc<Item>) {
        (*self).on_item(cx, def_id, item)
    }

    fn on_field(&mut self, cx: &mut Context, def_id: DefId, f: Arc<Field>) {
        (*self).on_field(cx, def_id, f)
    }

    fn on_variant(&mut self, cx: &mut Context, def_id: DefId, variant: Arc<EnumVariant>) {
        (*self).on_variant(cx, def_id, variant)
    }

    fn on_emit(&mut self, cx: &mut Context) {
        (*self).on_emit(cx)
    }
}

#[allow(clippy::single_match)]
pub fn walk_item<P: Plugin + ?Sized>(p: &mut P, cx: &mut Context, _def_id: DefId, item: Arc<Item>) {
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
    _cx: &mut Context,
    _def_id: DefId,
    _field: Arc<Field>,
) {
}

pub fn walk_variant<P: Plugin + ?Sized>(
    _p: &mut P,
    _cx: &mut Context,
    _def_id: DefId,
    _variant: Arc<EnumVariant>,
) {
}

pub struct BoxedPlugin;

impl Plugin for BoxedPlugin {
    fn on_item(&mut self, cx: &mut Context, def_id: DefId, item: Arc<Item>) {
        if let Item::Message(s) = &*item {
            s.fields.iter().for_each(|f| {
                if let ty::Path(p) = &f.ty.kind {
                    if cx.type_graph().is_nested(p.did, def_id) {
                        cx.with_adjust(f.did, |adj| adj.set_boxed())
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
    attrs: Vec<Attribute>,
}

impl<F> AutoDerivePlugin<F>
where
    F: Fn(&Ty) -> PredicateResult,
{
    pub fn new(attrs: Vec<Attribute>, f: F) -> Self {
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
    F: Fn(&Ty) -> PredicateResult,
{
    fn on_item(&mut self, cx: &mut Context, def_id: DefId, item: Arc<Item>) {
        self.can_derive(cx, def_id, &mut HashSet::default(), &mut HashSet::default());
        walk_item(self, cx, def_id, item)
    }

    fn on_emit(&mut self, cx: &mut Context) {
        self.can_derive.iter().for_each(|(def_id, can_derive)| {
            if !matches!(can_derive, CanDerive::No) {
                cx.with_adjust(*def_id, |adj| adj.add_attrs(&self.attrs));
            }
        })
    }
}

impl<T> Plugin for Box<T>
where
    T: Plugin + ?Sized,
{
    fn on_item(&mut self, cx: &mut Context, def_id: DefId, item: Arc<Item>) {
        self.deref_mut().on_item(cx, def_id, item)
    }

    fn on_field(&mut self, cx: &mut Context, def_id: DefId, f: Arc<Field>) {
        self.deref_mut().on_field(cx, def_id, f)
    }

    fn on_emit(&mut self, cx: &mut Context) {
        self.deref_mut().on_emit(cx)
    }
}

pub struct WithAttrsPlugin(pub Vec<syn::Attribute>);

impl Plugin for WithAttrsPlugin {
    fn on_item(&mut self, cx: &mut Context, def_id: DefId, item: Arc<Item>) {
        match &*item {
            Item::Message(_) | Item::Enum(_) | Item::NewType(_) => {
                cx.with_adjust(def_id, |adj| adj.add_attrs(&self.0))
            }
            _ => {}
        }
        walk_item(self, cx, def_id, item)
    }
}

pub struct ImplDefaultPlugin;

impl Plugin for ImplDefaultPlugin {
    fn on_item(&mut self, cx: &mut Context, def_id: DefId, item: Arc<Item>) {
        match &*item {
            Item::Message(m) => {
                let name = m.name.0.as_syn_ident();

                if m.fields.iter().all(|f| f.default.is_none()) {
                    cx.with_adjust(def_id, |adj| {
                        adj.add_attrs(&[parse_quote!(#[derive(Default)])])
                    });
                } else {
                    let fields = m
                        .fields
                        .iter()
                        .map(|f| {
                            let name = cx.rust_name(f.did).as_syn_ident();
                            let default = cx.default_val(f);

                            if let Some(default) = default {
                                quote! { #name: #default.into() }
                            } else {
                                quote! { #name: Default::default() }
                            }
                        })
                        .collect::<Vec<_>>();
                    cx.with_adjust(def_id, |adj| {
                        adj.add_impl(quote! {
                            impl Default for #name {
                                fn default() -> Self {
                                    #name {
                                        #(#fields),*
                                    }
                                }
                            }
                        })
                    });
                };
            }
            Item::NewType(_) => cx.with_adjust(def_id, |adj| {
                adj.add_attrs(&[parse_quote!(#[derive(Default)])])
            }),
            Item::Enum(e) => {
                if !e.variants.is_empty() {
                    cx.with_adjust(def_id, |adj| {
                        adj.add_attrs(&[
                            parse_quote!(#[derive(::pilota::derivative::Derivative)]),
                            parse_quote!(#[derivative(Default)]),
                        ]);
                    });

                    if let Some(v) = e.variants.first() {
                        cx.with_adjust(v.did, |adj| {
                            adj.add_attrs(&[parse_quote!(#[derivative(Default)])]);
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
    fn on_item(&mut self, cx: &mut Context, def_id: DefId, item: Arc<Item>) {
        match &*item {
            Item::Enum(e) if e.repr.is_some() => {
                let name_str = &*cx.rust_name(def_id);
                let name = name_str.as_syn_ident();
                let num_ty = match e.repr {
                    Some(EnumRepr::I32) => quote!(i32),
                    None => return,
                };
                let variants = e
                    .variants
                    .iter()
                    .map(|v| {
                        let variant_name_str = cx.rust_name(v.did);
                        let variant_name = variant_name_str.as_syn_ident();
                        quote!(
                            #variant_name => ::std::result::Result::Ok(#name::#variant_name),
                        )
                    })
                    .collect_vec();

                let nums = e
                    .variants
                    .iter()
                    .map(|v| {
                        let variant_name_str = cx.rust_name(v.did);
                        let variant_name = variant_name_str.as_syn_ident();
                        quote!(const #variant_name: #num_ty = #name::#variant_name as #num_ty;)
                    })
                    .collect_vec();

                cx.with_adjust(def_id, |adj| {
                    adj.add_impl(quote! {
                        impl ::std::convert::From<#name> for #num_ty {
                            fn from(e: #name) -> Self {
                                e as _
                            }
                        }

                        impl ::std::convert::TryFrom<#num_ty> for #name {
                            type Error = ::pilota::EnumConvertError<#num_ty>;

                            #[allow(non_upper_case_globals)]
                            fn try_from(v: i32) -> Result<Self, ::pilota::EnumConvertError<#num_ty>> {
                                #(#nums)*
                                match v {
                                    #(
                                        #variants
                                    )*
                                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(v, #name_str)),
                                }
                            }
                        }
                    })
                });
            }
            _ => {}
        }
        walk_item(self, cx, def_id, item)
    }
}
