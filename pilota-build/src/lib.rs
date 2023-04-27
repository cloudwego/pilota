#![doc(
    html_logo_url = "https://github.com/cloudwego/pilota/raw/main/.github/assets/logo.png?sanitize=true"
)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

mod util;

pub mod codegen;
pub mod db;
pub(crate) mod errors;
mod fmt;
mod index;
mod ir;
mod middle;
pub mod parser;
mod resolve;
mod symbol;
pub mod tags;
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

pub mod plugin;

pub use codegen::{
    protobuf::ProtobufBackend, thrift::ThriftBackend, traits::CodegenBackend, Codegen,
};
use db::{RirDatabase, RootDatabase};
use fmt::fmt_file;
use middle::{
    context::{tls::CONTEXT, CollectMode},
    rir::NodeKind,
    type_graph::TypeGraph,
};
pub use middle::{
    context::{Context, SourceType},
    rir, ty,
};
use parser::{protobuf::ProtobufParser, thrift::ThriftParser, ParseResult, Parser};
use plugin::{
    AutoDerivePlugin, BoxedPlugin, EnumNumPlugin, ImplDefaultPlugin, PredicateResult,
    WithAttrsPlugin,
};
pub use plugin::{BoxClonePlugin, ClonePlugin, Plugin};
use resolve::{ResolveResult, Resolver};
use salsa::{Durability, ParallelDatabase};
pub use symbol::{DefId, IdentName};
use syn::parse_quote;
pub use tags::TagId;

pub trait MakeBackend: Sized {
    type Target: CodegenBackend;
    fn make_backend(self, context: Arc<Context>) -> Self::Target;
}

pub struct MkThriftBackend;

impl MakeBackend for MkThriftBackend {
    type Target = ThriftBackend;

    fn make_backend(self, context: Arc<Context>) -> Self::Target {
        ThriftBackend::new(context)
    }
}

pub struct MkProtobufBackend;

impl MakeBackend for MkProtobufBackend {
    type Target = ProtobufBackend;

    fn make_backend(self, context: Arc<Context>) -> Self::Target {
        ProtobufBackend::new(context)
    }
}

pub struct Builder<MkB, P> {
    source_type: SourceType,
    mk_backend: MkB,
    parser: P,
    plugins: Vec<Box<dyn Plugin>>,
    ignore_unused: bool,
    touches: Vec<(std::path::PathBuf, Vec<String>)>,
    change_case: bool,
}

