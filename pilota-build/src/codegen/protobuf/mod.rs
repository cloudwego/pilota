use std::sync::Arc;

use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::{
    db::RirDatabase,
    middle::ty::{self},
    rir::{self, Field, FieldKind, Item, NodeKind},
    tags::protobuf::{OneOf, ProstType},
    ty::Ty,
    CodegenBackend, Context, DefId, IdentName,
};

pub struct ProtobufBackend {
    cx: Arc<Context>,
    _zero_copy: bool,
}

impl ProtobufBackend {
    pub(crate) fn new(cx: Arc<Context>) -> Self {
        ProtobufBackend {
            cx,
            _zero_copy: false,
        }
    }
}

pub enum Category {
    Scalar,
    Message,
    Map,
}

impl ProtobufBackend {
    fn codegen_encoded_len(
        &self,
        ident: TokenStream,
        ty: &Ty,
        id: u32,
        kind: FieldKind,
    ) -> TokenStream {
        let category = self.ty_category(ty);

        let tag = id;

        match category {
            Category::Scalar => {
                let encoded_len_fn = match ty.kind {
                    ty::TyKind::Vec(_) => quote!(encoded_len_repeated),
                    _ => quote!(encoded_len),
                };

                let module = self.ty_module(&ty);

                let encoded_len_fn = quote!(::pilota::prost::encoding::#module::#encoded_len_fn);

                match kind {
                    FieldKind::Required => quote!(#encoded_len_fn(#tag, &#ident)),
                    FieldKind::Optional => {
                        quote!(#ident.as_ref().map_or(0, |value| #encoded_len_fn(#tag, value)))
                    }
                }
            }
            Category::Message => {
                if let ty::TyKind::Vec(_) = ty.kind {
                    quote!(
                        ::prost::encoding::message::encoded_len_repeated(#tag, &#ident)
                    )
                } else {
                    match kind {
                        FieldKind::Required => quote!(
                            ::prost::encoding::message::encoded_len(#tag, &#ident)
                        ),
                        FieldKind::Optional => quote!(
                            #ident.as_ref().map_or(0, |msg| ::prost::encoding::message::encoded_len(#tag, msg))
                        ),
                    }
                }
            }
            Category::Map => {
                let ty::TyKind::Map(key_ty, value_ty) = &ty.kind
                else {
                    unreachable!();
                };

                let key_module = self.ty_module(&key_ty);
                let value_module = self.ty_module(&value_ty);

                let key_encoded_len_fn =
                    quote!(::pilota::prost::encoding::#key_module::encoded_len);
                let value_encoded_len_fn =
                    quote!(::pilota::prost::encoding::#value_module::encoded_len);

                quote!(
                    ::pilota::prost::encoding::map::encoded_len(#key_encoded_len_fn, #value_encoded_len_fn, #tag, #ident)
                )
            }
        }
    }

    fn ty_category(&self, ty: &Ty) -> Category {
        let mut ty = ty;
        if let ty::TyKind::Vec(inner) = &ty.kind {
            ty = &*inner;
        }

        let is_plain_enum = |def_id: DefId| -> bool {
            let node = self.cx.node(def_id).unwrap();
            if let NodeKind::Item(item) = node.kind {
                if let Item::Enum(_) = &*item {
                    if !self.cx.contains_tag::<OneOf>(node.tags) {
                        return true;
                    }
                }
            }
            false
        };

        match &ty.kind {
            ty::TyKind::String
            | ty::TyKind::FastStr
            | ty::TyKind::U8
            | ty::TyKind::Bool
            | ty::TyKind::I8
            | ty::TyKind::I16
            | ty::TyKind::I32
            | ty::TyKind::I64
            | ty::TyKind::UInt32
            | ty::TyKind::UInt64
            | ty::TyKind::F32
            | ty::TyKind::F64
            | ty::TyKind::BytesVec
            | ty::TyKind::Bytes => Category::Scalar,
            ty::TyKind::Map(..) => Category::Map,
            ty::TyKind::Path(path) if is_plain_enum(path.did) => Category::Scalar,
            _ => Category::Message,
        }
    }

    fn ty_module(&self, ty: &ty::Ty) -> Ident {
        let prost_type = self
            .cx
            .tags(ty.tags_id)
            .and_then(|tags| tags.get::<ProstType>().copied());
        Ident::new(
            match ty.kind {
                ty::TyKind::String | ty::TyKind::FastStr => "string",
                ty::TyKind::Bool => "bool",
                ty::TyKind::BytesVec | ty::TyKind::Bytes => "bytes",
                ty::TyKind::I32 if prost_type == Some(ProstType::SFixed32) => "int32",
                ty::TyKind::I32 => "int32",
                ty::TyKind::I64 if prost_type == Some(ProstType::SFixed64) => "int64",
                ty::TyKind::I64 => "int64",
                ty::TyKind::UInt32 if prost_type == Some(ProstType::Fixed32) => "sfixed32",
                ty::TyKind::UInt32 if prost_type == Some(ProstType::SInt32) => "sint32",
                ty::TyKind::UInt32 => "uint32",
                ty::TyKind::UInt64 if prost_type == Some(ProstType::Fixed64) => "sfixed64",
                ty::TyKind::UInt64 if prost_type == Some(ProstType::SInt64) => "sint64",
                ty::TyKind::UInt64 => "uint64",
                ty::TyKind::F32 => "float",
                ty::TyKind::F64 => "double",
                ty::TyKind::Path(_) => "message",
                _ => unreachable!("{:?}", ty.kind),
            },
            Span::call_site(),
        )
    }

    fn codegen_encode(&self, ident: TokenStream, ty: &Ty, id: u32, kind: FieldKind) -> TokenStream {
        let category = self.ty_category(ty);

        let tag = id as u32;

        match category {
            Category::Scalar => {
                let encode_fn = match ty.kind {
                    ty::TyKind::Vec(_) => quote!(encode_repeated),
                    _ => quote!(encode),
                };

                let module = self.ty_module(&ty);

                let encode_fn = quote!(::pilota::prost::encoding::#module::#encode_fn);

                match kind {
                    FieldKind::Required => quote!(#encode_fn(#tag, &#ident, buf);),
                    FieldKind::Optional => {
                        quote! {
                            if let Some(_pilota_inner_value) = #ident.as_ref() {
                                ::pilota::prost::encoding::#module::encode(#tag, _pilota_inner_value, buf);
                            }
                        }
                    }
                }
            }
            Category::Message => {
                if let ty::TyKind::Vec(_) = ty.kind {
                    quote!(
                        for msg in &#ident {
                            ::pilota::prost::encoding::message::encode(#tag, msg, buf);
                        }
                    )
                } else {
                    match kind {
                        FieldKind::Required => quote!(
                            ::pilota::prost::encoding::message::encode(#tag, &#ident, buf);
                        ),
                        FieldKind::Optional => quote!(
                            if let Some(_pilota_inner_value) = #ident.as_ref() {
                                ::pilota::prost::encoding::message::encode(#tag, _pilota_inner_value, buf);
                            }
                        ),
                    }
                }
            }

            Category::Map => {
                let ty::TyKind::Map(key_ty, value_ty) = &ty.kind
                else {
                    unreachable!();
                };

                let key_module = self.ty_module(&key_ty);
                let value_module = self.ty_module(&value_ty);

                let key_encode_fn = quote!(::pilota::prost::encoding::#key_module::encode);
                let value_encode_fn = quote!(::pilota::prost::encoding::#value_module::encode);

                quote!(
                    ::pilota::prost::encoding::hash_map::encode(#key_encode_fn, #value_encode_fn, #tag, #ident, buf)
                )
            }
        }
    }

    fn field_tags(&self, field: &Field) -> impl Iterator<Item = u32> {
        match &field.ty.kind {
            ty::TyKind::Path(path) => {
                if let Some(node) = &self.cx.node(path.did) {
                    if self.cx.contains_tag::<OneOf>(node.tags) {
                        let item = self.cx.expect_item(path.did);
                        let e = match &*item {
                            Item::Enum(e) => e,
                            _ => unreachable!(),
                        };
                        return None.into_iter().chain(
                            e.variants
                                .iter()
                                .map(|v| v.id.unwrap() as u32)
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }
            _ => {}
        }
        Some(field.id as u32).into_iter().chain(vec![])
    }

    fn codegen_merge_field(&self, ident: TokenStream, ty: &Ty, kind: FieldKind) -> TokenStream {
        match self.ty_category(ty) {
            Category::Scalar | Category::Message => {
                let merge_fn = match ty.kind {
                    ty::TyKind::Vec(_) => quote!(merge_repeated),
                    _ => quote!(merge),
                };

                let module = self.ty_module(&ty);
                let merge_fn = quote!(::pilota::prost::encoding::#module::#merge_fn);

                match kind {
                    FieldKind::Required => quote!(#merge_fn(wire_type, #ident, buf, ctx)),
                    FieldKind::Optional => quote!(#merge_fn(wire_type,
                        #ident.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx)),
                }
            }
            Category::Map => {
                let ty::TyKind::Map(key_ty, value_ty) = &ty.kind
                else {
                    unreachable!();
                };

                let key_mod = self.ty_module(&key_ty);
                let value_mod = self.ty_module(&value_ty);

                let key_merge_fn = quote!(::pilota::prost::encoding::#key_mod::merge);
                let value_merge_fn = quote!(::pilota::prost::encoding::#value_mod::merge);

                quote! {
                    ::pilota::prost::encoding::hash_map::merge(#key_merge_fn, #value_merge_fn, &mut #ident, buf, ctx)
                }
            }
        }
    }
}

impl CodegenBackend for ProtobufBackend {
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut TokenStream, s: &rir::Message) {
        let name = self.cx.rust_name(def_id).as_syn_ident();
        let encoded_len = s.fields.iter().map(|field| {
            let field_name = self.cx.rust_name(field.did).as_syn_ident();
            self.codegen_encoded_len(
                quote! {self.#field_name},
                &field.ty,
                field.id as u32,
                field.kind,
            )
        });
        let encode = s.fields.iter().map(|field| {
            let field_name = self.cx.rust_name(field.did).as_syn_ident();
            self.codegen_encode(
                quote!(self.#field_name),
                &field.ty,
                field.id as u32,
                field.kind,
            )
        });

        let merge = s.fields.iter().map(|field| {
            let field_ident = self.cx.rust_name(field.did).as_syn_ident();
            let merge =
                self.codegen_merge_field(quote!(_inner_pilota_value), &field.ty, field.kind);
            let tags = self.field_tags(field).map(|tag| quote!(#tag));
            let tags = Itertools::intersperse(tags, quote!(|));

            quote! {
                #(#tags)* => {
                    let mut _inner_pilota_value = &mut self.#field_ident;
                    #merge.map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(#field_ident));
                        error
                    })
                },
            }
        });

        let struct_name = if s.fields.is_empty() {
            quote!()
        } else {
            quote!(
                const STRUCT_NAME: &'static str = stringify!(#name);
            )
        };

        stream.extend(quote!(
            impl ::pilota::prost::Message for #name {

                #[inline]
                fn encoded_len(&self) -> usize {
                    0 #(+ #encoded_len)*
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B) where B: ::pilota::prost::bytes::BufMut {
                    #(#encode)*
                }

                #[allow(unused_variables)]
                fn merge_field<B>(
                    &mut self,
                    tag: u32,
                    wire_type: ::pilota::prost::encoding::WireType,
                    buf: &mut B,
                    ctx: ::pilota::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
                where B: ::pilota::prost::bytes::Buf {
                    #struct_name
                    match tag {
                        #(#merge)*
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
        ));
    }

    fn codegen_newtype_impl(&self, _def_id: DefId, _stream: &mut TokenStream, _t: &rir::NewType) {
        unreachable!()
    }

    fn codegen_enum_impl(&self, def_id: DefId, stream: &mut TokenStream, e: &rir::Enum) {
        let node = self.cx.node(def_id).unwrap();
        if !self.cx.contains_tag::<OneOf>(node.tags) {
            return;
        }
        let name = self.cx.rust_name(def_id).as_syn_ident();

        let encoded_len = e.variants.iter().map(|variant| {
            let encoded_len = self.codegen_encoded_len(
                quote! {value},
                variant.fields.first().unwrap(),
                variant.id.unwrap() as u32,
                FieldKind::Required,
            );
            let variant_name = self.cx.rust_name(variant.did).as_syn_ident();
            quote!(#name::#variant_name(ref value) => #encoded_len)
        });

        let encode = e.variants.iter().map(|variant| {
            let encode = self.codegen_encode(
                quote! {value},
                variant.fields.first().unwrap(),
                variant.id.unwrap() as u32,
                FieldKind::Required,
            );
            let variant_name = self.cx.rust_name(variant.did).as_syn_ident();
            quote!(#name::#variant_name(ref value) => #encode)
        });

        let merge = e.variants.iter().map(|variant| {
            let tag = variant.id.unwrap() as u32;
            let variant_name = self.cx.rust_name(variant.did).as_syn_ident();
            let merge = self.codegen_merge_field(quote!{value}, variant.fields.first().unwrap(), FieldKind::Required);
            quote! {
                #tag => {
                    match field {
                        ::core::option::Option::Some(#name::#variant_name(ref mut value)) => {
                            #merge
                        },
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            #merge.map(|_| *field = ::core::option::Option::Some(#name::#variant_name(owned_value)))
                        },
                    }
                }
            }
        });

        stream.extend(quote! {
            impl ::pilota::prost::Message for #name {
                pub fn encode<B>(&self, buf: &mut B) where B: ::prost::bytes::BufMut {
                    match *self {
                        #(#encode,)*
                    }
                }

                /// Returns the encoded length of the message without a length delimiter.
                #[inline]
                pub fn encoded_len(&self) -> usize {
                    match *self {
                        #(#encoded_len,)*
                    }
                }

                 /// Decodes an instance of the message from a buffer, and merges it into self.
                pub fn merge<B>(
                    field: &mut ::core::option::Option<#name>,
                    tag: u32,
                    wire_type: ::prost::encoding::WireType,
                    buf: &mut B,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError>
                where B: ::prost::bytes::Buf {
                    match tag {
                        #(#merge,)*
                        _ => unreachable!(concat!("invalid ", stringify!(#name), " tag: {}"), tag),
                    }
                }
            }
        });
    }
}
