use faststr::FastStr;

use super::{decode_helper::DecodeHelper, ThriftBackend};
use crate::{
    db::RirDatabase,
    middle::{rir, ty, ty::Ty},
    symbol::EnumRepr,
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
            ty::F64 | ty::OrderedF64 => "::pilota::thrift::TType::Double".into(),
            ty::Uuid => "::pilota::thrift::TType::Uuid".into(),
            ty::Vec(_) => "::pilota::thrift::TType::List".into(),
            ty::Set(_) | ty::BTreeSet(_) => "::pilota::thrift::TType::Set".into(),
            ty::Map(_, _) | ty::BTreeMap(_, _) => "::pilota::thrift::TType::Map".into(),
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
            ty::String => format!("__protocol.write_string({ident})?;").into(),
            ty::FastStr => format!("__protocol.write_faststr(({ident}).clone())?;").into(),
            ty::Void => r#"__protocol.write_struct_begin(&*::pilota::thrift::VOID_IDENT)?;__protocol.write_struct_end()?;"#.into(),
            ty::U8 => format!("__protocol.write_byte(*{ident})?;").into(),
            ty::Bool => format!("__protocol.write_bool(*{ident})?;").into(),
            ty::BytesVec => format!("__protocol.write_bytes_vec({ident})?;").into(),
            ty::Bytes => format!("__protocol.write_bytes({ident}.clone())?;").into(),
            ty::I8 => format!("__protocol.write_i8(*{ident})?;").into(),
            ty::I16 => format!("__protocol.write_i16(*{ident})?;").into(),
            ty::I32 => format!("__protocol.write_i32(*{ident})?;").into(),
            ty::I64 => format!("__protocol.write_i64(*{ident})?;").into(),
            ty::F64 => format!("__protocol.write_double(*{ident})?;").into(),
            ty::OrderedF64 => format!("__protocol.write_double({ident}.0)?;").into(),
            ty::Uuid => format!("__protocol.write_uuid({ident})?;").into(),
            ty::Vec(ty) => {
                let el_ttype = self.ttype(ty);
                let write_el = self.codegen_encode_ty(ty, "val".into());

                format! {
                    r#"__protocol.write_list({el_ttype}, &{ident}, |__protocol, val| {{
                        {write_el}
                        ::std::result::Result::Ok(())
                    }})?;"#
                }
                .into()
            }
            ty::Set(k) => {
                self.encode_set(k, ident, "set")
            }
            ty::BTreeSet(k) => {
                self.encode_set(k, ident, "btree_set")
            }
            ty::Map(k, v) => {
                self.encode_map(k, v, ident, "map")
            }
            ty::BTreeMap(k, v) => {
                self.encode_map(k, v, ident, "btree_map")
            }
            ty::Path(_) => format!("__protocol.write_struct({ident})?;").into(),
            ty::Arc(ty) => self.codegen_encode_ty(ty, ident),
            _ => unimplemented!(),
        }
    }

    #[inline]
    fn encode_set(&self, ty: &Ty, ident: FastStr, name: &str) -> FastStr {
        let write_el = self.codegen_encode_ty(ty, "val".into());
        let el_ttype = self.ttype(ty);
        format! {
            r#"__protocol.write_{name}({el_ttype}, &{ident}, |__protocol, val| {{
                {write_el}
                ::std::result::Result::Ok(())
            }})?;"#
        }
        .into()
    }

    #[inline]
    fn encode_map(&self, k: &Ty, v: &Ty, ident: FastStr, name: &str) -> FastStr {
        let key_ttype = self.ttype(k);
        let val_ttype = self.ttype(v);
        let write_key = self.codegen_encode_ty(k, "key".into());
        let write_val = self.codegen_encode_ty(v, "val".into());

        format! {
            r#"__protocol.write_{name}({key_ttype}, {val_ttype}, &{ident}, |__protocol, key| {{
                {write_key}
                ::std::result::Result::Ok(())
            }}, |__protocol, val| {{
                {write_val}
                ::std::result::Result::Ok(())
            }})?;"#
        }
        .into()
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
            ty::String => format!("__protocol.write_string_field({id}, {ident})?;").into(),
            ty::FastStr => {
                format!("__protocol.write_faststr_field({id}, ({ident}).clone())?;").into()
            }
            ty::Void => "".into(),
            ty::U8 => format!("__protocol.write_byte_field({id}, *{ident})?;").into(),
            ty::Bool => format!("__protocol.write_bool_field({id}, *{ident})?;").into(),
            ty::BytesVec => format!("__protocol.write_bytes_vec_field({id}, {ident})?;").into(),
            ty::Bytes => format!("__protocol.write_bytes_field({id}, ({ident}).clone())?;").into(),
            ty::I8 => format!("__protocol.write_i8_field({id}, *{ident})?;").into(),
            ty::I16 => format!("__protocol.write_i16_field({id}, *{ident})?;").into(),
            ty::I32 => format!("__protocol.write_i32_field({id}, *{ident})?;").into(),
            ty::I64 => format!("__protocol.write_i64_field({id}, *{ident})?;").into(),
            ty::F64 => format!("__protocol.write_double_field({id}, *{ident})?;").into(),
            ty::OrderedF64 => format!("__protocol.write_double_field({id}, {ident}.0)?;").into(),
            ty::Uuid => format!("__protocol.write_uuid_field({id}, *{ident})?;").into(),
            ty::Vec(ty) => {
                let el_ttype = self.ttype(ty);
                let write_el = self.codegen_encode_ty(ty, "val".into());

                format! {
                    r#"__protocol.write_list_field({id}, {el_ttype}, &{ident}, |__protocol, val| {{
                        {write_el}
                        ::std::result::Result::Ok(())
                    }})?;"#
                }
                .into()
            }
            ty::Set(k) => {
                self.encode_set_field(k, id, ident, "set")
            }
            ty::BTreeSet(k) => {
                self.encode_set_field(k, id, ident, "btree_set")
            }
            ty::Map(k, v) => {
                self.encode_map_field(k, v, id, ident, "map")
            }
            ty::BTreeMap(k, v) => {
                self.encode_map_field(k, v, id, ident, "btree_map")
            }
            ty::Path(p) if self.is_i32_enum(p.did) => {
                format!("__protocol.write_i32_field({id}, ({ident}).inner())?;").into()
            }
            ty::Path(p) => match self.cx.expect_item(p.did).as_ref() {
                rir::Item::NewType(nt) => {
                    let ttype = self.ttype(&nt.ty);
                    format!("__protocol.write_struct_field({id}, {ident}, {ttype})?;").into()
                }
                _ => format!(
                    "__protocol.write_struct_field({id}, {ident}, ::pilota::thrift::TType::Struct)?;"
                )
                .into(),
            },
            ty::Arc(ty) => self.codegen_encode_field(id, ty, ident),
            _ => unimplemented!(),
        }
    }

    #[inline]
    fn encode_set_field(&self, ty: &Ty, id: i16, ident: FastStr, name: &str) -> FastStr {
        let write_el = self.codegen_encode_ty(ty, "val".into());
        let el_ttype = self.ttype(ty);
        format! {
            r#"__protocol.write_{name}_field({id}, {el_ttype}, &{ident}, |__protocol, val| {{
                {write_el}
                ::std::result::Result::Ok(())
            }})?;"#
        }
        .into()
    }

    #[inline]
    fn encode_map_field(&self, k: &Ty, v: &Ty, id: i16, ident: FastStr, name: &str) -> FastStr {
        let key_ttype = self.ttype(k);
        let val_ttype = self.ttype(v);
        let write_key = self.codegen_encode_ty(k, "key".into());
        let write_val = self.codegen_encode_ty(v, "val".into());

        format! {
            r#"__protocol.write_{name}_field({id}, {key_ttype}, {val_ttype}, &{ident}, |__protocol, key| {{
                {write_key}
                ::std::result::Result::Ok(())
            }}, |__protocol, val| {{
                {write_val}
                ::std::result::Result::Ok(())
            }})?;"#
        }
        .into()
    }

    pub(crate) fn codegen_ty_size(&self, ty: &Ty, ident: FastStr) -> FastStr {
        match &ty.kind {
            ty::String => format!("__protocol.string_len({ident})").into(),
            ty::FastStr => format!("__protocol.faststr_len({ident})").into(),
            ty::Void => "__protocol.void_len()".into(),
            ty::U8 => format!("__protocol.byte_len(*{ident})").into(),
            ty::Bool => format!("__protocol.bool_len(*{ident})").into(),
            ty::BytesVec => format!("__protocol.bytes_vec_len({ident})").into(),
            ty::Bytes => format!("__protocol.bytes_len({ident})").into(),
            ty::I8 => format!("__protocol.i8_len(*{ident})").into(),
            ty::I16 => format!("__protocol.i16_len(*{ident})").into(),
            ty::I32 => format!("__protocol.i32_len(*{ident})").into(),
            ty::I64 => format!("__protocol.i64_len(*{ident})").into(),
            ty::F64 => format!("__protocol.double_len(*{ident})").into(),
            ty::OrderedF64 => format!("__protocol.double_len({ident}.0)").into(),
            ty::Uuid => format!("__protocol.uuid_len(*{ident})").into(),
            ty::Vec(el) => {
                let add_el = self.codegen_ty_size(el, "el".into());
                let el_ttype = self.ttype(el);
                format! {
                    r#"__protocol.list_len({el_ttype}, {ident}, |__protocol, el| {{
                        {add_el}
                    }})"#
                }
                .into()
            }
            ty::Set(k) => self.set_size(k, ident, "set"),
            ty::BTreeSet(k) => self.set_size(k, ident, "btree_set"),
            ty::Map(k, v) => self.map_size(k, v, ident, "map"),
            ty::BTreeMap(k, v) => self.map_size(k, v, ident, "btree_map"),
            ty::Path(_) => format!("__protocol.struct_len({ident})").into(),
            ty::Arc(ty) => self.codegen_ty_size(ty, ident),
            _ => unimplemented!(),
        }
    }

    #[inline]
    fn set_size(&self, ty: &Ty, ident: FastStr, name: &str) -> FastStr {
        let add_el = self.codegen_ty_size(ty, "el".into());
        let el_ttype = self.ttype(ty);
        format! {
            r#"__protocol.{name}_len({el_ttype}, {ident}, |__protocol, el| {{
                {add_el}
            }})"#
        }
        .into()
    }

    #[inline]
    fn map_size(&self, k: &Ty, v: &Ty, ident: FastStr, name: &str) -> FastStr {
        let add_key = self.codegen_ty_size(k, "key".into());
        let add_val = self.codegen_ty_size(v, "val".into());
        let k_ttype = self.ttype(k);
        let v_ttype = self.ttype(v);

        format! {
            r#"__protocol.{name}_len({k_ttype}, {v_ttype}, {ident}, |__protocol, key| {{
                {add_key}
            }}, |__protocol, val| {{
                {add_val}
            }})"#
        }
        .into()
    }

    pub(crate) fn codegen_field_size(&self, ty: &Ty, id: i16, ident: FastStr) -> FastStr {
        match &ty.kind {
            ty::String => format!("__protocol.string_field_len(Some({id}), &{ident})").into(),
            ty::FastStr => format!("__protocol.faststr_field_len(Some({id}), {ident})").into(),
            ty::Void => "0".into(),
            ty::U8 => format!("__protocol.byte_field_len(Some({id}), *{ident})").into(),
            ty::Bool => format!("__protocol.bool_field_len(Some({id}), *{ident})").into(),
            ty::BytesVec => format!("__protocol.bytes_vec_field_len(Some({id}), {ident})").into(),
            ty::Bytes => format!("__protocol.bytes_field_len(Some({id}), {ident})").into(),
            ty::I8 => format!("__protocol.i8_field_len(Some({id}), *{ident})").into(),
            ty::I16 => format!("__protocol.i16_field_len(Some({id}), *{ident})").into(),
            ty::I32 => format!("__protocol.i32_field_len(Some({id}), *{ident})").into(),
            ty::I64 => format!("__protocol.i64_field_len(Some({id}), *{ident})").into(),
            ty::F64 => format!("__protocol.double_field_len(Some({id}), *{ident}) ").into(),
            ty::OrderedF64 => format!("__protocol.double_field_len(Some({id}), {ident}.0) ").into(),
            ty::Uuid => format!("__protocol.uuid_field_len(Some({id}), *{ident}) ").into(),
            ty::Vec(el) => {
                let add_el = self.codegen_ty_size(el, "el".into());
                let el_ttype = self.ttype(el);
                format! {
                    r#"__protocol.list_field_len(Some({id}), {el_ttype}, {ident}, |__protocol, el| {{
                        {add_el}
                    }})"#
                }
                .into()
            }
            ty::Set(k) => self.set_field_size(k, id, ident, "set"),
            ty::BTreeSet(k) => self.set_field_size(k, id, ident, "btree_set"),
            ty::Map(k, v) => self.map_field_size(k, v, id, ident, "map"),
            ty::BTreeMap(k, v) => self.map_field_size(k, v, id, ident, "btree_map"),
            ty::Path(p) if self.is_i32_enum(p.did) => {
                format!("__protocol.i32_field_len(Some({id}), ({ident}).inner())").into()
            }
            ty::Path(_) => format!("__protocol.struct_field_len(Some({id}), {ident})").into(),
            ty::Arc(ty) => self.codegen_field_size(ty, id, ident),
            _ => unimplemented!(),
        }
    }

    #[inline]
    fn set_field_size(&self, ty: &Ty, id: i16, ident: FastStr, name: &str) -> FastStr {
        let add_el = self.codegen_ty_size(ty, "el".into());
        let el_ttype = self.ttype(ty);
        format! {
            r#"__protocol.{name}_field_len(Some({id}), {el_ttype}, {ident}, |__protocol, el| {{
                {add_el}
            }})"#
        }
        .into()
    }

    #[inline]
    fn map_field_size(&self, k: &Ty, v: &Ty, id: i16, ident: FastStr, name: &str) -> FastStr {
        let add_key = self.codegen_ty_size(k, "key".into());
        let add_val = self.codegen_ty_size(v, "val".into());
        let k_ttype = self.ttype(k);
        let v_ttype = self.ttype(v);

        format! {
            r#"__protocol.{name}_field_len(Some({id}), {k_ttype}, {v_ttype}, {ident}, |__protocol, key| {{
                {add_key}
            }}, |__protocol, val| {{
                {add_val}
            }})"#
        }
        .into()
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
            ty::OrderedF64 => {
                let read_double = helper.codegen_read_double();
                format!("::pilota::OrderedFloat({read_double})").into()
            }
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
                            let mut val: ::std::vec::Vec<{ty_rust_name}> = ::std::vec::Vec::with_capacity(list_ident.size);
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
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
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
            ty::Set(ty) => self.decode_set(
                ty,
                helper,
                "::pilota::AHashSet::with_capacity(list_ident.size)",
            ),
            ty::BTreeSet(ty) => self.decode_set(ty, helper, "::std::collections::BTreeSet::new()"),
            ty::Map(key_ty, val_ty) => self.decode_map(
                key_ty,
                val_ty,
                helper,
                "::pilota::AHashMap::with_capacity(map_ident.size)",
            ),
            ty::BTreeMap(key_ty, val_ty) => self.decode_map(
                key_ty,
                val_ty,
                helper,
                "::std::collections::BTreeMap::new()",
            ),
            ty::Path(_) => helper
                .codegen_item_decode(format!("{}", self.codegen_item_ty(ty.kind.clone())).into()),
            ty::Arc(ty) => {
                let inner = self.codegen_decode_ty(helper, ty);
                format!("::std::sync::Arc::new({inner})").into()
            }
            _ => unimplemented!(),
        }
    }

    #[inline]
    fn decode_set(&self, ty: &Ty, helper: &DecodeHelper, new: &str) -> FastStr {
        let read_set_begin = helper.codegen_read_set_begin();
        let read_set_end = helper.codegen_read_set_end();
        let read_el = self.codegen_decode_ty(helper, ty);
        format! {r#"{{let list_ident = {read_set_begin};
                    let mut val = {new};
                    for _ in 0..list_ident.size {{
                        val.insert({read_el});
                    }};
                    {read_set_end};
                    val}}"#}
        .into()
    }

    #[inline]
    fn decode_map(&self, key_ty: &Ty, val_ty: &Ty, helper: &DecodeHelper, new: &str) -> FastStr {
        let read_el_key = self.codegen_decode_ty(helper, key_ty);
        let read_el_val = self.codegen_decode_ty(helper, val_ty);
        let read_map_begin = helper.codegen_read_map_begin();
        let read_map_end = helper.codegen_read_map_end();
        format! {
                    r#"{{
                        let map_ident = {read_map_begin};
                        let mut val = {new};
                        for _ in 0..map_ident.size {{
                            val.insert({read_el_key}, {read_el_val});
                        }}
                        {read_map_end};
                        val
                    }}"#
        }
        .into()
    }
}
