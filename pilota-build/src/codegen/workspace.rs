use std::{borrow::Cow, path::PathBuf, process::Command, sync::Arc};

use anyhow::bail;
use faststr::FastStr;
use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::CodegenItem;
use crate::{
    db::RirDatabase,
    fmt::fmt_file,
    middle::context::{CrateId, DefLocation},
    rir::{self, ItemPath},
    Codegen, CodegenBackend, Context, DefId,
};

#[derive(Clone)]
pub struct Workspace<B> {
    base_dir: Arc<std::path::Path>,
    cg: Codegen<B>,
}

fn run_cmd(cmd: &mut Command) -> Result<(), anyhow::Error> {
    let status = cmd.status()?;

    if !status.success() {
        bail!("run cmd {:?} failed", cmd)
    }

    Ok(())
}

struct CrateInfo {
    name: FastStr,
    main_mod_path: Option<ItemPath>,
    deps: Vec<FastStr>,
    workspace_deps: Vec<FastStr>,
    items: Vec<DefId>,
    re_pubs: Vec<DefId>,
    user_gen: Option<String>,
}

impl<B> Workspace<B>
where
    B: CodegenBackend + Send,
{
    fn cx(&self) -> &Context {
        &self.cg
    }

    pub fn new(base_dir: PathBuf, cg: Codegen<B>) -> Self {
        Workspace {
            base_dir: Arc::from(base_dir),
            cg,
        }
    }

    pub fn group_defs(&self, entry_def_ids: &[DefId]) -> Result<(), anyhow::Error> {
        let location_map = self.collect_def_ids(entry_def_ids);
        let entry_map = location_map.iter().into_group_map_by(|item| item.1);

        let entry_deps = entry_map
            .iter()
            .map(|(k, v)| {
                let def_ids = v.iter().map(|i| i.0).copied().collect_vec();
                let deps = self.collect_def_ids(&def_ids).into_iter().collect_vec();
                (k, deps)
            })
            .collect::<FxHashMap<_, _>>();

        if !self.base_dir.exists() {
            std::fs::create_dir_all(&*self.base_dir).unwrap();
        }

        let this = self.clone();

        let members = entry_map
            .keys()
            .filter_map(|k| {
                if let DefLocation::Fixed(..) = k {
                    let name = self.cx().crate_name(k);
                    Some(format!("    \"{name}\""))
                } else {
                    None
                }
            })
            .join(",\n");

        let mut cargo_toml = toml::from_str::<toml::Value>(&unsafe {
            String::from_utf8_unchecked(std::fs::read(self.base_dir.join("Cargo.toml")).unwrap())
        })
        .unwrap();

        crate::codegen::toml::merge_tomls(
            &mut cargo_toml,
            toml::from_str::<toml::Value>(&format!(
                r#"[workspace]
    members = [
    {members}
    ]

    [workspace.dependencies]
    pilota = "*"
    anyhow = "1"
    volo = "*"
    volo-thrift = "*""#
            ))
            .unwrap(),
        );

        let workspace_deps = cargo_toml
            .get("workspace")
            .unwrap()
            .get("dependencies")
            .unwrap()
            .as_table()
            .unwrap()
            .keys()
            .map(|s| FastStr::new(s))
            .collect_vec();

        std::fs::write(
            self.base_dir.join("Cargo.toml"),
            toml::to_string_pretty(&cargo_toml).unwrap(),
        )?;

        entry_deps
            .par_iter()
            .try_for_each_with(this, |this, (k, deps)| {
                let name = this.cx().crate_name(k);
                let deps = deps.iter().filter(|dep| dep.1 != ***k).collect_vec();
                let (main_mod_path, re_pubs, deps) = match k {
                    DefLocation::Fixed(_, path) => (
                        Some(path.clone()),
                        deps.iter().map(|v| v.0).collect_vec(),
                        deps.iter()
                            .map(|dep| this.cx().crate_name(&dep.1))
                            .sorted()
                            .dedup()
                            .collect_vec(),
                    ),
                    DefLocation::Dynamic => (None, vec![], vec![]),
                };
                this.create_crate(
                    &this.base_dir,
                    CrateInfo {
                        main_mod_path,
                        workspace_deps: workspace_deps.clone(),
                        name,
                        re_pubs,
                        items: entry_map[*k].iter().map(|(k, _)| **k).collect_vec(),
                        deps,
                        user_gen: this.cx().plugin_gen.get(k).map(|v| v.value().clone()),
                    },
                )
            })?;

        Ok(())
    }

    fn collect_def_ids(&self, input: &[DefId]) -> FxHashMap<DefId, DefLocation> {
        use crate::middle::ty::Visitor;
        struct PathCollector<'a> {
            map: &'a mut FxHashMap<DefId, DefLocation>,
            cx: &'a Context,
        }

        impl crate::ty::Visitor for PathCollector<'_> {
            fn visit_path(&mut self, path: &crate::rir::Path) {
                collect(self.cx, path.did, self.map)
            }
        }

        fn collect(cx: &Context, def_id: DefId, map: &mut FxHashMap<DefId, DefLocation>) {
            if let Some(_location) = map.get_mut(&def_id) {
                return;
            }
            if !matches!(&*cx.item(def_id).unwrap(), rir::Item::Mod(_)) {
                let file_id = cx.node(def_id).unwrap().file_id;

                if cx.input_files().contains(&file_id) {
                    let type_graph = cx.type_graph();
                    let node = type_graph.node_map[&def_id];
                    for from in type_graph
                        .graph
                        .neighbors_directed(node, petgraph::Direction::Incoming)
                    {
                        let from_def_id = type_graph.id_map[&from];
                        let from_file_id = cx.node(from_def_id).unwrap().file_id;
                        if !cx.input_files().contains(&from_file_id)
                            || map
                                .get(&from_def_id)
                                .map(|v| match v {
                                    DefLocation::Fixed(_, _) => false,
                                    DefLocation::Dynamic => true,
                                })
                                .unwrap_or_default()
                        {
                            map.insert(def_id, DefLocation::Dynamic);
                            return;
                        }
                    }
                    let file = cx.file(file_id).unwrap();
                    map.insert(
                        def_id,
                        DefLocation::Fixed(CrateId { main_file: file_id }, file.package.clone()),
                    );
                } else {
                    map.insert(def_id, DefLocation::Dynamic);
                }
            }

            let node = cx.node(def_id).unwrap();
            tracing::trace!("collecting {:?}", node.expect_item().symbol_name());

            node.related_nodes
                .iter()
                .for_each(|def_id| collect(cx, *def_id, map));

            let item = node.expect_item();

            match item {
                rir::Item::Message(m) => m
                    .fields
                    .iter()
                    .for_each(|f| PathCollector { cx, map }.visit(&f.ty)),
                rir::Item::Enum(e) => e
                    .variants
                    .iter()
                    .flat_map(|v| &v.fields)
                    .for_each(|ty| PathCollector { cx, map }.visit(ty)),
                rir::Item::Service(s) => {
                    s.extend.iter().for_each(|p| collect(cx, p.did, map));
                    s.methods
                        .iter()
                        .flat_map(|m| m.args.iter().map(|f| &f.ty).chain(std::iter::once(&m.ret)))
                        .for_each(|ty| PathCollector { cx, map }.visit(ty));
                }
                rir::Item::NewType(n) => PathCollector { cx, map }.visit(&n.ty),
                rir::Item::Const(c) => {
                    PathCollector { cx, map }.visit(&c.ty);
                }
                rir::Item::Mod(m) => {
                    m.items.iter().for_each(|i| collect(cx, *i, map));
                }
            }
        }
        let mut map = FxHashMap::default();

        input.iter().for_each(|def_id| {
            collect(&self.cg, *def_id, &mut map);
        });

        map
    }

    fn create_crate(
        &self,
        base_dir: impl AsRef<std::path::Path>,
        info: CrateInfo,
    ) -> anyhow::Result<()> {
        if !base_dir.as_ref().join(&*info.name).exists() {
            run_cmd(
                Command::new("cargo")
                    .arg("init")
                    .arg("--lib")
                    .current_dir(base_dir.as_ref())
                    .arg(&*info.name),
            )?;
        };

        let cargo_toml_path = base_dir.as_ref().join(&*info.name).join("Cargo.toml");

        let mut cargo_toml = toml::from_str::<toml::Value>(&unsafe {
            String::from_utf8_unchecked(std::fs::read(&cargo_toml_path)?)
        })
        .unwrap();

        let deps = info
            .deps
            .iter()
            .map(|s| Cow::from(format!(r#"{} = {{ path = "../{}" }}"#, s, s)))
            .chain(
                info.workspace_deps
                    .iter()
                    .map(|s| Cow::from(format!(r#"{s}.workspace = true"#))),
            )
            .join("\n");

        super::toml::merge_tomls(
            &mut cargo_toml,
            toml::from_str::<toml::Value>(&format!("[dependencies]\n{deps}")).unwrap(),
        );

        std::fs::write(
            &cargo_toml_path,
            toml::to_string_pretty(&cargo_toml).unwrap(),
        )?;

        let mut lib_rs_stream = String::default();
        lib_rs_stream.push_str("include!(\"gen.rs\");\n");
        lib_rs_stream.push_str("pub use gen::*;\n\n");

        if let Some(user_gen) = info.user_gen {
            if !user_gen.is_empty() {
                lib_rs_stream.push_str("include!(\"custom.rs\");\n");

                let mut custom_rs_stream = String::default();
                custom_rs_stream.push_str(&user_gen);

                let custom_rs = base_dir.as_ref().join(&*info.name).join("src/custom.rs");

                std::fs::write(&custom_rs, custom_rs_stream)?;

                fmt_file(custom_rs);
            }
        }

        let mut gen_rs_stream = String::default();
        self.cg.write_items(
            &mut gen_rs_stream,
            info.items
                .iter()
                .map(|def_id| CodegenItem::from(*def_id))
                .chain(info.re_pubs.into_iter().map(|def_id| CodegenItem {
                    def_id,
                    kind: super::CodegenKind::RePub,
                })),
        );
        if let Some(main_mod_path) = info.main_mod_path {
            gen_rs_stream.push_str(&format!(
                "pub use {}::*;",
                main_mod_path.iter().map(|item| item.to_string()).join("::")
            ));
        }
        gen_rs_stream = format! {r#"pub mod gen {{
            #![allow(warnings, clippy::all)]
            {gen_rs_stream}
        }}"#};

        let lib_rs_stream = lib_rs_stream.lines().map(|s| s.trim_end()).join("\n");
        let gen_rs_stream = gen_rs_stream.lines().map(|s| s.trim_end()).join("\n");

        let lib_rs = base_dir.as_ref().join(&*info.name).join("src/lib.rs");
        let gen_rs = base_dir.as_ref().join(&*info.name).join("src/gen.rs");

        std::fs::write(&lib_rs, lib_rs_stream)?;
        std::fs::write(&gen_rs, gen_rs_stream)?;

        fmt_file(lib_rs);
        fmt_file(gen_rs);

        Ok(())
    }

    pub(crate) fn write_crates(self) -> anyhow::Result<()> {
        self.group_defs(&self.cx().codegen_items)
    }
}
