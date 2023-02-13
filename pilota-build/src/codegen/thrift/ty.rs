use proc_macro2::TokenStream;
use quote::quote;

use super::{decode_helper::DecodeHelper, ThriftBackend};
use crate::{
    db::RirDatabase,
    middle::{rir, ty, ty::Ty},
};

impl ThriftBackend {
    pub(crate) fn ttype(&self, ty: &Ty) -> TokenStream {
        match &ty.kind {
            ty::String | ty::FastStr => quote! {::pilota::thrift::TType::Binary},
            ty::Void => quote! {::pilota::thrift::TType::Void},
            ty::U8 => quote! {::pilota::thrift::TType::I8},
            ty::Bool => quote! {::pilota::thrift::TType::Bool},
            ty::BytesVec | ty::Bytes => quote! {::pilota::thrift::TType::Binary},
            ty::I8 => quote! {::pilota::thrift::TType::I8},
            ty::I16 => quote! { ::pilota::thrift::TType::I16 },
            ty::I32 => quote! { ::pilota::thrift::TType::I32 },
            ty::I64 => quote! { ::pilota::thrift::TType::I64 },
            ty::F64 => quote! { ::pilota::thrift::TType::Double },
            ty::Vec(_) => quote! { ::pilota::thrift::TType::List },
            ty::Set(_) => quote! { ::pilota::thrift::TType::Set },
            ty::Map(_, _) => quote! { ::pilota::thrift::TType::Map },
            ty::Path(path) => {
                let item = self.expect_item(path.did);
                match &*item {
                    rir::Item::Message(_) => {
                        quote! { ::pilota::thrift::TType::Struct }
                    }
                    rir::Item::Enum(_) => {
                        quote! { ::pilota::thrift::TType::I32 }
                    }
                    rir::Item::NewType(t) => self.ttype(&t.ty),
                    _ => panic!("unsupported type {:?}", item),
                }
            }
            ty::Arc(ty) => self.ttype(ty),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_encode_ty(&self, ty: &Ty, ident: &TokenStream) -> TokenStream {
        match &ty.kind {
            ty::String => quote! { protocol.write_string(#ident)?; },
            ty::FastStr => quote! { protocol.write_faststr((#ident).clone())?; },
            ty::Void => quote! {
                protocol.write_struct_begin(&*::pilota::thrift::VOID_IDENT)?;
                protocol.write_struct_end()?;
            },
            ty::U8 => quote! { protocol.write_byte(*#ident)?; },
            ty::Bool => quote! { protocol.write_bool(*#ident)?; },
            ty::BytesVec => quote! { protocol.write_bytes_vec(&#ident)?;},
            ty::Bytes => quote! { protocol.write_bytes(#ident.clone())?;},
            ty::I8 => quote! { protocol.write_i8(*#ident)?; },
            ty::I16 => quote! { protocol.write_i16(*#ident)?; },
            ty::I32 => quote! { protocol.write_i32(*#ident)?; },
            ty::I64 => quote! { protocol.write_i64(*#ident)?; },
            ty::F64 => quote! { protocol.write_double(*#ident)?; },
            ty::Vec(ty) => {
                let el_ttype = self.ttype(ty);
                let write_el = self.codegen_encode_ty(ty, &quote!(val));

                quote! {
                    protocol.write_list(#el_ttype, &#ident, |protocol, val| {
                        #write_el
                        Ok(())
                    })?;
                }
            }
            ty::Set(ty) => {
                let write_el = self.codegen_encode_ty(ty, &quote!(val));
                let el_ttype = self.ttype(ty);

                quote! {
                    protocol.write_set(#el_ttype, &#ident, |protocol, val| {
                        #write_el
                        Ok(())
                    })?;
                }
            }
            ty::Map(k, v) => {
                let key_ttype = self.ttype(k);
                let val_ttype = self.ttype(v);
                let write_key = self.codegen_encode_ty(k, &quote!(key));
                let write_val = self.codegen_encode_ty(v, &quote!(val));
                quote! {
                    protocol.write_map(#key_ttype, #val_ttype, &#ident, |protocol, key| {
                        #write_key
                        Ok(())
                    }, |protocol, val| {
                        #write_val
                        Ok(())
                    })?;
                }
            }
            ty::Path(_) => quote! { ::pilota::thrift::Message::encode(#ident, protocol)?; },
            ty::Arc(ty) => self.codegen_encode_ty(ty, ident),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_encode_field(
        &self,
        id: i16,
        ty: &Ty,
        ident: &TokenStream,
    ) -> TokenStream {
        match &ty.kind {
            ty::String => quote! { protocol.write_string_field(#id, &#ident)?; },
            ty::FastStr => quote! { protocol.write_faststr_field(#id, (#ident).clone())?; },
            ty::Void => quote! {
                protocol.write_void_field(#id)?;
            },
            ty::U8 => quote! { protocol.write_byte_field(#id, *#ident)?; },
            ty::Bool => quote! { protocol.write_bool_field(#id, *#ident)?; },
            ty::BytesVec => quote! { protocol.write_bytes_vec_field(#id, &#ident)?;},
            ty::Bytes => quote! { protocol.write_bytes_field(#id, (#ident).clone())?;},
            ty::I8 => quote! { protocol.write_i8_field(#id, *#ident)?; },
            ty::I16 => quote! { protocol.write_i16_field(#id, *#ident)?; },
            ty::I32 => quote! { protocol.write_i32_field(#id, *#ident)?; },
            ty::I64 => quote! { protocol.write_i64_field(#id, *#ident)?; },
            ty::F64 => quote! { protocol.write_double_field(#id, *#ident)?; },
            ty::Vec(ty) => {
                let el_ttype = self.ttype(ty);
                let write_el = self.codegen_encode_ty(ty, &quote!(val));

                quote! {
                    protocol.write_list_field(#id, #el_ttype, &#ident, |protocol, val| {
                        #write_el
                        Ok(())
                    })?;
                }
            }
            ty::Set(ty) => {
                let write_el = self.codegen_encode_ty(ty, &quote!(val));
                let el_ttype = self.ttype(ty);

                quote! {
                    protocol.write_set_field(#id, #el_ttype, &#ident, |protocol, val| {
                        #write_el
                        Ok(())
                    })?;
                }
            }
            ty::Map(k, v) => {
                let key_ttype = self.ttype(k);
                let val_ttype = self.ttype(v);
                let write_key = self.codegen_encode_ty(k, &quote!(key));
                let write_val = self.codegen_encode_ty(v, &quote!(val));
                quote! {
                    protocol.write_map_field(#id, #key_ttype, #val_ttype, &#ident, |protocol, key| {
                        #write_key
                        Ok(())
                    }, |protocol, val| {
                        #write_val
                        Ok(())
                    })?;
                }
            }
            ty::Path(_) => quote! { protocol.write_message(#id, #ident)?; },
            ty::Arc(ty) => self.codegen_encode_field(id, ty, ident),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_ty_size(&self, ty: &Ty, ident: &TokenStream) -> TokenStream {
        match &ty.kind {
            ty::String => quote! { protocol.write_string_len(&#ident) },
            ty::FastStr => quote! { protocol.write_faststr_len(#ident) },
            ty::Void => {
                quote! { protocol.write_void_len() }
            }
            ty::U8 => {
                quote! { protocol.write_byte_len(*#ident) }
            }
            ty::Bool => quote! { protocol.write_bool_len(*#ident) },
            ty::BytesVec => quote! { protocol.write_bytes_vec_len(#ident)},
            ty::Bytes => quote! { protocol.write_bytes_len(#ident)},
            ty::I8 => quote! { protocol.write_i8_len(*#ident) },
            ty::I16 => quote! { protocol.write_i16_len(*#ident) },
            ty::I32 => quote! { protocol.write_i32_len(*#ident) },
            ty::I64 => quote! { protocol.write_i64_len(*#ident) },
            ty::F64 => quote! { protocol.write_double_len(*#ident) },
            ty::Vec(el) => {
                let add_el = self.codegen_ty_size(el, &quote!(el));
                let el_ttype = self.ttype(el);
                quote! {
                    protocol.write_list_len(#el_ttype, #ident, |protocol, el| {
                        #add_el
                    })
                }
            }
            ty::Set(el) => {
                let add_el = self.codegen_ty_size(el, &quote!(el));
                let el_ttype = self.ttype(el);
                quote! {
                    protocol.write_set_len(#el_ttype, #ident, |protocol, el| {
                        #add_el
                    })
                }
            }
            ty::Map(k, v) => {
                let add_key = self.codegen_ty_size(k, &quote!(key));
                let add_val = self.codegen_ty_size(v, &quote!(val));
                let k_ttype = self.ttype(k);
                let v_ttype = self.ttype(v);

                quote! {
                    protocol.write_map_len(#k_ttype, #v_ttype, #ident, |protocol, key| {
                        #add_key
                    }, |protocol, val| {
                        #add_val
                    })
                }
            }
            ty::Path(_) => quote! { ::pilota::thrift::Message::size(#ident, protocol) },
            ty::Arc(ty) => self.codegen_ty_size(ty, ident),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_field_size(&self, ty: &Ty, id: i16, ident: &TokenStream) -> TokenStream {
        match &ty.kind {
            ty::String => quote! { protocol.write_string_field_len(Some(#id), &#ident) },
            ty::FastStr => quote! { protocol.write_faststr_field_len(Some(#id), #ident) },
            ty::Void => {
                quote! { protocol.write_void_field_len(Some(#id)) }
            }
            ty::U8 => {
                quote! { protocol.write_byte_field_len(Some(#id), *#ident) }
            }
            ty::Bool => quote! { protocol.write_bool_field_len(Some(#id), *#ident) },
            ty::BytesVec => quote! { protocol.write_bytes_vec_field_len(Some(#id), #ident)},
            ty::Bytes => quote! { protocol.write_bytes_field_len(Some(#id), #ident)},
            ty::I8 => quote! { protocol.write_i8_field_len(Some(#id), *#ident) },
            ty::I16 => quote! { protocol.write_i16_field_len(Some(#id), *#ident) },
            ty::I32 => quote! { protocol.write_i32_field_len(Some(#id), *#ident) },
            ty::I64 => quote! { protocol.write_i64_field_len(Some(#id), *#ident) },
            ty::F64 => quote! { protocol.write_double_field_len(Some(#id), *#ident) },
            ty::Vec(el) => {
                let add_el = self.codegen_ty_size(el, &quote! { el });
                let el_ttype = self.ttype(el);
                quote! {
                    protocol.write_list_field_len(Some(#id), #el_ttype, #ident, |protocol, el| {
                        #add_el
                    })
                }
            }
            ty::Set(el) => {
                let add_el = self.codegen_ty_size(el, &quote! { el });
                let el_ttype = self.ttype(el);
                quote! {
                    protocol.write_set_field_len(Some(#id), #el_ttype, #ident, |protocol, el| {
                        #add_el
                    })
                }
            }
            ty::Map(k, v) => {
                let add_key = self.codegen_ty_size(k, &quote! { key });
                let add_val = self.codegen_ty_size(v, &quote! { val });
                let k_ttype = self.ttype(k);
                let v_ttype = self.ttype(v);

                quote! {
                    protocol.write_map_field_len(Some(#id), #k_ttype, #v_ttype, #ident, |protocol, key| { #add_key }, |protocol, val| { #add_val })
                }
            }
            ty::Path(_) => quote! { ::pilota::thrift::Message::size(#ident, protocol) },
            ty::Arc(ty) => self.codegen_field_size(ty, id, ident),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_decode_ty(&self, helper: &DecodeHelper, ty: &Ty) -> TokenStream {
        match &ty.kind {
            ty::String => helper.codegen_read_string(),
            ty::FastStr => helper.codegen_read_faststr(),
            ty::Void => {
                let read_struct_begin = helper.codegen_read_struct_begin();
                let read_struct_end = helper.codegen_read_struct_end();
                quote! {{
                    #read_struct_begin;
                    #read_struct_end;
                    ()
                }}
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
            ty::Vec(ty) => {
                let read_list_begin = helper.codegen_read_list_begin();
                let read_list_end = helper.codegen_read_list_end();
                let read_el = self.codegen_decode_ty(helper, ty);
                quote! {
                    {
                        let list_ident  = #read_list_begin;
                        let mut val = Vec::with_capacity(list_ident.size);
                        for _ in 0..list_ident.size {
                            val.push(#read_el);
                        };
                        #read_list_end;
                        val
                    }
                }
            }
            ty::Set(ty) => {
                let read_set_begin = helper.codegen_read_set_begin();
                let read_set_end = helper.codegen_read_set_end();
                let read_el = self.codegen_decode_ty(helper, ty);
                quote! {{
                    let list_ident  = #read_set_begin;
                    let mut val = ::std::collections::HashSet::with_capacity(list_ident.size);
                    for _ in 0..list_ident.size {
                        val.insert(#read_el);
                    };
                    #read_set_end;
                    val
                }}
            }
            ty::Map(key_ty, val_ty) => {
                let read_el_key = self.codegen_decode_ty(helper, key_ty);
                let read_el_val = self.codegen_decode_ty(helper, val_ty);

                let read_map_begin = helper.codegen_read_map_begin();
                let read_map_end = helper.codegen_read_map_end();

                quote! {
                    {
                        let map_ident = #read_map_begin;
                        let mut val = ::std::collections::HashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            let el_key = #read_el_key;
                            let el_val = #read_el_val;

                            val.insert(el_key, el_val);
                        }
                        #read_map_end;
                        val
                    }
                }
            }
            ty::Path(_) => helper.codegen_item_decode(),
            ty::Arc(ty) => {
                let inner = self.codegen_decode_ty(helper, ty);
                quote! { ::std::sync::Arc::new(#inner) }
            }
            _ => unimplemented!(),
        }
    }
}
