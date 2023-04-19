use std::{collections::HashMap, ops::Deref, sync::Arc};

use faststr::FastStr;
use fxhash::FxHashMap;
use pkg_tree::PkgNode;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use traits::CodegenBackend;

use crate::{
    db::RirDatabase,
    middle::{
        self,
        context::tls::CUR_ITEM,
        rir::{self},
    },
    symbol::{DefId, EnumRepr, IdentName},
    tags::EnumMode,
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
    pkgs: FxHashMap<Arc<[FastStr]>, TokenStream>,
}

impl<B> Deref for Codegen<B> {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<B> Codegen<B> {
    pub fn new(cx: Arc<Context>, backend: B) -> Self {
        Codegen {
            zero_copy: false,
            cx,
            backend,
            pkgs: Default::default(),
        }
    }
}

impl<B> Codegen<B>
where
    B: CodegenBackend,
{
    pub fn write_struct(&mut self, def_id: DefId, stream: &mut TokenStream, s: &rir::Message) {
        let name = self.rust_name(def_id).as_syn_ident();

        let fields = s.fields.iter().map(|f| {
            let name = self.rust_name(f.did).as_syn_ident();
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
            tracing::trace!("write item {}", item.symbol_name());
            let adjust = self.adjust(def_id);
            let attrs = adjust.iter().flat_map(|a| a.attrs());

            let impls = adjust.iter().flat_map(|a| &a.impls);
            stream.extend(quote::quote!(
                #(#impls)*
            ));

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

                    let name = self.rust_name(def_id).as_syn_ident();
                    stream.extend(quote::quote! {
                        pub mod #name {
                            #inner
                        }
                    })
                }
            };
        })
    }

    pub fn write_enum_as_new_type(
        &mut self,
        def_id: DefId,
        stream: &mut TokenStream,
        e: &middle::rir::Enum,
    ) {
        let name = self.rust_name(def_id).as_syn_ident();

        let repr = match e.repr {
            Some(EnumRepr::I32) => quote!(i32),
            _ => panic!(),
        };

        let variants = e.variants.iter().map(|v| {
            let name = self.rust_name(v.did).shouty_snake_case().as_syn_ident();

            let discr = v.discr.unwrap();
            let discr = match e.repr {
                Some(EnumRepr::I32) => discr as i32,
                None => panic!(),
            };

            quote::quote! {
                pub const #name: Self = Self(#discr);
            }
        });

        stream.extend(quote::quote! {
            #[derive(Clone, PartialEq, Copy)]
            #[repr(transparent)]
            pub struct #name(#repr);

            impl #name {
                #(#variants)*

                pub fn inner(&self) -> #repr {
                    self.0
                }
            }

            impl ::std::convert::From<#repr> for #name {
                fn from(value: #repr) -> Self {
                    Self(value)
                }
            }
        });

        self.backend.codegen_enum_impl(def_id, stream, e);
    }

    pub fn write_enum(&mut self, def_id: DefId, stream: &mut TokenStream, e: &middle::rir::Enum) {
        if self
            .node_tags(def_id)
            .unwrap()
            .get::<EnumMode>()
            .filter(|s| **s == EnumMode::NewType)
            .is_some()
        {
            return self.write_enum_as_new_type(def_id, stream, e);
        }
        let name = self.rust_name(def_id).as_syn_ident();

        let mut repr = if e.variants.is_empty() {
            quote! {}
        } else {
            match e.repr {
                Some(EnumRepr::I32) => quote! {
                   #[repr(i32)]
                },
                None => quote! {},
            }
        };

        if e.repr.is_some() {
            repr.extend(quote! { #[derive(Copy)] })
        }

        let variants = e.variants.iter().map(|v| {
            let name = self.rust_name(v.did).as_syn_ident();

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
        let name = self.rust_name(def_id).as_syn_ident();
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
        let name = self.rust_name(def_id).as_syn_ident();
        let ty = self.codegen_item_ty(t.ty.kind.clone());
        stream.extend(quote::quote! {
            #[derive(Clone, PartialEq)]
            pub struct #name(pub #ty);

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

    pub fn write_const(&mut self, did: DefId, stream: &mut TokenStream, c: &middle::rir::Const) {
        let mut ty = self.codegen_ty(did);

        let name = self.rust_name(did);

        stream.extend(self.cx.def_lit(&name, &c.lit, &mut ty))
    }

    pub(crate) fn write_mods(&mut self, mods: HashMap<Arc<[FastStr]>, Vec<DefId>>) {
        mods.iter().for_each(|(p, def_ids)| {
            let stream: &mut TokenStream =
                unsafe { std::mem::transmute(self.pkgs.entry(p.clone()).or_default()) };

            let span = tracing::span!(tracing::Level::TRACE, "write_mod", path = ?p);

            let _enter = span.enter();
            for def_id in def_ids.iter() {
                self.write_item(stream, *def_id)
            }
        })
    }

    pub fn link(mut self, ns_name: &str) -> TokenStream {
        fn write_stream(
            pkgs: &mut FxHashMap<Arc<[FastStr]>, TokenStream>,
            stream: &mut TokenStream,
            nodes: &[PkgNode],
        ) {
            for node in nodes {
                let mut inner_stream = TokenStream::default();
                if let Some(node_stream) = pkgs.remove(&node.path) {
                    inner_stream.extend(node_stream);
                }

                write_stream(pkgs, &mut inner_stream, &node.children);
                let name = node.ident();
                if name.clone().unwrap_or_default() == "" {
                    stream.extend(inner_stream);
                    return;
                }

                let name = name.unwrap().as_syn_ident();
                stream.extend(quote! {
                    pub mod #name {
                        #inner_stream
                    }
                });
            }
        }
        let mut stream = TokenStream::default();
        let pkg_node = PkgNode::from_pkgs(&self.pkgs.keys().map(|k| &**k).collect::<Vec<_>>());

        write_stream(&mut self.pkgs, &mut stream, &pkg_node);

        let ns_name = format_ident!("{}", ns_name);

        quote! {
            pub mod #ns_name {
                #![allow(warnings, clippy::all)]
                #stream
            }
        }
    }
}
