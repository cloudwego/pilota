#![doc(
    html_logo_url = "https://github.com/cloudwego/pilota/raw/main/.github/assets/logo.png?sanitize=true"
)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod codegen;
pub mod db;
mod fmt;
mod index;
mod ir;
mod middle;
pub mod parser;
mod resolve;
mod symbol;
pub mod tags;
mod util;
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

pub mod plugin;
mod test;

use codegen::protobuf::{ProstPlugin, ProtobufBackend};
pub use codegen::{thrift::ThriftBackend, traits::CodegenBackend, Codegen};
use db::RootDatabase;
use fmt::fmt_file;
use fxhash::FxHashMap;
use middle::{
    context::tls::CONTEXT,
    rir::{NodeKind, Pkg},
    type_graph::TypeGraph,
};
pub use middle::{rir, ty};
use parser::{protobuf::ProtobufParser, thrift::ThriftParser, ParseResult, Parser};
use plugin::{
    AutoDerivePlugin, BoxedPlugin, EnumNumPlugin, ImplDefaultPlugin, PredicateResult,
    WithAttrsPlugin,
};
pub use plugin::{BoxClonePlugin, ClonePlugin, Plugin};
use resolve::{ResolveResult, Resolver};
use salsa::{Durability, ParallelDatabase};
pub use symbol::DefId;
use syn::parse_quote;

use crate::db::RirDatabase;
pub use crate::middle::context::Context;

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
    mk_backend: MkB,
    parser: P,
    plugins: Vec<Box<dyn Plugin>>,
}

impl Builder<MkThriftBackend, ThriftParser> {
    pub fn thrift() -> Self {
        Builder {
            mk_backend: MkThriftBackend,
            parser: ThriftParser::default(),
            plugins: vec![
                Box::new(WithAttrsPlugin(vec![parse_quote!(#[derive(Debug)])])),
                Box::new(ImplDefaultPlugin),
                Box::new(EnumNumPlugin),
            ],
        }
    }
}

impl Builder<MkProtobufBackend, ProtobufParser> {
    pub fn protobuf() -> Self {
        Builder {
            mk_backend: MkProtobufBackend,
            parser: ProtobufParser::default(),
            plugins: vec![Box::new(ProstPlugin)],
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
            mk_backend,
            parser: self.parser,
            plugins: self.plugins,
        }
    }

    pub fn plugin<Plu: Plugin + 'static>(mut self, p: Plu) -> Self {
        self.plugins.push(Box::new(p));

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
        let ParseResult { files } = self.parser.parse();

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

        let mut pkgs = FxHashMap::default();

        db.files().iter().for_each(|(_, f)| {
            let pkg_path = &f.package;
            let pkg = pkgs.entry(pkg_path.clone()).or_insert_with(|| Pkg {
                path: pkg_path.clone(),
                items: Default::default(),
            });

            pkg.items.extend_from_slice(&f.items);
        });

        let pkgs = Arc::from(
            pkgs.into_iter()
                .map(|(path, pkg)| (path, Arc::from(pkg)))
                .collect::<FxHashMap<_, _>>(),
        );

        db.set_pkgs_with_durability(pkgs.clone(), Durability::HIGH);

        let mut cx = Context::new(db.snapshot());
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
                if matches!(ty.kind, ty::Map(_, _) | ty::Set(_) | ty::F64) {
                    PredicateResult::No
                } else {
                    PredicateResult::GoOn
                }
            },
        ));

        self.plugins.into_iter().for_each(|p| cx.exec_plugin(p));

        let context = Arc::from(cx);
        CONTEXT.set(&context.clone(), || {
            let mut cg = Codegen::new(context.clone(), self.mk_backend.make_backend(context));
            cg.write_pkgs(&pkgs.keys().cloned().collect::<Vec<_>>());

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
