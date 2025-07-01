use std::{path::PathBuf, sync::Arc};

use pilota::FastStr;

use crate::{
    Context,
    middle::rir::{self, Method},
    symbol::DefId,
};

pub trait CodegenBackend: Clone {
    const PROTOCOL: &'static str;

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
    fn codegen_file_descriptor(&self, _stream: &mut String, _f: &rir::File, _has_direct: bool) {}
    fn codegen_register_mod_file_descriptor(
        &self,
        _stream: &mut String,
        _mods: &[(Arc<[FastStr]>, Arc<PathBuf>)],
    ) {
    }
    fn codegen_pilota_buf_trait(&self, _stream: &mut String) {}
}
