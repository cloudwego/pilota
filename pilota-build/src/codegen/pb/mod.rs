use std::ops::Deref;

use faststr::FastStr;
use itertools::Itertools;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::{
    CodegenBackend, Context, DefId,
    db::RirDatabase,
    middle::ty::{self},
    rir::{self, Field, FieldKind, Item, NodeKind},
    tags::protobuf::{OneOf, ProstType},
    ty::Ty,
};

#[derive(Clone)]
pub struct ProtobufBackend {
    cx: Context,
}

impl ProtobufBackend {
    pub fn new(cx: Context) -> Self {
        ProtobufBackend { cx }
    }
}

impl Deref for ProtobufBackend {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

pub enum Category {
    Scalar,
    Message,
    Map,
}

impl ProtobufBackend {
    fn codegen_encoded_len(&self, ident: FastStr, ty: &Ty, id: u32, kind: FieldKind) -> FastStr {
        let category = self.ty_category(ty);

        let tag = id;

        match category {
            Category::Scalar => {
                let encoded_len_fn = match ty.kind {
                    ty::TyKind::Vec(_) => quote!(encoded_len_repeated),
                    _ => quote!(encoded_len),
                };

                let module = self.ty_module(ty);

                let encoded_len_fn = format!("::pilota::pb::encoding::{module}::{encoded_len_fn}");

                match kind {
                    FieldKind::Required => format!("{encoded_len_fn}({tag}, &{ident})").into(),
                    FieldKind::Optional => format!(
                        "{ident}.as_ref().map_or(0, |value| {encoded_len_fn}({tag}, value))"
                    )
                    .into(),
                }
            }
            Category::Message => {
                if let ty::TyKind::Vec(_) = ty.kind {
                    format!(
                        "::pilota::pb::encoding::message::encoded_len_repeated({tag}, &{ident})"
                    )
                    .into()
                } else {
                    let encoded_len: FastStr = if self.is_one_of(ty) {
                        "msg.encoded_len()".into()
                    } else {
                        let ident: FastStr = match kind {
                            FieldKind::Required => format!("&{ident}").into(),
                            FieldKind::Optional => "msg".into(),
                        };
                        format!("::pilota::pb::encoding::message::encoded_len({tag}, {ident})")
                            .into()
                    };

                    match kind {
                        FieldKind::Required => format!("{encoded_len}").into(),
                        FieldKind::Optional => {
                            format!("{ident}.as_ref().map_or(0, |msg| {encoded_len})").into()
                        }
                    }
                }
            }
            Category::Map => {
                let ty::TyKind::Map(key_ty, value_ty) = &ty.kind else {
                    unreachable!();
                };

                let key_module = self.ty_module(key_ty);
                let value_module = self.ty_module(value_ty);

                let key_encoded_len_fn = quote!(::pilota::pb::encoding::#key_module::encoded_len);
                let value_encoded_len_fn =
                    quote!(::pilota::pb::encoding::#value_module::encoded_len);

                format!("::pilota::pb::encoding::hash_map::encoded_len({key_encoded_len_fn}, {value_encoded_len_fn}, {tag}, &{ident})").into()
            }
        }
    }

    fn is_plain_enum(&self, def_id: DefId) -> bool {
        let node = self.cx.node(def_id).unwrap();
        if let NodeKind::Item(item) = node.kind {
            if let Item::Enum(_) = &*item {
                if !self.cx.contains_tag::<OneOf>(node.tags) {
                    return true;
                }
            }
        }
        false
    }

    fn is_one_of_item(&self, def_id: DefId) -> bool {
        let node = self.cx.node(def_id).unwrap();
        if let NodeKind::Item(item) = node.kind {
            if let Item::Enum(_) = &*item {
                if self.cx.contains_tag::<OneOf>(node.tags) {
                    return true;
                }
            }
        }
        false
    }

    fn is_one_of(&self, ty: &Ty) -> bool {
        let mut ty = ty;
        if let ty::TyKind::Vec(inner) = &ty.kind {
            ty = inner;
        }
        matches!(&ty.kind, ty::TyKind::Path(p) if self.is_one_of_item(p.did))
    }

    fn ty_category(&self, ty: &Ty) -> Category {
        let mut ty = ty;
        if let ty::TyKind::Vec(inner) = &ty.kind {
            ty = inner;
        }

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
            ty::TyKind::Path(path) if self.is_plain_enum(path.did) => Category::Scalar,
            _ => Category::Message,
        }
    }

    fn ty_module(&self, ty: &ty::Ty) -> Ident {
        let mut ty = ty;
        if let ty::TyKind::Vec(inner) = &ty.kind {
            ty = inner;
        }
        let pb_type = self
            .cx
            .tags(ty.tags_id)
            .and_then(|tags| tags.get::<ProstType>().copied());
        Ident::new(
            match ty.kind {
                ty::TyKind::String => "string",
                ty::TyKind::FastStr => "faststr",
                ty::TyKind::Bool => "bool",
                ty::TyKind::BytesVec | ty::TyKind::Bytes => "bytes",
                ty::TyKind::I32 if pb_type == Some(ProstType::SFixed32) => "sfixed32",
                ty::TyKind::I32 => "int32",
                ty::TyKind::I64 if pb_type == Some(ProstType::SFixed64) => "sfixed64",
                ty::TyKind::I64 => "int64",
                ty::TyKind::UInt32 if pb_type == Some(ProstType::Fixed32) => "fixed32",
                ty::TyKind::UInt32 if pb_type == Some(ProstType::SInt32) => "sint32",
                ty::TyKind::UInt32 => "uint32",
                ty::TyKind::UInt64 if pb_type == Some(ProstType::Fixed64) => "fixed64",
                ty::TyKind::UInt64 if pb_type == Some(ProstType::SInt64) => "sint64",
                ty::TyKind::UInt64 => "uint64",
                ty::TyKind::F32 => "float",
                ty::TyKind::F64 => "double",
                ty::TyKind::Path(ref path) if self.is_plain_enum(path.did) => "int32",
                ty::TyKind::Path(_) => "message",
                _ => unreachable!("{:?}", ty.kind),
            },
            Span::call_site(),
        )
    }

    fn codegen_encode(&self, ident: FastStr, ty: &Ty, id: u32, kind: FieldKind) -> FastStr {
        let category = self.ty_category(ty);

        let tag = id;

        match category {
            Category::Scalar => {
                let encode_fn = match ty.kind {
                    ty::TyKind::Vec(_) => quote!(encode_repeated),
                    _ => quote!(encode),
                };

                let module = self.ty_module(ty);

                let encode_fn = format!("::pilota::pb::encoding::{module}::{encode_fn}");

                match kind {
                    FieldKind::Required => format!("{encode_fn}({tag}, &{ident}, buf);").into(),
                    FieldKind::Optional => format! {
                        r#"if let Some(_pilota_inner_value) = {ident}.as_ref() {{
                                {encode_fn}({tag}, _pilota_inner_value, buf);
                            }};"#
                    }
                    .into(),
                }
            }
            Category::Message => {
                if let ty::TyKind::Vec(_) = ty.kind {
                    format!(
                        r#"for msg in &{ident} {{
                            ::pilota::pb::encoding::message::encode({tag}, msg, buf);
                        }};"#
                    )
                    .into()
                } else {
                    let encode: FastStr = if self.is_one_of(ty) {
                        "_pilota_inner_value.encode(buf);".into()
                    } else {
                        let ident: FastStr = match kind {
                            FieldKind::Required => format!("(&{ident})").into(),
                            FieldKind::Optional => "_pilota_inner_value".into(),
                        };
                        format!("::pilota::pb::encoding::message::encode({tag}, {ident}, buf);")
                            .into()
                    };

                    match kind {
                        FieldKind::Required => encode,
                        FieldKind::Optional => format!(
                            r#"if let Some(_pilota_inner_value) = {ident}.as_ref() {{ {encode} }}"#
                        )
                        .into(),
                    }
                }
            }

