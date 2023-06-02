use std::{ops::Deref, sync::Arc};

use faststr::FastStr;
use itertools::Itertools;

use super::traits::CodegenBackend;
use crate::{
    middle::{
        context::Context,
        rir::{self, Enum, Field, Message, Method, NewType, Service},
    },
    rir::EnumVariant,
    symbol::{DefId, EnumRepr, Symbol},
    tags::{thrift::EntryMessage, EnumMode},
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
        name: Symbol,
        encode: String,
        size: String,
        decode: String,
        decode_async: String,
    ) -> String {
        format! {r#"
            #[::async_trait::async_trait]
            impl ::pilota::thrift::Message for {name} {{
                fn encode<T: ::pilota::thrift::TOutputProtocol>(
                    &self,
                    protocol: &mut T,
                ) -> ::std::result::Result<(),::pilota::thrift::EncodeError> {{
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TOutputProtocolExt;
                    {encode}
                }}

                fn decode<T: ::pilota::thrift::TInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self,::pilota::thrift::DecodeError>  {{
                    {decode}
                }}

                async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                    protocol: &mut T,
                ) -> ::std::result::Result<Self,::pilota::thrift::DecodeError> {{
                    {decode_async}
                }}

                fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {{
                    #[allow(unused_imports)]
                    use ::pilota::thrift::TLengthProtocolExt;
                    {size}
                }}
            }}"#}
    }

    fn codegen_impl_message_with_helper<F: Fn(&DecodeHelper) -> String>(
        &self,
        name: Symbol,
        encode: String,
        size: String,
        decode: F,
    ) -> String {
        let decode_stream = decode(&DecodeHelper::new(false));
        let decode_async_stream = decode(&DecodeHelper::new(true));
        self.codegen_impl_message(name, encode, size, decode_stream, decode_async_stream)
    }

    fn codegen_decode(&self, helper: &DecodeHelper, s: &rir::Message) -> String {
        let fields = s.fields.iter().map(|f| self.rust_name(f.did)).join(",");

        let required_without_default_fields = s
            .fields
            .iter()
            .filter(|f| !f.is_optional() && self.default_val(f).is_none())
            .map(|f| self.rust_name(f.did))
            .collect_vec();

        let def_fields = s
            .fields
            .iter()
            .map(|f| {
                let field_name = self.rust_name(f.did);
                let mut v = "None".into();

                if let Some((default, is_const)) = self.cx.default_val(f) {
                    if is_const {
                        v = default;

                        if f.is_optional() {
                            v = format!("Some({v})").into()
                        }
                    }
                };

                format!("let mut {field_name} = {v};")
            })
            .join("");

        let set_default_fields = s
            .fields
            .iter()
            .filter_map(|f| {
                let field_name = self.rust_name(f.did);
                if let Some((default, is_const)) = self.cx.default_val(f) {
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
                } else {
                    None
                }
            })
            .join("\n");

        let read_struct_begin = helper.codegen_read_struct_begin();
        let read_struct_end = helper.codegen_read_struct_end();
        let read_fields = self.codegen_decode_fields(helper, &s.fields);

        let verify_required_fields = required_without_default_fields
            .iter()
            .map(|s| {
                format!(
                    r#"let Some({s}) = {s} else {{
                return Err(
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
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
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }}.await"#
            }
        } else {
            format! {
                r#"(|| {{
                    {read_fields}
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }})()"#
            }
        };

        let format_msg = format!("decode struct `{}` field(#{{}}) failed", s.name);

        format! {
            r#"
            {def_fields}

            let mut __pilota_decoding_field_id = None;

            {read_struct_begin};
            if let Err(err) = {read_fields} {{
                if let Some(field_id) = __pilota_decoding_field_id {{
                    return Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(err)),
                        format!("{format_msg}", field_id),
                    ));
                }} else {{
                    return Err(err)
                }}
            }};
            {read_struct_end};


            {verify_required_fields}

            {set_default_fields}

            let data = Self {{
                {fields}
            }};
            Ok(data)
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

    fn codegen_entry_enum(&self, _def_id: DefId, _stream: &mut String, _e: &rir::Enum) {
        // TODO
    }

    fn codegen_decode_fields<'a>(
        &'a self,
        helper: &DecodeHelper,
        fields: &'a [Arc<Field>],
    ) -> String {
        let read_field_begin = helper.codegen_read_field_begin();
        let match_fields = fields
            .iter()
            .map(|f| {
                let field_ident = self.rust_name(f.did);
                let ttype = self.ttype(&f.ty);
                let mut read_field = self.codegen_decode_ty(helper, &f.ty);
                let field_id = f.id as i16;
                if self.field_is_box(f) {
                    read_field = format!("::std::boxed::Box::new({read_field})").into();
                };

                if f.is_optional() || {
                    if let Some((_, is_const)) = self.cx.default_val(f) {
                        !is_const
                    } else {
                        true
                    }
                } {
                    read_field = format!("Some({read_field})").into()
                }

                format!(
                    r#"Some({field_id}) if field_ident.field_type == {ttype}  => {{
                    {field_ident} = {read_field};
                }},"#
                )
            })
            .join("");
        let skip_ttype = helper.codegen_skip_ttype("field_ident.field_type".into());
        let read_field_end = helper.codegen_read_field_end();

        format! {
            r#"loop {{
                let field_ident = {read_field_begin};
                if field_ident.field_type == ::pilota::thrift::TType::Stop {{
                    break;
                }}
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {{
                    {match_fields}
                    _ => {{
                        {skip_ttype};
                    }},
                }}

                {read_field_end};
            }};"#
        }
    }
}

