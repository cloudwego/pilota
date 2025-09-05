use std::{path::PathBuf, str::FromStr, sync::Arc};

use faststr::FastStr;
use heck::ToUpperCamelCase;
use itertools::Itertools;
use normpath::PathExt;
use pilota_thrift_parser::{
    parser::Parser as _,
    {self as thrift_parser},
};
use pilota_thrift_reflect::thrift_reflection;
use rustc_hash::{FxHashMap, FxHashSet};
use thrift_parser::Annotations;

use crate::{
    IdentName,
    index::Idx,
    ir::{self, Arg, Enum, EnumVariant, FieldKind, File, Item, ItemKind, Path},
    symbol::{EnumRepr, FileId, Ident},
    tags::{Annotation, PilotaName, RustWrapperArc, Tags},
    util::error_abort,
};

fn generate_short_uuid() -> FastStr {
    let uuid: [u8; 4] = rand::random();
    FastStr::new(hex::encode(uuid))
}

#[salsa::db]
#[derive(Default, Clone)]
struct ThriftSourceDatabase {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for ThriftSourceDatabase {}

impl ThriftSourceDatabase {
    fn file_text(&self, path: PathBuf) -> Arc<str> {
        Arc::from(unsafe { String::from_utf8_unchecked(std::fs::read(path).unwrap()) })
    }

    fn parse(&self, path: PathBuf) -> Arc<thrift_parser::File> {
        let text = self.file_text(path.clone());
        let res = thrift_parser::File::parse(&text);
        if res.is_err() {
            println!("cargo:warning={}", path.display());
        }
        let mut ast = res.unwrap().1;
        ast.path = Arc::from(path);
        ast.uuid = generate_short_uuid();
        let descriptor = thrift_reflection::FileDescriptor::from(&ast);
        ast.descriptor = descriptor.serialize();
        Arc::from(ast)
    }
}

#[derive(Debug)]
pub struct LowerResult {
    pub files: Vec<Arc<File>>,
    pub file_ids_map: FxHashMap<Arc<PathBuf>, FileId>,
    pub file_paths: FxHashMap<FileId, Arc<PathBuf>>,
}

pub trait Lower<Ast> {
    fn lower(&mut self, file: Ast) -> FileId;

    fn finish(self) -> LowerResult;
}

pub struct ThriftLower {
    cur_file: Option<Arc<thrift_parser::File>>,
    next_file_id: FileId,
    db: ThriftSourceDatabase,
    files: FxHashMap<FileId, Arc<File>>,
    file_ids_map: FxHashMap<Arc<PathBuf>, FileId>,
    file_paths: FxHashMap<FileId, Arc<PathBuf>>,
    include_dirs: Vec<PathBuf>,
    packages: FxHashMap<Path, Vec<Arc<PathBuf>>>,
    service_name_duplicates: FxHashSet<String>,
}

impl ThriftLower {
    fn new(db: ThriftSourceDatabase, include_dirs: Vec<PathBuf>) -> Self {
        ThriftLower {
            cur_file: None,
            next_file_id: FileId::from_u32(0),
            db,
            files: FxHashMap::default(),
            file_ids_map: FxHashMap::default(),
            file_paths: FxHashMap::default(),
            include_dirs,
            packages: Default::default(),
            service_name_duplicates: Default::default(),
        }
    }

    pub fn with_cur_file<F>(&mut self, file: Arc<thrift_parser::File>, f: F) -> Arc<File>
    where
        F: FnOnce(&mut Self) -> ir::File,
    {
        let old_file = self.cur_file.clone();
        self.cur_file = Some(file);

        let f = Arc::from(f(self));
        self.cur_file = old_file;
        self.files.insert(f.id, f.clone());
        f
    }

    fn lower_path(&self, path: &thrift_parser::Path) -> ir::Path {
        Path {
            segments: Arc::from_iter(path.segments.iter().map(|i| self.lower_ident(i))),
        }
    }

    fn mk_item(&self, kind: ItemKind, tags: Arc<Tags>) -> ir::Item {
        ir::Item {
            kind,
            tags,
            related_items: Default::default(),
        }
    }

