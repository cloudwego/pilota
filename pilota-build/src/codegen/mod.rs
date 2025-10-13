use std::{
    io::Write,
    ops::Deref,
    path::{Path, PathBuf},
};

use ahash::{AHashMap, AHashSet};
use dashmap::{DashMap, mapref::one::RefMut};
use faststr::FastStr;
use itertools::Itertools;
use normpath::PathExt;
use pkg_tree::PkgNode;
use quote::quote;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use traits::CodegenBackend;

use self::workspace::Workspace;
use crate::{
    Context, Symbol,
    db::RirDatabase,
    dedup::def_id_equal,
    fmt::fmt_file,
    middle::{
        self,
        context::{Mode, tls::CUR_ITEM},
        rir,
    },
    rir::{Item, NodeKind},
    symbol::{DefId, EnumRepr, FileId, ModPath},
    tags::protobuf::Deprecated,
};

pub(crate) mod pkg_tree;
pub mod toml;
pub(crate) mod traits;

mod workspace;

pub mod pb;
pub mod thrift;

#[derive(Clone)]
pub struct Codegen<B> {
    backend: B,
}

impl<B> Deref for Codegen<B>
where
    B: CodegenBackend,
{
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        self.backend.cx()
    }
}

impl<B> Codegen<B> {
    pub fn new(backend: B) -> Self {
        Codegen { backend }
    }
}

#[derive(Clone, Copy)]
pub enum CodegenKind {
    Direct,
    RePub,
}

#[derive(Clone, Copy)]
pub struct CodegenItem {
    def_id: DefId,
    kind: CodegenKind,
}

impl From<DefId> for CodegenItem {
    fn from(value: DefId) -> Self {
        CodegenItem {
            def_id: value,
            kind: CodegenKind::Direct,
        }
    }
}

