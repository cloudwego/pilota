use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{ir::File, symbol::FileId};

pub(crate) mod protobuf;
pub(crate) mod thrift;

use rustc_hash::FxHashMap;
pub use thrift::ThriftParser;

pub use self::protobuf::ProtobufParser;

pub struct ParseResult {
    pub files: Vec<Arc<File>>,
    pub(crate) input_files: Vec<FileId>,
    pub(crate) file_ids_map: FxHashMap<Arc<PathBuf>, FileId>,
}

pub trait Parser {
    fn input<P: AsRef<Path>>(&mut self, path: P);

    fn inputs<P: AsRef<Path>>(&mut self, paths: impl IntoIterator<Item = P>) {
        paths.into_iter().for_each(|p| self.input(p))
    }

    fn include_dirs(&mut self, dirs: Vec<PathBuf>);

    fn nonstandard_snake_case(&mut self, _nonstandard: bool) {}

    fn parse(self) -> ParseResult;
}
