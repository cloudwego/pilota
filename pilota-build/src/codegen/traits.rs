use crate::{
    middle::rir::{self, Method},
    symbol::DefId,
    Context,
};

pub trait CodegenBackend: Clone {
    fn cx(&self) -> &Context;
    fn codegen_struct_impl(&self, _def_id: DefId, _stream: &mut String, _s: &rir::Message) {}
    fn codegen_service_impl(&self, _def_id: DefId, _stream: &mut String, _s: &rir::Service) {}
    fn codegen_service_method(&self, _service_def_id: DefId, _method: &Method) -> String {
        Default::default()
    }
    /// Generate methods for the service trait and use global paths to generate
    /// the types of the arguments and outputs.
    fn codegen_service_method_with_global_path(
        &self,
        _service_def_id: DefId,
        _method: &Method,
    ) -> String {
        Default::default()
    }
    fn codegen_enum_impl(&self, _def_id: DefId, _stream: &mut String, _e: &rir::Enum) {}
    fn codegen_newtype_impl(&self, _def_id: DefId, _stream: &mut String, _t: &rir::NewType) {}
}
