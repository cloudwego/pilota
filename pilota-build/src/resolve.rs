use std::{ptr::NonNull, sync::Arc};

use ahash::AHashMap;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    errors,
    index::Idx,
    ir::{self, visit::Visitor},
    middle::{
        self,
        ext::pb::ExtendeeType,
        rir::{
            Arg, Const, DefKind, Enum, EnumVariant, Field, FieldKind, File, Item, ItemPath,
            Literal, Message, Method, MethodSource, NewType, Node, NodeKind, Path, Service,
        },
        ty::{self, Ty},
    },
    rir::Mod,
    symbol::{DefId, EnumRepr, FileId, Ident, Symbol},
    tags::{RustType, RustWrapperArc, TagId, Tags, protobuf::OptionalRepeated},
    ty::{Folder, TyKind},
};

struct ModuleData {
    resolutions: SymbolTable,
    _kind: DefKind,
}

#[derive(Clone, Copy)]
enum ModuleId {
    File(FileId),
    Node(DefId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Namespace {
    Value,
    Ty,
    Mod,
}

pub struct CollectDef<'a> {
    resolver: &'a mut Resolver,
    parent: Option<ModuleId>,
}

impl<'a> CollectDef<'a> {
    pub fn new(resolver: &'a mut Resolver) -> Self {
        CollectDef {
            resolver,
            parent: None,
        }
    }
}

impl CollectDef<'_> {
    fn def_item(&mut self, item: &ir::Item, ns: Namespace) -> DefId {
        let parent = self.parent.as_ref().unwrap();
        let did = self.resolver.did_counter.inc_one();
        let table = match parent {
            ModuleId::File(file_id) => self.resolver.file_sym_map.entry(*file_id).or_default(),
            ModuleId::Node(def_id) => {
                &mut self
                    .resolver
                    .def_modules
                    .get_mut(def_id)
                    .unwrap()
                    .resolutions
            }
        };

        let name = item.name();

        tracing::debug!("def {} with DefId({:?})", name, did);

        if match ns {
            Namespace::Value => table.value.insert(name.clone(), did),
            Namespace::Ty => table.ty.insert(name.clone(), did),
            Namespace::Mod => table.mods.insert(name.clone(), did),
        }
        .is_some()
        {
            self.resolver
                .errors
                .emit_error(format!("duplicate definition of `{name}`"));
        };

        self.resolver.def_modules.insert(
            did,
            ModuleData {
                resolutions: Default::default(),
                _kind: match &item.kind {
                    ir::ItemKind::Message(_)
                    | ir::ItemKind::Enum(_)
                    | ir::ItemKind::Service(_)
                    | ir::ItemKind::NewType(_) => DefKind::Type,
                    ir::ItemKind::Const(_) => DefKind::Value,
                    ir::ItemKind::Mod(_) => DefKind::Mod,
                    ir::ItemKind::Use(_) => unreachable!(),
                },
            },
        );

        did
    }

    fn def_sym(&mut self, ns: Namespace, sym: Symbol) {
        let parent = match self.parent.unwrap() {
            ModuleId::File(_) => panic!(),
            ModuleId::Node(def_id) => def_id,
        };

        tracing::debug!("def {} for {:?} in {:?}", sym, parent, ns);

        let table = match ns {
            Namespace::Value => {
                &mut self
                    .resolver
                    .def_modules
                    .get_mut(&parent)
                    .unwrap()
                    .resolutions
                    .value
            }
            Namespace::Ty => {
                &mut self
                    .resolver
                    .def_modules
                    .get_mut(&parent)
                    .unwrap()
                    .resolutions
                    .ty
            }
            Namespace::Mod => {
                &mut self
                    .resolver
                    .def_modules
                    .get_mut(&parent)
                    .unwrap()
                    .resolutions
                    .mods
            }
        };
        let def_id = self.resolver.did_counter.inc_one();
        table.insert(sym, def_id);
    }
}

