#![doc(
    html_logo_url = "https://github.com/cloudwego/pilota/raw/main/.github/assets/logo.png?sanitize=true"
)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![allow(clippy::mutable_key_type)]

mod util;

pub mod codegen;
pub mod db;
pub(crate) mod errors;
pub mod fmt;
mod index;
mod ir;
pub mod middle;
pub mod parser;
mod resolve;
mod symbol;

use faststr::FastStr;
pub use symbol::{ModPath, Symbol};
use tempfile::tempdir;
pub mod tags;
use std::{path::PathBuf, sync::Arc};

mod dedup;
pub mod plugin;

pub use codegen::{Codegen, thrift::ThriftBackend, traits::CodegenBackend};
use db::RootDatabase;
pub use middle::{
    context::{Context, SourceType},
    rir, ty,
};
use middle::{
    context::{ContextBuilder, Mode, WorkspaceInfo, tls::CONTEXT},
    rir::NodeKind,
    type_graph::TypeGraph,
    workspace_graph::WorkspaceGraph,
};
use parser::{ParseResult, Parser, protobuf::ProtobufParser, thrift::ThriftParser};
use plugin::{AutoDerivePlugin, BoxedPlugin, ImplDefaultPlugin, PredicateResult, WithAttrsPlugin};
pub use plugin::{BoxClonePlugin, ClonePlugin, Plugin};
use resolve::{ResolveResult, Resolver};
pub use symbol::{DefId, IdentName};
pub use tags::TagId;

use crate::{codegen::pb::ProtobufBackend, middle::root_selector::SelectionKind};

pub trait MakeBackend: Sized {
    type Target: CodegenBackend;
    fn make_backend(self, context: Context) -> Self::Target;
}

pub struct MkThriftBackend;

impl MakeBackend for MkThriftBackend {
    type Target = ThriftBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        ThriftBackend::new(context)
    }
}

pub struct MkPbBackend;

impl MakeBackend for MkPbBackend {
    type Target = ProtobufBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        ProtobufBackend::new(context)
    }
}

pub struct Builder<MkB, P> {
    source_type: SourceType,
    mk_backend: MkB,
    parser: P,
    plugins: Vec<Box<dyn Plugin>>,
    ignore_unused: bool,
    split: bool,
    touches: Vec<(std::path::PathBuf, Vec<String>)>,
    change_case: bool,
    touch_files: Vec<std::path::PathBuf>,
    keep_unknown_fields: Vec<std::path::PathBuf>,
    dedups: Vec<FastStr>,
    special_namings: Vec<FastStr>,
    common_crate_name: FastStr,
    with_descriptor: bool,
    with_field_mask: bool,
    temp_dir: Option<tempfile::TempDir>,
    with_comments: bool,
}

impl Builder<MkThriftBackend, ThriftParser> {
    pub fn thrift() -> Self {
        Builder {
            source_type: SourceType::Thrift,
            mk_backend: MkThriftBackend,
            parser: ThriftParser::default(),
            plugins: vec![
                Box::new(WithAttrsPlugin(Arc::from(["#[derive(Debug)]".into()]))),
                Box::new(ImplDefaultPlugin),
            ],
            touches: Vec::default(),
            ignore_unused: true,
            change_case: true,
            touch_files: Vec::default(),
            keep_unknown_fields: Vec::default(),
            dedups: Vec::default(),
            special_namings: Vec::default(),
            common_crate_name: "common".into(),
            split: false,
            with_descriptor: false,
            with_field_mask: false,
            temp_dir: None,
            with_comments: false,
        }
    }
}

