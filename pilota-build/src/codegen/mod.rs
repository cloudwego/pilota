use std::{collections::HashMap, ops::Deref, sync::Arc};

use fxhash::{FxHashMap, FxHashSet};
use heck::ToShoutySnakeCase;
use itertools::Itertools;
use pkg_tree::PkgNode;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use traits::CodegenBackend;

use crate::{
    db::RirDatabase,
    middle::{
        self,
        context::tls::CUR_ITEM,
        rir::{self, ItemPath, Literal},
        ty::{AdtDef, AdtKind, CodegenTy},
    },
    symbol::{DefId, EnumRepr, IdentName},
    ty::Visitor,
    Context,
};

pub(crate) mod pkg_tree;
pub(crate) mod traits;

pub mod protobuf;
pub mod thrift;

pub struct Codegen<B> {
    backend: B,
    zero_copy: bool,
    cx: Arc<Context>,
    pkgs: FxHashMap<ItemPath, TokenStream>,
    input: Vec<DefId>,
}

impl<B> Deref for Codegen<B> {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<B> Codegen<B> {
    pub fn new(cx: Arc<Context>, backend: B, input: Vec<DefId>) -> Self {
        Codegen {
            zero_copy: false,
            cx,
            backend,
            pkgs: Default::default(),
            input,
        }
    }
}

impl<B> Codegen<B>
where
    B: CodegenBackend,
{
    pub fn write_struct(&mut self, def_id: DefId, stream: &mut TokenStream, s: &rir::Message) {
        let name = format_ident!("{}", (&**s.name).struct_ident());

        let fields = s.fields.iter().map(|f| {
            let name = format_ident!("{}", &f.name.to_snake_case());
            let adjust = self.adjust(f.did);
            let ty = self.codegen_item_ty(f.ty.kind.clone());
            let mut ty = quote::quote! { #ty };

            if let Some(adjust) = adjust {
                if adjust.boxed() {
                    ty = quote::quote! { ::std::boxed::Box<#ty> }
                }
            }

            if f.is_optional() {
                ty = quote::quote! { ::std::option::Option<#ty> }
            }

            let attrs = adjust.iter().flat_map(|a| a.attrs());

            quote::quote! {
                #(#attrs)*
                pub #name: #ty,
            }
        });

        let lifetime = self.zero_copy.then(|| quote!(<'de>)).into_iter();

        stream.extend(quote::quote! {
            #[derive(Clone, PartialEq)]
            pub struct #name #(#lifetime)* {
                #(#fields)*
            }
        });

        self.backend.codegen_struct_impl(def_id, stream, s);
    }

    pub fn write_item(&mut self, stream: &mut TokenStream, def_id: DefId) {
        CUR_ITEM.set(&def_id, || {
            let item = self.item(def_id).unwrap();
            let adjust = self.adjust(def_id);
            let attrs = adjust.iter().flat_map(|a| a.attrs());

            stream.extend(quote::quote! {
                #(#attrs)*
            });

            match &*item {
                middle::rir::Item::Message(s) => self.write_struct(def_id, stream, s),
                middle::rir::Item::Enum(e) => self.write_enum(def_id, stream, e),
                middle::rir::Item::Service(s) => self.write_service(def_id, stream, s),
                middle::rir::Item::NewType(t) => self.write_new_type(def_id, stream, t),
                middle::rir::Item::Const(c) => self.write_const(def_id, stream, c),
                middle::rir::Item::Mod(m) => {
                    let mut inner = TokenStream::default();
                    m.items
                        .iter()
                        .for_each(|def_id| self.write_item(&mut inner, *def_id));

                    let name = format_ident!("{}", m.name.to_snake_case());
                    stream.extend(quote::quote! {
                        pub mod #name {
                            #inner
                        }
                    })
                }
            };
        })
    }

    pub fn write_enum(&mut self, def_id: DefId, stream: &mut TokenStream, e: &middle::rir::Enum) {
        let name = format_ident!("{}", (&**e.name).struct_ident());

        let mut repr = match e.repr {
            Some(EnumRepr::I32) => quote! {
               #[repr(i32)]
            },
            None => quote! {},
        };

        if e.repr.is_some() {
            repr.extend(quote! { #[derive(Copy)] })
        }

        let variants = e.variants.iter().map(|v| {
            let name = format_ident!("{}", (&**v.name).variant_ident());

            let adjust = self.adjust(v.did);
            let attrs = adjust.iter().flat_map(|a| a.attrs());
            let fields = v
                .fields
                .iter()
                .map(|ty| self.codegen_item_ty(ty.kind.clone()))
                .collect::<Vec<_>>();

            let fields_stream = if fields.is_empty() {
                TokenStream::default()
            } else {
                quote::quote! {
                    (#(#fields),*)
                }
            };

            let discr = v.discr.map(|x| {
                let x = isize::try_from(x).unwrap();
                let x = match e.repr {
                    Some(EnumRepr::I32) => x as i32,
                    None => panic!(),
                };
                quote! { = #x }
            });

            quote::quote! {
                #(#attrs)*
                #name #fields_stream #discr,
            }
        });

        stream.extend(quote::quote! {
            #[derive(Clone, PartialEq)]
            #repr
            pub enum #name {
                #(#variants)*
            }
        });

        self.backend.codegen_enum_impl(def_id, stream, e);
    }

    pub fn write_service(
        &mut self,
        def_id: DefId,
        stream: &mut TokenStream,
        s: &middle::rir::Service,
    ) {
        let name = format_ident!("{}", s.name.to_upper_camel_case());
        let methods = self.service_methods(def_id);

        let methods = methods
            .iter()
            .map(|m| self.backend.codegen_service_method(def_id, m));

        stream.extend(quote::quote! {
            #[::async_trait::async_trait]
            pub trait #name {
                #(#methods)*
            }
        });
        self.backend.codegen_service_impl(def_id, stream, s);
    }

    pub fn write_new_type(
        &mut self,
        def_id: DefId,
        stream: &mut TokenStream,
        t: &middle::rir::NewType,
    ) {
        let name = format_ident!("{}", &t.name.to_upper_camel_case());
        let ty = self.codegen_item_ty(t.ty.kind.clone());
        stream.extend(quote::quote! {
            #[derive(Clone, PartialEq)]
            pub struct #name(#ty);

            impl ::std::ops::Deref for #name {
                type Target = #ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl From<#ty> for #name {
                fn from(v: #ty) -> Self {
                    Self(v)
                }
            }
        });
        self.backend.codegen_newtype_impl(def_id, stream, t);
    }

    fn def_lit(&mut self, name: &str, lit: &Literal, ty: &CodegenTy) -> TokenStream {
        let should_lazy_static = ty.should_lazy_static();
        let name = format_ident!("{}", name.to_shouty_snake_case());
        if should_lazy_static {
            let lit = self.lit_as_rvalue(lit, ty);
            quote::quote! {
                ::pilota::lazy_static::lazy_static! {
                    pub static ref #name: #ty = #lit;
                }
            }
        } else {
            let lit = self.lit_into_ty(lit, ty);
            quote::quote! {
                pub const #name: #ty = #lit;
            }
        }
    }

    pub fn write_const(&mut self, did: DefId, stream: &mut TokenStream, c: &middle::rir::Const) {
        let ty = self.codegen_ty(did);

        stream.extend(self.def_lit(&c.name, &c.lit, &ty))
    }

    fn ident_into_ty(
        &mut self,
        did: DefId,
        ident_ty: &CodegenTy,
        target: &CodegenTy,
    ) -> TokenStream {
        if ident_ty == target {
            let stream = self.cur_related_item_path(did);
            return quote! { #stream };
        }
        panic!("invalid convert {:?} to {:?}", ident_ty, target)
    }

    fn lit_as_rvalue(&mut self, lit: &Literal, ty: &CodegenTy) -> TokenStream {
        match (lit, ty) {
            (Literal::Map(m), CodegenTy::LazyStaticRef(map)) => match &**map {
                CodegenTy::Map(k_ty, v_ty) => {
                    let k_ty = &**k_ty;
                    let v_ty = &**v_ty;
                    let len = m.len();
                    let kvs = m.iter().map(|(k, v)| {
                        let k = self.lit_into_ty(k, k_ty);
                        let v = self.lit_into_ty(v, v_ty);
                        quote! {
                            map.insert(#k, #v);
                        }
                    });
                    let stream = quote::quote! {
                        {
                            let mut map = ::std::collections::HashMap::with_capacity(#len);
                            #(#kvs)*
                            map
                        }
                    };
                    stream
                }
                _ => panic!("invalid map type {:?}", map),
            },
            (Literal::Map(_), _) => panic!(),
            _ => self.lit_into_ty(lit, ty),
        }
    }

    fn lit_into_ty(&mut self, lit: &Literal, ty: &CodegenTy) -> TokenStream {
        match (lit, ty) {
            (Literal::Path(p), ty) => {
                let ident_ty = self.codegen_ty(p.did);

                self.ident_into_ty(p.did, &ident_ty, ty)
            }
            (Literal::String(s), CodegenTy::Str) => {
                let s = &**s;
                quote! { #s }
            }
            (Literal::String(s), CodegenTy::String) => {
                let s = &**s;
                quote! { #s.to_string() }
            }
            (Literal::Int(i), CodegenTy::I16) => {
                let i = *i as i16;
                quote! { #i }
            }
            (Literal::Int(i), CodegenTy::I32) => {
                let i = *i as i32;
                quote! { #i }
            }
            (Literal::Int(i), CodegenTy::I64) => {
                let i = *i as i64;
                quote! { #i }
            }
            (Literal::Float(f), CodegenTy::F64) => {
                let f = f.parse::<f64>().unwrap();
                quote! { #f }
            }
            (
                l,
                CodegenTy::Adt(AdtDef {
                    kind: AdtKind::NewType(inner_ty),
                    did,
                }),
            ) => {
                let ident = self.cur_related_item_path(*did);
                let stream = self.lit_into_ty(l, inner_ty);
                quote! { #ident(#stream) }
            }
            // Literal::List(_) => todo!(),
            (Literal::Map(_), CodegenTy::StaticRef(map)) => match &**map {
                CodegenTy::Map(_, _) => {
                    let lazy_map =
                        self.def_lit("inner_map", lit, &CodegenTy::LazyStaticRef(map.clone()));
                    let stream = quote::quote! {
                        {
                            #lazy_map
                            &*inner_map
                        }
                    };
                    stream
                }
                _ => panic!("invalid map type {:?}", map),
            },
            _ => panic!("unexpected literal {:?} with ty {:?}", lit, ty),
        }
    }

    fn collect_items(&self) -> FxHashSet<DefId> {
        struct PathCollector<'a> {
            set: &'a mut FxHashSet<DefId>,
            cx: &'a Context,
        }

        impl super::ty::Visitor for PathCollector<'_> {
            fn visit_path(&mut self, path: &crate::rir::Path) {
                collect(self.cx, path.did, self.set)
            }
        }

        fn collect(cx: &Context, def_id: DefId, set: &mut FxHashSet<DefId>) {
            if set.contains(&def_id) {
                return;
            }
            if !matches!(&*cx.item(def_id).unwrap(), rir::Item::Mod(_)) {
                set.insert(def_id);
            }

            let node = cx.node(def_id).unwrap();

            node.related_nodes
                .iter()
                .for_each(|def_id| collect(cx, *def_id, set));

            let item = node.expect_item();

            match item {
                rir::Item::Message(m) => m
                    .fields
                    .iter()
                    .for_each(|f| PathCollector { cx, set }.visit(&f.ty)),
                rir::Item::Enum(e) => e
                    .variants
                    .iter()
                    .flat_map(|v| &v.fields)
                    .for_each(|ty| PathCollector { cx, set }.visit(ty)),
                rir::Item::Service(s) => {
                    s.extend.iter().for_each(|p| collect(cx, p.did, set));
                    s.methods
                        .iter()
                        .flat_map(|m| m.args.iter().map(|f| &f.ty).chain(std::iter::once(&m.ret)))
                        .for_each(|ty| PathCollector { cx, set }.visit(ty));
                }
                rir::Item::NewType(n) => PathCollector { cx, set }.visit(&n.ty),
                rir::Item::Const(c) => {
                    PathCollector { cx, set }.visit(&c.ty);
                }
                rir::Item::Mod(m) => m.items.iter().for_each(|m| {
                    let item = cx.item(*m).unwrap();
                    if matches!(&*item, rir::Item::Mod(_) | rir::Item::Const(_)) {
                        collect(cx, *m, set)
                    }
                }),
            }
        }
        let mut set = FxHashSet::default();

        self.input.iter().for_each(|def_id| {
            collect(&self.cx, *def_id, &mut set);
        });

        set
    }

    fn collect_pkgs(&mut self, remove_unused: bool) -> HashMap<ItemPath, Vec<DefId>> {
        if remove_unused {
            let def_ids = self.collect_items();
            def_ids
                .into_iter()
                .into_group_map_by(|def_id| self.cx.mod_path(*def_id))
        } else {
            let files = self.files();
            let mut map: HashMap<_, Vec<DefId>> = HashMap::with_capacity(files.len());
            for file in files.values() {
                map.entry(file.package.clone())
                    .or_default()
                    .extend_from_slice(&file.items);
            }

            map
        }
    }

    pub fn write_pkgs(&mut self, remove_unused: bool) {
        let mods = self.collect_pkgs(remove_unused);

        mods.iter().for_each(|(p, def_ids)| {
            let stream: &mut TokenStream =
                unsafe { std::mem::transmute(self.pkgs.entry(p.clone()).or_default()) };

            for def_id in def_ids.iter() {
                self.write_item(stream, *def_id)
            }
        })
    }

    pub fn link(mut self, ns_name: &str) -> TokenStream {
        fn write_stream(
            pkgs: &mut FxHashMap<ItemPath, TokenStream>,
            stream: &mut TokenStream,
            nodes: &[PkgNode],
        ) {
            for node in nodes {
                let name = format_ident!("{}", node.ident());
                let mut inner_stream = TokenStream::default();
                if let Some(node_stream) = pkgs.remove(&node.path) {
                    inner_stream.extend(node_stream);
                }

                write_stream(pkgs, &mut inner_stream, &node.children);

                stream.extend(quote! {
                    pub mod #name {
                        #inner_stream
                    }
                });
            }
        }
        let mut stream = TokenStream::default();
        let pkg_node = PkgNode::from_pkgs(&self.pkgs.keys().cloned().collect::<Vec<_>>());

        write_stream(&mut self.pkgs, &mut stream, &pkg_node);

        let ns_name = format_ident!("{}", ns_name);

        quote! {
            pub mod #ns_name {
                #![allow(unused_variables, dead_code, missing_docs, clippy::unused_unit, clippy::needless_borrow, unused_mut)]
                #stream
            }
        }
    }
}