impl<B> Codegen<B>
where
    B: CodegenBackend + Send,
{
    fn is_deprecated(&self, def_id: DefId) -> bool {
        self.node_tags(def_id)
            .and_then(|tags| tags.get::<Deprecated>().map(|d| d.0))
            .unwrap_or_default()
    }

    pub fn write_struct(&self, def_id: DefId, stream: &mut String, s: &rir::Message) {
        let name = self.rust_name(def_id);

        let mut fields = s
            .fields
            .iter()
            .map(|f| {
                let name = self.rust_name(f.did);
                self.with_adjust(f.did, |adjust| {
                    let ty = self.codegen_item_ty(f.ty.kind.clone());
                    let mut ty = format!("{ty}");

                    if let Some(adjust) = adjust {
                        if adjust.boxed() {
                            ty = format!("::std::boxed::Box<{ty}>")
                        }
                    }

                    if f.is_optional() {
                        ty = format!("::std::option::Option<{ty}>")
                    }

                    let attrs = adjust.iter().flat_map(|a| a.attrs()).join("");

                    let deprecated_attr = if self.is_deprecated(f.did) {
                        "#[deprecated]\n"
                    } else {
                        ""
                    };

                    format! {
                        r#"{attrs}
                        {deprecated_attr}pub {name}: {ty},"#
                    }
                })
            })
            .join("\n");

        if self.cache.keep_unknown_fields.contains(&def_id) {
            fields.push_str("pub _unknown_fields: ::pilota::BytesVec,");
        }

        if !s.is_wrapper && self.config.with_field_mask {
            fields.push_str(
                "pub _field_mask: ::std::option::Option<::pilota_thrift_fieldmask::FieldMask>,",
            );
        }

        let deprecated_attr = if self.is_deprecated(def_id) {
            "#[deprecated]\n"
        } else {
            ""
        };

        stream.push_str(&format! {
            r#"#[derive(Clone, PartialEq)]
                {deprecated_attr}pub struct {name} {{
                    {fields}
                }}"#
        });

        self.backend.codegen_struct_impl(def_id, stream, s);
    }

    pub fn write_item(
        &self,
        stream: &mut String,
        item: CodegenItem,
        dup: &mut AHashMap<FastStr, Vec<DefId>>,
    ) {
        CUR_ITEM.set(&item.def_id, || match item.kind {
            CodegenKind::Direct => {
                if !self.duplicate(dup, item.def_id) {
                    let def_id = item.def_id;
                    let item = self.item(def_id).unwrap();
                    tracing::trace!("write item {}", item.symbol_name());
                    self.with_adjust(def_id, |adjust| {
                        let attrs = adjust.iter().flat_map(|a| a.attrs()).join("\n");

                        let impls = adjust
                            .iter()
                            .flat_map(|a| &a.nested_items)
                            .sorted()
                            .join("\n");
                        stream.push_str(&impls);
                        stream.push_str(&attrs);
                    });

                    match &*item {
                        middle::rir::Item::Message(s) => {
                            self.write_struct(def_id, stream, s);
                        }
                        middle::rir::Item::Enum(e) => self.write_enum(def_id, stream, e),
                        middle::rir::Item::Service(s) => self.write_service(def_id, stream, s),
                        middle::rir::Item::NewType(t) => self.write_new_type(def_id, stream, t),
                        middle::rir::Item::Const(c) => self.write_const(def_id, stream, c),
                        middle::rir::Item::Mod(m) => {
                            let name = self.rust_name(def_id);
                            let mut inner = Default::default();
                            self.backend.codegen_pilota_buf_trait(&mut inner);
                            m.items.iter().for_each(|def_id| {
                                self.write_item(&mut inner, (*def_id).into(), dup)
                            });

                            if self.config.with_descriptor && m.extensions.has_extendees() {
                                let cur_pkg = self.item_path(def_id);
                                self.backend.codegen_mod_exts(
                                    &mut inner,
                                    &name,
                                    &cur_pkg,
                                    &m.extensions,
                                );
                            }

                            let name = self.rust_name(def_id);
                            stream.push_str(&format! {
                                r#"pub mod {name} {{
                                    {inner}
                                }}"#
                            });
                        }
                    }
                };
            }
            CodegenKind::RePub => {
                let path = self
                    .item_path(item.def_id)
                    .iter()
                    .map(|item| item.to_string())
                    .join("::");
                stream.push_str(format!("pub use ::{path};").as_str());
            }
        })
    }

    fn duplicate(&self, dup: &mut AHashMap<FastStr, Vec<DefId>>, def_id: DefId) -> bool {
        let name = self.rust_name(def_id);
        if !self.cache.dedups.contains(&name.0) {
            return false;
        }
        let dup = dup.entry(name.0).or_default();
        for id in dup.iter() {
            if def_id_equal(self.nodes(), *id, def_id) {
                return true;
            }
        }
        dup.push(def_id);
        false
    }

    pub fn write_enum_as_new_type(
        &self,
        def_id: DefId,
        stream: &mut String,
        e: &middle::rir::Enum,
    ) {
        let name = self.rust_name(def_id);

        let repr = match e.repr {
            Some(EnumRepr::I32) => quote!(i32),
            _ => panic!(),
        };

        let variants = e
            .variants
            .iter()
            .map(|v| {
                let name = self.rust_name(v.did);

                let discr = v.discr.unwrap();
                let discr = match e.repr {
                    Some(EnumRepr::I32) => discr as i32,
                    None => panic!(),
                };

                let deprecated_attr = if self.is_deprecated(v.did) {
                    "#[deprecated]\n"
                } else {
                    ""
                };

                (
                    format!("{deprecated_attr}pub const {name}: Self = Self({discr});"),
                    format!("Self({discr}) => ::std::string::String::from(\"{name}\"),"),
                )
            })
            .collect::<Vec<_>>();
        let variants_const = variants.iter().map(|(v, _)| v).join("");
        let variants_as_str_fields = variants.iter().map(|(_, v)| v).join("");
        let try_from_arms = e
            .variants
            .iter()
            .map(|v| {
                let name = self.rust_name(v.did);
                let discr = v.discr.unwrap();
                let discr = match e.repr {
                    Some(EnumRepr::I32) => discr as i32,
                    None => panic!(),
                };
                format!("{discr} => Some(Self::{name}),")
            })
            .join("\n");

        let deprecated_attr = if self.is_deprecated(def_id) {
            "#[deprecated]\n"
        } else {
            ""
        };

        let impl_enum_message = if self.config.with_descriptor {
            self.backend.codegen_impl_enum_message(&name)
        } else {
            Default::default()
        };

        stream.push_str(&format! {
            r#"#[derive(Clone, PartialEq, Copy)]
            #[repr(transparent)]
            {deprecated_attr}pub struct {name}({repr});

            impl {name} {{
                {variants_const}

                pub fn inner(&self) -> {repr} {{
                    self.0
                }}

                pub fn to_string(&self) -> ::std::string::String {{
                    match self {{
                        {variants_as_str_fields}
                        Self(val) => val.to_string(),
                    }}
                }}

                pub fn try_from_{repr}(value: {repr}) -> ::std::option::Option<Self> {{
                    match value {{
                        {try_from_arms}
                        _ => None,
                    }}
                }}
            }}

            {impl_enum_message}

            impl ::std::convert::From<{repr}> for {name} {{
                fn from(value: {repr}) -> Self {{
                    Self(value)
                }}
            }}

            impl ::std::convert::From<{name}> for {repr} {{
                fn from(value: {name}) -> {repr} {{
                    value.0
                }}
            }}

            "#
        });

        self.backend.codegen_enum_impl(def_id, stream, e);
    }

    pub fn write_enum(&self, def_id: DefId, stream: &mut String, e: &middle::rir::Enum) {
        if e.repr.is_some() {
            return self.write_enum_as_new_type(def_id, stream, e);
        }
        let name = self.rust_name(def_id);

        let mut keep = true;
        let mut variants = e
            .variants
            .iter()
            .map(|v| {
                let name = self.rust_name(v.did);

                self.with_adjust(v.did, |adjust| {
                    let attrs = adjust.iter().flat_map(|a| a.attrs()).join("\n");

                    let fields = v
                        .fields
                        .iter()
                        .map(|ty| self.codegen_item_ty(ty.kind.clone()).to_string())
                        .join(",");

                    let fields_stream = if fields.is_empty() {
                        keep = false;
                        Default::default()
                    } else {
                        format!("({fields})")
                    };

                    format!(
                        r#"{attrs}
                        {name} {fields_stream},"#
                    )
                })
            })
            .join("\n");

        if self.cache.keep_unknown_fields.contains(&def_id) && keep {
            variants.push_str("_UnknownFields(::pilota::BytesVec),");
        }
        stream.push_str(&format! {
            r#"
            #[derive(Clone, PartialEq)]
            pub enum {name} {{
                {variants}
            }}
            "#
        });

        self.backend.codegen_enum_impl(def_id, stream, e);
    }

    pub fn write_service(&self, def_id: DefId, stream: &mut String, s: &middle::rir::Service) {
        let name = self.rust_name(def_id);
        let methods = self.service_methods(def_id);

        let methods = methods
            .iter()
            .map(|m| self.backend.codegen_service_method(def_id, m))
            .join("\n");

        let deprecated_attr = if self.is_deprecated(def_id) {
            "#[deprecated]\n"
        } else {
            ""
        };

        stream.push_str(&format! {
            r#"
            {deprecated_attr}pub trait {name} {{
                {methods}
            }}
            "#
        });
        self.backend.codegen_service_impl(def_id, stream, s);
    }

    /// get service information for volo-cli init, return path of service and
    /// methods
    pub fn get_init_service(&self, def_id: DefId) -> (String, String) {
        CUR_ITEM.set(&def_id, || {
            let service_name = self.rust_name(def_id).to_string();
            let mod_prefix = self.mod_path(def_id);
            let service_path = if mod_prefix.is_empty() {
                service_name
            } else {
                let mod_path = mod_prefix.iter().map(|item| item.to_string()).join("::");
                format!("{mod_path}::{service_name}")
            };
            tracing::debug!("service_path: {}", service_path);
            let methods = self.service_methods(def_id);

            let methods = methods
                .iter()
                .map(|m| {
                    self.backend
                        .codegen_service_method_with_global_path(def_id, m)
                })
                .join("\n");

            (service_path, methods)
        })
    }

    // pick first service as init service from idlservice
    pub fn pick_init_service(&self, path: PathBuf) -> anyhow::Result<(String, String)> {
        // convert path to absolute path to match with file_id_map
        let path = path
            .normalize()
            .map_err(|e| {
                anyhow::Error::msg(format!(
                    "Normalize path {} failed: {}, please check service path",
                    path.display(),
                    e
                ))
            })?
            .into_path_buf();
        tracing::debug!("path {:?}", path);
        let file_id: FileId = self.file_id(path).unwrap();
        let item = self
            .cache
            .codegen_items
            .iter()
            .copied()
            .filter(|def_id| {
                // select service kind
                let item = self.item(*def_id).unwrap();
                matches!(&*item, middle::rir::Item::Service(_))
            })
            .find(
                // check for same file
                |def_id| self.node(*def_id).unwrap().file_id == file_id,
            );
        match item {
            Some(def_id) => Ok(self.get_init_service(def_id)),
            None => Err(anyhow::anyhow!("No service found.")),
        }
    }

    pub fn write_new_type(&self, def_id: DefId, stream: &mut String, t: &middle::rir::NewType) {
        let name = self.rust_name(def_id);
        let ty = self.codegen_item_ty(t.ty.kind.clone());
        stream.push_str(&format! {
            r#"
            #[derive(Clone, PartialEq)]
            pub struct {name}(pub {ty});

            impl ::std::ops::Deref for {name} {{
                type Target = {ty};

                fn deref(&self) -> &Self::Target {{
                    &self.0
                }}
            }}

            impl From<{ty}> for {name} {{
                fn from(v: {ty}) -> Self {{
                    Self(v)
                }}
            }}

            "#
        });
        self.backend.codegen_newtype_impl(def_id, stream, t);
    }

    pub fn write_const(&self, did: DefId, stream: &mut String, c: &middle::rir::Const) {
        let mut ty = self.codegen_ty(did);

        let name = self.rust_name(did);

        if name.to_string().starts_with("__PILOTA_PB_EXT_") {
            return;
        }

        stream.push_str(&self.def_lit(&name, &c.lit, &mut ty).unwrap())
    }

    pub fn write_workspace(self, base_dir: PathBuf) -> anyhow::Result<()> {
        let ws = Workspace::new(base_dir, self);
        ws.write_crates()
    }

    pub fn write_items(
        &self,
        stream: &mut String,
        mod_items: AHashMap<ModPath, Vec<CodegenItem>>,
        file_has_direct: AHashMap<FileId, bool>,
        base_dir: &Path,
    ) {
        // 1. global level
        // 1.1 register mod file descriptor
        if self.config.with_descriptor {
            let mods_files_with_direct_items = mod_items
                .keys()
                .flat_map(|mod_path| {
                    self.cache
                        .mod_files
                        .get(mod_path)
                        .unwrap()
                        .iter()
                        .filter_map(|file_id| {
                            if *file_has_direct.get(file_id).unwrap()
                                && let Some(file_path) = self.file_paths().get(file_id)
                            {
                                Some((mod_path.clone(), file_path.clone()))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            if !mods_files_with_direct_items.is_empty() {
                self.backend
                    .codegen_register_mod_file_descriptor(stream, &mods_files_with_direct_items);
            }
        }

        // 2. mod stream level
        let mut pkgs: DashMap<ModPath, String> = Default::default();
        let this = self.clone();
        mod_items
            .par_iter()
            .for_each_with(this, |this, (mod_path, items)| {
                let mut stream = pkgs.entry(mod_path.clone()).or_default();
                // 2.1 file
                for file_id in this.cache.mod_files.get(mod_path).unwrap().iter() {
                    if this.config.with_descriptor && *file_has_direct.get(file_id).unwrap() {
                        let file = this.file(*file_id).unwrap();
                        // 2.1.1 file descriptor
                        this.backend.codegen_file_descriptor_at_mod(
                            &mut stream,
                            &file,
                            mod_path,
                            *file_has_direct.get(file_id).unwrap(),
                        );

                        // 2.1.2 file extensions
                        let file_pkg = file
                            .package
                            .iter()
                            .map(|s| s.0.clone())
                            .collect::<Vec<_>>()
                            .join("::");
                        if file_pkg == "google::protobuf" {
                            continue;
                        }
                        let mod_pkg = mod_path.iter().cloned().collect::<Vec<_>>().join("::");
                        if file_pkg == mod_pkg {
                            let filename_lower = this
                                .file_name(*file_id)
                                .unwrap()
                                .replace(".", "_")
                                .to_lowercase();
                            if file.extensions.has_extendees() {
                                this.backend.codegen_file_exts(
                                    &mut stream,
                                    &filename_lower,
                                    &file.package,
                                    &file.extensions,
                                );
                            }
                        }
                    }
                }

                // 2.2 mod
                let mut dup = AHashMap::default();

                let span = tracing::span!(tracing::Level::TRACE, "write_mod", path = ?mod_path);
                let _enter = span.enter();

                if let Some(mod_idx) = this.cache.mod_idxes.get(mod_path) {
                    let mod_item = this.item(*mod_idx).unwrap();

                    if let middle::rir::Item::Mod(m) = &*mod_item {
                        if this.config.with_descriptor && m.extensions.has_extendees() {
                            let name = this.rust_name(*mod_idx);
                            let cur_pkg = this.item_path(*mod_idx).clone();
                            this.backend.codegen_mod_exts(
                                &mut stream,
                                &name,
                                &cur_pkg,
                                &m.extensions,
                            );
                        }
                    }
                }

                if this.config.split {
                    Self::write_split_mod(this, base_dir, p, def_ids, &mut stream, &mut dup);
                } else {
                    for def_id in def_ids.iter() {
                        this.write_item(&mut stream, *def_id, &mut dup)
                    }
                }
            });

        let keys = pkgs.iter().map(|kv| kv.key().clone()).collect_vec();
        let pkg_node = PkgNode::from_pkgs(&keys.iter().map(|s| &**s).collect_vec());
        tracing::debug!(?pkg_node);

        self.write_stream(&mut pkgs, stream, &pkg_node);
    }

    fn write_stream(
        &self,
        pkgs: &mut DashMap<ModPath, String>,
        stream: &mut String,
        nodes: &[PkgNode],
    ) {
        for node in nodes.iter().sorted_by_key(|x| &x.path) {
            let mut inner_stream = String::default();
            if let Some((_, node_stream)) = pkgs.remove(&node.path) {
                inner_stream.push_str(&node_stream);
            }

            self.write_stream(pkgs, &mut inner_stream, &node.children);
            let name = node.ident();
            if name.clone().unwrap_or_default() == "" {
                stream.push_str(&inner_stream);
                return;
            }

            let name = Symbol::from(name.unwrap());
            let mut pilota_buf_trait = Default::default();
            self.backend.codegen_pilota_buf_trait(&mut pilota_buf_trait);
            stream.push_str(&format! {
                r#"
                pub mod {name} {{
                    {pilota_buf_trait}
                    {inner_stream}
                }}
                "#
            });
        }
    }

    fn write_split_mod(
        this: &mut Codegen<B>,
        base_dir: &Path,
        mod_path: &ModPath,
        def_ids: &[CodegenItem],
        stream: &mut RefMut<ModPath, String>,
        dup: &mut AHashMap<FastStr, Vec<DefId>>,
    ) {
        let base_mod_name = mod_path.iter().map(|s| s.to_string()).join("/");
        let mod_file_name = format!("{base_mod_name}/mod.rs");
        let mut mod_stream = String::new();

        let mut existing_file_names: AHashSet<String> = AHashSet::new();

        for def_id in def_ids.iter() {
            let mut item_stream = String::new();
            let node = this.db.node(def_id.def_id).unwrap();
            let name_prefix = match node.kind {
                NodeKind::Item(ref item) => match item.as_ref() {
                    Item::Message(_) => "message",
                    Item::Enum(_) => "enum",
                    Item::Service(_) => "service",
                    Item::NewType(_) => "new_type",
                    Item::Const(_) => "const",
                    Item::Mod(_) => "mod",
                },
                NodeKind::Variant(_) => "variant",
                NodeKind::Field(_) => "field",
                NodeKind::Method(_) => "method",
                NodeKind::Arg(_) => "arg",
            };

            let mod_dir = base_dir.join(base_mod_name.clone());

            let simple_name = format!("{}_{}", name_prefix, node.name());
            let unique_name = Self::generate_unique_name(&existing_file_names, &simple_name);
            existing_file_names.insert(unique_name.to_ascii_lowercase().clone());
            let file_name = format!("{unique_name}.rs");
            this.write_item(&mut item_stream, *def_id, dup);

            let full_path = mod_dir.join(file_name.clone());
            std::fs::create_dir_all(mod_dir).unwrap();

            let item_stream = item_stream.lines().map(|s| s.trim_end()).join("\n");
            let mut file =
                std::io::BufWriter::new(std::fs::File::create(full_path.clone()).unwrap());
            file.write_all(item_stream.as_bytes()).unwrap();
            file.flush().unwrap();
            fmt_file(full_path);

            mod_stream.push_str(format!("include!(\"{file_name}\");\n").as_str());
        }

        let mod_path = base_dir.join(&mod_file_name);
        let mod_stream = mod_stream.lines().map(|s| s.trim_end()).join("\n");
        let mut mod_file = std::io::BufWriter::new(std::fs::File::create(&mod_path).unwrap());
        mod_file.write_all(mod_stream.as_bytes()).unwrap();
        mod_file.flush().unwrap();
        fmt_file(&mod_path);

        stream.push_str(format!("include!(\"{mod_file_name}\");\n").as_str());
    }

    /**
        On Windows and macOS, files names are case-insensitive
        To avoid problems when generating files for services with similar names, e.g.
        testService and TestService, such names are de-duplicated by adding a number to their nam5e
    */
    fn generate_unique_name(existing_names: &AHashSet<String>, simple_name: &str) -> String {
        let mut counter = 1;
        let mut name = simple_name.to_string();
        while existing_names.contains(name.to_ascii_lowercase().as_str()) {
            counter += 1;
            name = format!("{simple_name}_{counter}")
        }
        name
    }

    fn collect_codegen_items(
        &self,
    ) -> (AHashMap<ModPath, Vec<CodegenItem>>, AHashMap<FileId, bool>) {
        let mut file_has_direct = AHashMap::default();

        let mod_items = self
            .cache
            .mod_items
            .iter()
            .map(|(mod_path, items)| {
                file_has_direct.extend(
                    self.cache
                        .mod_files
                        .values()
                        .flat_map(|file_ids| file_ids.iter().map(|file_id| (*file_id, true))),
                );
                (
                    mod_path.clone(),
                    items
                        .iter()
                        .map(|def_id| (*def_id).into())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<AHashMap<_, _>>();
        (mod_items, file_has_direct)
    }

    pub fn write_file(self, ns_name: Symbol, file_name: impl AsRef<Path>) {
        let base_dir = file_name.as_ref().parent().unwrap();
        let mut stream = String::default();
        self.backend.codegen_pilota_buf_trait(&mut stream);

        let (mod_items, file_has_direct) = self.collect_codegen_items();

        self.write_items(&mut stream, mod_items, file_has_direct, base_dir);

        stream = format! {r#"pub mod {ns_name} {{
                #![allow(warnings, clippy::all)]
                {stream}
            }}"#};
        let stream = stream.lines().map(|s| s.trim_end()).join("\n");
        let mut file = std::io::BufWriter::new(std::fs::File::create(&file_name).unwrap());
        file.write_all(stream.as_bytes()).unwrap();
        file.flush().unwrap();
        fmt_file(file_name)
    }

    pub fn r#gen(self) -> anyhow::Result<()> {
        match &*self.source.mode.clone() {
            Mode::Workspace(info) => self.write_workspace(info.dir.clone()),
            Mode::SingleFile { file_path: p } => {
                self.write_file(
                    FastStr::new(
                        p.file_name()
                            .and_then(|s| s.to_str())
                            .and_then(|s| s.split('.').next())
                            .unwrap(),
                    )
                    .into(),
                    p,
                );
                Ok(())
            }
        }
    }
}