impl Builder<MkPbBackend, ProtobufParser> {
    pub fn pb() -> Self {
        let (out_dir, temp_dir) = match std::env::var("OUT_DIR") {
            Ok(out_dir_str) => (PathBuf::from(out_dir_str), None),
            _ => {
                let temp_dir = tempdir().unwrap();
                (temp_dir.path().to_path_buf(), Some(temp_dir))
            }
        };
        let include_dir = out_dir.join("pilota_proto");

        std::fs::create_dir_all(&include_dir).expect("Failed to create pilota_proto directory");

        let pilota_proto_src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("proto/pilota.proto");

        std::fs::copy(&pilota_proto_src, include_dir.join("pilota.proto"))
            .expect("Failed to copy pilota.proto");

        let mut parser = ProtobufParser::default();
        parser.include_dirs(vec![include_dir]);

        Builder {
            source_type: SourceType::Protobuf,
            mk_backend: MkPbBackend,
            parser,
            plugins: vec![
                Box::new(WithAttrsPlugin(Arc::from(["#[derive(Debug)]".into()]))),
                Box::new(ImplDefaultPlugin),
            ],
            touches: Vec::default(),
            ignore_unused: true,
            change_case: true,
            touch_files: Vec::default(),
            keep_unknown_fields: Vec::default(),
            dedups: Vec::default(),
            special_namings: Vec::default(),
            common_crate_name: "common".into(),
            split: false,
            with_descriptor: false,
            with_field_mask: false,
            temp_dir,
            with_comments: false,
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
            touch_files: self.touch_files,
            keep_unknown_fields: self.keep_unknown_fields,
            dedups: self.dedups,
            special_namings: self.special_namings,
            common_crate_name: self.common_crate_name,
            split: self.split,
            with_descriptor: self.with_descriptor,
            with_field_mask: self.with_field_mask,
            temp_dir: self.temp_dir,
            with_comments: self.with_comments,
        }
    }

    pub fn plugin<Plu: Plugin + 'static>(mut self, p: Plu) -> Self {
        self.plugins.push(Box::new(p));

        self
    }

    pub fn split_generated_files(mut self, split: bool) -> Self {
        self.split = split;
        self
    }

    pub fn change_case(mut self, change_case: bool) -> Self {
        self.change_case = change_case;
        self
    }

    /// Don't generate items which are unused by the main service
    pub fn ignore_unused(mut self, flag: bool) -> Self {
        self.ignore_unused = flag;
        self
    }

    /// Generate items even them are not used.
    ///
    /// This is ignored if `ignore_unused` is false.
    ///
    /// Entries whose `items` list is empty are silently ignored, because an
    /// empty `touch` would otherwise defeat the fallback logic that relies on
    /// `touches.is_empty()` and never produce any effect on its own.
    pub fn touch(
        mut self,
        item: impl IntoIterator<Item = (PathBuf, Vec<impl Into<String>>)>,
    ) -> Self {
        self.touches.extend(item.into_iter().filter_map(|s| {
            let items = s.1.into_iter().map(|s| s.into()).collect::<Vec<_>>();
            if items.is_empty() {
                None
            } else {
                Some((s.0, items))
            }
        }));
        self
    }

    /// Generate all non-`mod` top-level items from the specified input files.
    ///
    /// This is mainly intended for IDLs without any `service` definitions.
    /// When `ignore_unused(false)` is set, the selection is still promoted to
    /// `RootSelection::All`, so `touch_files` has no effect.
    pub fn touch_files(mut self, item: impl IntoIterator<Item = PathBuf>) -> Self {
        self.touch_files.extend(item);
        self
    }

    pub fn keep_unknown_fields(mut self, item: impl IntoIterator<Item = PathBuf>) -> Self {
        self.keep_unknown_fields.extend(item);
        self
    }

    pub fn dedup(mut self, item: impl IntoIterator<Item = FastStr>) -> Self {
        self.dedups.extend(item);
        self
    }

    pub fn special_namings(mut self, item: impl IntoIterator<Item = FastStr>) -> Self {
        self.special_namings.extend(item);
        self
    }

    pub fn common_crate_name(mut self, name: FastStr) -> Self {
        self.common_crate_name = name;
        self
    }

    pub fn with_descriptor(mut self, on: bool) -> Self {
        self.with_descriptor = on;
        self
    }

    pub fn with_field_mask(mut self, on: bool) -> Self {
        self.with_field_mask = on;
        self
    }

    /// Generate comments for the generated code
    pub fn with_comments(mut self, on: bool) -> Self {
        self.with_comments = on;
        self
    }
}

