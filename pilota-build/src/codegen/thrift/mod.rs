use std::{ops::Deref, sync::Arc};

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::traits::CodegenBackend;
use crate::{
    middle::{
        context::Context,
        rir::{self, Enum, Field, Message, Method, NewType, Service},
    },
    symbol::{DefId, EnumRepr, IdentName},
    tags::thrift::EntryMessage,
};

mod ty;

pub use self::decode_helper::DecodeHelper;

mod decode_helper;

pub struct ThriftBackend {
    cx: Arc<Context>,
}

impl ThriftBackend {
    pub fn new(cx: Arc<Context>) -> Self {
        ThriftBackend { cx }
    }
}

impl Deref for ThriftBackend {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl ThriftBackend {
    fn codegen_encode_fields_size<'a>(
        &'a self,
        fields: &'a [Arc<rir::Field>],
    ) -> impl Iterator<Item = TokenStream> + 'a {
        fields.iter().map(|f| {
            let field_name = self.rust_name(f.did).as_syn_ident();
            let field_name_str = &**f.name;
            let ty = self.ttype(&f.ty);
            let field_id = f.id as i16;
            let write_field = self.codegen_ty_size(&f.ty, &format_ident!("value"));

            let is_optional = f.is_optional();

            let write = quote::quote! {
                protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                    name: Some(#field_name_str),
                    field_type: #ty,
                    id: Some(#field_id),
                }) + #write_field + protocol.write_field_end_len()
            };

            if is_optional {
                quote! {
                    if let Some(value) = self.#field_name.as_ref() {
                        #write
                    } else { 0 }
                }
            } else {
                quote! {{
                    let value = &self.#field_name;
                    #write
                }}
            }
        })
    }

    fn codegen_encode_fields<'a>(
        &'a self,
        fields: &'a [Arc<rir::Field>],
    ) -> impl Iterator<Item = TokenStream> + 'a {
        fields.iter().map(|f| {
            let field_name = self.rust_name(f.did).as_syn_ident();
            let field_name_str = &**f.name;
            let ty = self.ttype(&f.ty);
            let field_id = f.id as i16;
            let write_field = self.codegen_encode_ty(&f.ty, &format_ident!("value"));

            let is_optional = f.is_optional();

            let write = quote::quote! {
                protocol.write_field_begin(&::pilota::thrift::TFieldIdentifier {
                    name: Some(#field_name_str),
                    field_type: #ty,
                    id: Some(#field_id),
                })?;
                #write_field
                protocol.write_field_end()?;
            };

            if is_optional {
                quote! {
                    if let Some(value) = self.#field_name.as_ref() {
                        #write
                    };
                }
            } else {
                quote! {{
                    let value = &self.#field_name;
                    #write
                }}
            }
        })
    }

    fn codegen_impl_message(
        &self,
        name: &Ident,
        encode: TokenStream,
        size: TokenStream,
        decode: TokenStream,
        decode_async: TokenStream,
    ) -> TokenStream {
        quote! {
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for #name {
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(),::pilota::thrift::Error> {
                    #encode
                }

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self,::pilota::thrift::Error>  {
                    #decode
                }

                async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                    protocol: &mut::pilota::thrift::TAsyncBinaryProtocol<C>,
                ) -> ::std::result::Result<Self,::pilota::thrift::Error> {
                    #decode_async
                }
            }

            impl ::pilota::thrift::Size for #name {
                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &T) -> usize {
                    #size
                }
            }
        }
    }

    fn codegen_impl_message_with_helper<F: Fn(&DecodeHelper) -> TokenStream>(
        &self,
        name: &Ident,
        encode: TokenStream,
        size: TokenStream,
        decode: F,
    ) -> TokenStream {
        let decode_stream = decode(&DecodeHelper::new(false));
        let decode_async_stream = decode(&DecodeHelper::new(true));
        self.codegen_impl_message(name, encode, size, decode_stream, decode_async_stream)
    }

    fn codegen_decode(&self, helper: &DecodeHelper, s: &rir::Message) -> TokenStream {
        let mut required_field_names = Vec::with_capacity(s.fields.len());
        let mut optional_field_names = Vec::with_capacity(s.fields.len());
        s.fields.iter().for_each(|f| {
            if f.is_optional() {
                optional_field_names.push(self.rust_name(f.did).as_syn_ident())
            } else {
                required_field_names.push(self.rust_name(f.did).as_syn_ident())
            }
        });

        let read_struct_begin = helper.codegen_read_struct_begin();
        let read_struct_end = helper.codegen_read_struct_end();
        let read_fields = self.codegen_decode_fields(helper, &s.fields);
        let required_errs = required_field_names
            .iter()
            .map(|i| format!("field {} is required", i));

        quote! {
            #(let mut #required_field_names = None;)*
            #(let mut #optional_field_names = None;)*

            #read_struct_begin;
            #read_fields;
            #read_struct_end;

            #(let #required_field_names = if let Some(#required_field_names) = #required_field_names {
                #required_field_names
            } else {
                return Err(
                    ::pilota::thrift::Error::Protocol(
                        ::pilota::thrift::ProtocolError::new(
                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                             #required_errs.to_string()
                         )
                     )
                 )
            };)*

            let data = Self {
                #(#optional_field_names,)*
                #(#required_field_names,)*
            };
            Ok(data)
        }
    }

    #[inline]
    fn field_is_box(&self, f: &Field) -> bool {
        match self.adjust(f.did) {
            Some(a) => a.boxed(),
            None => false,
        }
    }

    fn codegen_entry_enum(
        &self,
        _def_id: DefId,
        _stream: &mut proc_macro2::TokenStream,
        _e: &rir::Enum,
    ) {
        // TODO
    }

    fn codegen_decode_fields<'a>(
        &'a self,
        helper: &DecodeHelper,
        fields: &'a [Arc<Field>],
    ) -> TokenStream {
        let read_field_begin = helper.codegen_read_field_begin();
        let match_fields = fields.iter().map(|f| {
            let field_ident = self.rust_name(f.did).as_syn_ident();
            let ttype = self.ttype(&f.ty);
            let mut read_field = self.codegen_decode_ty(helper, &f.ty);
            let field_id = f.id as i16;
            if self.field_is_box(f) {
                read_field = quote! {::std::boxed::Box::new(#read_field) };
            };

            quote! {
                Some(#field_id) if field_ident.field_type == #ttype  => {
                    #field_ident = Some(#read_field);
                },
            }
        });
        let skip_ttype = helper.codegen_skip_ttype(quote! { field_ident.field_type });
        let read_field_end = helper.codegen_read_field_end();
        quote! {
            loop {
                let field_ident = #read_field_begin;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    break;
                }
                let field_id = field_ident.id;
                match field_id {
                    #(#match_fields)*
                    _ => {
                        #skip_ttype;
                    },
                }

                #read_field_end;
            };
        }
    }
}

