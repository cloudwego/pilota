use faststr::FastStr;

use super::{decode_helper::DecodeHelper, ThriftBackend};
use crate::{
    db::RirDatabase,
    middle::{rir, ty, ty::Ty},
    symbol::EnumRepr,
    tags::EnumMode,
    DefId,
};

impl ThriftBackend {
    pub(crate) fn ttype(&self, ty: &Ty) -> FastStr {
        match &ty.kind {
            ty::String | ty::FastStr => "::pilota::thrift::TType::Binary".into(),
            ty::Void => "::pilota::thrift::TType::Void".into(),
            ty::U8 => "::pilota::thrift::TType::I8".into(),
            ty::Bool => "::pilota::thrift::TType::Bool".into(),
            ty::BytesVec | ty::Bytes => "::pilota::thrift::TType::Binary".into(),
            ty::I8 => "::pilota::thrift::TType::I8".into(),
            ty::I16 => "::pilota::thrift::TType::I16".into(),
            ty::I32 => "::pilota::thrift::TType::I32".into(),
            ty::I64 => "::pilota::thrift::TType::I64".into(),
            ty::F64 => "::pilota::thrift::TType::Double".into(),
            ty::Uuid => "::pilota::thrift::TType::Uuid".into(),
            ty::Vec(_) => "::pilota::thrift::TType::List".into(),
            ty::Set(_) => "::pilota::thrift::TType::Set".into(),
            ty::Map(_, _) => "::pilota::thrift::TType::Map".into(),
            ty::Path(path) => {
                let item = self.expect_item(path.did);
                match &*item {
                    rir::Item::Message(_) => "::pilota::thrift::TType::Struct".into(),
                    rir::Item::Enum(e) => {
                        if e.repr.is_some() {
                            "::pilota::thrift::TType::I32".into()
                        } else {
                            "::pilota::thrift::TType::Struct".into()
                        }
                    }
                    rir::Item::NewType(t) => self.ttype(&t.ty),
                    _ => panic!("unsupported type {:?}", item),
                }
            }
            ty::Arc(ty) => self.ttype(ty),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_encode_ty(&self, ty: &Ty, ident: FastStr) -> FastStr {
        match &ty.kind {
            ty::String => format!("protocol.write_string({ident})?;").into(),
            ty::FastStr => format!("protocol.write_faststr(({ident}).clone())?;").into(),
            ty::Void => r#"protocol.write_struct_begin(&*::pilota::thrift::VOID_IDENT)?;protocol.write_struct_end()?;"#.into(),
            ty::U8 => format!("protocol.write_byte(*{ident})?;").into(),
            ty::Bool => format!("protocol.write_bool(*{ident})?;").into(),
            ty::BytesVec => format!("protocol.write_bytes_vec({ident})?;").into(),
            ty::Bytes => format!("protocol.write_bytes({ident}.clone())?;").into(),
            ty::I8 => format!("protocol.write_i8(*{ident})?;").into(),
            ty::I16 => format!("protocol.write_i16(*{ident})?;").into(),
            ty::I32 => format!("protocol.write_i32(*{ident})?;").into(),
            ty::I64 => format!("protocol.write_i64(*{ident})?;").into(),
            ty::F64 => format!("protocol.write_double(*{ident})?;").into(),
            ty::Uuid => format!("protocol.write_uuid({ident})?;").into(),
            ty::Vec(ty) => {
                let el_ttype = self.ttype(ty);
                let write_el = self.codegen_encode_ty(ty, "val".into());

                format! {
                    r#"protocol.write_list({el_ttype}, &{ident}, |protocol, val| {{
                        {write_el}
                        Ok(())
                    }})?;"#
                }
                .into()
            }
            ty::Set(ty) => {
                let write_el = self.codegen_encode_ty(ty, "val".into());
                let el_ttype = self.ttype(ty);

                format! {
                    r#"protocol.write_set({el_ttype}, &{ident}, |protocol, val| {{
                        {write_el}
                        Ok(())
                    }})?;"#
                }
                .into()
            }
            ty::Map(k, v) => {
                let key_ttype = self.ttype(k);
                let val_ttype = self.ttype(v);
                let write_key = self.codegen_encode_ty(k, "key".into());
                let write_val = self.codegen_encode_ty(v, "val".into());

                format! {
                    r#"protocol.write_map({key_ttype}, {val_ttype}, &{ident}, |protocol, key| {{
                        {write_key}
                        Ok(())
                    }}, |protocol, val| {{
                        {write_val}
                        Ok(())
                    }})?;"#
                }
                .into()
            }
            ty::Path(_) => format!("protocol.write_struct({ident})?;").into(),
            ty::Arc(ty) => self.codegen_encode_ty(ty, ident),
            _ => unimplemented!(),
        }
    }

    fn is_i32_enum(&self, def_id: DefId) -> bool {
        let item = self.expect_item(def_id);
        match &*item {
            rir::Item::Enum(e) if e.repr == Some(EnumRepr::I32) => return true,
            _ => {}
        }

        false
    }

    pub(crate) fn codegen_encode_field(&self, id: i16, ty: &Ty, ident: FastStr) -> FastStr {
        match &ty.kind {
            ty::String => format!("protocol.write_string_field({id}, {ident})?;").into(),
            ty::FastStr => {
                format!("protocol.write_faststr_field({id}, ({ident}).clone())?;").into()
            }
            ty::Void => "".into(),
            ty::U8 => format!("protocol.write_byte_field({id}, *{ident})?;").into(),
            ty::Bool => format!("protocol.write_bool_field({id}, *{ident})?;").into(),
            ty::BytesVec => format!("protocol.write_bytes_vec_field({id}, {ident})?;").into(),
            ty::Bytes => format!("protocol.write_bytes_field({id}, ({ident}).clone())?;").into(),
            ty::I8 => format!("protocol.write_i8_field({id}, *{ident})?;").into(),
            ty::I16 => format!("protocol.write_i16_field({id}, *{ident})?;").into(),
            ty::I32 => format!("protocol.write_i32_field({id}, *{ident})?;").into(),
            ty::I64 => format!("protocol.write_i64_field({id}, *{ident})?;").into(),
            ty::F64 => format!("protocol.write_double_field({id}, *{ident})?;").into(),
            ty::Uuid => format!("protocol.write_uuid_field({id}, *{ident})?;").into(),
            ty::Vec(ty) => {
                let el_ttype = self.ttype(ty);
                let write_el = self.codegen_encode_ty(ty, "val".into());

                format! {
                    r#"protocol.write_list_field({id}, {el_ttype}, &{ident}, |protocol, val| {{
                        {write_el}
                        Ok(())
                    }})?;"#
                }
                .into()
            }
            ty::Set(ty) => {
                let write_el = self.codegen_encode_ty(ty, "val".into());
                let el_ttype = self.ttype(ty);

                format! {
                    r#"protocol.write_set_field({id}, {el_ttype}, &{ident}, |protocol, val| {{
                        {write_el}
                        Ok(())
                    }})?;"#
                }
                .into()
            }
            ty::Map(k, v) => {
                let key_ttype = self.ttype(k);
                let val_ttype = self.ttype(v);
                let write_key = self.codegen_encode_ty(k, "key".into());
                let write_val = self.codegen_encode_ty(v, "val".into());

                format! {
                    r#"protocol.write_map_field({id}, {key_ttype}, {val_ttype}, &{ident}, |protocol, key| {{
                        {write_key}
                        Ok(())
                    }}, |protocol, val| {{
                        {write_val}
                        Ok(())
                    }})?;"#
                }
                .into()
            }
            ty::Path(p) if self.is_i32_enum(p.did) => {
                let v = match self
                    .cx
                    .node_tags(p.did)
                    .unwrap()
                    .get::<EnumMode>()
                    .copied()
                    .unwrap_or(EnumMode::Enum)
                {
                    EnumMode::NewType => format!("({ident}).inner()"),
                    EnumMode::Enum => format!("(*{ident}).into()"),
                };

                format!("protocol.write_i32_field({id}, {v})?;").into()
            }
            ty::Path(p) => match self.cx.expect_item(p.did).as_ref() {
                rir::Item::NewType(nt) => {
                    let ttype = self.ttype(&nt.ty);
                    format!("protocol.write_struct_field({id}, {ident}, {ttype})?;").into()
                }
                _ => format!(
                    "protocol.write_struct_field({id}, {ident}, ::pilota::thrift::TType::Struct)?;"
                )
                .into(),
            },
            ty::Arc(ty) => self.codegen_encode_field(id, ty, ident),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_ty_size(&self, ty: &Ty, ident: FastStr) -> FastStr {
        match &ty.kind {
            ty::String => format!("protocol.string_len({ident})").into(),
            ty::FastStr => format!("protocol.faststr_len({ident})").into(),
            ty::Void => "protocol.void_len()".into(),
            ty::U8 => format!("protocol.byte_len(*{ident})").into(),
            ty::Bool => format!("protocol.bool_len(*{ident})").into(),
            ty::BytesVec => format!("protocol.bytes_vec_len({ident})").into(),
            ty::Bytes => format!("protocol.bytes_len({ident})").into(),
            ty::I8 => format!("protocol.i8_len(*{ident})").into(),
            ty::I16 => format!("protocol.i16_len(*{ident})").into(),
            ty::I32 => format!("protocol.i32_len(*{ident})").into(),
            ty::I64 => format!("protocol.i64_len(*{ident})").into(),
            ty::F64 => format!("protocol.double_len(*{ident})").into(),
            ty::Uuid => format!("protocol.uuid_len(*{ident})").into(),
            ty::Vec(el) => {
                let add_el = self.codegen_ty_size(el, "el".into());
                let el_ttype = self.ttype(el);
                format! {
                    r#"protocol.list_len({el_ttype}, {ident}, |protocol, el| {{
                        {add_el}
                    }})"#
                }
                .into()
            }
            ty::Set(el) => {
                let add_el = self.codegen_ty_size(el, "el".into());
                let el_ttype = self.ttype(el);
                format! {
                    r#"protocol.set_len({el_ttype}, {ident}, |protocol, el| {{
                        {add_el}
                    }})"#
                }
                .into()
            }
            ty::Map(k, v) => {
                let add_key = self.codegen_ty_size(k, "key".into());
                let add_val = self.codegen_ty_size(v, "val".into());
                let k_ttype = self.ttype(k);
                let v_ttype = self.ttype(v);

                format! {
                    r#"protocol.map_len({k_ttype}, {v_ttype}, {ident}, |protocol, key| {{
                        {add_key}
                    }}, |protocol, val| {{
                        {add_val}
                    }})"#
                }
                .into()
            }
            ty::Path(_) => format!("protocol.struct_len({ident})").into(),
            ty::Arc(ty) => self.codegen_ty_size(ty, ident),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_field_size(&self, ty: &Ty, id: i16, ident: FastStr) -> FastStr {
        match &ty.kind {
            ty::String => format!("protocol.string_field_len(Some({id}), &{ident})").into(),
            ty::FastStr => format!("protocol.faststr_field_len(Some({id}), {ident})").into(),
            ty::Void => "0".into(),
            ty::U8 => format!("protocol.byte_field_len(Some({id}), *{ident})").into(),
            ty::Bool => format!("protocol.bool_field_len(Some({id}), *{ident})").into(),
            ty::BytesVec => format!("protocol.bytes_vec_field_len(Some({id}), {ident})").into(),
            ty::Bytes => format!("protocol.bytes_field_len(Some({id}), {ident})").into(),
            ty::I8 => format!("protocol.i8_field_len(Some({id}), *{ident})").into(),
            ty::I16 => format!("protocol.i16_field_len(Some({id}), *{ident})").into(),
            ty::I32 => format!("protocol.i32_field_len(Some({id}), *{ident})").into(),
            ty::I64 => format!("protocol.i64_field_len(Some({id}), *{ident})").into(),
            ty::F64 => format!("protocol.double_field_len(Some({id}), *{ident}) ").into(),
            ty::Uuid => format!("protocol.uuid_field_len(Some({id}), *{ident}) ").into(),
            ty::Vec(el) => {
                let add_el = self.codegen_ty_size(el, "el".into());
                let el_ttype = self.ttype(el);
                format! {
                    r#"protocol.list_field_len(Some({id}), {el_ttype}, {ident}, |protocol, el| {{
                        {add_el}
                    }})"#
                }
                .into()
            }
            ty::Set(el) => {
                let add_el = self.codegen_ty_size(el, "el".into());
                let el_ttype = self.ttype(el);
                format! {
                    r#"protocol.set_field_len(Some({id}), {el_ttype}, {ident}, |protocol, el| {{
                        {add_el}
                    }})"#
                }
                .into()
            }
            ty::Map(k, v) => {
                let add_key = self.codegen_ty_size(k, "key".into());
                let add_val = self.codegen_ty_size(v, "val".into());
                let k_ttype = self.ttype(k);
                let v_ttype = self.ttype(v);

                format!("protocol.map_field_len(Some({id}), {k_ttype}, {v_ttype}, {ident}, |protocol, key| {{ {add_key} }}, |protocol, val| {{ {add_val} }})").into()
            }
            ty::Path(p) if self.is_i32_enum(p.did) => {
                let v = match self
                    .cx
                    .node_tags(p.did)
                    .unwrap()
                    .get::<EnumMode>()
                    .copied()
                    .unwrap_or(EnumMode::Enum)
                {
                    EnumMode::NewType => format!("({ident}).inner()"),
                    EnumMode::Enum => format!("(*{ident}).into()"),
                };
                format!("protocol.i32_field_len(Some({id}), {v})").into()
            }
            ty::Path(_) => format!("protocol.struct_field_len(Some({id}), {ident})").into(),
            ty::Arc(ty) => self.codegen_field_size(ty, id, ident),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_decode_ty(&self, helper: &DecodeHelper, ty: &Ty) -> FastStr {
        match &ty.kind {
            ty::String => helper.codegen_read_string(),
            ty::FastStr => helper.codegen_read_faststr(),
            ty::Void => {
                let read_struct_begin = helper.codegen_read_struct_begin();
                let read_struct_end = helper.codegen_read_struct_end();
                format! {
                    r#"{{
                    {read_struct_begin}
                    {read_struct_end}
                    ()
                }}"#
                }
                .into()
            }
            ty::U8 => helper.codegen_read_byte(),
            ty::Bool => helper.codegen_read_bool(),
            ty::BytesVec => helper.codegen_read_bytes_vec(),
            ty::Bytes => helper.codegen_read_bytes(),
            ty::I8 => helper.codegen_read_i8(),
            ty::I16 => helper.codegen_read_i16(),
            ty::I32 => helper.codegen_read_i32(),
            ty::I64 => helper.codegen_read_i64(),
            ty::F64 => helper.codegen_read_double(),
            ty::Uuid => helper.codegen_read_uuid(),
            ty::Vec(ty) => {
                let read_list_begin = helper.codegen_read_list_begin();
                let read_list_end = helper.codegen_read_list_end();
                let read_el = self.codegen_decode_ty(helper, ty);
                let ty_rust_name = self.codegen_item_ty(ty.kind.clone());
                if !helper.is_async {
                    format! {
                        r#"unsafe {{
                            let list_ident = {read_list_begin};
                            let mut val: Vec<{ty_rust_name}> = Vec::with_capacity(list_ident.size);
                            for i in 0..list_ident.size {{
                                val.as_mut_ptr().offset(i as isize).write({read_el});
                            }};
                            val.set_len(list_ident.size);
                            {read_list_end};
                            val
                        }}"#
                    }
                    .into()
                } else {
                    format! {
                        r#"{{
                            let list_ident = {read_list_begin};
                            let mut val = Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {{
                                val.push({read_el});
                            }};
                            {read_list_end};
                            val
                        }}"#
                    }
                    .into()
                }
            }
            ty::Set(ty) => {
                let read_set_begin = helper.codegen_read_set_begin();
                let read_set_end = helper.codegen_read_set_end();
                let read_el = self.codegen_decode_ty(helper, ty);
                format! {r#"{{let list_ident = {read_set_begin};
                    let mut val = ::std::collections::HashSet::with_capacity_and_hasher(
                        list_ident.size,
                        ::pilota::Hasher::new(),
                    );
                    for _ in 0..list_ident.size {{
                        val.insert({read_el});
                    }};
                    {read_set_end};
                    val}}"#}
                .into()
            }
            ty::Map(key_ty, val_ty) => {
                let read_el_key = self.codegen_decode_ty(helper, key_ty);
                let read_el_val = self.codegen_decode_ty(helper, val_ty);

                let read_map_begin = helper.codegen_read_map_begin();
                let read_map_end = helper.codegen_read_map_end();

                format! {
                    r#"{{
                        let map_ident = {read_map_begin};
                        let mut val = ::std::collections::HashMap::with_capacity_and_hasher(
                            map_ident.size,
                            ::pilota::Hasher::new(),
                        );
                        for _ in 0..map_ident.size {{
                            val.insert({read_el_key}, {read_el_val});
                        }}
                        {read_map_end};
                        val
                    }}"#
                }
                .into()
            }
            ty::Path(_) => helper
                .codegen_item_decode(format!("{}", self.codegen_item_ty(ty.kind.clone())).into()),
            ty::Arc(ty) => {
                let inner = self.codegen_decode_ty(helper, ty);
                format!("::std::sync::Arc::new({inner})").into()
            }
            _ => unimplemented!(),
        }
    }
}
