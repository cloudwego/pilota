use proc_macro2::TokenStream;

use crate::{
    middle::rir::{self, Method},
    symbol::DefId,
};

pub trait CodegenBackend {
    fn codegen_struct_impl(&self, _def_id: DefId, _stream: &mut TokenStream, _s: &rir::Message) {}
    fn codegen_service_impl(&self, _def_id: DefId, _stream: &mut TokenStream, _s: &rir::Service) {}
    fn codegen_service_method(&self, _service_def_id: DefId, _method: &Method) -> TokenStream {
        TokenStream::default()
    }
    fn codegen_enum_impl(&self, _def_id: DefId, _stream: &mut TokenStream, _e: &rir::Enum) {}
    fn codegen_newtype_impl(&self, _def_id: DefId, _stream: &mut TokenStream, _t: &rir::NewType) {}
}