            Category::Map => {
                let ty::TyKind::Map(key_ty, value_ty) = &ty.kind else {
                    unreachable!();
                };

                let key_module = self.ty_module(key_ty);
                let value_module = self.ty_module(value_ty);

                let key_encode_fn = quote!(::pilota::pb::encoding::#key_module::encode);
                let value_encode_fn = quote!(::pilota::pb::encoding::#value_module::encode);

                let key_encoded_len_fn = quote!(::pilota::pb::encoding::#key_module::encoded_len);
                let value_encoded_len_fn =
                    quote!(::pilota::pb::encoding::#value_module::encoded_len);

                format!("::pilota::pb::encoding::hash_map::encode({key_encode_fn}, {key_encoded_len_fn}, {value_encode_fn}, {value_encoded_len_fn}, {tag}, &{ident}, buf);").into()
            }
        }
    }

    fn field_tags(&self, field: &Field) -> impl Iterator<Item = u32> + use<> {
        if let ty::TyKind::Path(path) = &field.ty.kind {
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
        Some(field.id as u32).into_iter().chain(vec![])
    }

    fn codegen_merge_field(&self, ident: FastStr, ty: &Ty, kind: FieldKind) -> FastStr {
        match self.ty_category(ty) {
            Category::Scalar | Category::Message => {
                let merge_fn = match ty.kind {
                    ty::TyKind::Vec(_) => quote!(merge_repeated),
                    _ => quote!(merge),
                };

                if self.is_one_of(ty) {
                    let did = match &ty.kind {
                        ty::TyKind::Path(p) => p.did,
                        _ => unreachable!(),
                    };

                    let path = self.cx.cur_related_item_path(did);
                    format!("{path}::merge(&mut {ident}, tag, wire_type, buf, ctx)").into()
                } else {
                    let module = self.ty_module(ty);
                    let merge_fn = format!("::pilota::pb::encoding::{module}::{merge_fn}");

                    match kind {
                        FieldKind::Required => {
                            format!("{merge_fn}(wire_type, {ident}, buf, ctx)").into()
                        }
                        FieldKind::Optional => format!(
                            r#"{merge_fn}(wire_type, {ident}.get_or_insert_with(::core::default::Default::default), buf, ctx)"#
                        )
                        .into(),
                    }
                }
            }
            Category::Map => {
                let ty::TyKind::Map(key_ty, value_ty) = &ty.kind else {
                    unreachable!();
                };

                let key_mod = self.ty_module(key_ty);
                let value_mod = self.ty_module(value_ty);

                let key_merge_fn = format!("::pilota::pb::encoding::{key_mod}::merge");
                let value_merge_fn = format!("::pilota::pb::encoding::{value_mod}::merge");

                format!("::pilota::pb::encoding::hash_map::merge({key_merge_fn}, {value_merge_fn}, &mut {ident}, buf, ctx)").into()
            }
        }
    }
}

impl CodegenBackend for ProtobufBackend {
    const PROTOCOL: &'static str = "protobuf";

    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &rir::Message) {
        let name = self.cx.rust_name(def_id);
        let mut encoded_len = s
            .fields
            .iter()
            .map(|field| {
                let field_name = self.cx.rust_name(field.did);
                let len = self.codegen_encoded_len(
                    format!("self.{field_name}").into(),
                    &field.ty,
                    field.id as u32,
                    field.kind,
                );
                FastStr::from(format!("+ {len}"))
            })
            .join("");

        let mut encode = s
            .fields
            .iter()
            .map(|field| {
                let field_name = self.cx.rust_name(field.did);
                self.codegen_encode(
                    format!("self.{field_name}").into(),
                    &field.ty,
                    field.id as u32,
                    field.kind,
                )
            })
            .join("");

        let merge = s
            .fields
            .iter()
            .map(|field| {
                let field_ident = self.cx.rust_name(field.did);
                let merge =
                    self.codegen_merge_field("_inner_pilota_value".into(), &field.ty, field.kind);
                let mut tags = self.field_tags(field).map(|tag| tag.to_string());
                let tags = tags.join("|");

                format! {
                    r#"{tags} => {{
                    let mut _inner_pilota_value = &mut self.{field_ident};
                    {merge}.map_err(|mut error| {{
                        error.push(STRUCT_NAME, stringify!({field_ident}));
                        error
                    }})
                }},"#
                }
            })
            .join("");

        let struct_name = if s.fields.is_empty() {
            "".into()
        } else {
            format!("const STRUCT_NAME: &'static str = stringify!({name});")
        };

        // add unknown fields
        let keep = self.keep_unknown_fields.contains(&def_id);

        let mut unknown_fields = "";
        let mut skip_field = "::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx)";

        if keep {
            unknown_fields = r#"
            let mut _unknown_fields = &mut self._unknown_fields;"#;
            encoded_len.push_str(" + self._unknown_fields.size()");
            encode.push_str(
                r#"for bytes in self._unknown_fields.list.iter() {
                    buf.put_slice(bytes.as_ref());
                }"#,
            );

            skip_field = r#"{
                ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx)?;
                let end = buf.chunk().as_ptr();
                let len = end as usize - ctx.raw_bytes_cursor();
                let val = ctx.raw_bytes_split_to(len);
                _unknown_fields.push_back(val);
                Ok(())
            }"#;
        }

        stream.push_str(&format!(
            r#"
            impl ::pilota::pb::Message for {name} {{
                #[inline]
                fn encoded_len(&self) -> usize {{
                    0 {encoded_len}
                }}

                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {{
                    {encode}
                }}

                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::pilota::pb::encoding::WireType,
                    buf: &mut ::pilota::Bytes,
                    ctx: &mut ::pilota::pb::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {{
                    {struct_name}
                    {unknown_fields}
                    match tag {{
                        {merge}
                        _ => {skip_field}
                    }}
                }}
            }}
            "#
        ));
    }

    fn codegen_newtype_impl(&self, _def_id: DefId, _stream: &mut String, _t: &rir::NewType) {
        unreachable!()
    }

    fn codegen_enum_impl(&self, def_id: DefId, stream: &mut String, e: &rir::Enum) {
        let node = self.cx.node(def_id).unwrap();
        if !self.cx.contains_tag::<OneOf>(node.tags) {
            return;
        }
        let name = self.cx.rust_name(def_id);

        let encoded_len = e
            .variants
            .iter()
            .map(|variant| {
                let encoded_len = self.codegen_encoded_len(
                    "*value".into(),
                    variant.fields.first().unwrap(),
                    variant.id.unwrap() as u32,
                    FieldKind::Required,
                );
                let variant_name = self.cx.rust_name(variant.did);
                format!("{name}::{variant_name}(value) => {encoded_len}")
            })
            .join(",");

        let encode = e
            .variants
            .iter()
            .map(|variant| {
                let encode = self.codegen_encode(
                    "*value".into(),
                    variant.fields.first().unwrap(),
                    variant.id.unwrap() as u32,
                    FieldKind::Required,
                );
                let variant_name = self.cx.rust_name(variant.did);
                format!("{name}::{variant_name}(value) => {{ {encode} }}")
            })
            .join(",");

        let merge = e.variants.iter().map(|variant| {
            let tag = variant.id.unwrap() as u32;
            let variant_name = self.cx.rust_name(variant.did);
            let merge = self.codegen_merge_field(
                "value".into(),
                variant.fields.first().unwrap(),
                FieldKind::Required,
            );
            format! {
                r#"{tag} => {{
                    match field {{
                        ::core::option::Option::Some({name}::{variant_name}(value)) => {{
                            {merge}?;
                        }},
                        _ => {{
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            {merge}?;
                            *field = ::core::option::Option::Some({name}::{variant_name}(owned_value));
                        }},
                    }}
                }},"#
            }
        }).join("");

        stream.push_str(&format! {
            r#"impl {name} {{
                pub fn encode(&self, buf: &mut Self::BufMut) {{
                    match self {{
                        {encode}
                    }}
                }}

                #[inline]
                pub fn encoded_len(&self) -> usize {{
                    match self {{
                        {encoded_len}
                    }}
                }}

                #[inline]
                pub fn merge(
                    field: &mut ::core::option::Option<Self>,
                    tag: u32,
                    wire_type: ::pilota::pb::encoding::WireType,
                    buf: &mut ::pilota::Bytes,
                    ctx: &mut ::pilota::pb::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {{
                    match tag {{
                        {merge}
                        _ => unreachable!(concat!("invalid ", stringify!({name}), " tag: {{}}"), tag),
                    }};
                    ::core::result::Result::Ok(())
                }}
            }}"#
        });
    }

    fn cx(&self) -> &Context {
        &self.cx
    }
}