pub enum Output {
    Workspace(PathBuf),
    File(PathBuf),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IdlService {
    pub path: PathBuf,
    pub config: serde_yaml::Value,
}

impl IdlService {
    pub fn from_path(p: PathBuf) -> Self {
        IdlService {
            path: p,
            config: Default::default(),
        }
    }
}

impl<MkB, P> Builder<MkB, P>
where
    MkB: MakeBackend + Send,
    MkB::Target: Send,
    P: Parser,
{
    pub fn compile(
        self,
        services: impl IntoIterator<Item = impl AsRef<std::path::Path>>,
        out: Output,
    ) {
        let services = services
            .into_iter()
            .map(|path| IdlService {
                config: serde_yaml::Value::default(),
                path: path.as_ref().to_owned(),
            })
            .collect();

        self.compile_with_config(services, out)
    }

    #[allow(clippy::too_many_arguments)]
    #[deprecated(
        since = "0.13.8",
        note = "`build_cx` does not forward the `touch_files` configuration and is kept only \
                for backward compatibility. Use `Builder::compile_with_config` or \
                `Builder::init_service` instead, which honour every builder option."
    )]
    pub fn build_cx(
        services: Vec<IdlService>,
        out: Option<Output>,
        parser: P,
        touches: Vec<(PathBuf, Vec<String>)>,
        ignore_unused: bool,
        source_type: SourceType,
        change_case: bool,
        keep_unknown_fields: Vec<PathBuf>,
        dedups: Vec<FastStr>,
        special_namings: Vec<FastStr>,
        common_crate_name: FastStr,
        split: bool,
        with_descriptor: bool,
        with_field_mask: bool,
        with_comments: bool,
    ) -> Context {
        Self::build_cx_impl(
            services,
            out,
            parser,
            touches,
            ignore_unused,
            source_type,
            change_case,
            vec![],
            keep_unknown_fields,
            dedups,
            special_namings,
            common_crate_name,
            split,
            with_descriptor,
            with_field_mask,
            with_comments,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn build_cx_impl(
        services: Vec<IdlService>,
        out: Option<Output>,
        mut parser: P,
        touches: Vec<(PathBuf, Vec<String>)>,
        ignore_unused: bool,
        source_type: SourceType,
        change_case: bool,
        touch_files: Vec<PathBuf>,
        keep_unknown_fields: Vec<PathBuf>,
        dedups: Vec<FastStr>,
        special_namings: Vec<FastStr>,
        common_crate_name: FastStr,
        split: bool,
        with_descriptor: bool,
        with_field_mask: bool,
        with_comments: bool,
    ) -> Context {
        parser.inputs(services.iter().map(|s| &s.path));
        let ParseResult {
            files,
            input_files,
            file_ids_map,
            file_paths,
            file_names,
        } = parser.parse();

        let ResolveResult {
            files,
            nodes,
            tags,
            args,
            pb_ext_indexes,
            pb_ext_indexes_used,
        } = Resolver::default().resolve_files(&files);

        let items = nodes.iter().filter_map(|(k, v)| match &v.kind {
            NodeKind::Item(item) => Some((*k, item.clone())),
            _ => None,
        });

        let type_graph = TypeGraph::from_items(items.clone());
        let workspace_graph = WorkspaceGraph::from_items(items);

        // Build the database using the builder pattern
        let db = RootDatabase::default()
            .with_file_ids_map(file_ids_map)
            .with_file_paths(file_paths)
            .with_file_names(file_names)
            .with_files(files.into_iter())
            .with_nodes(nodes)
            .with_tags(tags, type_graph)
            .with_args(args)
            .with_workspace_graph(workspace_graph)
            .with_input_files(input_files.clone())
            .with_pb_ext_indexes(pb_ext_indexes)
            .with_pb_exts_used(pb_ext_indexes_used);

        let root_selection = middle::root_selector::RootSelector::new(
            &db,
            touches,
            touch_files,
            input_files,
            ignore_unused,
        )
        .select();

        let mut cx = ContextBuilder::new(
            db,
            match out {
                Some(Output::Workspace(dir)) => Mode::Workspace(WorkspaceInfo {
                    dir,
                    location_map: Default::default(),
                }),
                Some(Output::File(p)) => Mode::SingleFile { file_path: p },
                None => Mode::SingleFile {
                    file_path: Default::default(),
                },
            },
            vec![],
        );

        let selection_kind = root_selection.kind();

        cx.collect(root_selection);

        cx.keep(keep_unknown_fields);

        cx.build(
            Arc::from(services),
            source_type,
            selection_kind,
            change_case,
            dedups,
            special_namings,
            common_crate_name,
            split,
            with_descriptor,
            with_field_mask,
            !ignore_unused,
            with_comments,
        )
    }

    pub fn compile_with_config(self, services: Vec<IdlService>, out: Output) {
        let _ = tracing_subscriber::fmt::try_init();

        let cx = Self::build_cx_impl(
            services,
            Some(out),
            self.parser,
            self.touches,
            self.ignore_unused,
            self.source_type,
            self.change_case,
            self.touch_files,
            self.keep_unknown_fields,
            self.dedups,
            self.special_namings,
            self.common_crate_name,
            self.split,
            self.with_descriptor,
            self.with_field_mask,
            self.with_comments,
        );

        cx.exec_plugin(BoxedPlugin);

        cx.exec_plugin(AutoDerivePlugin::new(
            Arc::from(["#[derive(PartialOrd)]".into()]),
            |ty| {
                let mut ty = ty;
                while let ty::Vec(_ty) = &ty.kind {
                    ty = _ty;
                }
                if matches!(ty.kind, ty::Map(_, _) | ty::Set(_)) {
                    PredicateResult::No
                } else {
                    PredicateResult::GoOn
                }
            },
        ));

        cx.exec_plugin(AutoDerivePlugin::new(
            Arc::from(["#[derive(Hash, Eq, Ord)]".into()]),
            |ty| {
                let mut ty = ty;
                while let ty::Vec(_ty) = &ty.kind {
                    ty = _ty;
                }
                if matches!(ty.kind, ty::Map(_, _) | ty::Set(_) | ty::F64 | ty::F32) {
                    PredicateResult::No
                } else {
                    PredicateResult::GoOn
                }
            },
        ));

        CONTEXT.set(&cx, || {
            self.plugins.into_iter().for_each(|p| cx.exec_plugin(p));
        });

        std::thread::scope(|scope| {
            let pool = rayon::ThreadPoolBuilder::new();
            let pool = pool
                .spawn_handler(|thread| {
                    let mut builder = std::thread::Builder::new();
                    if let Some(name) = thread.name() {
                        builder = builder.name(name.to_string());
                    }
                    if let Some(size) = thread.stack_size() {
                        builder = builder.stack_size(size);
                    }

                    let cx = cx.clone();
                    builder.spawn_scoped(scope, move || {
                        CONTEXT.set(&cx, || thread.run());
                    })?;
                    Ok(())
                })
                .build()?;

            pool.install(move || {
                let cg = Codegen::new(self.mk_backend.make_backend(cx));
                cg.r#gen().unwrap();
            });

            Ok::<_, rayon::ThreadPoolBuildError>(())
        })
        .unwrap();
    }

    // gen service_global_name and methods for certain service in IdlService
    pub fn init_service(self, service: IdlService) -> anyhow::Result<(String, String)> {
        let _ = tracing_subscriber::fmt::try_init();
        let path = service.path.clone();
        let cx = Self::build_cx_impl(
            vec![service],
            None,
            self.parser,
            self.touches,
            self.ignore_unused,
            self.source_type,
            self.change_case,
            self.touch_files,
            self.keep_unknown_fields,
            self.dedups,
            self.special_namings,
            self.common_crate_name,
            self.split,
            self.with_descriptor,
            self.with_field_mask,
            self.with_comments,
        );

        if matches!(cx.source.selection_kind, SelectionKind::Explicit) {
            Ok(Default::default())
        } else {
            std::thread::scope(|_scope| {
                CONTEXT.set(&cx.clone(), move || {
                    Codegen::new(self.mk_backend.make_backend(cx)).pick_init_service(path)
                })
            })
        }
    }
}

mod test;

#[cfg(test)]
mod touch_semantics_tests {
    use std::path::PathBuf;

    #[test]
    fn touch_with_empty_items_is_ignored() {
        let builder =
            crate::Builder::thrift().touch([(PathBuf::from("a.thrift"), Vec::<String>::new())]);
        assert!(builder.touches.is_empty());
    }

    #[test]
    fn touch_with_non_empty_items_is_kept() {
        let builder =
            crate::Builder::thrift().touch([(PathBuf::from("a.thrift"), vec!["Foo".to_string()])]);
        assert_eq!(builder.touches.len(), 1);
        let (path, items) = &builder.touches[0];
        assert_eq!(path, &PathBuf::from("a.thrift"));
        assert_eq!(items, &vec!["Foo".to_string()]);
    }

    #[test]
    fn touch_mixed_filters_empty_entries() {
        let builder = crate::Builder::thrift().touch([
            (PathBuf::from("a.thrift"), Vec::<String>::new()),
            (PathBuf::from("b.thrift"), vec!["Bar".to_string()]),
        ]);
        assert_eq!(builder.touches.len(), 1);
        assert_eq!(builder.touches[0].0, PathBuf::from("b.thrift"));
    }

    #[test]
    fn touch_files_alias_populates_touch_files() {
        let builder = crate::Builder::thrift().touch_files([PathBuf::from("a.thrift")]);
        assert_eq!(builder.touch_files, vec![PathBuf::from("a.thrift")]);
    }
}