impl ir::visit::Visitor for CollectDef<'_> {
    fn visit_file(&mut self, file: Arc<ir::File>) {
        self.parent = Some(ModuleId::File(file.id));
        ir::visit::walk_file(self, file);
        self.parent = None;
    }

    fn visit_item(&mut self, item: Arc<ir::Item>) {
        if let Some(did) = match &item.kind {
            ir::ItemKind::Message(_)
            | ir::ItemKind::Enum(_)
            | ir::ItemKind::Service(_)
            | ir::ItemKind::NewType(_) => Some(self.def_item(&item, Namespace::Ty)),
            ir::ItemKind::Const(_) => Some(self.def_item(&item, Namespace::Value)),
            ir::ItemKind::Mod(_) => Some(self.def_item(&item, Namespace::Mod)),
            ir::ItemKind::Use(_) => None,
        } {
            let prev_parent = self.parent.replace(ModuleId::Node(did));
            if let ir::ItemKind::Enum(e) = &item.kind {
                e.variants.iter().for_each(|e| {
                    self.def_sym(Namespace::Value, (*e.name).clone());
                })
            }
            ir::visit::walk_item(self, item);
            self.parent = prev_parent;
        }
    }
}

#[derive(Default, Debug)]
pub struct SymbolTable {
    pub(crate) value: AHashMap<Symbol, DefId>,
    pub(crate) ty: AHashMap<Symbol, DefId>,
    pub(crate) mods: AHashMap<Symbol, DefId>,
}

pub struct Resolver {
    pub(crate) did_counter: DefId,
    pub(crate) file_sym_map: FxHashMap<FileId, SymbolTable>,
    def_modules: FxHashMap<DefId, ModuleData>,
    blocks: Vec<NonNull<SymbolTable>>,
    parent_node: Option<DefId>,
    nodes: FxHashMap<DefId, Node>,
    tags_id_counter: TagId,
    tags: FxHashMap<TagId, Arc<Tags>>,
    cur_file: Option<FileId>,
    ir_files: FxHashMap<FileId, Arc<ir::File>>,
    errors: errors::Handler,
    args: FxHashSet<DefId>,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            tags_id_counter: TagId::from_usize(0),
            tags: Default::default(),
            blocks: Default::default(),
            def_modules: Default::default(),
            did_counter: DefId::from_usize(0),
            file_sym_map: Default::default(),
            nodes: Default::default(),
            ir_files: Default::default(),
            errors: Default::default(),
            cur_file: None,
            parent_node: None,
            args: Default::default(),
        }
    }
}

pub struct ResolveResult {
    pub files: FxHashMap<FileId, Arc<File>>,
    pub nodes: FxHashMap<DefId, Node>,
    pub tags: FxHashMap<TagId, Arc<Tags>>,
    pub args: FxHashSet<DefId>,
}

pub struct ResolvedSymbols {
    ty: Vec<DefId>,
    value: Vec<DefId>,
    r#mod: Vec<DefId>,
}

impl Resolver {
    fn get_def_id(&self, ns: Namespace, sym: &Symbol) -> DefId {
        if let Some(parent) = self.parent_node {
            *match ns {
                Namespace::Value => self.def_modules[&parent].resolutions.value.get(sym),
                Namespace::Ty => self.def_modules[&parent].resolutions.ty.get(sym),
                Namespace::Mod => self.def_modules[&parent].resolutions.mods.get(sym),
            }
            .unwrap()
        } else {
            let cur_file = &self.file_sym_map[&self.cur_file.unwrap()];
            *match ns {
                Namespace::Value => cur_file.value.get(sym),
                Namespace::Ty => cur_file.ty.get(sym),
                Namespace::Mod => cur_file.mods.get(sym),
            }
            .unwrap()
        }
    }

    pub fn resolve_files(mut self, files: &[Arc<ir::File>]) -> ResolveResult {
        files.iter().for_each(|f| {
            let mut collect = CollectDef::new(&mut self);
            collect.visit_file(f.clone());
            self.ir_files.insert(f.id, f.clone());
        });

        self.errors.abort_if_errors();

        let files = files
            .iter()
            .map(|f| (f.id, Arc::from(self.lower_file(f))))
            .collect::<FxHashMap<_, _>>();

        self.errors.abort_if_errors();

        ResolveResult {
            tags: self.tags,
            files,
            nodes: self.nodes,
            args: self.args,
        }
    }