impl CodegenBackend for ThriftBackend {
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &Message) {
        let name = self.cx.rust_name(def_id);
        let name_str = &**s.name;
        let encode_fields = self.codegen_encode_fields(&s.fields).join("");
        let encode_fields_size = self
            .codegen_encode_fields_size(&s.fields)
            .map(|s| format!("{s} +"))
            .join("");
        stream.push_str(&self.codegen_impl_message_with_helper(
            name,
            format! {
                r#"let struct_ident =::pilota::thrift::TStructIdentifier {{
                    name: "{name_str}",
                }};

                protocol.write_struct_begin(&struct_ident)?;
                {encode_fields}
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
                "#
            },
            format! {
                r#"protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {{
                    name: "{name_str}",
                }}) + {encode_fields_size} protocol.write_field_stop_len() + protocol.write_struct_end_len()"#
            },
            |helper| self.codegen_decode(helper, s),
        ));
    }

    fn codegen_service_impl(&self, _def_id: DefId, _stream: &mut String, _s: &Service) {}

    fn codegen_service_method(&self, _service_def_id: DefId, _m: &Method) -> String {
        Default::default()
    }

    fn codegen_enum_impl(&self, def_id: DefId, stream: &mut String, e: &Enum) {
        let name = self.rust_name(def_id);
        let is_entry_message = self.node_contains_tag::<EntryMessage>(def_id);
        let v = match self
            .cx
            .node_tags(def_id)
            .unwrap()
            .get::<EnumMode>()
            .copied()
            .unwrap_or(EnumMode::Enum)
        {
            EnumMode::NewType => "self.inner()",
            EnumMode::Enum => "*self as i32",
        };
        match e.repr {
            Some(EnumRepr::I32) => stream.push_str(&self.codegen_impl_message_with_helper(
                name.clone(),
                format! {
                    r#"protocol.write_i32({v})?;
                    Ok(())
                    "#
                },
                format!("protocol.write_i32_len({v})"),
                |helper| {
                    let read_i32 = helper.codegen_read_i32();
                    let err_msg_tmpl = format!("invalid enum value for {}, value: {{}}", name);
                    format! {
                        r#"let value = {read_i32};
                        Ok(::std::convert::TryFrom::try_from(value).map_err(|err|
                            ::pilota::thrift::DecodeError::new(
                                ::pilota::thrift::DecodeErrorKind::InvalidData,
                                format!("{err_msg_tmpl}", value)
                            ))?)"#
                    }
                },
            )),
            None if is_entry_message => self.codegen_entry_enum(def_id, stream, e),
            None => {
                let name = self.rust_name(def_id);
                let name_str = &**e.name;
                let encode_variants = e
                    .variants
                    .iter()
                    .map(|v| {
                        let variant_name = self.rust_name(v.did);
                        assert_eq!(v.fields.len(), 1);
                        let variant_id = v.id.unwrap() as i16;
                        let encode =
                            self.codegen_encode_field(variant_id, &v.fields[0], "value".into());
                        format! {
                            r#"{name}::{variant_name}(ref value) => {{
                            {encode}
                        }},"#
                        }
                    })
                    .join("");

                let variants_size = e
                    .variants
                    .iter()
                    .map(|v| {
                        let variant_name = self.rust_name(v.did);
                        let variant_id = v.id.unwrap() as i16;
                        let size =
                            self.codegen_field_size(&v.fields[0], variant_id, "value".into());

                        format! {
                            r#"{name}::{variant_name}(ref value) => {{
                            {size}
                        }},"#
                        }
                    })
                    .join("");

                let variant_is_void = |v: &EnumVariant| {
                    &*v.name.sym == "Ok" && v.fields.len() == 1 && v.fields[0].kind == TyKind::Void
                };

                stream.push_str(&self.codegen_impl_message_with_helper(
                    name.clone(),
                    format! {
                        r#"protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {{
                            name: "{name_str}",
                        }})?;
                        match self {{
                            {encode_variants}
                        }}
                        protocol.write_field_stop()?;
                        protocol.write_struct_end()?;
                        Ok(())"#
                    },
                    format! {
                        r#"protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {{
                            name: "{name_str}",
                        }}) + match self {{
                            {variants_size}
                        }} +  protocol.write_field_stop_len() + protocol.write_struct_end_len()"#
                    },
                    |helper| {
                        let read_struct_begin = helper.codegen_read_struct_begin();
                        let read_field_begin = helper.codegen_read_field_begin();
                        let read_field_end = helper.codegen_read_field_end();
                        let read_struct_end = helper.codegen_read_struct_end();
                        let skip = helper.codegen_skip_ttype("field_ident.field_type".into());
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
                                    Some(format! {
                                        r#"Some({variant_id}) => {{
                                    if ret.is_none() {{
                                        ret = Some({name}::{variant_name}({decode}));
                                    }} else {{
                                        return Err(::pilota::thrift::DecodeError::new(
                                            ::pilota::thrift::DecodeErrorKind::InvalidData,
                                            "received multiple fields for union from remote Message"
                                        ));
                                    }}
                                }},"#
                                    })
                                }
                            })
                            .join("");

                        let handle_none_ret: FastStr =
                            if e.variants.first().filter(|v| variant_is_void(v)).is_some() {
                                format!("Ok({name}::Ok(()))").into()
                            } else {
                                format!(
                                    r#"Err(::pilota::thrift::DecodeError::new(
                                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                                        "received empty union from remote Message")
                                    )"#
                                )
                                .into()
                            };

                        format! {
                            r#"let mut ret = None;
                            {read_struct_begin};
                            loop {{
                                let field_ident = {read_field_begin};
                                if field_ident.field_type == ::pilota::thrift::TType::Stop {{
                                    break;
                                }}
                                match field_ident.id {{
                                    {fields}
                                    _ => {{
                                        {skip};
                                    }},
                                }}
                            }}
                            {read_field_end};
                            {read_struct_end};
                            if let Some(ret) = ret {{
                                Ok(ret)
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
            name.clone(),
            format! {
                r#"{encode}
                Ok(())"#
            },
            format!("{encode_size}"),
            |helper| {
                let decode = self.codegen_decode_ty(helper, &t.ty);
                format!("Ok({name}({decode}))")
            },
        ));
    }

    fn cx(&self) -> &Context {
        &self.cx
    }
}
