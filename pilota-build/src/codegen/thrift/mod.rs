use std::{ops::Deref, sync::Arc};

use faststr::FastStr;
use itertools::Itertools;

use super::traits::CodegenBackend;
use crate::{
    db::RirDatabase,
    middle::{
        context::Context,
        rir::{self, Enum, Field, Message, Method, NewType, Service},
    },
    rir::EnumVariant,
    symbol::{DefId, EnumRepr, Symbol},
    tags::thrift::EntryMessage,
    ty::TyKind,
};

mod ty;

pub use self::decode_helper::DecodeHelper;

mod decode_helper;

#[derive(Clone)]
pub struct ThriftBackend {
    cx: Context,
}

impl ThriftBackend {
    pub fn new(cx: Context) -> Self {
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
    ) -> impl Iterator<Item = FastStr> + 'a {
        fields.iter().map(|f| {
            let field_name = self.rust_name(f.did);
            let is_optional = f.is_optional();
            let field_id = f.id as i16;
            let write_field = if is_optional {
                self.codegen_field_size(&f.ty, field_id, "value".into())
            } else {
                self.codegen_field_size(&f.ty, field_id, format!("&self.{field_name}").into())
            };

            if is_optional {
                format!("self.{field_name}.as_ref().map_or(0, |value| {write_field})").into()
            } else {
                write_field
            }
        })
    }

    fn codegen_encode_fields<'a>(
        &'a self,
        fields: &'a [Arc<rir::Field>],
    ) -> impl Iterator<Item = FastStr> + 'a {
        fields.iter().map(|f| {
            let field_name = self.rust_name(f.did);
            let field_id = f.id as i16;
            let is_optional = f.is_optional();
            let write_field = if is_optional {
                self.codegen_encode_field(field_id, &f.ty, "value".into())
            } else {
                self.codegen_encode_field(field_id, &f.ty, format!("&self.{field_name}").into())
            };

            if is_optional {
                format! {
                    r#"if let Some(value) = self.{field_name}.as_ref() {{
                        {write_field}
                    }}"#
                }
                .into()
            } else {
                write_field
            }
        })
    }

    fn codegen_impl_message(
        &self,
        _def_id: DefId,
        name: Symbol,
        encode: String,
        size: String,
        decode: String,
        decode_async: String,
    ) -> String {
        // FIXME: here we will encounter problems when the type is indirect recursive
        // such as `struct A { a: Vec<A> }`.
        // Just use the boxed future for now.
        // let decode_async_fn = if self.cx().db.type_graph().is_cycled(def_id) {
        //     format!(
        //         r#"fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
        //         protocol: &'a mut T,
        //     ) -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output =
        //       ::std::result::Result<Self, ::pilota::thrift::ThriftException>> + Send
        // + 'a>> {{ ::std::boxed::Box::pin(async move {{ {decode_async}
        // }})     }}"#
        //     )
        // } else {
        //     format!(
        //         r#"async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
        //         protocol: &mut T,
        //     ) -> ::std::result::Result<Self,::pilota::thrift::ThriftException> {{
        //       {decode_async}
        //     }}"#
        //     )
        // };
        let decode_async_fn = format!(
            r#"fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
            __protocol: &'a mut T,
        ) -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>> + Send + 'a>> {{
            ::std::boxed::Box::pin(async move {{
                {decode_async}
            }})
        }}"#
        );
        format! {r#"
            impl ::pilota::thrift::Message for {name} {{
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    __protocol: &mut T,
                ) -> ::std::result::Result<(),::pilota::thrift::ThriftException> {{
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    {encode}
                }}

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    __protocol: &mut T,
                ) -> ::std::result::Result<Self,::pilota::thrift::ThriftException>  {{
                    #[allow(unused_imports)]
                    use ::pilota::{{thrift::TLengthProtocolExt, Buf}};
                    {decode}
                }}

                {decode_async_fn}

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {{
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    {size}
                }}
            }}"#}
    }

    fn codegen_impl_message_with_helper<F: Fn(&DecodeHelper) -> String>(
        &self,
        def_id: DefId,
        name: Symbol,
        encode: String,
        size: String,
        decode: F,
    ) -> String {
        let decode_stream = decode(&DecodeHelper::new(false));
        let decode_async_stream = decode(&DecodeHelper::new(true));
        self.codegen_impl_message(
            def_id,
            name,
            encode,
            size,
            decode_stream,
            decode_async_stream,
        )
    }

    fn codegen_decode(
        &self,
        helper: &DecodeHelper,
        s: &rir::Message,
        name: Symbol,
        keep: bool,
        is_arg: bool,
    ) -> String {
        let def_fields_num = if keep && is_arg && !helper.is_async {
            "let mut __pilota_fields_num = 0;"
        } else {
            ""
        };

        let mut def_fields = s
            .fields
            .iter()
            .map(|f| {
                let field_name = f.local_var_name();
                let mut v = "None".into();

                if let Some((default, is_const)) = self.cx.default_val(f) {
                    if is_const {
                        v = default;

                        if f.is_optional() {
                            v = format!("Some({v})").into()
                        }
                    }
                };

                let mut s = format!("let mut {field_name} = {v};");
                if keep && is_arg && !helper.is_async {
                    s.push_str("__pilota_fields_num += 1;");
                }
                s
            })
            .join("");

        if keep && !helper.is_async {
            def_fields.push_str("let mut _unknown_fields = ::pilota::LinkedBytes::new();");
        }

        let set_default_fields = s
            .fields
            .iter()
            .filter_map(|f| {
                let field_name = f.local_var_name();
                match self.cx.default_val(f) { Some((default, is_const)) => {
                    if !is_const {
                        if f.is_optional() {
                            Some(format! {
                                r#"if {field_name}.is_none() {{
                                {field_name} = Some({default});
                            }}"#
                            })
                        } else {
                            Some(format!(
                                r#"let {field_name} = {field_name}.unwrap_or_else(|| {default});"#
                            ))
                        }
                    } else {
                        None
                    }
                } _ => {
                    None
                }}
            })
            .join("\n");

        let read_struct_begin = helper.codegen_read_struct_begin();
        let read_struct_end = helper.codegen_read_struct_end();
        let read_fields = self.codegen_decode_fields(helper, &s.fields, keep, is_arg);

        let required_without_default_fields = s
            .fields
            .iter()
            .filter(|f| !f.is_optional() && self.default_val(f).is_none())
            .map(|f| (self.rust_name(f.did), f.local_var_name()))
            .collect_vec();

        let verify_required_fields = required_without_default_fields
            .iter()
            .map(|(s, v)| {
                format!(
                    r#"let Some({v}) = {v} else {{
                return ::std::result::Result::Err(
                    ::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field {s} is required".to_string()
                    )
                )
            }}; "#
                )
            })
            .join("\n");

        let read_fields = if helper.is_async {
            format! {
                r#"async {{
                    {read_fields}
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }}.await"#
            }
        } else {
            format! {
                r#"(|| {{
                    {read_fields}
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }})()"#
            }
        };

        let format_msg = format!("decode struct `{}` field(#{{}}) failed", name);

        let mut fields = s
            .fields
            .iter()
            .map(|f| format!("{}: {}", self.rust_name(f.did), f.local_var_name()))
            .join(",");

        if keep {
            if !fields.is_empty() {
                fields.push_str(", ");
            }
            if !helper.is_async {
                fields.push_str("_unknown_fields");
            } else {
                fields.push_str("_unknown_fields: ::pilota::LinkedBytes::new()");
            }
        }

        format! {
            r#"
            {def_fields_num}
            {def_fields}

            let mut __pilota_decoding_field_id = None;

            {read_struct_begin};
            if let ::std::result::Result::Err(mut err) = {read_fields} {{
                if let Some(field_id) = __pilota_decoding_field_id {{
                    err.prepend_msg(&format!("{format_msg}, caused by: ", field_id));
                }}
                return ::std::result::Result::Err(err);
            }};
            {read_struct_end};

            {verify_required_fields}

            {set_default_fields}

            let data = Self {{
                {fields}
            }};
            ::std::result::Result::Ok(data)
            "#
        }
    }

    #[inline]
    fn field_is_box(&self, f: &Field) -> bool {
        self.with_adjust(f.did, |adj| match adj {
            Some(a) => a.boxed(),
            None => false,
        })
    }

    fn codegen_entry_enum(&self, _def_id: DefId, _stream: &mut str, _e: &rir::Enum) {
        // TODO
    }

    fn codegen_decode_fields<'a>(
        &'a self,
        helper: &DecodeHelper,
        fields: &'a [Arc<Field>],
        keep: bool,
        is_arg: bool,
    ) -> String {
        let record_ptr = if keep && !helper.is_async {
            r#"let mut __pilota_offset = 0;
            let __pilota_begin_ptr = __protocol.buf().chunk().as_ptr();"#
        } else {
            ""
        };
        let read_field_begin = helper.codegen_read_field_begin();
        let field_begin_len = helper.codegen_field_begin_len(keep);
        let match_fields = fields
            .iter()
            .map(|f| {
                let field_ident = f.local_var_name();
                let ttype = self.ttype(&f.ty);
                let mut read_field = self.codegen_decode_ty(helper, &f.ty);
                let field_id = f.id as i16;
                if self.field_is_box(f) {
                    read_field = format!("::std::boxed::Box::new({read_field})").into();
                };

                if f.is_optional() || {
                    match self.cx.default_val(f) {
                        Some((_, is_const)) => !is_const,
                        _ => true,
                    }
                } {
                    read_field = format!("Some({read_field})").into();
                }

                let fields_num = if keep && !helper.is_async && is_arg {
                    "__pilota_fields_num -= 1;"
                } else {
                    ""
                };

                format!(
                    r#"Some({field_id}) if field_ident.field_type == {ttype}  => {{
                    {field_ident} = {read_field};
                    {fields_num}
                }},"#
                )
            })
            .join("");
        let mut skip_ttype = helper.codegen_skip_ttype("field_ident.field_type".into());
        if keep && !helper.is_async {
            skip_ttype = format!("__pilota_offset += {skip_ttype}")
        }

        let write_unknown_field = if keep && !helper.is_async {
            "_unknown_fields.push_back(__protocol.get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?);"
        } else {
            ""
        };

        let read_field_end = helper.codegen_read_field_end();
        let field_end_len = helper.codegen_field_end_len(keep);
        let field_stop_len = helper.codegen_field_stop_len(keep);

        let skip_all = if keep && !helper.is_async && is_arg {
            "if __pilota_fields_num == 0 {
                let __pilota_remaining = __protocol.buf().remaining();
                _unknown_fields.push_back(__protocol.get_bytes(None, __pilota_remaining - 2)?);
                break;
            }"
        } else {
            ""
        };

        format! {
            r#"loop {{
                {skip_all}
                {record_ptr}
                let field_ident = {read_field_begin};
                if field_ident.field_type == ::pilota::thrift::TType::Stop {{
                    {field_stop_len}
                    break;
                }} else {{
                    {field_begin_len}
                }}
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {{
                    {match_fields}
                    _ => {{
                        {skip_ttype};
                        {write_unknown_field}
                    }},
                }}

                {read_field_end};
                {field_end_len}

            }};"#
        }
    }
}