impl CodegenBackend for ThriftBackend {
    fn codegen_struct_impl(
        &self,
        def_id: DefId,
        stream: &mut proc_macro2::TokenStream,
        s: &Message,
    ) {
        let name = self.cx.rust_name(def_id);
        let name_str = &**s.name;
        let encode_fields = self.codegen_encode_fields(&s.fields);
        let encode_fields_size = self.codegen_encode_fields_size(&s.fields);
        stream.extend(self.codegen_impl_message_with_helper(
            &name.as_syn_ident(),
            quote! {
                let struct_ident =::pilota::thrift::TStructIdentifier {
                    name: #name_str,
                };

                protocol.write_struct_begin(&struct_ident)?;
                #(#encode_fields)*
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            },
            quote! {
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: #name_str,
                }) + #(#encode_fields_size+)*  protocol.write_field_stop_len() + protocol.write_struct_end_len()
            },
            |helper| self.codegen_decode(helper, s),
        ));
    }

    fn codegen_service_impl(
        &self,
        _def_id: DefId,
        _stream: &mut proc_macro2::TokenStream,
        _s: &Service,
    ) {
    }

    fn codegen_service_method(&self, _service_def_id: DefId, _m: &Method) -> TokenStream {
        TokenStream::default()
    }

    fn codegen_enum_impl(&self, def_id: DefId, stream: &mut proc_macro2::TokenStream, e: &Enum) {
        let name = self.rust_name(def_id).as_syn_ident();
        let is_entry_message = self.node_contains_tag::<EntryMessage>(def_id);
        match e.repr {
            Some(EnumRepr::I32) => stream.extend(self.codegen_impl_message_with_helper(
                &name,
                quote! {
                    protocol.write_i32(*self as i32)?;
                    Ok(())
                },
                {
                    quote! {
                            protocol.write_i32_len(*self as i32)
                    }
                },
                |helper| {
                    let read_i32 = helper.codegen_read_i32();
                    let err_msg_tmpl = format!("invalid enum value for {}, value: {{}}", name);
                    quote! {
                        let value = #read_i32;
                        Ok(Self::try_from(value).map_err(|err|
                            ::pilota::thrift::new_protocol_error(
                                ::pilota::thrift::ProtocolErrorKind::InvalidData,
                                format!(#err_msg_tmpl,  value)
                            ))?)
                    }
                },
            )),
            None if is_entry_message => self.codegen_entry_enum(def_id, stream, e),
            None => {
                let name = self.rust_name(def_id).as_syn_ident();
                let name_str = &**e.name;
                let encode_variants = e.variants.iter().map(|v| {
                    let variant_name = self.rust_name(v.did).as_syn_ident();
                    let variant_name_str = &**v.name;
                    assert_eq!(v.fields.len(), 1);
                    let ty = self.ttype(&v.fields[0]);
                    let variant_id = v.id.unwrap() as i16;
                    let encode = self.codegen_encode_ty(&v.fields[0], &format_ident!("value"));
                    quote! {
                        #name::#variant_name(ref value) => {
                            protocol.write_field_begin(&::pilota::thrift::TFieldIdentifier {
                                name: Some(#variant_name_str),
                                field_type: #ty,
                                id: Some(#variant_id),
                            })?;
                            #encode
                            protocol.write_field_end()?;
                        },
                    }
                });

                let variants_size = e.variants.iter().map(|v| {
                    let variant_name = self.rust_name(v.did).as_syn_ident();
                    let variant_name_str = &**v.name;
                    let ty = self.ttype(&v.fields[0]);
                    let variant_id = v.id.unwrap() as i16;
                    let size = self.codegen_ty_size(&v.fields[0], &format_ident!("value"));
                    quote! {
                        #name::#variant_name(ref value) => {
                            protocol.write_field_begin_len(&::pilota::thrift::TFieldIdentifier {
                                name: Some(#variant_name_str),
                                field_type: #ty,
                                id: Some(#variant_id),
                            }) + #size + protocol.write_field_end_len()
                        },
                    }
                });

                stream.extend(self.codegen_impl_message_with_helper(
                    &name,
                    quote! {
                        protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                            name: #name_str,
                        })?;
                        match self {
                            #(#encode_variants)*
                        }
                        protocol.write_field_stop()?;
                        protocol.write_struct_end()?;
                        Ok(())
                    },
                    quote! {
                        protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                            name: #name_str,
                        }) + match self {
                            #(#variants_size)*
                        } +  protocol.write_field_stop_len() + protocol.write_struct_end_len()
                    },
                    |helper| {
                        let read_struct_begin = helper.codegen_read_struct_begin();
                        let read_field_begin = helper.codegen_read_field_begin();
                        let read_field_end = helper.codegen_read_field_end();
                        let read_struct_end = helper.codegen_read_struct_end();
                        let skip = helper.codegen_skip_ttype(quote! { field_ident.field_type });
                        let fields = e.variants.iter().map(|v| {
                            let variant_name = self.cx.rust_name(v.did).as_syn_ident();
                            assert_eq!(v.fields.len(), 1);
                            let variant_id = v.id.unwrap() as i16;
                            let decode = self.codegen_decode_ty(helper, &v.fields[0]);
                            quote! {
                                Some(#variant_id) => {
                                    if ret.is_none() {
                                        ret = Some(#name::#variant_name(#decode));
                                    } else {
                                        return Err(::pilota::thrift::new_protocol_error(
                                            ::pilota::thrift::ProtocolErrorKind::InvalidData,
                                            "received multiple fields for union from remote Message"
                                        ));
                                    }
                                },
                            }
                        });
                        quote! {
                            let mut ret = None;
                            #read_struct_begin;
                            loop {
                                let field_ident = #read_field_begin;
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                    break;
                                }
                                let field_id = field_ident.id;
                                match field_id {
                                    #(#fields)*
                                    _ => {
                                        #skip;
                                    },
                                }
                            }
                            #read_field_end;
                            #read_struct_end;
                            if let Some(ret) = ret {
                                Ok(ret)
                            } else {
                                Err(::pilota::thrift::new_protocol_error(
                                    ::pilota::thrift::ProtocolErrorKind::InvalidData,
                                    "received empty union from remote Message")
                                )
                            }
                        }
                    },
                ))
            }
            #[allow(unreachable_patterns)]
            _ => {}
        }
    }

    fn codegen_newtype_impl(
        &self,
        def_id: DefId,
        stream: &mut proc_macro2::TokenStream,
        t: &NewType,
    ) {
        let name = self.rust_name(def_id).as_syn_ident();
        let encode = self.codegen_encode_ty(&t.ty, &format_ident!("value"));
        let encode_size = self.codegen_ty_size(&t.ty, &format_ident!("value"));

        stream.extend(self.codegen_impl_message_with_helper(
            &name,
            quote! {
                let value = &**self;
                #encode
                Ok(())
            },
            quote! {
                {
                    let value = &**self;
                    #encode_size
                }
            },
            |helper| {
                let decode = self.codegen_decode_ty(helper, &t.ty);
                quote! { Ok(#name(#decode)) }
            },
        ));
    }
}
