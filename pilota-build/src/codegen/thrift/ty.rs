use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{decode_helper::DecodeHelper, ThriftBackend};
use crate::{
    db::RirDatabase,
    middle::{rir, ty, ty::Ty},
};

impl ThriftBackend {
    pub(crate) fn ttype(&self, ty: &Ty) -> TokenStream {
        match &ty.kind {
            ty::String => quote! {::pilota::thrift::TType::String},
            ty::Void => quote! {::pilota::thrift::TType::Void},
            ty::U8 => quote! {::pilota::thrift::TType::I08},
            ty::Bool => quote! {::pilota::thrift::TType::Bool},
            ty::Bytes => quote! {::pilota::thrift::TType::String},
            ty::I8 => quote! {::pilota::thrift::TType::I08},
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
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_encode_ty(&self, ty: &Ty, ident: &Ident) -> TokenStream {
        match &ty.kind {
            ty::String => quote! { protocol.write_string(#ident)?; },
            ty::Void => quote! {
                protocol.write_struct_begin(&*::pilota::thrift::VOID_IDENT)?;
                protocol.write_struct_end()?;
            },
            ty::U8 => quote! { protocol.write_byte(*#ident)?; },
            ty::Bool => quote! { protocol.write_bool(*#ident)?; },
            ty::Bytes => quote! { protocol.write_bytes(&#ident)?;},
            ty::I8 => quote! { protocol.write_i8(*#ident)?; },
            ty::I16 => quote! { protocol.write_i16(*#ident)?; },
            ty::I32 => quote! { protocol.write_i32(*#ident)?; },
            ty::I64 => quote! { protocol.write_i64(*#ident)?; },
            ty::F64 => quote! { protocol.write_double(*#ident)?; },
            ty::Vec(ty) => {
                let el_ttype = self.ttype(ty);
                let write_el = self.codegen_encode_ty(ty, &format_ident!("val"));

                quote! {
                    let list_ident =::pilota::thrift::TListIdentifier{
                        element_type: #el_ttype,
                        size: #ident.len(),
                    };
                    protocol.write_list_begin(&list_ident)?;
                    for val in #ident {
                        #write_el
                    }
                    protocol.write_list_end()?;
                }
            }
            ty::Set(ty) => {
                let write_el = self.codegen_encode_ty(ty, &format_ident!("val"));
                let el_ttype = self.ttype(ty);

                quote! {
                    let list_ident =::pilota::thrift::TSetIdentifier{
                        element_type: #el_ttype,
                        size: #ident.len(),
                    };
                    protocol.write_set_begin(&list_ident)?;
                    for val in #ident {
                        #write_el
                    }
                    protocol.write_set_end()?;
                }
            }
            ty::Map(k, v) => {
                let key_ttype = self.ttype(k);
                let val_ttype = self.ttype(v);
                let write_key = self.codegen_encode_ty(k, &format_ident!("key"));
                let write_val = self.codegen_encode_ty(v, &format_ident!("val"));
                quote! {
                    let map_ident = ::pilota::thrift::TMapIdentifier{
                        key_type: #key_ttype,
                        value_type: #val_ttype,
                        size: #ident.len(),
                    };

                    protocol.write_map_begin(&map_ident)?;
                    for (key, val) in #ident.iter() {
                        #write_key
                        #write_val
                    }

                    protocol.write_map_end()?;
                }
            }
            ty::Path(_) => quote! { ::pilota::thrift::Message::encode(#ident, protocol)?; },
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_ty_size(&self, ty: &Ty, ident: &Ident) -> TokenStream {
        match &ty.kind {
            ty::String => quote! { protocol.write_string_len(&#ident) },
            ty::Void => {
                quote! { protocol.write_struct_begin_len(&*::pilota::thrift::VOID_IDENT) +  protocol.write_struct_end_len() }
            }
            ty::U8 => {
                quote! { protocol.write_byte_len(*#ident) }
            }
            ty::Bool => quote! { protocol.write_bool_len(*#ident) },
            ty::Bytes => quote! { protocol.write_bytes_len(#ident)},
            ty::I8 => quote! { protocol.write_i8_len(*#ident) },
            ty::I16 => quote! { protocol.write_i16_len(*#ident) },
            ty::I32 => quote! { protocol.write_i32_len(*#ident) },
            ty::I64 => quote! { protocol.write_i64_len(*#ident) },
            ty::F64 => quote! { protocol.write_double_len(*#ident) },
            ty::Vec(el) => {
                let add_el = self.codegen_ty_size(el, &format_ident!("el"));
                let el_ttype = self.ttype(el);
                quote! {
                    {
                        let list_ident = ::pilota::thrift::TListIdentifier{
                            element_type: #el_ttype,
                            size: #ident.len(),
                        };

                        protocol.write_list_begin_len(&list_ident) + {
                            let mut size = 0;
                            for el in #ident {
                                size += #add_el;
                            }
                            size
                        } + protocol.write_list_end_len()
                    }
                }
            }
            ty::Set(el) => {
                let add_el = self.codegen_ty_size(el, &format_ident!("el"));
                let el_ttype = self.ttype(el);
                quote! {
                    {
                        let set_id = ::pilota::thrift::TSetIdentifier{
                            element_type: #el_ttype,
                            size: #ident.len(),
                        };

                        protocol.write_set_begin_len(&set_id) + {
                            let mut size = 0;
                            for el in #ident {
                                size += #add_el;
                            }
                            size
                        } + protocol.write_set_end_len()
                    }
                }
            }
            ty::Map(k, v) => {
                let add_key = self.codegen_ty_size(k, &format_ident!("key"));
                let add_val = self.codegen_ty_size(v, &format_ident!("val"));
                let k_ttype = self.ttype(k);
                let v_ttype = self.ttype(v);

                quote! {
                    {
                        let map_id = ::pilota::thrift::TMapIdentifier {
                            key_type: #k_ttype,
                            value_type: #v_ttype,
                            size: #ident.len(),
                        };

                        protocol.write_map_begin_len(&map_id) + {
                            let mut size = 0;
                            for (key, val) in #ident {
                                size += #add_key;
                                size += #add_val;
                            }
                            size
                        } + protocol.write_map_end_len()
                    }
                }
            }
            ty::Path(_) => quote! { ::pilota::thrift::Size::size(#ident, protocol) },
            _ => unimplemented!(),
        }
    }

    pub(crate) fn codegen_decode_ty(&self, helper: &DecodeHelper, ty: &Ty) -> TokenStream {
        match &ty.kind {
            ty::String => helper.codegen_read_string(),
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
            _ => unimplemented!(),
        }
    }
}
