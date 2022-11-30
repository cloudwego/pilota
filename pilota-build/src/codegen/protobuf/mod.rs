use std::sync::Arc;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;

use crate::{
    db::RirDatabase,
    middle::ty::{self, Ty},
    rir::{self, Field, FieldKind, Path},
    symbol::EnumRepr,
    tags::protobuf::{OneOf, ProstType},
    ty::{fold_ty, Folder},
    CodegenBackend, Context, DefId,
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
    fn mk_ty_attr(&self, cx: &Context, ty: &Ty, cur_def_id: DefId) -> TokenStream {
        if let Some(prost_type) = cx
            .tags(ty.tags_id)
            .as_ref()
            .and_then(|tags| tags.get::<ProstType>())
        {
            return match prost_type {
                ProstType::SInt32 => quote!(sint32),
                ProstType::SInt64 => quote!(sint64),
                ProstType::Fixed32 => quote!(fixed32),
                ProstType::Fixed64 => quote!(fixed64),
                ProstType::SFixed32 => quote!(sfixed32),
                ProstType::SFixed64 => quote!(sfixed64),
            };
        }
        match &ty.kind {
            ty::String | ty::SmolStr => quote!(string),
            ty::Bool => quote!(bool),
            ty::BytesVec | ty::Bytes => quote!(bytes),
            ty::I32 => quote!(int32),
            ty::I64 => quote!(int64),
            ty::UInt32 => quote!(uint32),
            ty::UInt64 => quote!(uint64),
            ty::F32 => quote!(float),
            ty::F64 => quote!(double),
            ty::Vec(ty) => {
                let el = self.mk_ty_attr(cx, ty, cur_def_id);
                quote!(#el, repeated)
            }
            ty::Map(k, v) => {
                let key = self.mk_ty_attr(cx, k, cur_def_id);
                let val = self.mk_ty_attr(cx, v, cur_def_id);
                let ty = quote!(#key, #val).to_string();
                quote!(map = #ty)
            }
            ty::Path(rir::Path { did, .. }) => {
                if let rir::Item::Enum(_) = &*cx.item(*did).unwrap() {
                    let path = cx
                        .related_item_path(cur_def_id, *did)
                        .to_token_stream()
                        .to_string();
                    quote!(enumeration = #path)
                } else {
                    quote!(message)
                }
            }
            ty::Arc(ty) => self.mk_ty_attr(cx, ty, cur_def_id),
            ty::Set(_) | ty::Void | ty::U8 | ty::I16 | ty::I8 => {
                unreachable!()
            }
        }
    }
}

struct ReplaceEnum<'cx> {
    cx: &'cx Context,
}

impl Folder for ReplaceEnum<'_> {
    fn fold_ty(&mut self, ty: &Ty) -> Ty {
        if let ty::TyKind::Path(Path { did, .. }) = ty.kind {
            if let rir::Item::Enum(e) = &*self.cx.item(did).unwrap() {
                if e.repr == Some(EnumRepr::I32) {
                    return Ty {
                        kind: ty::TyKind::I32,
                        tags_id: ty.tags_id,
                    };
                }
            }
        }

        fold_ty(self, ty)
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
                    let ty = self.mk_ty_attr(cx, &v.fields[0], def_id);
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

        let attrs = match target_enum {
            Some(target_enum) if one_of => {
                let path = cx
                    .related_item_path(item_def_id, target_def_id.unwrap())
                    .to_token_stream()
                    .to_string();

                let tags = target_enum.variants.iter().map(|v| v.id.unwrap()).join(",");
                quote!(oneof = #path, tags = #tags)
            }
            _ => {
                let optional = matches!(f.kind, FieldKind::Optional)
                    .then(|| quote! {optional})
                    .into_iter();
                let ty = self.mk_ty_attr(cx, &f.ty, item_def_id);
                let tag = format!("{}", f.id);
                quote!(#ty, tag = #tag #(, #optional)*)
            }
        };

        unsafe {
            (Arc::as_ptr(&f) as *mut Field).as_mut().unwrap().ty = ReplaceEnum { cx }.fold_ty(&f.ty)
        };

        cx.with_adjust(def_id, |adj| {
            adj.add_attrs(&[parse_quote!(#[prost(#attrs)])]);
        });
        crate::plugin::walk_filed(self, cx, def_id, f)
    }
}