    fn lower_service(&self, service: &thrift_parser::Service) -> Vec<ir::Item> {
        let service_name = if self
            .service_name_duplicates
            .contains(&service.name.to_upper_camel_case())
        {
            service.name.to_string()
        } else {
            service.name.to_upper_camel_case()
        };

        let service_tags = self.extract_tags(&service.annotations);
        let arc_wrapper = service_tags.get::<RustWrapperArc>().is_some_and(|v| v.0);

        let mut function_names: FxHashMap<FastStr, Vec<String>> = FxHashMap::default();
        service.functions.iter().for_each(|func| {
            let name = self
                .extract_tags(&func.annotations)
                .get::<PilotaName>()
                .map(|name| name.0.to_string())
                .unwrap_or_else(|| func.name.to_string());
            function_names
                .entry(name.as_str().upper_camel_ident())
                .or_default()
                .push(name);
        });
        let function_name_duplicates = function_names
            .iter()
            .filter(|(_, v)| v.len() > 1)
            .map(|(k, _)| k.as_str())
            .collect::<FxHashSet<_>>();

        let kind = ir::ItemKind::Service(ir::Service {
            name: self.lower_ident(&service.name),
            extend: service
                .extends
                .as_ref()
                .into_iter()
                .map(|e| self.lower_path(e))
                .collect(),
            methods: service
                .functions
                .iter()
                .map(|f| {
                    self.lower_method(&service_name, f, &function_name_duplicates, arc_wrapper)
                })
                .collect(),
        });
        let mut service_item = self.mk_item(kind, Default::default());
        let mut result = vec![];

        let mut related_items = Vec::default();

        service.functions.iter().for_each(|f| {
            let exception = f
                .throws
                .iter()
                .map(|f| ir::EnumVariant {
                    id: Some(f.id),
                    name: if f.name.is_empty() {
                        match &f.ty.0 {
                            thrift_parser::Ty::Path(p) => {
                                self.lower_ident(p.segments.last().unwrap())
                            }
                            _ => panic!(""),
                        }
                    } else {
                        self.lower_ident(&f.name)
                    },
                    tags: Default::default(),
                    discr: None,
                    fields: vec![self.lower_ty(&f.ty)],
                })
                .collect::<Vec<_>>();

            let tags = self.extract_tags(&f.annotations);
            let name = tags
                .get::<PilotaName>()
                .map(|name| name.0.clone())
                .unwrap_or_else(|| FastStr::new(f.name.0.clone()));

            let upper_camel_ident = name.as_str().upper_camel_ident();
            let method_name = if function_name_duplicates.contains(upper_camel_ident.as_str()) {
                name
            } else {
                upper_camel_ident
            };

            let name: Ident = format!("{}{}ResultRecv", service_name, method_name).into();
            let kind = ir::ItemKind::Enum(ir::Enum {
                name: name.clone(),
                variants: std::iter::once(ir::EnumVariant {
                    id: Some(0),
                    name: "Ok".into(),
                    tags: Default::default(),
                    discr: None,
                    fields: vec![self.lower_method_relative_ty(&f.result_type, arc_wrapper)],
                })
                .chain(exception.clone())
                .collect(),
                repr: None,
            });
            related_items.push(name.clone());
            let mut tags = Tags::default();
            tags.insert(crate::tags::KeepUnknownFields(false));
            tags.insert(crate::tags::PilotaName(name.sym.0));
            result.push(self.mk_item(kind, tags.into()));

            let name: Ident = format!("{service_name}{method_name}ResultSend").into();
            let kind = ir::ItemKind::Enum(ir::Enum {
                name: name.clone(),
                variants: std::iter::once(ir::EnumVariant {
                    id: Some(0),
                    name: "Ok".into(),
                    tags: Default::default(),
                    discr: None,
                    fields: vec![self.lower_method_relative_ty(&f.result_type, arc_wrapper)],
                })
                .chain(exception.clone())
                .collect(),
                repr: None,
            });
            related_items.push(name.clone());
            let mut tags = Tags::default();
            tags.insert(crate::tags::KeepUnknownFields(false));
            tags.insert(crate::tags::PilotaName(name.sym.0));
            result.push(self.mk_item(kind, tags.into()));

            if !exception.is_empty() {
                let name: Ident = format!("{service_name}{method_name}Exception").into();
                let kind = ir::ItemKind::Enum(ir::Enum {
                    name: name.clone(),
                    variants: exception,
                    repr: None,
                });
                related_items.push(name.clone());
                let mut tags = Tags::default();
                tags.insert(crate::tags::KeepUnknownFields(false));
                tags.insert(crate::tags::PilotaName(name.sym.0));
                result.push(self.mk_item(kind, tags.into()));
            }

            let name: Ident = format!("{service_name}{method_name}ArgsSend").into();
            let kind = ir::ItemKind::Message(ir::Message {
                name: name.clone(),
                fields: f
                    .arguments
                    .iter()
                    .map(|a| self.lower_method_arg_field(a, arc_wrapper))
                    .collect(),
                is_wrapper: true,
                extensions: Default::default(),
            });
            related_items.push(name.clone());
            let mut tags = Tags::default();
            tags.insert(crate::tags::KeepUnknownFields(false));
            tags.insert(crate::tags::PilotaName(name.sym.0));
            result.push(self.mk_item(kind, tags.into()));

            let name: Ident = format!("{service_name}{method_name}ArgsRecv").into();
            let kind = ir::ItemKind::Message(ir::Message {
                name: name.clone(),
                fields: f
                    .arguments
                    .iter()
                    .map(|a| self.lower_method_arg_field(a, arc_wrapper))
                    .collect(),
                is_wrapper: true,
                extensions: Default::default(),
            });
            related_items.push(name.clone());
            let mut tags: Tags = Tags::default();
            tags.insert(crate::tags::KeepUnknownFields(false));
            tags.insert(crate::tags::PilotaName(name.sym.0));
            result.push(self.mk_item(kind, tags.into()));
        });

        service_item.related_items = related_items;
        result.push(service_item);
        result
    }