    fn modify_ty_by_tags(&mut self, mut ty: Ty, tags: &Tags) -> Ty {
        match ty.kind {
            ty::FastStr
                if tags
                    .get::<RustType>()
                    .map(|repr| repr == "string")
                    .unwrap_or(false) =>
            {
                ty.kind = ty::String;
            }
            ty::Bytes
                if tags
                    .get::<RustType>()
                    .map(|repr| repr == "vec")
                    .unwrap_or(false) =>
            {
                ty.kind = ty::BytesVec;
            }
            _ => {}
        }

        if let Some(repr) = tags.get::<RustType>() {
            if repr == "btree" {
                struct BTreeFolder;
                impl Folder for BTreeFolder {
                    fn fold_ty(&mut self, ty: &Ty) -> Ty {
                        let kind = match &ty.kind {
                            TyKind::Vec(inner) => {
                                TyKind::Vec(Arc::new(self.fold_ty(inner.as_ref())))
                            }
                            TyKind::Set(inner) => {
                                TyKind::BTreeSet(Arc::new(self.fold_ty(inner.as_ref())))
                            }
                            TyKind::Map(k, v) => TyKind::BTreeMap(
                                Arc::new(self.fold_ty(k.as_ref())),
                                Arc::new(self.fold_ty(v.as_ref())),
                            ),
                            kind => kind.clone(),
                        };
                        Ty {
                            kind,
                            tags_id: ty.tags_id,
                        }
                    }
                }
                ty = BTreeFolder.fold_ty(&ty);
            } else if repr == "ordered_f64" {
                ty.kind = ty::OrderedF64;
            }
        };

        if let Some(RustWrapperArc(true)) = tags.get::<RustWrapperArc>() {
            struct ArcFolder;
            impl Folder for ArcFolder {
                fn fold_ty(&mut self, ty: &Ty) -> Ty {
                    let kind = match &ty.kind {
                        TyKind::Vec(inner) => TyKind::Vec(Arc::new(self.fold_ty(inner.as_ref()))),
                        TyKind::Set(inner) => TyKind::Set(Arc::new(self.fold_ty(inner.as_ref()))),
                        TyKind::BTreeSet(inner) => {
                            TyKind::BTreeSet(Arc::new(self.fold_ty(inner.as_ref())))
                        }
                        TyKind::Map(k, v) => {
                            TyKind::Map(k.clone(), Arc::new(self.fold_ty(v.as_ref())))
                        }
                        TyKind::BTreeMap(k, v) => {
                            TyKind::BTreeMap(k.clone(), Arc::new(self.fold_ty(v.as_ref())))
                        }
                        TyKind::Path(_) | TyKind::String | TyKind::BytesVec => {
                            TyKind::Arc(Arc::new(ty.clone()))
                        }
                        _ => panic!("ty: {ty:?} is unnecessary to be wrapped by Arc"),
                    };
                    Ty {
                        kind,
                        tags_id: ty.tags_id,
                    }
                }
            }
            ArcFolder.fold_ty(&ty)
        } else {
            ty
        }
    }

    #[tracing::instrument(level = "debug", skip_all, fields(name = &**f.name))]
    fn lower_field(&mut self, f: &ir::Field) -> Arc<Field> {
        tracing::info!("lower filed {}, ty: {:?}", f.name, f.ty.kind);
        let did = self.did_counter.inc_one();
        let tags_id = self.tags_id_counter.inc_one();
        self.tags.insert(tags_id, f.tags.clone());
        let ty = self.lower_type(&f.ty, false);
        let ty = self.modify_ty_by_tags(ty, &f.tags);

        let mut kind = match f.kind {
            ir::FieldKind::Required => FieldKind::Required,
            ir::FieldKind::Optional => FieldKind::Optional,
        };
        if let Some(OptionalRepeated(true)) = f.tags.get::<OptionalRepeated>() {
            kind = FieldKind::Optional;
        }

        let f = Arc::from(Field {
            did,
            id: f.id,
            kind,
            name: f.name.clone(),
            ty,
            tags_id,
            default: f.default.as_ref().map(|d| self.lower_lit(d)),
        });

        self.nodes
            .insert(did, self.mk_node(NodeKind::Field(f.clone()), tags_id));

        f
    }

