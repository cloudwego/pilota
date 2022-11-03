use std::{path::PathBuf, str::FromStr, sync::Arc};

use fxhash::FxHashMap;
use heck::ToUpperCamelCase;
use itertools::Itertools;
use normpath::PathExt;
use pilota_thrift_parser as thrift_parser;
use pilota_thrift_parser::parser::Parser as _;
use salsa::ParallelDatabase;
use thrift_parser::Annotations;

use crate::{
    index::Idx,
    ir,
    ir::{Arg, Enum, EnumVariant, FieldKind, File, Item, ItemKind, Path},
    symbol::{EnumRepr, FileId, Ident},
    tags::{Annotation, PilotaName, RustType, Tags},
    ty::BytesRepr,
    util::error_abort,
};

#[salsa::query_group(SourceDatabaseStorage)]
trait SourceDatabase {
    fn file_text(&self, path: PathBuf) -> Arc<String>;
    fn parse(&self, path: PathBuf) -> Arc<thrift_parser::File>;
}

fn file_text(_db: &dyn SourceDatabase, path: PathBuf) -> Arc<String> {
    Arc::new(unsafe { String::from_utf8_unchecked(std::fs::read(path).unwrap()) })
}

fn parse(db: &dyn SourceDatabase, path: PathBuf) -> Arc<thrift_parser::File> {
    let text = db.file_text(path.clone());
    let mut ast = thrift_parser::File::parse(&text).unwrap().1;
    ast.path = Arc::from(path);
    Arc::from(ast)
}

#[derive(Default)]
#[salsa::database(SourceDatabaseStorage)]
struct ThriftSourceDatabase {
    storage: salsa::Storage<ThriftSourceDatabase>,
}

impl salsa::Database for ThriftSourceDatabase {}

impl salsa::ParallelDatabase for ThriftSourceDatabase {
    fn snapshot(&self) -> salsa::Snapshot<ThriftSourceDatabase> {
        salsa::Snapshot::new(ThriftSourceDatabase {
            storage: self.storage.snapshot(),
        })
    }
}

#[derive(Debug)]
pub struct LowerResult {
    pub files: Vec<Arc<File>>,
    pub file_ids_map: FxHashMap<Arc<PathBuf>, FileId>,
}

pub trait Lower<Ast> {
    fn lower(&mut self, file: Ast) -> FileId;

    fn finish(self) -> LowerResult;
}

pub struct ThriftLower {
    cur_file: Option<Arc<thrift_parser::File>>,
    next_file_id: FileId,
    db: salsa::Snapshot<ThriftSourceDatabase>,
    files: FxHashMap<FileId, Arc<File>>,
    file_ids_map: FxHashMap<Arc<PathBuf>, FileId>,
    include_dirs: Vec<PathBuf>,
    packages: FxHashMap<Path, Vec<Arc<PathBuf>>>,
}

