use std::sync::Arc;

use quote::{quote, ToTokens};
pub use TyKind::*;

use super::{context::tls::with_cx, rir::Path};
use crate::{db::RirDatabase, symbol::DefId, tags::TagId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TyKind {
    String,
    FastStr,
    Void,
    U8,
    Bool,
    BytesVec,
    Bytes,
    I8,
    I16,
    I32,
    I64,
    UInt32,
    UInt64,
    F32,
    F64,
    Vec(Arc<Ty>),
    Set(Arc<Ty>),
    Map(Arc<Ty>, Arc<Ty>),
    Arc(Arc<Ty>),
    Path(Path),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ty {
    pub kind: TyKind,
    pub tags_id: TagId,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct AdtDef {
    pub did: DefId,
    pub kind: AdtKind,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum AdtKind {
    Struct,
    Enum,
    NewType(Arc<CodegenTy>),
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum CodegenTy {
    FastStr,
    String, // for protobuf
    Str,    // static str
    Void,
    U8,
    Bool,
    I8,
    I16,
    I32,
    I64,
    UInt32,
    UInt64,
    F32,
    F64,
    Bytes,
    LazyStaticRef(Arc<CodegenTy>),
    StaticRef(Arc<CodegenTy>),
    Vec(Arc<CodegenTy>),
    Array(Arc<CodegenTy>, usize),
    Set(Arc<CodegenTy>),
    Map(Arc<CodegenTy>, Arc<CodegenTy>),
    Adt(AdtDef),
    Arc(Arc<CodegenTy>),
}

impl CodegenTy {
    pub fn should_lazy_static(&self) -> bool {
        match self {
            CodegenTy::String
            | CodegenTy::FastStr
            | CodegenTy::LazyStaticRef(_)
            | CodegenTy::StaticRef(_)
            | CodegenTy::Vec(_)
            | CodegenTy::Map(_, _) => true,
            CodegenTy::Adt(AdtDef {
                did: _,
                kind: AdtKind::NewType(inner),
            }) => inner.should_lazy_static(),
            _ => false,
        }
    }
}

impl ToTokens for CodegenTy {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            CodegenTy::String => tokens.extend(quote! { ::std::string::String }),
            CodegenTy::FastStr => tokens.extend(quote! { ::pilota::FastStr }),
            CodegenTy::Str => tokens.extend(quote! { &'static str }),
            CodegenTy::Void => tokens.extend(quote! { () }),
            CodegenTy::U8 => tokens.extend(quote! { u8 }),
            CodegenTy::Bool => tokens.extend(quote! { bool }),
            CodegenTy::I8 => tokens.extend(quote! { i8 }),
            CodegenTy::I16 => tokens.extend(quote! { i16 }),
            CodegenTy::I32 => tokens.extend(quote! { i32 }),
            CodegenTy::I64 => tokens.extend(quote! { i64 }),
            CodegenTy::F64 => tokens.extend(quote! { f64 }),
            CodegenTy::UInt32 => tokens.extend(quote! { u32 }),
            CodegenTy::UInt64 => tokens.extend(quote! { u64 }),
            CodegenTy::F32 => tokens.extend(quote! { f32 }),
            CodegenTy::StaticRef(ty) => {
                let ty = &**ty;
                tokens.extend(quote! { &'static #ty })
            }
            CodegenTy::Vec(ty) => {
                let ty = &**ty;
                tokens.extend(quote! { ::std::vec::Vec<#ty> })
            }
            CodegenTy::Array(ty, size) => {
                let ty = &**ty;
                tokens.extend(quote! { [#ty; #size] })
            }
            CodegenTy::Set(ty) => {
                let ty = &**ty;
                tokens.extend(quote! { ::std::collections::HashSet<#ty> })
            }
            CodegenTy::Map(k, v) => {
                let k = &**k;
                let v = &**v;
                tokens.extend(quote! { ::std::collections::HashMap<#k, #v> })
            }
            CodegenTy::Adt(def) => with_cx(|cx| {
                let path = cx.cur_related_item_path(def.did);
                tokens.extend(quote! { #path })
            }),
            CodegenTy::Arc(ty) => {
                let ty = &**ty;
                tokens.extend(quote!( ::std::sync::Arc<#ty> ))
            }
            CodegenTy::LazyStaticRef(ty) => ty.to_tokens(tokens),
            CodegenTy::Bytes => tokens.extend(quote! { ::pilota::Bytes }),
        }
    }
}

impl TyKind {
    pub(crate) fn to_codegen_item_ty(&self) -> CodegenTy {
        DefaultTyTransformer.codegen_item_ty(self)
    }

    pub(crate) fn to_codegen_const_ty(&self) -> CodegenTy {
        ConstTyTransformer.codegen_item_ty(self)
    }
}

pub trait TyTransformer {
    #[inline]
    fn string(&self) -> CodegenTy {
        CodegenTy::String
    }

    #[inline]
    fn faststr(&self) -> CodegenTy {
        CodegenTy::FastStr
    }

    #[inline]
    fn void(&self) -> CodegenTy {
        CodegenTy::Void
    }

    #[inline]
    fn u8(&self) -> CodegenTy {
        CodegenTy::U8
    }

    #[inline]
    fn bool(&self) -> CodegenTy {
        CodegenTy::Bool
    }

    #[inline]
    fn bytes(&self) -> CodegenTy {
        CodegenTy::Bytes
    }

    #[inline]
    fn bytes_vec(&self) -> CodegenTy {
        CodegenTy::Vec(Arc::new(CodegenTy::U8))
    }

    #[inline]
    fn i8(&self) -> CodegenTy {
        CodegenTy::I8
    }

    #[inline]
    fn i16(&self) -> CodegenTy {
        CodegenTy::I16
    }

    #[inline]
    fn i32(&self) -> CodegenTy {
        CodegenTy::I32
    }

    #[inline]
    fn uint32(&self) -> CodegenTy {
        CodegenTy::UInt32
    }

    #[inline]
    fn i64(&self) -> CodegenTy {
        CodegenTy::I64
    }

    #[inline]
    fn uint64(&self) -> CodegenTy {
        CodegenTy::UInt64
    }

    #[inline]
    fn f64(&self) -> CodegenTy {
        CodegenTy::F64
    }

    #[inline]
    fn f32(&self) -> CodegenTy {
        CodegenTy::F32
    }

    #[inline]
    fn arc(&self, ty: &Ty) -> CodegenTy {
        CodegenTy::Arc(Arc::from(self.codegen_item_ty(&ty.kind)))
    }

    #[inline]
    fn vec(&self, ty: &Ty) -> CodegenTy {
        CodegenTy::Vec(Arc::from(self.codegen_item_ty(&ty.kind)))
    }

    #[inline]
    fn set(&self, ty: &Ty) -> CodegenTy {
        CodegenTy::Set(Arc::from(self.codegen_item_ty(&ty.kind)))
    }

    #[inline]
    fn map(&self, key: &Ty, value: &Ty) -> CodegenTy {
        let key = self.codegen_item_ty(&key.kind);
        let value = self.codegen_item_ty(&value.kind);
        CodegenTy::Map(Arc::from(key), Arc::from(value))
    }

    #[inline]
    fn path(&self, path: &Path) -> CodegenTy {
        let did = path.did;
        with_cx(|cx| cx.codegen_ty(did))
    }

    #[inline]
    fn stream(&self) -> CodegenTy {
        todo!();
    }

    #[inline]
    fn codegen_item_ty(&self, ty: &TyKind) -> CodegenTy {
        match &ty {
            String => self.string(),
            FastStr => self.faststr(),
            Void => self.void(),
            U8 => self.u8(),
            Bool => self.bool(),
            BytesVec => self.bytes_vec(),
            Bytes => self.bytes(),
            I8 => self.i8(),
            I16 => self.i16(),
            I32 => self.i32(),
            I64 => self.i64(),
            F64 => self.f64(),
            Vec(ty) => self.vec(ty),
            Set(ty) => self.set(ty),
            Map(k, v) => self.map(k, v),
            Path(path) => self.path(path),
            UInt32 => self.uint32(),
            UInt64 => self.uint64(),
            F32 => self.f32(),
            Arc(ty) => self.arc(ty),
        }
    }
}

pub enum StringRepr {
    FastStr,
    String,
}

pub(crate) struct DefaultTyTransformer;

impl TyTransformer for DefaultTyTransformer {}

pub(crate) struct ConstTyTransformer;

impl ConstTyTransformer {
    #[inline]
    fn dyn_codegen_item_ty(&self, kind: &TyKind) -> CodegenTy {
        let mut ty = self.codegen_item_ty(&kind);
        if let CodegenTy::Array(_inner, _) = ty {
            ty = CodegenTy::Vec(_inner);
        }
        ty
    }
}

impl TyTransformer for ConstTyTransformer {
    #[inline]
    fn string(&self) -> CodegenTy {
        CodegenTy::Str
    }

    #[inline]
    fn faststr(&self) -> CodegenTy {
        CodegenTy::Str
    }

    #[inline]
    fn vec(&self, ty: &Ty) -> CodegenTy {
        CodegenTy::Array(Arc::from(self.codegen_item_ty(&ty.kind)), 0)
    }

    #[inline]
    fn set(&self, ty: &Ty) -> CodegenTy {
        CodegenTy::StaticRef(Arc::from(CodegenTy::Set(Arc::from(
            self.dyn_codegen_item_ty(&ty.kind),
        ))))
    }

    #[inline]
    fn map(&self, key: &Ty, value: &Ty) -> CodegenTy {
        let key = self.dyn_codegen_item_ty(&key.kind);
        let value = self.dyn_codegen_item_ty(&value.kind);
        CodegenTy::StaticRef(Arc::from(CodegenTy::Map(Arc::from(key), Arc::from(value))))
    }
}

pub(crate) trait Visitor: Sized {
    fn visit_path(&mut self, _path: &Path) {}

    fn visit_vec(&mut self, el: &Ty) {
        self.visit(el)
    }

    fn visit_set(&mut self, el: &Ty) {
        self.visit(el)
    }

    fn visit_map(&mut self, k: &Ty, v: &Ty) {
        self.visit(k);
        self.visit(v);
    }

    fn visit(&mut self, ty: &Ty) {
        walk_ty(self, ty)
    }
}

pub(crate) trait Folder: Sized {
    fn fold_ty(&mut self, ty: &Ty) -> Ty {
        fold_ty(self, ty)
    }
}

pub(crate) fn fold_ty<F: Folder>(f: &mut F, ty: &Ty) -> Ty {
    let kind = match &ty.kind {
        String => TyKind::String,
        FastStr => TyKind::FastStr,
        Void => TyKind::Void,
        U8 => TyKind::U8,
        Bool => TyKind::Bool,
        BytesVec => TyKind::BytesVec,
        Bytes => TyKind::Bytes,
        I8 => TyKind::I8,
        I16 => TyKind::I16,
        I32 => TyKind::I32,
        I64 => TyKind::I64,
        F64 => TyKind::F64,
        Vec(ty) => TyKind::Vec(f.fold_ty(ty).into()),
        Set(ty) => TyKind::Set(f.fold_ty(ty).into()),
        Map(k, v) => TyKind::Map(fold_ty(f, k).into(), fold_ty(f, v).into()),
        Path(path) => TyKind::Path(path.clone()),
        UInt32 => TyKind::UInt32,
        UInt64 => TyKind::UInt64,
        F32 => TyKind::F32,
        Arc(ty) => TyKind::Arc(f.fold_ty(ty).into()),
    };

    Ty {
        kind,
        tags_id: ty.tags_id,
    }
}

pub(crate) fn walk_ty<V: Visitor>(v: &mut V, ty: &Ty) {
    match &ty.kind {
        Vec(el) => v.visit_vec(el),
        Set(el) => v.visit_set(el),
        Map(key, value) => v.visit_map(key, value),
        Path(p) => v.visit_path(p),
        _ => {}
    }
}