impl Builder<MkThriftBackend, ThriftParser> {
    pub fn thrift() -> Self {
        Builder {
            source_type: SourceType::Thrift,
            mk_backend: MkThriftBackend,
            parser: ThriftParser::default(),
            plugins: vec![
                Box::new(WithAttrsPlugin(vec![parse_quote!(#[derive(Debug)])])),
                Box::new(ImplDefaultPlugin),
                Box::new(EnumNumPlugin),
            ],
            touches: Vec::default(),
            ignore_unused: true,
            change_case: true,
        }
    }
}

impl Builder<MkProtobufBackend, ProtobufParser> {
    pub fn protobuf() -> Self {
        Builder {
            source_type: SourceType::Protobuf,
            mk_backend: MkProtobufBackend,
            parser: ProtobufParser::default(),
            plugins: vec![
                Box::new(WithAttrsPlugin(vec![parse_quote!(#[derive(Debug)])])),
                Box::new(ImplDefaultPlugin),
                Box::new(EnumNumPlugin),
            ],
            touches: Vec::default(),
            ignore_unused: true,
            change_case: true,
        }
    }
}

impl<MkB, P> Builder<MkB, P>
where
    P: Parser,
{
    pub fn include_dirs(mut self, include_dirs: Vec<PathBuf>) -> Self {
        self.parser.include_dirs(include_dirs);
        self
    }
}

impl<MkB, P> Builder<MkB, P> {
    pub fn with_backend<B: MakeBackend>(self, mk_backend: B) -> Builder<B, P> {
        Builder {
            source_type: self.source_type,
            mk_backend,
            parser: self.parser,
            plugins: self.plugins,
            ignore_unused: self.ignore_unused,
            touches: self.touches,
            change_case: self.change_case,
        }
    }

    pub fn plugin<Plu: Plugin + 'static>(mut self, p: Plu) -> Self {
        self.plugins.push(Box::new(p));

        self
    }

    pub fn change_case(mut self, change_case: bool) -> Self {
        self.change_case = change_case;
        self
    }

    /**
     * Don't generate items which are unused by the main service
     */
    pub fn ignore_unused(mut self, flag: bool) -> Self {
        self.ignore_unused = flag;
        self
    }

    /**
     * Generate items even them are not used.
     *
     * This is ignored if `ignore_unused` is false
     */
    pub fn touch(
        mut self,
        item: impl IntoIterator<Item = (PathBuf, Vec<impl Into<String>>)>,
    ) -> Self {
        self.touches.extend(
            item.into_iter()
                .map(|s| (s.0, s.1.into_iter().map(|s| s.into()).collect())),
        );
        self
    }
}

impl<MkB, P> Builder<MkB, P>
where
    MkB: MakeBackend,
    P: Parser,
{
    pub fn compile<O: AsRef<Path>>(mut self, files: &[impl AsRef<Path>], out: O) {
        let _ = tracing_subscriber::fmt::try_init();

        let mut db = RootDatabase::default();
        self.parser.inputs(files);
        let ParseResult {
            files,
            input_files,
            file_ids_map,
        } = self.parser.parse();
        db.set_file_ids_map_with_durability(Arc::new(file_ids_map), Durability::HIGH);

        let ResolveResult { files, nodes, tags } = Resolver::default().resolve_files(&files);
        db.set_files_with_durability(Arc::new(files), Durability::HIGH);
        let items = nodes.iter().filter_map(|(k, v)| {
            if let NodeKind::Item(item) = &v.kind {
                Some((*k, item.clone()))
            } else {
                None
            }
        });

        let type_graph = Arc::from(TypeGraph::from_items(items));
        db.set_type_graph_with_durability(type_graph, Durability::HIGH);
        db.set_nodes_with_durability(Arc::new(nodes), Durability::HIGH);

        let mut cx = Context::new(self.source_type, db.snapshot());
        cx.change_case = self.change_case;
        cx.set_tags_map(tags);

        cx.exec_plugin(BoxedPlugin);

        cx.exec_plugin(AutoDerivePlugin::new(
            vec![parse_quote!(#[derive(PartialOrd)])],
            |ty| {
                let ty = match &ty.kind {
                    ty::Vec(ty) => ty,
                    _ => ty,
                };
                if matches!(ty.kind, ty::Map(_, _) | ty::Set(_)) {
                    PredicateResult::No
                } else {
                    PredicateResult::GoOn
                }
            },
        ));

        cx.exec_plugin(AutoDerivePlugin::new(
            vec![parse_quote!(#[derive(Hash, Eq, Ord)])],
            |ty| {
                let ty = match &ty.kind {
                    ty::Vec(ty) => ty,
                    _ => ty,
                };
                if matches!(ty.kind, ty::Map(_, _) | ty::Set(_) | ty::F64 | ty::F32) {
                    PredicateResult::No
                } else {
                    PredicateResult::GoOn
                }
            },
        ));

        let mut input = Vec::with_capacity(input_files.len());
        for file_id in input_files {
            let file = cx.file(file_id).unwrap();
            file.items.iter().for_each(|def_id| {
                if matches!(&*cx.item(*def_id).unwrap(), rir::Item::Service(_)) {
                    input.push(*def_id)
                }
            });
        }

        let mods = cx.collect_pkgs(if self.ignore_unused {
            CollectMode::OnlyUsed {
                touches: self.touches,
                input,
            }
        } else {
            CollectMode::All
        });

        self.plugins.into_iter().for_each(|p| cx.exec_plugin(p));

        let context = Arc::from(cx);
        CONTEXT.set(&context.clone(), || {
            let mut cg = Codegen::new(context.clone(), self.mk_backend.make_backend(context));
            cg.write_mods(mods);

            let file_name = out
                .as_ref()
                .file_name()
                .and_then(|s| s.to_str())
                .and_then(|s| s.split('.').next())
                .unwrap();

            let stream = cg.link(file_name);

            let mut file = std::io::BufWriter::new(std::fs::File::create(&out).unwrap());
            file.write_all(stream.to_string().as_bytes()).unwrap();
            file.flush().unwrap();
            fmt_file(out)
        });
    }
}

mod test;