    fn mk_node(&self, kind: NodeKind, tags: TagId) -> Node {
        Node {
            related_nodes: Default::default(),
            tags,
            parent: self.parent_node,
            file_id: self.cur_file.unwrap(),
            kind,
        }
    }

    fn lower_type(&mut self, ty: &ir::Ty, is_args: bool) -> Ty {
        let kind = match &ty.kind {
            ir::TyKind::String => ty::FastStr,
            ir::TyKind::Void => ty::Void,
            ir::TyKind::U8 => ty::U8,
            ir::TyKind::Bool => ty::Bool,
            ir::TyKind::Bytes => ty::Bytes,
            ir::TyKind::I8 => ty::I8,
            ir::TyKind::I16 => ty::I16,
            ir::TyKind::I32 => ty::I32,
            ir::TyKind::I64 => ty::I64,
            ir::TyKind::F64 => ty::F64,
            ir::TyKind::Uuid => ty::Uuid,
            ir::TyKind::Vec(ty) => ty::Vec(Arc::from(self.lower_type(ty, false))),
            ir::TyKind::Set(ty) => ty::Set(Arc::from(self.lower_type_for_hash_key(ty, false))),
            ir::TyKind::Map(k, v) => ty::Map(
                Arc::from(self.lower_type_for_hash_key(k, false)),
                Arc::from(self.lower_type(v, false)),
            ),
            ir::TyKind::Path(p) => ty::Path(self.lower_path(p, Namespace::Ty, is_args)),
            ir::TyKind::UInt64 => ty::UInt64,
            ir::TyKind::UInt32 => ty::UInt32,
            ir::TyKind::F32 => ty::F32,
        };
        let tags_id = self.tags_id_counter.inc_one();

        self.tags.insert(tags_id, ty.tags.clone());

        Ty { kind, tags_id }
    }

    fn lower_type_for_hash_key(&mut self, ty: &ir::Ty, is_args: bool) -> Ty {
        let kind = match &ty.kind {
            ir::TyKind::String => ty::FastStr,
            ir::TyKind::Void => ty::Void,
            ir::TyKind::U8 => ty::U8,
            ir::TyKind::Bool => ty::Bool,
            ir::TyKind::Bytes => ty::Bytes,
            ir::TyKind::I8 => ty::I8,
            ir::TyKind::I16 => ty::I16,
            ir::TyKind::I32 => ty::I32,
            ir::TyKind::I64 => ty::I64,
            ir::TyKind::F64 => ty::OrderedF64,
            ir::TyKind::Uuid => ty::Uuid,
            ir::TyKind::Vec(ty) => ty::Vec(Arc::from(self.lower_type_for_hash_key(ty, false))),
            ir::TyKind::Set(ty) => ty::Set(Arc::from(self.lower_type_for_hash_key(ty, false))),
            ir::TyKind::Map(k, v) => ty::Map(
                Arc::from(self.lower_type_for_hash_key(k, false)),
                Arc::from(self.lower_type(v, false)),
            ),
            ir::TyKind::Path(p) => ty::Path(self.lower_path(p, Namespace::Ty, is_args)),
            ir::TyKind::UInt64 => ty::UInt64,
            ir::TyKind::UInt32 => ty::UInt32,
            ir::TyKind::F32 => ty::F32,
        };
        let tags_id = self.tags_id_counter.inc_one();

        self.tags.insert(tags_id, ty.tags.clone());

        Ty { kind, tags_id }
    }