    fn lower_method(
        &self,
        service_name: &String,
        method: &thrift_parser::Function,
        function_name_duplicates: &FxHashSet<&str>,
        arc_wrapper: bool,
    ) -> ir::Method {
        let tags = self.extract_tags(&method.annotations);
        let name = tags
            .get::<PilotaName>()
            .map(|name| name.0.clone())
            .unwrap_or_else(|| FastStr::new(method.name.0.clone()));

        let upper_camel_ident = name.as_str().upper_camel_ident();
        let method_name = if function_name_duplicates.contains(upper_camel_ident.as_str()) {
            name
        } else {
            upper_camel_ident
        };

        ir::Method {
            name: self.lower_ident(&method.name),
            args: method
                .arguments
                .iter()
                .map(|a| {
                    let mut tags = self.extract_tags(&a.annotations);
                    if arc_wrapper && !tags.contains::<RustWrapperArc>() {
                        tags.insert(RustWrapperArc(true));
                    }
                    Arg {
                        ty: self.lower_method_relative_ty(&a.ty, arc_wrapper),
                        id: a.id,
                        name: self.lower_ident(&a.name),
                        tags: Arc::new(tags),
                        attribute: match a.attribute {
                            pilota_thrift_parser::Attribute::Required => FieldKind::Required,
                            pilota_thrift_parser::Attribute::Optional
                            | pilota_thrift_parser::Attribute::Default => FieldKind::Optional,
                        },
                    }
                })
                .collect(),
            ret: self.lower_method_relative_ty(&method.result_type, arc_wrapper),
            oneway: method.oneway,
            tags: tags.into(),
            exceptions: if method.throws.is_empty() {
                None
            } else {
                Some(Path {
                    segments: Arc::from([Ident::from(format!(
                        "{service_name}{method_name}Exception",
                    ))]),
                })
            },
        }
    }

    fn lower_enum(&self, e: &thrift_parser::Enum) -> ir::Enum {
        ir::Enum {
            name: self.lower_ident(&e.name),
            variants: e
                .values
                .iter()
                .map(|v| ir::EnumVariant {
                    id: None,
                    name: self.lower_ident(&v.name),
                    discr: v.value.map(|v| v.0),
                    fields: vec![],
                    tags: self.extract_tags(&v.annotations).into(),
                })
                .collect(),
            repr: Some(EnumRepr::I32),
        }
    }

    fn lower_lit(&self, l: &thrift_parser::ConstValue) -> ir::Literal {
        match &l {
            thrift_parser::ConstValue::Bool(b) => ir::Literal::Bool(*b),
            thrift_parser::ConstValue::Path(p) => ir::Literal::Path(self.lower_path(p)),
            thrift_parser::ConstValue::String(s) => ir::Literal::String(Arc::from(s.0.as_str())),
            thrift_parser::ConstValue::Int(i) => ir::Literal::Int(i.0),
            thrift_parser::ConstValue::Double(d) => ir::Literal::Float(d.0.clone()),
            thrift_parser::ConstValue::List(inner) => {
                ir::Literal::List(inner.iter().map(|i| self.lower_lit(i)).collect())
            }
            thrift_parser::ConstValue::Map(kvs) => ir::Literal::Map(
                kvs.iter()
                    .map(|(k, v)| (self.lower_lit(k), self.lower_lit(v)))
                    .collect(),
            ),
        }
    }

