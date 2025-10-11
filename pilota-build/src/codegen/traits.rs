use std::{path::PathBuf, sync::Arc};

use crate::{
    Context,
    middle::{
        ext::{FileExts, ModExts},
        rir::{self, Method},
    },
    symbol::{DefId, ModPath, Symbol},
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
        _mods: &[(ModPath, Arc<PathBuf>)],
    ) {
    }
    fn codegen_pilota_buf_descriptor_trait(&self, _stream: &mut String) {}

    fn codegen_file_descriptor_at_mod(
        &self,
        stream: &mut String,
        f: &rir::File,
        _mod_path: &ModPath,
        has_direct: bool,
    ) {
        self.codegen_file_descriptor(stream, f, has_direct);
    }

    // pb only, for pb options
    fn codegen_file_exts(
        &self,
        _stream: &mut String,
        _suffix: &str,
        _cur_pkg: &[Symbol],
        _extensions: &FileExts,
    ) {
    }

    // pb only, for pb options
    fn codegen_mod_exts(
        &self,
        _stream: &mut String,
        _suffix: &str,
        _cur_pkg: &[Symbol],
        _extensions: &ModExts,
    ) {
    }

    // pb only, for pb options
    fn codegen_impl_enum_message(&self, _name: &str) -> String {
        Default::default()
    }

    // pb only
    fn codegen_impl_enum_descriptor_getter(&self, def_id: DefId, e: &rir::Enum) -> String {
        Default::default()
    }
}
