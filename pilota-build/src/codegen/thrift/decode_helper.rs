use paste::paste;
use proc_macro2::TokenStream;
use quote::quote;

pub struct DecodeHelper {
    is_async: bool,
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
            pub fn [<codegen_ $m>](&self) -> TokenStream {
                if self.is_async {
                    quote::quote! {
                        protocol.$m().await?
                    }
                } else {
                    quote::quote! {
                        protocol.$m()?
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

    pub fn codegen_skip_ttype(&self, tt: TokenStream) -> TokenStream {
        if self.is_async {
            quote! {
                protocol.skip(#tt).await?
            }
        } else {
            quote! {
                protocol.skip(#tt)?
            }
        }
    }

    pub fn codegen_item_decode(&self) -> TokenStream {
        if self.is_async {
            quote! { ::pilota::thrift::Message::decode_async(protocol).await? }
        } else {
            quote! { ::pilota::thrift::Message::decode(protocol)? }
        }
    }
}