    fn lower_const(&self, c: &thrift_parser::Constant) -> ir::Const {
        ir::Const {
            name: self.lower_ident(&c.name),
            ty: self.lower_ty(&c.r#type),
            lit: self.lower_lit(&c.value),
        }
    }

    fn lower_typedef(&self, t: &thrift_parser::Typedef) -> ir::NewType {
        ir::NewType {
            name: self.lower_ident(&t.alias),
            ty: self.lower_ty(&t.r#type),
        }
    }

    fn lower_item(&self, item: &thrift_parser::Item) -> Vec<ir::Item> {
        let single = match item {
            thrift_parser::Item::Typedef(t) => ir::ItemKind::NewType(self.lower_typedef(t)),
            thrift_parser::Item::Constant(c) => ir::ItemKind::Const(self.lower_const(c)),
            thrift_parser::Item::Enum(e) => ir::ItemKind::Enum(self.lower_enum(e)),
            thrift_parser::Item::Struct(s) => ir::ItemKind::Message(self.lower_struct(s)),
            thrift_parser::Item::Union(u) => ir::ItemKind::Enum(self.lower_union(u)),
            thrift_parser::Item::Exception(s) => ir::ItemKind::Message(self.lower_struct(s)),
            thrift_parser::Item::Service(s) => return self.lower_service(s),
            _ => return vec![],
        };

        let empty_annotations = Annotations::default();

        let annotations = match item {
            thrift_parser::Item::Typedef(t) => &t.annotations,
            thrift_parser::Item::Constant(c) => &c.annotations,
            thrift_parser::Item::Enum(e) => &e.annotations,
            thrift_parser::Item::Struct(s) => &s.annotations,
            thrift_parser::Item::Union(u) => &u.annotations,
            thrift_parser::Item::Exception(e) => &e.annotations,
            thrift_parser::Item::Service(s) => &s.annotations,
            _ => &empty_annotations,
        };

        let tags = self.extract_tags(annotations);

        vec![self.mk_item(single, tags.into())]
    }

    fn lower_union(&self, union: &thrift_parser::Union) -> Enum {
        Enum {
            name: self.lower_ident(&union.name),
            variants: union
                .fields
                .iter()
                .map(|f| EnumVariant {
                    id: Some(f.id),
                    name: self.lower_ident(&f.name),
                    discr: None,
                    fields: vec![self.lower_ty(&f.ty)],
                    tags: Default::default(),
                })
                .collect(),
            repr: None,
        }
    }

    fn lower_ident(&self, s: &thrift_parser::Ident) -> Ident {
        Ident::from(s.0.clone())
    }

    fn extract_tags_with_arc_wrapper(&self, annotations: &Annotations, arc_wrapper: bool) -> Tags {
        let mut tags = self.extract_tags(annotations);
        if arc_wrapper && !tags.contains::<RustWrapperArc>() {
            tags.insert(RustWrapperArc(true));
        }
        tags
    }

    fn lower_method_relative_ty(&self, ty: &thrift_parser::Type, arc_wrapper: bool) -> ir::Ty {
        self.lower_ty_with_tags(ty, self.extract_tags_with_arc_wrapper(&ty.1, arc_wrapper))
    }

    fn lower_ty(&self, ty: &thrift_parser::Type) -> ir::Ty {
        let tags = self.extract_tags(&ty.1);
        self.lower_ty_with_tags(ty, tags)
    }

    fn lower_ty_with_tags(&self, ty: &thrift_parser::Type, tags: Tags) -> ir::Ty {
        let kind = match &ty.0 {
            thrift_parser::Ty::String => ir::TyKind::String,
            thrift_parser::Ty::Void => ir::TyKind::Void,
            thrift_parser::Ty::Byte => ir::TyKind::I8,
            thrift_parser::Ty::Bool => ir::TyKind::Bool,
            thrift_parser::Ty::Binary => ir::TyKind::Bytes,
            thrift_parser::Ty::I8 => ir::TyKind::I8,
            thrift_parser::Ty::I16 => ir::TyKind::I16,
            thrift_parser::Ty::I32 => ir::TyKind::I32,
            thrift_parser::Ty::I64 => ir::TyKind::I64,
            thrift_parser::Ty::Double => ir::TyKind::F64,
            thrift_parser::Ty::Uuid => ir::TyKind::Uuid,
            thrift_parser::Ty::List { value, .. } => ir::TyKind::Vec(self.lower_ty(value).into()),
            thrift_parser::Ty::Set { value, .. } => ir::TyKind::Set(self.lower_ty(value).into()),
            thrift_parser::Ty::Map { key, value, .. } => {
                ir::TyKind::Map(self.lower_ty(key).into(), self.lower_ty(value).into())
            }
            thrift_parser::Ty::Path(path) => ir::TyKind::Path(self.lower_path(path)),
        };

        ir::Ty {
            kind,
            tags: tags.into(),
        }
    }

    fn lower_method_arg_field(&self, f: &thrift_parser::Field, arc_wrapper: bool) -> ir::Field {
        let mut tags = self.extract_tags(&f.annotations);
        if arc_wrapper && !tags.contains::<RustWrapperArc>() {
            tags.insert(RustWrapperArc(true));
        }

        ir::Field {
            name: self.lower_ident(&f.name),
            id: f.id,
            ty: self.lower_method_relative_ty(&f.ty, arc_wrapper),
            kind: match f.attribute {
                pilota_thrift_parser::Attribute::Required => FieldKind::Required,
                _ => FieldKind::Optional,
            },
            tags: tags.into(),
            default: f.default.as_ref().map(|c| self.lower_lit(c)),
        }
    }

    fn lower_field(&self, f: &thrift_parser::Field) -> ir::Field {
        let tags = self.extract_tags(&f.annotations);
        self.lower_field_with_tags(f, tags)
    }

    fn lower_field_with_tags(&self, f: &thrift_parser::Field, tags: Tags) -> ir::Field {
        ir::Field {
            name: self.lower_ident(&f.name),
            id: f.id,
            ty: self.lower_ty(&f.ty),
            kind: match f.attribute {
                pilota_thrift_parser::Attribute::Required => FieldKind::Required,
                _ => FieldKind::Optional,
            },
            default: f.default.as_ref().map(|c| self.lower_lit(c)),
            tags: tags.into(),
        }
    }

    fn extract_tags(&self, annotations: &Annotations) -> Tags {
        let mut tags = Tags::default();
        macro_rules! with_tags {
            ($annotation: tt -> $($key: ty)|+) => {
                match $annotation.key.as_str()  {
                    $(<$key>::KEY => {
                        tags.insert(<$key>::from_str(&$annotation.value).unwrap());
                    }),+
                    _ => {},
                }
            };
        }

        annotations.iter().for_each(
            |annotation| with_tags!(annotation -> crate::tags::PilotaName | crate::tags::RustType | crate::tags::RustWrapperArc | crate::tags::SerdeAttribute),
        );

        tags
    }

    fn lower_struct(&self, s: &thrift_parser::StructLike) -> ir::Message {
        let mut seen_ids = FxHashSet::default();
        for field in &s.fields {
            if !seen_ids.insert(field.id) {
                panic!(
                    "duplicate ID `{}` in struct `{}`",
                    field.id,
                    self.lower_ident(&s.name),
                );
            }
        }
        ir::Message {
            name: self.lower_ident(&s.name),
            fields: s.fields.iter().map(|f| self.lower_field(f)).collect(),
            is_wrapper: false,
            extensions: Default::default(),
        }
    }

    fn lower_include(&mut self, s: &thrift_parser::Include) -> ir::Use {
        // add current file's dir to include dirs
        let current_dir = self.cur_file.as_ref().unwrap().path.parent().unwrap();
        let mut include_dirs = vec![current_dir.to_path_buf()];
        include_dirs.extend_from_slice(&self.include_dirs);

        // search for the first existing include path
        let target_dir = include_dirs.into_iter().find(|p| {
            let path = p.join(&s.path.0);
            path.exists()
        });
        let target_path = match target_dir {
            Some(dir) => dir.join(&s.path.0),
            None => {
                error_abort(format!("{}: include file not found", s.path.0));
            }
        };

        let ast = self
            .db
            .parse(target_path.normalize().unwrap().into_path_buf());

        let file_id = self.lower(ast);

        ir::Use { file: file_id }
    }
}

impl Lower<Arc<thrift_parser::File>> for ThriftLower {
    fn lower(&mut self, f: Arc<thrift_parser::File>) -> FileId {
        if let Some(file_id) = self.file_ids_map.get(&f.path) {
            return *file_id;
        }

        println!("cargo:rerun-if-changed={}", f.path.display());

        let file_id = self.next_file_id.inc_one();
        self.file_ids_map.insert(f.path.clone(), file_id);
        self.file_paths.insert(file_id, f.path.clone());

        let file = self.with_cur_file(f.clone(), |this| {
            let include_files = f
                .items
                .iter()
                .filter_map(|item| {
                    if let thrift_parser::Item::Include(i) = item {
                        Some(i)
                    } else {
                        None
                    }
                })
                .map(|i| {
                    (
                        i.path
                            .0
                            .split('/')
                            .next_back()
                            .unwrap()
                            .trim_end_matches(".thrift")
                            .split('.')
                            .map(FastStr::new)
                            .map(Ident::from)
                            .collect_vec(),
                        this.lower_include(i),
                    )
                })
                .collect::<Vec<_>>();

            let includes = include_files
                .iter()
                .map(|(_, file)| Item {
                    related_items: Default::default(),
                    kind: ir::ItemKind::Use(ir::Use { file: file.file }),
                    tags: Default::default(),
                })
                .collect::<Vec<_>>();

            let uses = include_files
                .into_iter()
                .map(|(name, u)| {
                    (
                        Path {
                            segments: name.into(),
                        },
                        u.file,
                    )
                })
                .collect::<Vec<(_, FileId)>>();

            let file_package = f
                .package
                .as_ref()
                .map(|p| this.lower_path(p))
                .unwrap_or_else(|| Path {
                    segments: Arc::from([f
                        .path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace('.', "_")
                        .into()]),
                });

            this.packages
                .entry(file_package.clone())
                .or_default()
                .push(f.path.clone());

            let mut service_names: FxHashMap<String, Vec<String>> = FxHashMap::default();
            f.items.iter().for_each(|item| {
                if let thrift_parser::Item::Service(service) = item {
                    service_names
                        .entry(service.name.to_upper_camel_case())
                        .or_default()
                        .push(service.name.to_string());
                }
            });
            this.service_name_duplicates.extend(
                service_names
                    .into_iter()
                    .filter(|(_, v)| v.len() > 1)
                    .map(|(k, _)| k),
            );

            let ret = ir::File {
                package: file_package,
                items: f
                    .items
                    .iter()
                    .flat_map(|i| this.lower_item(i))
                    .chain(includes)
                    .map(Arc::from)
                    .collect(),
                id: file_id,
                uses,
                descriptor: f.descriptor.clone(),
                extensions: Default::default(),
            };

            this.service_name_duplicates.clear();
            ret
        });

        file.id
    }

    fn finish(self) -> LowerResult {
        self.packages.iter().for_each(|(k, v)| {
            if v.len() > 1 {
                println!(
                    "cargo:warning={:?} has the same namespace `{}`, you may need to set namespace for these file \n",
                    v,
                    k.segments.iter().join(".")
                )
            }
        });
        LowerResult {
            files: self.files.into_values().collect::<Vec<_>>(),
            file_ids_map: self.file_ids_map,
            file_paths: self.file_paths,
        }
    }
}

#[derive(Default)]
pub struct ThriftParser {
    files: Vec<PathBuf>,
    db: ThriftSourceDatabase,
    include_dirs: Vec<PathBuf>,
}

impl super::Parser for ThriftParser {
    fn input<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.files.push(path.as_ref().into())
    }

    fn include_dirs(&mut self, dirs: Vec<PathBuf>) {
        self.include_dirs.extend(dirs);
    }

    fn parse(self) -> super::ParseResult {
        let db = self.db.clone();
        let mut lower = ThriftLower::new(self.db, self.include_dirs.clone());
        let mut input_files = Vec::default();

        self.files.iter().for_each(|f| {
            input_files.push(
                lower.lower(
                    db.parse(
                        f.to_path_buf()
                            .normalize()
                            .unwrap_or_else(|_| panic!("normalize path failed: {}", f.display()))
                            .into_path_buf(),
                    ),
                ),
            );
        });

        let result = lower.finish();

        super::ParseResult {
            files: result.files,
            input_files,
            file_ids_map: result.file_ids_map,
            file_paths: result.file_paths,
        }
    }
}