impl ThriftLower {
    fn new(db: salsa::Snapshot<ThriftSourceDatabase>, include_dirs: Vec<PathBuf>) -> Self {
        ThriftLower {
            cur_file: None,
            next_file_id: FileId::from_u32(0),
            db,
            files: FxHashMap::default(),
            file_ids_map: FxHashMap::default(),
            include_dirs,
            packages: Default::default(),
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

    fn lower_path(&mut self, path: &thrift_parser::Path) -> ir::Path {
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

    fn lower_service(&mut self, service: &thrift_parser::Service) -> Vec<ir::Item> {
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
                .map(|f| self.lower_method(service, f))
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

            let method_name = tags
                .get::<PilotaName>()
                .map(|name| &*name.0)
                .unwrap_or_else(|| &*f.name);

            let name: Ident = format!(
                "{}{}Result",
                service.name.as_str(),
                method_name.to_upper_camel_case()
            )
            .into();
            let kind = ir::ItemKind::Enum(ir::Enum {
                name: name.clone(),
                variants: std::iter::once(ir::EnumVariant {
                    id: Some(0),
                    name: "Ok".into(),
                    tags: Default::default(),
                    discr: None,
                    fields: vec![self.lower_ty(&f.result_type)],
                })
                .chain(exception.clone())
                .collect(),
                repr: None,
            });
            related_items.push(name);
            result.push(self.mk_item(kind, Default::default()));

            if !exception.is_empty() {
                let name: Ident = format!(
                    "{}{}Exception",
                    service.name.to_upper_camel_case().as_str(),
                    f.name.as_str().to_upper_camel_case()
                )
                .into();
                let kind = ir::ItemKind::Enum(ir::Enum {
                    name: name.clone(),
                    variants: exception,
                    repr: None,
                });
                related_items.push(name);
                result.push(self.mk_item(kind, Default::default()));
            }
            let name: Ident = format!(
                "{}{}Args",
                service.name.to_upper_camel_case().as_str(),
                method_name.to_upper_camel_case()
            )
            .into();
            let kind = ir::ItemKind::Message(ir::Message {
                name: name.clone(),
                fields: f.arguments.iter().map(|a| self.lower_field(a)).collect(),
            });
            related_items.push(name);
            result.push(self.mk_item(kind, Default::default()));
        });

        service_item.related_items = related_items;
        result.push(service_item);
        result
    }

    fn lower_method(
        &mut self,
        service: &thrift_parser::Service,
        method: &thrift_parser::Function,
    ) -> ir::Method {
        ir::Method {
            name: self.lower_ident(&method.name),
            args: method
                .arguments
                .iter()
                .map(|a| Arg {
                    ty: self.lower_ty(&a.ty),
                    id: a.id,
                    name: self.lower_ident(&a.name),
                })
                .collect(),
            ret: self.lower_ty(&method.result_type),
            oneway: method.oneway,
            tags: self.extract_tags(&method.annotations).into(),
            exceptions: if method.throws.is_empty() {
                None
            } else {
                Some(Path {
                    segments: Arc::from([Ident::from(format!(
                        "{}{}Exception",
                        service.name.to_upper_camel_case().as_str(),
                        method.name.to_upper_camel_case(),
                    ))]),
                })
            },
        }
    }

    fn lower_enum(&mut self, e: &thrift_parser::Enum) -> ir::Enum {
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
                    tags: Default::default(),
                })
                .collect(),
            repr: Some(EnumRepr::I32),
        }
    }

    fn lower_lit(&mut self, l: &thrift_parser::ConstValue) -> ir::Literal {
        match &l {
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

    fn lower_const(&mut self, c: &thrift_parser::Constant) -> ir::Const {
        ir::Const {
            name: self.lower_ident(&c.name),
            ty: self.lower_ty(&c.r#type),
            lit: self.lower_lit(&c.value),
        }
    }

    fn lower_typedef(&mut self, t: &thrift_parser::Typedef) -> ir::NewType {
        ir::NewType {
            name: self.lower_ident(&t.alias),
            ty: self.lower_ty(&t.r#type),
        }
    }

    fn lower_item(&mut self, item: &thrift_parser::Item) -> Vec<ir::Item> {
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

    fn lower_union(&mut self, union: &thrift_parser::Union) -> Enum {
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

    fn lower_ident(&mut self, s: &thrift_parser::Ident) -> Ident {
        Ident::from(s.0.clone())
    }

    fn lower_ty_with_tags(&mut self, ty: &thrift_parser::Ty, tags: &Tags) -> ir::Ty {
        let rust_type = tags.get::<RustType>();
        if let Some(rust_type) = rust_type {
            match &ty {
                thrift_parser::Ty::Binary if rust_type == "bytes" => {
                    let mut tags = Tags::default();
                    tags.insert(BytesRepr::Bytes);
                    return ir::Ty {
                        tags: tags.into(),
                        kind: ir::TyKind::Bytes,
                    };
                }
                _ => {}
            }
        }

        self.lower_ty(ty)
    }

    fn lower_ty(&mut self, ty: &thrift_parser::Ty) -> ir::Ty {
        let kind = match &ty {
            thrift_parser::Ty::String => ir::TyKind::String,
            thrift_parser::Ty::Void => ir::TyKind::Void,
            thrift_parser::Ty::Byte => ir::TyKind::U8,
            thrift_parser::Ty::Bool => ir::TyKind::Bool,
            thrift_parser::Ty::Binary => ir::TyKind::Bytes,
            thrift_parser::Ty::I8 => ir::TyKind::I8,
            thrift_parser::Ty::I16 => ir::TyKind::I16,
            thrift_parser::Ty::I32 => ir::TyKind::I32,
            thrift_parser::Ty::I64 => ir::TyKind::I64,
            thrift_parser::Ty::Double => ir::TyKind::F64,
            thrift_parser::Ty::List { value, .. } => ir::TyKind::Vec(self.lower_ty(value).into()),
            thrift_parser::Ty::Set { value, .. } => ir::TyKind::Set(self.lower_ty(value).into()),
            thrift_parser::Ty::Map { key, value, .. } => {
                ir::TyKind::Map(self.lower_ty(key).into(), self.lower_ty(value).into())
            }
            thrift_parser::Ty::Path(path) => ir::TyKind::Path(self.lower_path(path)),
        };

        ir::Ty {
            kind,
            tags: Default::default(),
        }
    }

    fn lower_field(&mut self, f: &thrift_parser::Field) -> ir::Field {
        let tags = self.extract_tags(&f.annotations);
        ir::Field {
            name: self.lower_ident(&f.name),
            id: f.id,
            ty: self.lower_ty_with_tags(&f.ty, &tags),
            kind: match f.attribute {
                thrift_parser::Attribute::Required => FieldKind::Required,
                _ => FieldKind::Optional,
            },
            tags: tags.into(),
        }
    }

    fn extract_tags(&mut self, annotations: &Annotations) -> Tags {
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
            |annotation| with_tags!(annotation -> crate::tags::PilotaName | crate::tags::RustType),
        );

        tags
    }

    fn lower_struct(&mut self, s: &thrift_parser::StructLike) -> ir::Message {
        ir::Message {
            name: self.lower_ident(&s.name),
            fields: s.fields.iter().map(|f| self.lower_field(f)).collect(),
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
                            .last()
                            .unwrap()
                            .trim_end_matches(".thrift")
                            .split('.')
                            .map(|s| Ident::from(s))
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

            ir::File {
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
            }
        });

        file.id
    }

    fn finish(self) -> LowerResult {
        self.packages.iter().for_each(|(k, v)| {
            if v.len() > 1 {
                println!(
                    "cargo:warning={:?} has the same namespace `{}`, you may need set namespace for these file \n",
                    v,
                    k.segments.iter().join(".")
                )
            }
        });
        LowerResult {
            files: self.files.into_values().collect::<Vec<_>>(),
            file_ids_map: self.file_ids_map,
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
        self.include_dirs = dirs;
    }

    fn parse(self) -> super::ParseResult {
        let mut lower = ThriftLower::new(self.db.snapshot(), self.include_dirs.clone());
        let mut input_files = Vec::default();

        self.files.iter().for_each(|f| {
            input_files.push(
                lower.lower(
                    self.db
                        .parse(f.to_path_buf().normalize().unwrap().into_path_buf()),
                ),
            );
        });

        let result = lower.finish();

        super::ParseResult {
            files: result.files,
            input_files,
            file_ids_map: result.file_ids_map,
        }
    }
}
