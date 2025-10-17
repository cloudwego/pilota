use std::ops::Deref;

use faststr::FastStr;
use itertools::Itertools;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::{
    CodegenBackend, Context, DefId, Symbol,
    db::RirDatabase,
    middle::{
        context::Mode,
        ext::{
            FileExts, ModExts,
            pb::{Extendee, ExtendeeKind, Extendees, FieldType},
        },
        ty::{self},
    },
    rir::{self, Field, FieldKind, Item, NodeKind},
    symbol::ModPath,
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
    #[inline]
    fn is_arc_message(&self, ty: &Ty) -> bool {
        let mut ty_ref = ty;
        if let ty::TyKind::Vec(inner) = &ty_ref.kind {
            ty_ref = inner;
        }
        matches!(ty_ref.kind, ty::TyKind::Arc(_))
    }

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
                let is_arc = self.is_arc_message(ty);

                if let ty::TyKind::Vec(_) = &ty.kind {
                    let encoded_len_fn = if is_arc {
                        "::pilota::pb::encoding::arc_message::encoded_len_repeated".to_string()
                    } else {
                        "::pilota::pb::encoding::message::encoded_len_repeated".to_string()
                    };

                    match kind {
                        FieldKind::Required => format!("{encoded_len_fn}({tag}, &{ident})").into(),
                        FieldKind::Optional => format!(
                            "{ident}.as_ref().map_or(0, |msg| {encoded_len_fn}({tag}, msg))"
                        )
                        .into(),
                    }
                } else {
                    let encoded_len: FastStr = if self.is_one_of(ty) {
                        "msg.encoded_len()".into()
                    } else {
                        let ident: FastStr = match kind {
                            FieldKind::Required => format!("&{ident}").into(),
                            FieldKind::Optional => "msg".into(),
                        };
                        if is_arc {
                            format!(
                                "::pilota::pb::encoding::arc_message::encoded_len({tag}, {ident})"
                            )
                            .into()
                        } else {
                            format!("::pilota::pb::encoding::message::encoded_len({tag}, {ident})")
                                .into()
                        }
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
            ty::TyKind::Arc(_) => Category::Message, // Arc 类型应该被归类为 Message
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
            match &ty.kind {
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
                ty::TyKind::Path(path) if self.is_plain_enum(path.did) => "int32",
                ty::TyKind::Path(_) => "message",
                ty::TyKind::Arc(inner) => {
                    return Ident::new(
                        &format!("arc_{}", self.ty_module(inner.as_ref())),
                        Span::call_site(),
                    );
                }
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
                // check for Arc type
                let is_arc = self.is_arc_message(ty);

                if let ty::TyKind::Vec(_) = ty.kind {
                    let encode_item_fn = if is_arc {
                        format!(r#"::pilota::pb::encoding::arc_message::encode({tag}, msg, buf);"#)
                    } else {
                        format!(r#"::pilota::pb::encoding::message::encode({tag}, msg, buf);"#)
                    };

                    match kind {
                        FieldKind::Required => format!(
                            r#"for msg in &{ident} {{
                                {encode_item_fn}
                            }};"#
                        )
                        .into(),
                        FieldKind::Optional => format!(
                            r#"if let Some(_pilota_inner_value) = {ident}.as_ref() {{
                                for msg in _pilota_inner_value {{
                                    {encode_item_fn}
                                }}
                            }}"#
                        )
                        .into(),
                    }
                } else {
                    let encode: FastStr = if self.is_one_of(ty) {
                        "_pilota_inner_value.encode(buf);".into()
                    } else {
                        let ident: FastStr = match kind {
                            FieldKind::Required => format!("(&{ident})").into(),
                            FieldKind::Optional => "_pilota_inner_value".into(),
                        };
                        if is_arc {
                            format!(
                                "::pilota::pb::encoding::arc_message::encode({tag}, {ident}, buf);"
                            )
                            .into()
                        } else {
                            format!("::pilota::pb::encoding::message::encode({tag}, {ident}, buf);")
                                .into()
                        }
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
        let idl_name = s.name.sym.0.clone();
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
        let keep = self.cache.keep_unknown_fields.contains(&def_id);

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

        if self.cx.config.with_descriptor {
            let file_id = self.cx.node(def_id).unwrap().file_id;
            match &self.cx.file_paths().get(&file_id) {
                Some(_) => {
                    let filename = self.file_name(file_id).unwrap().replace(".", "_");
                    let filename_lower = filename.to_lowercase();

                    let file = &self.cx.files().get(&file_id).unwrap().package;
                    let path = self.cx.item_path(def_id);
                    let super_mods = "super::".repeat(path.len() - file.len() - 1);

                    stream.push_str(&format!(
                    r#"
                    impl {name} {{
                        fn get_descriptor_proto() -> Option<&'static ::pilota::pb::descriptor::DescriptorProto> {{
                            let file_descriptor = {super_mods}file_descriptor_proto_{filename_lower}();
                            file_descriptor.get_message_descriptor_proto("{idl_name}")
                        }}
                    }}
                    "#
                    ));
                }
                _ => {}
            }
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

        // nested message exts are injected inside each message's mod by backend
        // hook
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

        let file_id = self.cx.node(def_id).unwrap().file_id;
        let mut getter_impl = match &e.oneof_parent {
            Some(ty) => match &self.cx.file_paths().get(&file_id) {
                Some(_) => {
                    let path = self.codegen_item_ty(ty.kind.clone());
                    let path = format!("{path}");
                    let name = self.cx.rust_name(def_id);
                    let idl_name = e.name.sym.0.clone();
                    format!(
                        r#"
                        impl {name} {{
                            fn get_descriptor_proto() -> Option<&'static ::pilota::pb::descriptor::OneofDescriptorProto> {{
                                let message_descriptor = {path}::get_descriptor_proto()?;
                                message_descriptor.get_oneof_descriptor_proto("{idl_name}")
                            }}
                        }}
                    "#
                    )
                }
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };

        if !self.config.with_descriptor {
            getter_impl.clear();
        }

        stream.push_str(&format! {
            r#"

            {getter_impl}

            impl {name} {{
                pub fn encode(&self, buf: &mut ::pilota::LinkedBytes) {{
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

    fn codegen_enum_descrptor_getter_impl(
        &self,
        def_id: DefId,
        stream: &mut String,
        e: &rir::Enum,
    ) {
        if !self.cx.config.with_descriptor {
            return;
        }

        let file_id = self.cx.node(def_id).unwrap().file_id;

        let getter_impl = match self.cx.file_paths().get(&file_id) {
            Some(_) => {
                let name = self.cx.rust_name(def_id);
                let idl_name = e.name.sym.0.clone();
                let filename = self.file_name(file_id).unwrap().replace(".", "_");
                let filename_lower = filename.to_lowercase();

                let file = &self.cx.files().get(&file_id).unwrap().package;
                let path = self.cx.item_path(def_id);
                let super_mods = "super::".repeat(path.len() - file.len() - 1);

                format!(
                    r#"

                    impl {name} {{
                        fn get_descriptor_proto() -> Option<&'static ::pilota::pb::descriptor::EnumDescriptorProto> {{
                            let file_descriptor = {super_mods}file_descriptor_proto_{filename_lower}();
                            file_descriptor.get_enum_descriptor_proto("{idl_name}")
                        }}
                    }}

                    "#
                )
            }
            _ => "".to_string(),
        };
        stream.push_str(&getter_impl);
    }

    fn cx(&self) -> &Context {
        &self.cx
    }

    fn codegen_pilota_trait(&self, stream: &mut String) {
        if self.cx.config.with_descriptor {
            stream.push_str("use ::pilota::{Buf as _, BufMut as _, pb::descriptor_getter::*};");
        } else {
            stream.push_str("use ::pilota::{Buf as _, BufMut as _};");
        }
    }

    fn codegen_file_descriptor(&self, stream: &mut String, f: &rir::File, has_direct: bool) {
        if self.file_paths().get(&f.file_id).is_none() {
            return;
        }

        let filename = self.file_name(f.file_id).unwrap().replace(".", "_");
        let filename_upper = filename.to_uppercase();
        let filename_lower = filename.to_lowercase();
        if has_direct {
            let descriptor = &f.descriptor;
            let super_mod = match &*self.source.mode {
                Mode::Workspace(_) => "crate::".to_string(),
                Mode::SingleFile { .. } => "super::".repeat(f.package.len()),
            };

            // dependency reflect builders
            let mut deps_builders = String::new();
            for dep in &f.uses {
                if let Some(dep_file) = self.file(*dep) {
                    // only include dependencies with include path in current crate
                    let has_include_path = self.file_ids_map().iter().any(|(_, id)| *id == *dep);
                    let pkg = dep_file
                        .package
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("::");

                    if pkg == "google::protobuf" {
                        let mod_path = dep_file
                            .extensions
                            .unwrap_as_pb()
                            .well_known_file_name
                            .mod_name();
                        if mod_path.is_empty() {
                            unreachable!(
                                "invalid well known file name: {:?}",
                                dep_file.extensions.unwrap_as_pb().well_known_file_name
                            );
                        }

                        deps_builders.push_str(&format!(
                            "deps.push({mod_path}::file_descriptor().clone());\n"
                        ));
                    } else if has_include_path && !pkg.is_empty() {
                        let dep_filename = self
                            .file_name(*dep)
                            .unwrap()
                            .replace(".", "_")
                            .to_lowercase();
                        deps_builders.push_str(&format!(
                            "deps.push({super_mod}{pkg}::file_descriptor_{dep_filename}().clone());\n"
                        ));
                    }
                }
            }

            stream.push_str(&format!(
                r#"
static FILE_DESCRIPTOR_BYTES_{filename_upper}: ::pilota::Bytes = ::pilota::Bytes::from_static({descriptor:?});
static FILE_DESCRIPTOR_PROTO_{filename_upper}: ::std::sync::LazyLock<::pilota::pb::descriptor::FileDescriptorProto> = ::std::sync::LazyLock::new(|| {{
        let data: &[u8] = FILE_DESCRIPTOR_BYTES_{filename_upper}.as_ref();
        ::pilota::pb::PbMessage::parse_from_bytes(data).expect("Failed to decode file descriptor")
}});
pub fn file_descriptor_proto_{filename_lower}() -> &'static ::pilota::pb::descriptor::FileDescriptorProto {{
    &*FILE_DESCRIPTOR_PROTO_{filename_upper}
}}

static FILE_DESCRIPTOR_{filename_upper}: ::std::sync::LazyLock<::pilota::pb::reflect::FileDescriptor> = ::std::sync::LazyLock::new(|| {{
    let mut deps = ::std::vec::Vec::new();
    {deps_builders}
    ::pilota::pb::reflect::FileDescriptor::new_dynamic(file_descriptor_proto_{filename_lower}().clone(), &deps)
        .expect("Failed to build dynamic FileDescriptor")
}});

pub fn file_descriptor_{filename_lower}() -> &'static ::pilota::pb::reflect::FileDescriptor {{
    &*FILE_DESCRIPTOR_{filename_upper}
}}
"#
            ));
        } else {
            match &*self.source.mode {
                Mode::Workspace(_) => {
                    let mod_prefix = f.package.iter().join("::");
                    let common_crate_name = &self.config.common_crate_name;
                    stream.push_str(&format!(
                        r#"
                        pub use ::{common_crate_name}::{mod_prefix}::get_file_descriptor_{filename_lower};
                        "#
                    ));
                }
                Mode::SingleFile { .. } => {}
            }
        }
    }

    fn codegen_file_descriptor_at_mod(
        &self,
        stream: &mut String,
        f: &rir::File,
        mod_path: &ModPath,
        has_direct: bool,
    ) {
        // only generate at file root mod, i.e., when mod_path equals package path
        let pkg: Vec<String> = f.package.iter().map(|s| s.to_string()).collect();
        let cur: Vec<String> = mod_path.iter().map(|s| s.to_string()).collect();
        if pkg == cur {
            self.codegen_file_descriptor(stream, f, has_direct);
        }
    }

    fn codegen_file_exts(
        &self,
        stream: &mut String,
        suffix: &str,
        cur_pkg: &[Symbol],
        extensions: &FileExts,
    ) {
        let pb_extendees = &extensions.unwrap_as_pb().extendees;
        self.codgen_pb_extendees(suffix, stream, cur_pkg, pb_extendees);
    }

    fn codegen_mod_exts(
        &self,
        stream: &mut String,
        suffix: &str,
        cur_pkg: &[Symbol],
        extensions: &ModExts,
    ) {
        let pb_extendees = &extensions.unwrap_as_pb().extendees;
        self.codgen_pb_extendees(suffix, stream, cur_pkg, pb_extendees);
    }

    fn codegen_impl_enum_message(&self, name: &str) -> String {
        format!(
            r#"
                impl ::pilota::pb::EnumMessage for {name} {{
                fn inner(&self) -> i32 {{
                    self.inner()
                }}

                fn to_string(&self) -> ::std::string::String {{
                    self.to_string()
                }}

                fn try_from_i32(value: i32) -> ::std::option::Option<Self> {{
                    {name}::try_from_i32(value)
                }}
            }}
        "#
        )
    }
}

impl ProtobufBackend {
    fn codgen_pb_extendees(
        &self,
        suffix: &str,
        stream: &mut String,
        cur_pkg: &[Symbol],
        extendees: &Extendees,
    ) {
        stream.push_str(&format!("pub mod exts_{suffix} {{\n"));
        for ext in &extendees.0 {
            if self.cx.config.touch_all || self.cx.db.pb_ext_used(&ext.index) {
                self.codgen_pb_custom_ext_field(stream, cur_pkg, ext);
            }
        }
        stream.push_str("}\n");
    }
    fn codgen_pb_custom_ext_field(&self, stream: &mut String, cur_pkg: &[Symbol], ext: &Extendee) {
        let tag_id = ext.index.tag_id;
        let extendee_ty = match ext.index.extendee_kind {
            ExtendeeKind::File => "::pilota::pb::descriptor::FileOptions",
            ExtendeeKind::Message => "::pilota::pb::descriptor::MessageOptions",
            ExtendeeKind::Field => "::pilota::pb::descriptor::FieldOptions",
            ExtendeeKind::Enum => "::pilota::pb::descriptor::EnumOptions",
            ExtendeeKind::EnumValue => "::pilota::pb::descriptor::EnumValueOptions",
            ExtendeeKind::Service => "::pilota::pb::descriptor::ServiceOptions",
            ExtendeeKind::Method => "::pilota::pb::descriptor::MethodOptions",
            ExtendeeKind::Oneof => "::pilota::pb::descriptor::OneofOptions",
        };
        let val_ty = match &ext.extendee_ty.item_ty.kind {
            ty::TyKind::Path(p) => {
                let cg = self.codegen_item_ty(ext.extendee_ty.item_ty.kind.clone());
                match &*self.source.mode {
                    Mode::Workspace(_) => cg.global_path("crate").to_string(),
                    Mode::SingleFile { .. } => {
                        let target_path = self.item_path(p.did);
                        let path = self
                            .source
                            .path_resolver
                            .related_path(cur_pkg, &target_path);
                        format!("super::{path}")
                    }
                }
            }
            ty::TyKind::String | ty::TyKind::FastStr => {
                "::pilota::pb::extension::StrOptionValueExtractor".to_string()
            }
            ty::TyKind::Bytes => "::pilota::pb::extension::BytesOptionValueExtractor".to_string(),
            ty::TyKind::Bool => "::pilota::pb::extension::BoolOptionValueExtractor".to_string(),
            ty::TyKind::I32 => "::pilota::pb::extension::Int32OptionValueExtractor".to_string(),
            ty::TyKind::I64 => "::pilota::pb::extension::Int64OptionValueExtractor".to_string(),
            ty::TyKind::UInt32 => "::pilota::pb::extension::UInt32OptionValueExtractor".to_string(),
            ty::TyKind::UInt64 => "::pilota::pb::extension::UInt64OptionValueExtractor".to_string(),
            ty::TyKind::F32 => "::pilota::pb::extension::FloatOptionValueExtractor".to_string(),
            ty::TyKind::F64 => "::pilota::pb::extension::DoubleOptionValueExtractor".to_string(),
            _ => {
                unreachable!(
                    "invalid pb extension type: {:?}",
                    ext.extendee_ty.item_ty.kind
                )
            }
        };

        let const_name = &*ext.name;

        match &ext.extendee_ty.field_ty {
            FieldType::Enum => {
                stream.push_str(&format!(
                    "pub const {const_name}: ::pilota::pb::extension::CustomExtEnumField<{extendee_ty}, {val_ty}> = ::pilota::pb::extension::CustomExtEnumField::new({tag_id});\n"
                ));
            }
            FieldType::Message => {
                stream.push_str(&format!(
                    "pub const {const_name}: ::pilota::pb::extension::CustomExtField<{extendee_ty}, ::pilota::pb::extension::MessageOptionValueExtractor<{val_ty}>> = ::pilota::pb::extension::CustomExtField::new({tag_id});\n"
                ));
            }
            _ => {
                stream.push_str(&format!(
                    "pub const {const_name}: ::pilota::pb::extension::CustomExtField<{extendee_ty}, {val_ty}> = ::pilota::pb::extension::CustomExtField::new({tag_id});\n"
                ));
            }
        }
    }
}
