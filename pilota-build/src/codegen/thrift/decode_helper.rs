use faststr::FastStr;
use paste::paste;

pub struct DecodeHelper {
    pub is_async: bool,
}

impl DecodeHelper {
    pub fn new(is_async: bool) -> Self {
        Self { is_async }
    }
}

macro_rules! protocol_method {
    ($m:ident) => {
        paste! {
            #[inline]
            pub fn [<codegen_ $m>](&self) -> faststr::FastStr {
                if self.is_async {
                    format!("protocol.{}().await?", stringify!($m)).into()
                } else {
                    format!("protocol.{}()?", stringify!($m)).into()
                }
            }
        }
    };
}

macro_rules! protocol_len {
    ($m:ident) => {
        paste! {
            #[inline]
            pub fn [<codegen_ $m>](&self, keep: bool) -> faststr::FastStr {
                if self.is_async {
                    Default::default()
                } else {
                    if keep {
                        format!("__pilota_offset += protocol.{}();", stringify!($m)).into()
                    } else {
                        format!("protocol.{}();", stringify!($m)).into()
                    }
                }
            }
        }
    };
}

impl DecodeHelper {
    protocol_method!(read_i8);
    protocol_method!(read_i16);
    protocol_method!(read_i32);
    protocol_method!(read_i64);
    protocol_method!(read_double);
    protocol_method!(read_bytes);
    protocol_method!(read_bytes_vec);
    protocol_method!(read_byte);
    protocol_method!(read_string);
    protocol_method!(read_faststr);
    protocol_method!(read_list_begin);
    protocol_method!(read_list_end);
    protocol_method!(read_set_begin);
    protocol_method!(read_set_end);
    protocol_method!(read_map_begin);
    protocol_method!(read_map_end);
    protocol_method!(read_struct_begin);
    protocol_method!(read_struct_end);
    protocol_method!(read_field_begin);
    protocol_method!(read_field_end);
    protocol_method!(read_bool);
    protocol_method!(read_uuid);

    protocol_len!(field_end_len);
    protocol_len!(field_stop_len);

    pub fn codegen_skip_ttype(&self, tt: FastStr) -> String {
        if self.is_async {
            format!("protocol.skip({tt}).await?")
        } else {
            format!("protocol.skip({tt})?")
        }
    }

    pub fn codegen_item_decode(&self, name: FastStr) -> FastStr {
        if self.is_async {
            format!(
                "<{} as ::pilota::thrift::Message>::decode_async(protocol).await?",
                name
            )
            .into()
        } else {
            "::pilota::thrift::Message::decode(protocol)?".into()
        }
    }

    pub fn codegen_field_begin_len(&self, keep: bool) -> FastStr {
        if self.is_async {
            Default::default()
        } else {
            if keep {
                "__pilota_offset += protocol.field_begin_len(field_ident.field_type, field_ident.id);".into()
            } else {
                "protocol.field_begin_len(field_ident.field_type, field_ident.id);".into()
            }
        }
    }
}