    fn find_path_in_table(
        &self,
        path: &[Ident],
        ns: Namespace,
        table: &SymbolTable,
    ) -> Option<DefId> {
        assert!(!path.is_empty());
        let mut status: ResolvedSymbols = ResolvedSymbols {
            ty: table
                .ty
                .get(&path[0].sym)
                .map_or_else(Default::default, |s| vec![*s]),
            value: table
                .value
                .get(&path[0].sym)
                .map_or_else(Default::default, |s| vec![*s]),
            r#mod: table
                .mods
                .get(&path[0].sym)
                .map_or_else(Default::default, |s| vec![*s]),
        };

        path[1..].iter().for_each(|i| {
            status = ResolvedSymbols {
                ty: [&status.ty, &status.value, &status.r#mod]
                    .into_iter()
                    .flatten()
                    .flat_map(|def_id| {
                        self.def_modules
                            .get(def_id)
                            .and_then(|module| module.resolutions.ty.get(&i.sym))
                    })
                    .copied()
                    .collect(),
                value: [&status.ty, &status.value, &status.r#mod]
                    .into_iter()
                    .flatten()
                    .flat_map(|def_id| {
                        self.def_modules
                            .get(def_id)
                            .and_then(|module| module.resolutions.value.get(&i.sym))
                    })
                    .copied()
                    .collect(),
                r#mod: [&status.ty, &status.value, &status.r#mod]
                    .into_iter()
                    .flatten()
                    .flat_map(|def_id| {
                        self.def_modules
                            .get(def_id)
                            .and_then(|module| module.resolutions.mods.get(&i.sym))
                    })
                    .copied()
                    .collect_vec(),
            };
        });

        assert!(status.value.len() <= 1);
        assert!(status.ty.len() <= 1);
        assert!(status.r#mod.len() <= 1);

        match ns {
            Namespace::Value => status.value.first(),
            Namespace::Ty => status.ty.first(),
            Namespace::Mod => status.r#mod.first(),
        }
        .copied()
    }

    fn lower_path(&mut self, path: &ir::Path, ns: Namespace, is_args: bool) -> Path {
        let segs = &path.segments;
        let cur_file = self.ir_files.get(self.cur_file.as_ref().unwrap()).unwrap();
        let path_kind = match ns {
            Namespace::Value => DefKind::Value,
            Namespace::Ty => DefKind::Type,
            Namespace::Mod => unreachable!(),
        };
        {
            let segs = match segs.strip_prefix(&*cur_file.package.segments) {
                Some(segs) => segs,
                _ => segs,
            };

            let def_id = self.blocks.iter().rev().find_map(|b| {
                let b = unsafe { b.as_ref() };
                self.find_path_in_table(segs, ns, b)
            });

            if let Some(def_id) = def_id {
                if is_args {
                    self.args.insert(def_id);
                }
                return Path {
                    kind: path_kind,
                    did: def_id,
                };
            }
        }
        let def_id = cur_file
            .uses
            .iter()
            .find_map(|f| match path.segments.strip_prefix(&*f.0.segments) {
                Some(rest) => {
                    let file = &self.file_sym_map[&f.1];
                    self.find_path_in_table(rest, ns, file)
                }
                _ => None,
            })
            .unwrap_or_else(|| {
                panic!(
                    "can not find path {} in file symbols {:?}, {:?}",
                    path,
                    self.file_sym_map.get(&self.cur_file.unwrap()),
                    cur_file.uses,
                )
            });

        if is_args {
            self.args.insert(def_id);
        }
        Path {
            kind: path_kind,
            did: def_id,
        }
    }

    #[tracing::instrument(level = "debug", skip(self, s), fields(name = &**s.name))]
    fn lower_message(&mut self, s: &ir::Message) -> Message {
        Message {
            name: s.name.clone(),
            fields: s.fields.iter().map(|f| self.lower_field(f)).collect(),
            is_wrapper: s.is_wrapper,
        }
    }

    fn lower_enum(&mut self, e: &ir::Enum) -> Enum {
        let mut next_discr = 0;
        Enum {
            name: e.name.clone(),
            variants: e
                .variants
                .iter()
                .map(|v| {
                    let tags_id = self.tags_id_counter.inc_one();
                    let did = self.get_def_id(Namespace::Value, &v.name);
                    if !v.tags.is_empty() {
                        self.tags.insert(tags_id, v.tags.clone());
                    }
                    let discr = v.discr.unwrap_or(next_discr);
                    let e = Arc::from(EnumVariant {
                        id: v.id,
                        did,
                        name: v.name.clone(),
                        discr: if e.repr == Some(EnumRepr::I32) {
                            Some(discr)
                        } else {
                            None
                        },
                        fields: v
                            .fields
                            .iter()
                            .map(|p| {
                                let ty = self.lower_type(p, false);
                                self.modify_ty_by_tags(ty, &p.tags)
                            })
                            .collect(),
                    });
                    next_discr = discr + 1;
                    self.nodes
                        .insert(did, self.mk_node(NodeKind::Variant(e.clone()), tags_id));
                    e
                })
                .collect(),
            repr: e.repr,
        }
    }

    fn lower_service(&mut self, s: &ir::Service) -> Service {
        Service {
            name: s.name.clone(),
            methods: s
                .methods
                .iter()
                .map(|m| {
                    let def_id = self.did_counter.inc_one();
                    let tags_id = self.tags_id_counter.inc_one();
                    self.tags.insert(tags_id, m.tags.clone());
                    let old_parent = self.parent_node.replace(def_id);
                    let method = Arc::from(Method {
                        def_id,
                        source: MethodSource::Own,
                        name: m.name.clone(),
                        args: m
                            .args
                            .iter()
                            .map(|a| {
                                let tags_id = self.tags_id_counter.inc_one();
                                self.tags.insert(tags_id, a.tags.clone());
                                let def_id = self.did_counter.inc_one();
                                let arg = Arc::new(Arg {
                                    def_id,
                                    ty: self.lower_type(&a.ty, true),
                                    name: a.name.clone(),
                                    id: a.id,
                                    tags_id,
                                    kind: match a.attribute {
                                        ir::FieldKind::Required => FieldKind::Required,
                                        ir::FieldKind::Optional => FieldKind::Optional,
                                    },
                                });
                                self.nodes.insert(
                                    def_id,
                                    self.mk_node(NodeKind::Arg(arg.clone()), tags_id),
                                );
                                arg
                            })
                            .collect(),
                        ret: self.lower_type(&m.ret, true),
                        oneway: m.oneway,
                        exceptions: m
                            .exceptions
                            .as_ref()
                            .map(|p| self.lower_path(p, Namespace::Ty, true)),
                    });
                    self.parent_node = old_parent;
                    self.nodes.insert(
                        def_id,
                        self.mk_node(NodeKind::Method(method.clone()), tags_id),
                    );

                    method
                })
                .collect(),
            extend: s
                .extend
                .iter()
                .map(|p| self.lower_path(p, Namespace::Ty, false))
                .collect(),
        }
    }

    fn lower_type_alias(&mut self, t: &ir::NewType, tags: &Tags) -> NewType {
        NewType {
            name: t.name.clone(),
            ty: {
                let ty = self.lower_type(&t.ty, false);
                self.modify_ty_by_tags(ty, tags)
            },
        }
    }

    fn lower_lit(&mut self, l: &ir::Literal) -> Literal {
        match l {
            ir::Literal::Bool(b) => Literal::Bool(*b),
            ir::Literal::Path(p) => Literal::Path(self.lower_path(p, Namespace::Value, false)),
            ir::Literal::String(s) => Literal::String(s.clone()),
            ir::Literal::Int(i) => Literal::Int(*i),
            ir::Literal::Float(f) => Literal::Float(f.clone()),
            ir::Literal::List(l) => Literal::List(l.iter().map(|l| self.lower_lit(l)).collect()),
            ir::Literal::Map(l) => Literal::Map(
                l.iter()
                    .map(|(k, v)| (self.lower_lit(k), self.lower_lit(v)))
                    .collect(),
            ),
        }
    }

    fn lower_const(&mut self, c: &ir::Const, tags: &Tags) -> Const {
        Const {
            name: c.name.clone(),
            ty: {
                let ty = self.lower_type(&c.ty, false);
                self.modify_ty_by_tags(ty, tags)
            },
            lit: self.lower_lit(&c.lit),
        }
    }

    fn lower_mod(&mut self, m: &ir::Mod, def_id: DefId) -> Mod {
        self.blocks.push(NonNull::from(
            &self.def_modules.get(&def_id).unwrap().resolutions,
        ));

        let items = m
            .items
            .iter()
            .filter_map(|i| self.lower_item(i))
            .collect::<Vec<_>>();

        self.blocks.pop();

        Mod {
            name: m.name.clone(),
            items,
            extensions: match &m.extensions {
                ir::ext::ModExts::Pb(exts) => middle::ext::ModExts::Pb(middle::ext::pb::ModExts {
                    extendees: exts
                        .extendees
                        .iter()
                        .map(|e| self.lower_pb_extendee(e))
                        .collect::<Vec<_>>(),
                }),
                ir::ext::ModExts::Thrift => middle::ext::ModExts::Thrift,
            },
        }
    }

    fn lower_item(&mut self, item: &ir::Item) -> Option<DefId> {
        if let ir::ItemKind::Use(_) = &item.kind {
            return None;
        }

        let name = item.name();
        let tags = &item.tags;

        let def_id = self.get_def_id(
            match &item.kind {
                ir::ItemKind::Const(_) => Namespace::Value,
                ir::ItemKind::Mod(_) => Namespace::Mod,
                _ => Namespace::Ty,
            },
            &name,
        );

        let old_parent = self.parent_node.replace(def_id);
        let related_items = &item.related_items;

        let item = Arc::new(match &item.kind {
            ir::ItemKind::Message(s) => Item::Message(self.lower_message(s)),
            ir::ItemKind::Enum(e) => Item::Enum(self.lower_enum(e)),
            ir::ItemKind::Service(s) => Item::Service(self.lower_service(s)),
            ir::ItemKind::NewType(t) => Item::NewType(self.lower_type_alias(t, tags)),
            ir::ItemKind::Const(c) => Item::Const(self.lower_const(c, tags)),
            ir::ItemKind::Mod(m) => Item::Mod(self.lower_mod(m, def_id)),
            ir::ItemKind::Use(_) => unreachable!(),
        });

        self.parent_node = old_parent;

        let tags_id = self.tags_id_counter.inc_one();
        self.tags.insert(tags_id, tags.clone());

        let mut node = self.mk_node(NodeKind::Item(item), tags_id);
        node.related_nodes = related_items
            .iter()
            .map(|i| {
                self.lower_path(
                    &ir::Path {
                        segments: Arc::from([i.clone()]),
                    },
                    Namespace::Ty,
                    false,
                )
                .did
            })
            .collect();

        self.nodes.insert(def_id, node);

        Some(def_id)
    }

    fn lower_file(&mut self, file: &ir::File) -> File {
        let old_file = self.cur_file.replace(file.id);
        let should_pop = self
            .file_sym_map
            .get(&file.id)
            .map(|block| self.blocks.push(NonNull::from(block)))
            .is_some();

        let f = File {
            items: file
                .items
                .iter()
                .filter_map(|item| self.lower_item(item))
                .collect(),

            file_id: file.id,
            package: ItemPath::from(
                file.package
                    .segments
                    .iter()
                    .map(|i| i.sym.clone())
                    .collect::<Vec<_>>(),
            ),
            uses: file.uses.iter().map(|(_, f)| *f).collect(),
            descriptor: file.descriptor.clone(),
            extensions: match &file.extensions {
                ir::ext::FileExts::Pb(exts) => {
                    middle::ext::FileExts::Pb(middle::ext::pb::FileExts {
                        extendees: exts
                            .extendees
                            .iter()
                            .map(|e| self.lower_pb_extendee(e))
                            .collect::<Vec<_>>(),
                    })
                }
                ir::ext::FileExts::Thrift => middle::ext::FileExts::Thrift,
            },
        };

        if should_pop {
            self.blocks.pop();
        }

        self.cur_file = old_file;
        f
    }

    fn lower_pb_extendee(&mut self, e: &ir::ext::pb::Extendee) -> Arc<middle::ext::pb::Extendee> {
        let extendee_index = e.index.into();
        Arc::new(middle::ext::pb::Extendee {
            name: e.name.clone(),
            index: extendee_index,
            extendee_ty: ExtendeeType {
                field_ty: e.extendee_ty.field_ty.into(),
                item_ty: self.lower_type(&e.extendee_ty.item_ty, false),
            },
        })
    }
}
