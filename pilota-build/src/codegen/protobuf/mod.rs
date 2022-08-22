use std::sync::Arc;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;

use crate::{
    db::RirDatabase,
    middle::ty::{self, Ty},
    rir::{self, Field, FieldKind},
    tags::protobuf::{Fixed32, Fixed64, OneOf, SFixed32, SFixed64, SInt32, SInt64},
    CodegenBackend, Context,
};

pub struct ProtobufBackend {
    _cx: Arc<Context>,
    #[allow(dead_code)]
    zero_copy: bool,
}

impl ProtobufBackend {
    pub(crate) fn new(cx: Arc<Context>) -> Self {
        ProtobufBackend {
            _cx: cx,
            zero_copy: false,
        }
    }
}

impl CodegenBackend for ProtobufBackend {}

pub struct ProstPlugin;

impl ProstPlugin {
    fn mk_ty_attr(&self, cx: &Context, ty: &Ty) -> TokenStream {
        match &ty.kind {
            ty::String => quote!(string),
            ty::Bool => quote!(bool),
            ty::Bytes => quote!(bytes),
            ty::I32 if cx.contains_tag::<SInt32>(ty.tags_id) => quote!(sint32),
            ty::I64 if cx.contains_tag::<SInt64>(ty.tags_id) => quote!(sint64),

            ty::I32 if cx.contains_tag::<SFixed32>(ty.tags_id) => quote!(sfixed32),
            ty::I64 if cx.contains_tag::<SFixed64>(ty.tags_id) => quote!(sfixed64),

            ty::I32 => quote!(int32),
            ty::I64 => quote!(int64),

            ty::UInt32 if cx.contains_tag::<Fixed32>(ty.tags_id) => quote!(fixed32),
            ty::UInt64 if cx.contains_tag::<Fixed64>(ty.tags_id) => quote!(fixed64),

            ty::UInt32 => quote!(uint32),
            ty::UInt64 => quote!(uint64),
            ty::F32 => quote!(float),
            ty::F64 => quote!(double),
            ty::Vec(ty) => {
                let el = self.mk_ty_attr(cx, ty);
                quote!(#el, repeated)
            }
            ty::Map(k, v) => {
                let key = self.mk_ty_attr(cx, k);
                let val = self.mk_ty_attr(cx, v);
                let ty = quote!(#key, #val).to_string();
                quote!(map = #ty)
            }
            ty::Path(_) => quote!(message),
            ty::Arc(ty) => self.mk_ty_attr(cx, ty),
            ty::Set(_) | ty::Void | ty::U8 | ty::I16 | ty::I8 => {
                unreachable!()
            }
        }
    }
}

impl crate::Plugin for ProstPlugin {
    fn on_item(&mut self, cx: &mut Context, def_id: crate::DefId, item: Arc<crate::rir::Item>) {
        match &*item {
            crate::rir::Item::Message(_) => cx.with_adjust(def_id, |adj| {
                adj.add_attrs(&[parse_quote!(#[derive(::prost::Message)])]);
            }),
            crate::rir::Item::Enum(e) if cx.node_contains_tag::<OneOf>(def_id) => {
                cx.with_adjust(def_id, |adj| {
                    adj.add_attrs(&[parse_quote!(#[derive(::prost::Oneof)])])
                });
                e.variants.iter().for_each(|v| {
                    let ty = self.mk_ty_attr(cx, &v.fields[0]);
                    let tag = v.id.unwrap().to_string();
                    cx.with_adjust(v.did, |adj| {
                        adj.add_attrs(&[parse_quote!(#[prost(#ty, tag = #tag)])])
                    })
                });
            }
            crate::rir::Item::Enum(_) => cx.with_adjust(def_id, |adj| {
                adj.add_attrs(&[parse_quote!(#[derive(::prost::Enumeration, Debug)])])
            }),
            _ => {}
        }

        crate::plugin::walk_item(self, cx, def_id, item)
    }

    fn on_field(&mut self, cx: &mut Context, def_id: crate::DefId, f: Arc<crate::rir::Field>) {
        let one_of = cx.node_contains_tag::<OneOf>(def_id);

        let item_def_id = cx.node(def_id).unwrap().parent.unwrap();

        let target_def_id = match &f.ty.kind {
            ty::TyKind::Path(p) => Some(p.did),
            _ => None,
        };

        let item = target_def_id.map(|target_def_id| cx.item(target_def_id).unwrap());

        let target_enum = item.as_deref().and_then(|item| match item {
            rir::Item::Enum(e) => Some(e),
            _ => None,
        });

        let attrs = if let Some(target_enum) = target_enum {
            if one_of {
                let path = cx
                    .related_item_path(item_def_id, target_def_id.unwrap())
                    .to_token_stream()
                    .to_string();

                let tags = target_enum.variants.iter().map(|v| v.id.unwrap()).join(",");
                quote!(oneof = #path, tags = #tags)
            } else {
                let path = cx
                    .related_item_path(item_def_id, target_def_id.unwrap())
                    .to_token_stream()
                    .to_string();
                let tag = format!("{}", f.id);

                // hack
                unsafe { (Arc::as_ptr(&f) as *mut Field).as_mut().unwrap() }.ty = Ty {
                    tags_id: f.ty.tags_id,
                    kind: ty::I32,
                };

                quote!(enumeration = #path, tag = #tag)
            }
        } else {
            let optional = matches!(f.kind, FieldKind::Optional)
                .then(|| quote! {optional})
                .into_iter();
            let ty = self.mk_ty_attr(cx, &f.ty);
            let tag = format!("{}", f.id);
            quote!(#ty, tag = #tag #(, #optional)*)
        };

        cx.with_adjust(def_id, |adj| {
            adj.add_attrs(&[parse_quote!(#[prost(#attrs)])]);
        });
        crate::plugin::walk_filed(self, cx, def_id, f)
    }
}