impl CodegenBackend for ThriftBackend {
    const PROTOCOL: &'static str = "thrift";

    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        let keep = self.keep_unknown_fields.contains(&def_id);
        let name = self.cx.rust_name(def_id);
        let mut encode_fields = self.codegen_encode_fields(&s.fields).join("");
        if keep {
            encode_fields.push_str(
                r#"for bytes in self._unknown_fields.list.iter() {
                                __protocol.write_bytes_without_len(bytes.clone());
                            }"#,
            );
        }
        let mut encode_fields_size = self
            .codegen_encode_fields_size(&s.fields)
            .map(|s| format!("{s} +"))
            .join("");
        if keep {
            encode_fields_size.push_str("self._unknown_fields.size() +");
        }
        stream.push_str(&self.codegen_impl_message_with_helper(
            def_id,
            name.clone(),
            format! {
                r#"let struct_ident =::pilota::thrift::TStructIdentifier {{
                    name: "{name}",
                }};

                __protocol.write_struct_begin(&struct_ident)?;
                {encode_fields}
                __protocol.write_field_stop()?;
                __protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
                "#
            },
            format! {
                r#"__protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {{
                    name: "{name}",
                }}) + {encode_fields_size} __protocol.field_stop_len() + __protocol.struct_end_len()"#
            },
            |helper| self.codegen_decode(helper, s, name.clone(), keep, self.is_arg(def_id)),
        ));
    }

    fn codegen_service_impl(&self, _def_id: DefId, _stream: &mut String, _s: &Service) {}

    fn codegen_service_method(&self, _service_def_id: DefId, _m: &Method) -> String {
        Default::default()
    }

    fn codegen_enum_impl(&self, def_id: DefId, stream: &mut String, e: &Enum) {
        let keep = self.keep_unknown_fields.contains(&def_id);
        let name = self.rust_name(def_id);
        let is_entry_message = self.node_contains_tag::<EntryMessage>(def_id);
        let v = "self.inner()";
        match e.repr {
            Some(EnumRepr::I32) => stream.push_str(&self.codegen_impl_message_with_helper(
                def_id,
                name.clone(),
                format! {
                    r#"__protocol.write_i32({v})?;
                    ::std::result::Result::Ok(())
                    "#
                },
                format!("__protocol.i32_len({v})"),
                |helper| {
                    let read_i32 = helper.codegen_read_i32();
                    let err_msg_tmpl = format!("invalid enum value for {}, value: {{}}", name);
                    format! {
                        r#"let value = {read_i32};
                        ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(|err|
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                format!("{err_msg_tmpl}", value)
                            ))?)"#
                    }
                },
            )),
            None if is_entry_message => self.codegen_entry_enum(def_id, stream, e),
            None => {
                let name = self.rust_name(def_id);
                let mut encode_variants = e
                    .variants
                    .iter()
                    .map(|v| {
                        let variant_name = self.rust_name(v.did);
                        assert_eq!(v.fields.len(), 1);
                        let variant_id = v.id.unwrap() as i16;
                        let encode =
                            self.codegen_encode_field(variant_id, &v.fields[0], "value".into());
                        format! {
                            r#"{name}::{variant_name}(value) => {{
                            {encode}
                        }},"#
                        }
                    })
                    .join("");
                if keep {
                    encode_variants.push_str(&format! {
                        "{name}::_UnknownFields(value) => {{
                                        for bytes in value.list.iter() {{
                                            __protocol.write_bytes_without_len(bytes.clone());
                                        }}
                                    }}",
                    });
                }

                if e.variants.is_empty() {
                    encode_variants.push_str("_ => {},");
                }

                let mut variants_size = e
                    .variants
                    .iter()
                    .map(|v| {
                        let variant_name = self.rust_name(v.did);
                        let variant_id = v.id.unwrap() as i16;
                        let size =
                            self.codegen_field_size(&v.fields[0], variant_id, "value".into());

                        format! {
                            r#"{name}::{variant_name}(value) => {{
                            {size}
                        }},"#
                        }
                    })
                    .join("");
                if keep {
                    variants_size.push_str(&format! {
                        "{name}::_UnknownFields(value) => {{
                                value.size()
                            }}",
                    })
                }

                if e.variants.is_empty() {
                    variants_size.push_str("_ => 0,");
                }

                let variant_is_void = |v: &EnumVariant| {
                    &*v.name.sym == "Ok" && v.fields.len() == 1 && v.fields[0].kind == TyKind::Void
                };

                stream.push_str(&self.codegen_impl_message_with_helper(def_id,
                    name.clone(),
                    format! {
                        r#"__protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {{
                            name: "{name}",
                        }})?;
                        match self {{
                            {encode_variants}
                        }}
                        __protocol.write_field_stop()?;
                        __protocol.write_struct_end()?;
                        ::std::result::Result::Ok(())"#
                    },
                        format! {
                            r#"__protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {{
                                name: "{name}",
                            }}) + match self {{
                                {variants_size}
                            }} +  __protocol.field_stop_len() + __protocol.struct_end_len()"#
                        },
                    |helper| {
                        let record_ptr = if keep && !helper.is_async {
                            r#"let mut __pilota_offset = 0;
                            let __pilota_begin_ptr = __protocol.buf().chunk().as_ptr();"#
                        } else {
                            ""
                        };
                        let read_struct_begin = helper.codegen_read_struct_begin();
                        let read_field_begin = helper.codegen_read_field_begin();
                        let field_begin_len = helper.codegen_field_begin_len(keep);
                        let read_field_end = helper.codegen_read_field_end();
                        let field_stop_len = helper.codegen_field_stop_len(keep);
                        let read_struct_end = helper.codegen_read_struct_end();
                        let mut skip = helper.codegen_skip_ttype("field_ident.field_type".into());
                        if keep && !helper.is_async {
                            skip = format!("__pilota_offset += {skip}")
                        }
                        let fields = e
                            .variants
                            .iter()
                            .flat_map(|v| {
                                if variant_is_void(v) {
                                    None
                                } else {
                                    let variant_name = self.cx.rust_name(v.did);
                                    assert_eq!(v.fields.len(), 1);
                                    let variant_id = v.id.unwrap() as i16;
                                    let decode = self.codegen_decode_ty(helper, &v.fields[0]);
                                    let decode_len =  if helper.is_async {
                                        Default::default()
                                    } else {
                                        let size = self.codegen_ty_size(
                                            &v.fields[0],
                                            "&field_ident".into()
                                        );
                                        if keep {
                                            format!(
                                                "__pilota_offset += {size};",
                                            )
                                        } else {
                                            format!(
                                                "{size};",
                                            )
                                        }
                                    };
                                    Some(format! {
                                        r#"Some({variant_id}) => {{
                                    if ret.is_none() {{
                                        let field_ident = {decode};
                                        {decode_len}
                                        ret = Some({name}::{variant_name}(field_ident));
                                    }} else {{
                                        return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                            "received multiple fields for union from remote Message"
                                        ));
                                    }}
                                }},"#
                                    })
                                }
                            })
                            .join("");
                        let write_unknown_field = if keep && !helper.is_async {
                            format!(
                                r#"if ret.is_none() {{
                                unsafe {{
                                    let mut __pilota_linked_bytes = ::pilota::LinkedBytes::new();
                                    __pilota_linked_bytes.push_back(__protocol.get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?);
                                    ret = Some({name}::_UnknownFields(__pilota_linked_bytes));
                                }}
                            }} else {{
                                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message"
                                ));
                            }}"#
                            )
                        } else {
                            Default::default()
                        };

                        let handle_none_ret: FastStr =
                            if e.variants.first().filter(|v| variant_is_void(v)).is_some() {
                                format!("::std::result::Result::Ok({name}::Ok(()))").into()
                            } else {
                                r#"::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received empty union from remote Message")
                                )"#.into()
                            };

                        format! {
                            r#"let mut ret = None;
                            {read_struct_begin};
                            loop {{
                                {record_ptr}
                                let field_ident = {read_field_begin};
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {{
                                    {field_stop_len}
                                    break;
                                }} else {{
                                    {field_begin_len}
                                }}
                                match field_ident.id {{
                                    {fields}
                                    _ => {{
                                        {skip};
                                        {write_unknown_field}
                                    }},
                                }}
                            }}
                            {read_field_end};
                            {read_struct_end};
                            if let Some(ret) = ret {{
                                ::std::result::Result::Ok(ret)
                            }} else {{
                                {handle_none_ret}
                            }}"#
                        }
                    },
                ))
            }
            #[allow(unreachable_patterns)]
            _ => {}
        }
    }

    fn codegen_newtype_impl(&self, def_id: DefId, stream: &mut String, t: &NewType) {
        let name = self.rust_name(def_id);
        let encode = self.codegen_encode_ty(&t.ty, "(&**self)".into());
        let encode_size = self.codegen_ty_size(&t.ty, "&**self".into());

        stream.push_str(&self.codegen_impl_message_with_helper(
            def_id,
            name.clone(),
            format! {
                r#"{encode}
                ::std::result::Result::Ok(())"#
            },
            format!("{encode_size}"),
            |helper| {
                let decode = self.codegen_decode_ty(helper, &t.ty);
                format!("::std::result::Result::Ok({name}({decode}))")
            },
        ));
    }

    fn cx(&self) -> &Context {
        &self.cx
    }
}
