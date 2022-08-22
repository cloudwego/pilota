use std::{ops::Deref, sync::Arc};

use super::{Annotations, Literal, Path};

/// Type with annotations
#[derive(Debug, Clone)]
pub struct Type(pub Ty, pub Annotations);

impl Deref for Type {
    type Target = Ty;
    fn deref(&self) -> &Ty {
        &self.0
    }
}
#[derive(Debug, Clone)]
pub struct CppType(pub Literal);

#[derive(Debug, Clone)]
pub enum Ty {
    String,
    Void,
    Byte,
    Bool,
    Binary,
    I8,
    I16,
    I32,
    I64,
    Double,
    List {
        value: Arc<Type>,
        cpp_type: Option<CppType>,
    },
    Set {
        value: Arc<Type>,
        cpp_type: Option<CppType>,
    },
    Map {
        key: Arc<Type>,
        value: Arc<Type>,
        cpp_type: Option<CppType>,
    },
    Path(Path),
}
