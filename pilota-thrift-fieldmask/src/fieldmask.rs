use std::{borrow::Cow, fmt::Display};

use ahash::AHashMap;
use pilota::FastStr;
use pilota_thrift_reflect::{ThriftType, descriptor::thrift_reflection::TypeDescriptor};
use thiserror::Error;

use crate::path::{PathError, PathIterator, PathToken, TokenData};

#[derive(Debug, Clone)]
pub struct PathDetail {
    path: FastStr,
    position: usize,
}

impl Display for PathDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "path '{path}' at position {position}",
            path = self.path,
            position = self.position
        )
    }
}

#[derive(Debug, Clone)]
pub struct TypeMismatchDetail {
    expected: FastStr,
    actual: FastStr,
    context: FastStr,
}

impl Display for TypeMismatchDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "type mismatch in {context}: expected '{expected}', actual '{actual}'",
            expected = self.expected,
            actual = self.actual,
            context = self.context,
        )
    }
}

#[derive(Debug, Clone, Error)]
pub enum FieldMaskError {
    #[error("path '{path}', parse error: {source}")]
    PathError {
        path: FastStr,
        #[source]
        source: Box<PathError>,
    },
    #[error("{path}, type descriptor error on '{type_name}': {message}")]
    DescriptorError {
        type_name: FastStr,
        message: FastStr,
        path: Box<PathDetail>,
    },
    #[error("{path}, field '{field_identifier}' not found in type '{parent_type}'")]
    FieldNotFound {
        field_identifier: FastStr,
        parent_type: FastStr,
        path: Box<PathDetail>,
    },
    #[error("{path}, {detail}")]
    TypeMismatch {
        detail: Box<TypeMismatchDetail>,
        path: Box<PathDetail>,
    },
    #[error("{path}, empty {collection_type} collection")]
    EmptyCollection {
        collection_type: FastStr,
        path: Box<PathDetail>,
    },
    #[error("{path}, conflict error: {message}")]
    ConflictError {
        message: FastStr,
        path: Box<PathDetail>,
    },
    #[error("{path}, invalid token type '{token_type}', expected '{expected}'")]
    InvalidToken {
        token_type: FastStr,
        expected: FastStr,
        path: Box<PathDetail>,
    },
    #[error("field mask error: {message}")]
    GenericError { message: String },
}

#[derive(Debug, Clone, Eq, serde::Serialize, serde::Deserialize)]
pub enum FieldMaskData {
    Invalid,
    Scalar,
    Struct {
        children: AHashMap<i32, Box<FieldMask>>, // field id -> sub mask
        is_all: bool,                            // is all fields
    },
    List {
        children: AHashMap<i32, Box<FieldMask>>, // index -> sub mask
        wildcard: Option<Box<FieldMask>>,        // wildcard sub mask
        is_all: bool,
    },
    StrMap {
        children: AHashMap<FastStr, Box<FieldMask>>, // key -> sub mask
        wildcard: Option<Box<FieldMask>>,            // wildcard sub mask
        is_all: bool,
    },
    IntMap {
        children: AHashMap<i32, Box<FieldMask>>, // key -> sub mask
        wildcard: Option<Box<FieldMask>>,        // wildcard sub mask
        is_all: bool,
    },
}

impl PartialEq for FieldMaskData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldMaskData::Invalid, FieldMaskData::Invalid) => true,
            (FieldMaskData::Scalar, FieldMaskData::Scalar) => true,
            (
                FieldMaskData::Struct {
                    children: c1,
                    is_all: w1,
                },
                FieldMaskData::Struct {
                    children: c2,
                    is_all: w2,
                },
            ) => {
                let mut children1 = c1.iter().collect::<Vec<_>>();
                let mut children2 = c2.iter().collect::<Vec<_>>();
                children1.sort_unstable();
                children2.sort_unstable();
                children1 == children2 && w1.eq(w2)
            }
            (
                FieldMaskData::List {
                    children: c1,
                    wildcard: w1,
                    is_all: a1,
                },
                FieldMaskData::List {
                    children: c2,
                    wildcard: w2,
                    is_all: a2,
                },
            ) => {
                let mut children1 = c1.iter().collect::<Vec<_>>();
                let mut children2 = c2.iter().collect::<Vec<_>>();
                children1.sort_unstable();
                children2.sort_unstable();
                children1 == children2 && w1.eq(w2) && a1.eq(a2)
            }
            (
                FieldMaskData::StrMap {
                    children: c1,
                    wildcard: w1,
                    is_all: a1,
                },
                FieldMaskData::StrMap {
                    children: c2,
                    wildcard: w2,
                    is_all: a2,
                },
            ) => {
                let mut children1 = c1.iter().collect::<Vec<_>>();
                let mut children2 = c2.iter().collect::<Vec<_>>();
                children1.sort_unstable();
                children2.sort_unstable();
                children1 == children2 && w1.eq(w2) && a1.eq(a2)
            }
            (
                FieldMaskData::IntMap {
                    children: c1,
                    wildcard: w1,
                    is_all: a1,
                },
                FieldMaskData::IntMap {
                    children: c2,
                    wildcard: w2,
                    is_all: a2,
                },
            ) => {
                let mut children1 = c1.iter().collect::<Vec<_>>();
                let mut children2 = c2.iter().collect::<Vec<_>>();
                children1.sort_unstable();
                children2.sort_unstable();
                children1 == children2 && w1.eq(w2) && a1.eq(a2)
            }
            _ => false,
        }
    }
}

impl Ord for FieldMaskData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (FieldMaskData::Invalid, FieldMaskData::Invalid) => std::cmp::Ordering::Equal,
            (FieldMaskData::Scalar, FieldMaskData::Scalar) => std::cmp::Ordering::Equal,
            (
                FieldMaskData::Struct {
                    children: c1,
                    is_all: w1,
                },
                FieldMaskData::Struct {
                    children: c2,
                    is_all: w2,
                },
            ) => {
                if c1.is_empty() && c2.is_empty() {
                    return w1.cmp(w2);
                }
                // wildcard is always greater
                if c1.is_empty() && *w1 {
                    return std::cmp::Ordering::Greater;
                }
                if c2.is_empty() && *w2 {
                    return std::cmp::Ordering::Less;
                }
                // compare children
                if c1.len() != c2.len() {
                    return c1.len().cmp(&c2.len());
                }
                let mut children1 = c1.iter().collect::<Vec<_>>();
                let mut children2 = c2.iter().collect::<Vec<_>>();
                children1.sort_unstable();
                children2.sort_unstable();
                children1.cmp(&children2)
            }
            (
                FieldMaskData::List {
                    children: c1,
                    wildcard: w1,
                    is_all: a1,
                },
                FieldMaskData::List {
                    children: c2,
                    wildcard: w2,
                    is_all: a2,
                },
            ) => {
                if c1.is_empty() && c2.is_empty() {
                    return a1.cmp(a2).then(w1.cmp(w2));
                }
                // wildcard is always greater
                if c1.is_empty() && *a1 {
                    return std::cmp::Ordering::Greater;
                }
                if c2.is_empty() && *a2 {
                    return std::cmp::Ordering::Less;
                }
                if c1.is_empty() && w1.is_some() {
                    return std::cmp::Ordering::Greater;
                }
                if c2.is_empty() && w2.is_some() {
                    return std::cmp::Ordering::Less;
                }
                // compare children
                if c1.len() != c2.len() {
                    return c1.len().cmp(&c2.len());
                }
                let mut children1 = c1.iter().collect::<Vec<_>>();
                let mut children2 = c2.iter().collect::<Vec<_>>();
                children1.sort_unstable();
                children2.sort_unstable();
                children1.cmp(&children2)
            }
            (
                FieldMaskData::StrMap {
                    children: c1,
                    wildcard: w1,
                    is_all: a1,
                },
                FieldMaskData::StrMap {
                    children: c2,
                    wildcard: w2,
                    is_all: a2,
                },
            ) => {
                if c1.is_empty() && c2.is_empty() {
                    return a1.cmp(a2).then(w1.cmp(w2));
                }
                // wildcard is always greater
                if c1.is_empty() && *a1 {
                    return std::cmp::Ordering::Greater;
                }
                if c2.is_empty() && *a2 {
                    return std::cmp::Ordering::Less;
                }
                if c1.is_empty() && w1.is_some() {
                    return std::cmp::Ordering::Greater;
                }
                if c2.is_empty() && w2.is_some() {
                    return std::cmp::Ordering::Less;
                }
                // compare children
                if c1.len() != c2.len() {
                    return c1.len().cmp(&c2.len());
                }
                let mut children1 = c1.iter().collect::<Vec<_>>();
                let mut children2 = c2.iter().collect::<Vec<_>>();
                children1.sort_unstable();
                children2.sort_unstable();
                children1.cmp(&children2)
            }
            (
                FieldMaskData::IntMap {
                    children: c1,
                    wildcard: w1,
                    is_all: a1,
                },
                FieldMaskData::IntMap {
                    children: c2,
                    wildcard: w2,
                    is_all: a2,
                },
            ) => {
                if c1.is_empty() && c2.is_empty() {
                    return a1.cmp(a2).then(w1.cmp(w2));
                }
                // wildcard is always greater
                if c1.is_empty() && *a1 {
                    return std::cmp::Ordering::Greater;
                }
                if c2.is_empty() && *a2 {
                    return std::cmp::Ordering::Less;
                }
                if c1.is_empty() && w1.is_some() {
                    return std::cmp::Ordering::Greater;
                }
                if c2.is_empty() && w2.is_some() {
                    return std::cmp::Ordering::Less;
                }
                // compare children
                if c1.len() != c2.len() {
                    return c1.len().cmp(&c2.len());
                }
                let mut children1 = c1.iter().collect::<Vec<_>>();
                let mut children2 = c2.iter().collect::<Vec<_>>();
                children1.sort_unstable();
                children2.sort_unstable();
                children1.cmp(&children2)
            }
            (FieldMaskData::Invalid, _) => std::cmp::Ordering::Less,
            (_, FieldMaskData::Invalid) => std::cmp::Ordering::Greater,
            (FieldMaskData::Scalar, _) => std::cmp::Ordering::Less,
            (_, FieldMaskData::Scalar) => std::cmp::Ordering::Greater,
            (FieldMaskData::Struct { .. }, _) => std::cmp::Ordering::Less,
            (_, FieldMaskData::Struct { .. }) => std::cmp::Ordering::Greater,
            (FieldMaskData::List { .. }, _) => std::cmp::Ordering::Less,
            (_, FieldMaskData::List { .. }) => std::cmp::Ordering::Greater,
            (FieldMaskData::StrMap { .. }, _) => std::cmp::Ordering::Less,
            (_, FieldMaskData::StrMap { .. }) => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for FieldMaskData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for FieldMaskData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            FieldMaskData::Invalid => 0.hash(state),
            FieldMaskData::Scalar => 1.hash(state),
            FieldMaskData::Struct { children, is_all } => {
                children.iter().for_each(|(k, v)| {
                    k.hash(state);
                    v.hash(state);
                });
                is_all.hash(state);
            }
            FieldMaskData::List {
                children,
                wildcard,
                is_all,
            } => {
                children.iter().for_each(|(k, v)| {
                    k.hash(state);
                    v.hash(state);
                });
                if let Some(wildcard) = wildcard {
                    wildcard.hash(state);
                }
                is_all.hash(state);
            }
            FieldMaskData::StrMap {
                children,
                wildcard,
                is_all,
            } => {
                children.iter().for_each(|(k, v)| {
                    k.hash(state);
                    v.hash(state);
                });
                if let Some(wildcard) = wildcard {
                    wildcard.hash(state);
                }
                is_all.hash(state);
            }
            FieldMaskData::IntMap {
                children,
                wildcard,
                is_all,
            } => {
                children.iter().for_each(|(k, v)| {
                    k.hash(state);
                    v.hash(state);
                });
                if let Some(wildcard) = wildcard {
                    wildcard.hash(state);
                }
                is_all.hash(state);
            }
        }
    }
}

impl Default for FieldMaskData {
    fn default() -> Self {
        Self::Invalid
    }
}

impl FieldMaskData {
    #[inline]
    fn new(desc: &TypeDescriptor) -> Self {
        let type_name = desc.name.as_str().into();
        match type_name {
            ThriftType::Path(_) => {
                if desc.get_struct_desc().is_some() {
                    FieldMaskData::Struct {
                        children: AHashMap::new(),
                        is_all: false,
                    }
                } else {
                    FieldMaskData::Scalar
                }
            }
            ThriftType::List => FieldMaskData::List {
                children: AHashMap::new(),
                wildcard: None,
                is_all: false,
            },
            ThriftType::Map => {
                if let Some(key_type) = &desc.key_type {
                    let type_name = key_type.name.as_str().into();
                    match type_name {
                        ThriftType::String | ThriftType::Binary => FieldMaskData::StrMap {
                            children: AHashMap::new(),
                            wildcard: None,
                            is_all: false,
                        },
                        ThriftType::I8
                        | ThriftType::I16
                        | ThriftType::I32
                        | ThriftType::I64
                        | ThriftType::Byte => FieldMaskData::IntMap {
                            children: AHashMap::new(),
                            wildcard: None,
                            is_all: false,
                        },
                        ThriftType::Path(_) if key_type.get_enum_i32().is_some() => {
                            FieldMaskData::IntMap {
                                children: AHashMap::new(),
                                wildcard: None,
                                is_all: false,
                            }
                        }
                        _ => FieldMaskData::Scalar,
                    }
                } else {
                    FieldMaskData::Invalid
                }
            }
            _ => FieldMaskData::Scalar,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            FieldMaskData::Invalid => "Invalid",
            FieldMaskData::Scalar => "Scalar",
            FieldMaskData::Struct { .. } => "Struct",
            FieldMaskData::List { .. } => "List",
            FieldMaskData::StrMap { .. } => "StrMap",
            FieldMaskData::IntMap { .. } => "IntMap",
        }
    }

    // this is not wildcard, it is the all fields mode and it must be the last
    // token, for example: struct a {
    //     f1: i32,
    // }
    // struct test {
    //     f1: i32,
    //     f2: list<a>,
    //     f3: a,
    // }
    // wildcard is "$.f2[*]", is_all is "$.f2", because the wildcard for f2 can also
    // be extend as "$.f2[*].f1"
    pub fn is_all(&self) -> bool {
        match self {
            FieldMaskData::Struct { is_all, .. }
            | FieldMaskData::List { is_all, .. }
            | FieldMaskData::StrMap { is_all, .. }
            | FieldMaskData::IntMap { is_all, .. } => *is_all,
            FieldMaskData::Scalar => true,
            FieldMaskData::Invalid => false,
        }
    }

    pub fn has_children(&self) -> bool {
        match self {
            FieldMaskData::Struct { children, .. } => !children.is_empty(),
            FieldMaskData::List {
                children, wildcard, ..
            } => !children.is_empty() || wildcard.is_some(),
            FieldMaskData::StrMap {
                children, wildcard, ..
            } => !children.is_empty() || wildcard.is_some(),
            FieldMaskData::IntMap {
                children, wildcard, ..
            } => !children.is_empty() || wildcard.is_some(),
            FieldMaskData::Scalar => false,
            FieldMaskData::Invalid => false,
        }
    }

    fn set_all(&mut self) -> Result<(), FieldMaskError> {
        match self {
            FieldMaskData::Struct { is_all, .. } => *is_all = true,
            FieldMaskData::List { is_all, .. } => *is_all = true,
            FieldMaskData::StrMap { is_all, .. } => *is_all = true,
            FieldMaskData::IntMap { is_all, .. } => *is_all = true,
            FieldMaskData::Scalar => {}
            data => {
                return Err(FieldMaskError::GenericError {
                    message: format!("Cannot set all fields on {:?}", data),
                });
            }
        }
        Ok(())
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct FieldMask {
    is_black: bool, // black list mode flag
    data: FieldMaskData,
}

#[derive(Debug, Clone, Default)]
pub struct Options {
    black_list_mode: bool,
}

impl Options {
    pub fn new() -> Self {
        Self {
            black_list_mode: false,
        }
    }

    pub fn with_black_list_mode(mut self, black_list_mode: bool) -> Self {
        self.black_list_mode = black_list_mode;
        self
    }
}

pub struct FieldMaskBuilder {
    opts: Options,
    desc: TypeDescriptor,
    paths: Vec<FastStr>,
}

impl FieldMaskBuilder {
    pub fn new<S: AsRef<str>>(desc: &TypeDescriptor, paths: &[S]) -> Self {
        Self {
            opts: Options::default(),
            desc: desc.clone(),
            paths: paths.iter().map(|s| FastStr::new(s.as_ref())).collect(),
        }
    }

    pub fn with_options(self, opts: Options) -> Self {
        Self { opts, ..self }
    }

    pub fn build(self) -> Result<FieldMask, FieldMaskError> {
        let mut fm = FieldMask::default();
        if self.opts.black_list_mode {
            fm.is_black = true;
        }
        fm.init(&self.desc, &self.paths)?;
        Ok(fm)
    }
}

impl FieldMask {
    pub fn reset(&mut self) {
        self.data = FieldMaskData::Invalid;
    }

    // default mode: include if field mask exists
    // black mode: include if field mask exists and not `all` mode
    pub fn exist(&self) -> bool {
        match &self.data {
            FieldMaskData::Invalid => self.is_black,
            FieldMaskData::Scalar => !self.is_black,
            FieldMaskData::Struct { .. }
            | FieldMaskData::List { .. }
            | FieldMaskData::StrMap { .. }
            | FieldMaskData::IntMap { .. } => !self.is_black || !self.all(),
        }
    }

    pub fn field(&self, id: i32) -> (Option<&FieldMask>, bool) {
        // (field_fm, is_exist)
        match &self.data {
            FieldMaskData::Struct { children, .. } => {
                let field_fm = children.get(&id).map(|f| f.as_ref());
                // default mode: include if field exists or self is `all` mode
                // black mode: include if field exists and not `all` mode and self is not `all`
                // mode
                let is_exist = (!self.is_black
                    && (field_fm.is_some_and(|f| f.exist()) || self.all()))
                    || (self.is_black && !self.all() && !field_fm.is_some_and(|f| f.all()));
                (field_fm, is_exist)
            }
            _ => (None, self.is_black), // not match, return true if black mode
        }
    }

    pub fn int(&self, id: i32) -> (Option<&FieldMask>, bool) {
        match &self.data {
            FieldMaskData::List {
                children, wildcard, ..
            } => {
                let item_fm = children
                    .get(&id)
                    .map(|f| f.as_ref())
                    .or_else(|| wildcard.as_ref().map(|f| f.as_ref()));
                let is_exist = (!self.is_black
                    && (self.all() || item_fm.is_some_and(|f| f.exist())))
                    || (self.is_black && !self.all() && !item_fm.is_some_and(|f| f.all()));
                (item_fm, is_exist)
            }
            FieldMaskData::IntMap {
                children, wildcard, ..
            } => {
                let item_fm = children
                    .get(&id)
                    .map(|f| f.as_ref())
                    .or_else(|| wildcard.as_ref().map(|f| f.as_ref()));
                let is_exist = (!self.is_black
                    && (self.all() || item_fm.is_some_and(|f| f.exist())))
                    || (self.is_black && !self.all() && !item_fm.is_some_and(|f| f.all()));
                (item_fm, is_exist)
            }
            _ => (None, self.is_black),
        }
    }

    pub fn str(&self, id: &str) -> (Option<&FieldMask>, bool) {
        match &self.data {
            FieldMaskData::StrMap {
                children, wildcard, ..
            } => {
                let item_fm = children
                    .get(id)
                    .map(|f| f.as_ref())
                    .or_else(|| wildcard.as_ref().map(|f| f.as_ref()));
                let is_exist = (!self.is_black
                    && (self.all() || item_fm.is_some_and(|f| f.exist())))
                    || (self.is_black && !self.all() && !item_fm.is_some_and(|f| f.all()));
                (item_fm, is_exist)
            }
            _ => (None, self.is_black),
        }
    }

    pub fn wildcard(&self) -> (Option<&FieldMask>, bool) {
        let item_fm = match &self.data {
            FieldMaskData::List { wildcard, .. }
            | FieldMaskData::StrMap { wildcard, .. }
            | FieldMaskData::IntMap { wildcard, .. } => wildcard.as_ref().map(|v| v.as_ref()),
            _ => None,
        };

        let is_exist = (!self.is_black && (self.all() || item_fm.is_some_and(|f| f.exist())))
            || (self.is_black && !self.all() && !item_fm.is_some_and(|f| f.all()));
        (item_fm, is_exist)
    }

    pub fn all(&self) -> bool {
        self.data.is_all()
    }

    pub fn is_black(&self) -> bool {
        self.is_black
    }

    pub fn typ(&self) -> &str {
        self.data.type_name()
    }

    pub fn for_each_child<F>(&self, mut scanner: F)
    where
        F: FnMut(&str, i32, &FieldMask) -> bool,
    {
        match &self.data {
            FieldMaskData::Scalar | FieldMaskData::Invalid => (),
            FieldMaskData::Struct { children, .. } => {
                for (&k, v) in children {
                    if !scanner("", k, v) {
                        return;
                    }
                }
            }
            FieldMaskData::List { children, .. } | FieldMaskData::IntMap { children, .. } => {
                for (&k, v) in children {
                    if !scanner("", k, v) {
                        return;
                    }
                }
            }
            FieldMaskData::StrMap { children, .. } => {
                for (k, v) in children {
                    if !scanner(k, 0, v) {
                        return;
                    }
                }
            }
        }
    }

    pub fn get_path<'a>(
        &'a self,
        desc: &TypeDescriptor,
        path: &str,
    ) -> Result<(Option<Cow<'a, FieldMask>>, bool), FieldMaskError> {
        let mut it = PathIterator::new(path).map_err(|source| FieldMaskError::PathError {
            path: FastStr::new(path),
            source: Box::new(source),
        })?;

        let mut cur_fm = self;
        let mut cur_desc = desc.clone();

        while it.has_next() {
            let token = it.next();

            match &token.data {
                TokenData::Root => {
                    continue;
                }
                TokenData::Field => {
                    let s =
                        cur_desc
                            .get_struct_desc()
                            .ok_or_else(|| FieldMaskError::TypeMismatch {
                                detail: Box::new(TypeMismatchDetail {
                                    expected: "Struct".into(),
                                    actual: cur_desc.name.clone(),
                                    context: "descriptor type check for field".into(),
                                }),
                                path: Box::new(PathDetail {
                                    path: FastStr::new(path),
                                    position: token.get_begin_pos(),
                                }),
                            })?;

                    if !matches!(cur_fm.data, FieldMaskData::Struct { .. }) && !cur_fm.all() {
                        return Err(FieldMaskError::TypeMismatch {
                            detail: Box::new(TypeMismatchDetail {
                                expected: "Struct".into(),
                                actual: FastStr::new(cur_fm.typ()),
                                context: "FieldMask type check for field".into(),
                            }),
                            path: Box::new(PathDetail {
                                path: FastStr::new(path),
                                position: token.get_begin_pos(),
                            }),
                        });
                    }

                    let field_token = it.next();

                    let field = match &field_token.data {
                        TokenData::LitInt(id) => s.find_field_by_id(*id).cloned(),
                        TokenData::LitStr(name) => s.find_field_by_name(name).cloned(),
                        TokenData::Any => {
                            // for struct, `*` means all fields
                            return Ok((Some(Cow::Borrowed(cur_fm)), true));
                        }
                        _ => {
                            return Err(FieldMaskError::InvalidToken {
                                token_type: FastStr::new(format!("{:?}", field_token.data)),
                                expected: "field name, field id or '*'".into(),
                                path: Box::new(PathDetail {
                                    path: FastStr::new(path),
                                    position: field_token.get_begin_pos(),
                                }),
                            });
                        }
                    }
                    .ok_or_else(|| FieldMaskError::FieldNotFound {
                        field_identifier: format!("{:?}", field_token.data).into(),
                        parent_type: cur_desc.name.clone(),
                        path: Box::new(PathDetail {
                            path: FastStr::new(path),
                            position: field_token.get_begin_pos(),
                        }),
                    })?;

                    let (next_fm, exist) = cur_fm.field(field.id);
                    if !exist {
                        return Ok((None, false));
                    }
                    if next_fm.is_none() {
                        return Ok((Some(Cow::Borrowed(cur_fm)), true));
                    }
                    cur_desc = field.r#type;
                    cur_fm = next_fm.unwrap();
                }
                TokenData::IndexL => {
                    let element_desc = cur_desc.value_type.as_deref().ok_or_else(|| {
                        FieldMaskError::DescriptorError {
                            type_name: cur_desc.name.clone(),
                            message: "collection has no value type".into(),
                            path: Box::new(PathDetail {
                                path: FastStr::new(path),
                                position: token.get_begin_pos(),
                            }),
                        }
                    })?;

                    if !matches!(cur_fm.data, FieldMaskData::List { .. }) && !cur_fm.all() {
                        return Err(FieldMaskError::TypeMismatch {
                            detail: Box::new(TypeMismatchDetail {
                                expected: "List".into(),
                                actual: FastStr::new(cur_fm.typ()),
                                context: "FieldMask type check for list".into(),
                            }),
                            path: Box::new(PathDetail {
                                path: FastStr::new(path),
                                position: token.get_begin_pos(),
                            }),
                        });
                    }

                    if cur_fm.all() {
                        // the path should be end here
                        return Ok((Some(Cow::Borrowed(cur_fm)), true));
                    }

                    let mut next_fm_for_loop = None;
                    let mut empty = true;
                    while it.has_next() {
                        let idx_token = it.next();
                        if idx_token.data == TokenData::IndexR {
                            if empty {
                                return Err(FieldMaskError::EmptyCollection {
                                    collection_type: "index collection".into(),
                                    path: Box::new(PathDetail {
                                        path: FastStr::new(path),
                                        position: idx_token.get_begin_pos(),
                                    }),
                                });
                            }
                            break;
                        }
                        empty = false;

                        if matches!(idx_token.data, TokenData::Elem) {
                            continue;
                        }

                        if let TokenData::LitInt(id) = idx_token.data {
                            let (next_fm, exist) = cur_fm.int(id);
                            if !exist {
                                return Ok((None, false));
                            }
                            if next_fm.is_none() {
                                return Ok((Some(Cow::Borrowed(cur_fm)), true));
                            }
                            next_fm_for_loop = next_fm;
                        } else if let TokenData::Any = idx_token.data {
                            let (next_fm, exist) = cur_fm.wildcard();
                            if !exist {
                                return Ok((None, false));
                            }
                            if next_fm.is_none() {
                                return Ok((Some(Cow::Borrowed(cur_fm)), true));
                            }
                            next_fm_for_loop = next_fm;
                        } else {
                            return Err(FieldMaskError::InvalidToken {
                                token_type: FastStr::new(format!("{:?}", idx_token.data)),
                                expected: "integer index or '*'".into(),
                                path: Box::new(PathDetail {
                                    path: FastStr::new(path),
                                    position: idx_token.get_begin_pos(),
                                }),
                            });
                        }
                    }

                    cur_desc = element_desc.clone();
                    if let Some(next) = next_fm_for_loop {
                        cur_fm = next;
                    } else {
                        return Ok((Some(Cow::Borrowed(cur_fm)), true));
                    }
                }
                TokenData::MapL => {
                    let element_desc = cur_desc.value_type.as_deref().ok_or_else(|| {
                        FieldMaskError::DescriptorError {
                            type_name: cur_desc.name.clone(),
                            message: "map has no value type".into(),
                            path: Box::new(PathDetail {
                                path: FastStr::new(path),
                                position: token.get_begin_pos(),
                            }),
                        }
                    })?;

                    if !matches!(
                        cur_fm.data,
                        FieldMaskData::StrMap { .. } | FieldMaskData::IntMap { .. }
                    ) && !cur_fm.all()
                    {
                        return Err(FieldMaskError::TypeMismatch {
                            detail: Box::new(TypeMismatchDetail {
                                expected: "IntMap or StrMap".into(),
                                actual: FastStr::new(cur_fm.typ()),
                                context: "FieldMask type check for map".into(),
                            }),
                            path: Box::new(PathDetail {
                                path: FastStr::new(path),
                                position: token.get_begin_pos(),
                            }),
                        });
                    }

                    if cur_fm.all() {
                        // the path should be end here
                        return Ok((Some(Cow::Borrowed(cur_fm)), true));
                    }

                    let mut next_fm_for_loop = None;
                    let mut empty = true;
                    while it.has_next() {
                        let key_token = it.next();
                        if key_token.data == TokenData::MapR {
                            if empty {
                                return Err(FieldMaskError::EmptyCollection {
                                    collection_type: "key collection".into(),
                                    path: Box::new(PathDetail {
                                        path: FastStr::new(path),
                                        position: key_token.get_begin_pos(),
                                    }),
                                });
                            }
                            break;
                        }
                        empty = false;

                        if matches!(key_token.data, TokenData::Elem) {
                            continue;
                        }

                        match &key_token.data {
                            TokenData::LitInt(id) => {
                                let (next_fm, exist) = cur_fm.int(*id);
                                if !exist {
                                    return Ok((None, false));
                                }
                                if next_fm.is_none() {
                                    return Ok((Some(Cow::Borrowed(cur_fm)), true));
                                }
                                next_fm_for_loop = next_fm;
                            }
                            TokenData::Str(key) => {
                                let (next_fm, exist) = cur_fm.str(key);
                                if !exist {
                                    return Ok((None, false));
                                }
                                if next_fm.is_none() {
                                    return Ok((Some(Cow::Borrowed(cur_fm)), true));
                                }
                                next_fm_for_loop = next_fm;
                            }
                            TokenData::Any => {
                                let (next_fm, exist) = cur_fm.wildcard();
                                if !exist {
                                    return Ok((None, false));
                                }
                                if next_fm.is_none() {
                                    return Ok((Some(Cow::Borrowed(cur_fm)), true));
                                }
                                next_fm_for_loop = next_fm;
                            }
                            _ => {
                                return Err(FieldMaskError::InvalidToken {
                                    token_type: FastStr::new(format!("{:?}", key_token.data)),
                                    expected: "integer, string or '*' as key".into(),
                                    path: Box::new(PathDetail {
                                        path: FastStr::new(path),
                                        position: key_token.get_begin_pos(),
                                    }),
                                });
                            }
                        }
                    }

                    cur_desc = element_desc.clone();
                    if let Some(next) = next_fm_for_loop {
                        cur_fm = next;
                    } else {
                        return Ok((Some(Cow::Borrowed(cur_fm)), true));
                    }
                }
                _ => {
                    return Err(FieldMaskError::InvalidToken {
                        token_type: FastStr::new(format!("{:?}", token.data)),
                        expected: "$ or . or [ or {".into(),
                        path: Box::new(PathDetail {
                            path: FastStr::new(path),
                            position: token.get_begin_pos(),
                        }),
                    });
                }
            }
        }
        Ok((Some(Cow::Borrowed(cur_fm)), !it.has_next()))
    }

    pub fn path_in_mask(&self, desc: &TypeDescriptor, path: &str) -> Result<bool, FieldMaskError> {
        let (_, exist) = self.get_path(desc, path)?;
        Ok(exist)
    }
}

// Methods for mutation
impl FieldMask {
    #[inline]
    fn mut_wildcard(&mut self, data: FieldMaskData) -> &mut FieldMask {
        let is_black = self.is_black;
        let wildcard = match &mut self.data {
            FieldMaskData::List { wildcard, .. } => wildcard,
            FieldMaskData::StrMap { wildcard, .. } => wildcard,
            FieldMaskData::IntMap { wildcard, .. } => wildcard,
            other => panic!("Cannot set wildcard on {:?}", other.type_name()),
        };

        wildcard.get_or_insert_with(|| Box::new(FieldMask { is_black, data }))
    }

    #[inline]
    fn set_and_get_sub_field(&mut self, id: i32, data: FieldMaskData) -> &mut FieldMask {
        let is_black = self.is_black;
        match &mut self.data {
            FieldMaskData::Struct { children, .. } => children
                .entry(id)
                .or_insert_with(|| Box::new(FieldMask { is_black, data })),
            other => panic!("Cannot set field_id on {:?}", other.type_name()),
        }
    }

    #[inline]
    fn set_int_key_mask(&mut self, id: i32, data: FieldMaskData) {
        let is_black = self.is_black;
        match &mut self.data {
            FieldMaskData::List { children, .. } | FieldMaskData::IntMap { children, .. } => {
                children
                    .entry(id)
                    .or_insert_with(|| Box::new(FieldMask { is_black, data }));
            }
            other => panic!("Cannot set int on {:?}", other.type_name()),
        }
    }

    #[inline]
    fn set_str_key_mask(&mut self, id: FastStr, data: FieldMaskData) {
        let is_black = self.is_black;
        match &mut self.data {
            FieldMaskData::StrMap { children, .. } => {
                children
                    .entry(id)
                    .or_insert_with(|| Box::new(FieldMask { is_black, data }));
            }
            other => panic!("Cannot set str on {:?}", other.type_name()),
        }
    }
}

// Methods for path processing
impl FieldMask {
    #[inline]
    fn init(&mut self, desc: &TypeDescriptor, paths: &[FastStr]) -> Result<(), FieldMaskError> {
        for path in paths {
            let mut it = PathIterator::new(path).map_err(|err| FieldMaskError::PathError {
                path: path.clone(),
                source: Box::new(err),
            })?;
            self.add_path(&mut it, desc, path)?;
        }
        Ok(())
    }

    #[inline]
    fn add_path(
        &mut self,
        it: &mut PathIterator,
        cur_desc: &TypeDescriptor,
        original_path: &FastStr,
    ) -> Result<(), FieldMaskError> {
        // if no more tokens for nested path, set as all
        if !it.has_next() {
            self.data.set_all()?;
            return Ok(());
        }

        let token = it.next();
        match &token.data {
            TokenData::Root => {
                if let FieldMaskData::Invalid = self.data {
                    self.data = FieldMaskData::new(cur_desc);
                }
                self.add_path(it, cur_desc, original_path)
            }
            TokenData::Field => self.add_field_path(it, &token, cur_desc, original_path),
            TokenData::IndexL => self.add_list_path(it, &token, cur_desc, original_path),
            TokenData::MapL => self.add_map_path(it, &token, cur_desc, original_path),
            _ => Err(FieldMaskError::InvalidToken {
                token_type: FastStr::new(format!("{:?}", token.data)),
                expected: FastStr::new("$ or . or [ or {"),
                path: Box::new(PathDetail {
                    path: original_path.clone(),
                    position: token.get_begin_pos(),
                }),
            }),
        }
    }

    fn add_field_path(
        &mut self,
        it: &mut PathIterator,
        token: &PathToken,
        cur_desc: &TypeDescriptor,
        original_path: &FastStr,
    ) -> Result<(), FieldMaskError> {
        let s = match cur_desc.get_struct_desc() {
            Some(s) => s,
            None => {
                return Err(FieldMaskError::TypeMismatch {
                    detail: Box::new(TypeMismatchDetail {
                        expected: "Struct".into(),
                        actual: cur_desc.name.clone(),
                        context: "descriptor type check for field".into(),
                    }),
                    path: Box::new(PathDetail {
                        path: original_path.clone(),
                        position: token.get_begin_pos(),
                    }),
                });
            }
        };

        if !matches!(self.data, FieldMaskData::Struct { .. }) {
            return Err(FieldMaskError::TypeMismatch {
                detail: Box::new(TypeMismatchDetail {
                    expected: "Struct".into(),
                    actual: FastStr::new(self.data.type_name()),
                    context: "FieldMask type check for field".into(),
                }),
                path: Box::new(PathDetail {
                    path: original_path.clone(),
                    position: token.get_begin_pos(),
                }),
            });
        }

        let field_token = it.next();
        if field_token.data == TokenData::EOF {
            return Err(FieldMaskError::InvalidToken {
                token_type: FastStr::new("EOF"),
                expected: FastStr::new("field name, field id or '*'"),
                path: Box::new(PathDetail {
                    path: original_path.clone(),
                    position: field_token.get_begin_pos(),
                }),
            });
        }

        match &field_token.data {
            TokenData::LitInt(id) => {
                let field =
                    s.find_field_by_id(*id)
                        .ok_or_else(|| FieldMaskError::FieldNotFound {
                            field_identifier: FastStr::new(id.to_string()),
                            parent_type: cur_desc.name.clone(),
                            path: Box::new(PathDetail {
                                path: original_path.clone(),
                                position: field_token.get_begin_pos(),
                            }),
                        })?;
                let sub_mask =
                    self.set_and_get_sub_field(field.id, FieldMaskData::new(&field.r#type));
                sub_mask.add_path(it, &field.r#type, original_path)
            }
            TokenData::LitStr(name) => {
                let field =
                    s.find_field_by_name(name)
                        .ok_or_else(|| FieldMaskError::FieldNotFound {
                            field_identifier: name.clone(),
                            parent_type: cur_desc.name.clone(),
                            path: Box::new(PathDetail {
                                path: original_path.clone(),
                                position: field_token.get_begin_pos(),
                            }),
                        })?;
                let sub_mask =
                    self.set_and_get_sub_field(field.id, FieldMaskData::new(&field.r#type));
                sub_mask.add_path(it, &field.r#type, original_path)
            }
            TokenData::Any => {
                self.data.set_all()?;
                Ok(())
            }
            _ => Err(FieldMaskError::InvalidToken {
                token_type: FastStr::new(format!("{:?}", field_token.data)),
                expected: FastStr::new("field name, field id or '*'"),
                path: Box::new(PathDetail {
                    path: original_path.clone(),
                    position: field_token.get_begin_pos(),
                }),
            }),
        }
    }

    fn add_list_path(
        &mut self,
        it: &mut PathIterator,
        token: &PathToken,
        cur_desc: &TypeDescriptor,
        original_path: &FastStr,
    ) -> Result<(), FieldMaskError> {
        if !matches!(self.data, FieldMaskData::List { .. }) {
            return Err(FieldMaskError::TypeMismatch {
                detail: Box::new(TypeMismatchDetail {
                    expected: "List".into(),
                    actual: FastStr::new(self.data.type_name()),
                    context: "FieldMask type check for list".into(),
                }),
                path: Box::new(PathDetail {
                    path: original_path.clone(),
                    position: token.get_begin_pos(),
                }),
            });
        }

        let element_desc =
            cur_desc
                .value_type
                .as_ref()
                .ok_or_else(|| FieldMaskError::DescriptorError {
                    type_name: cur_desc.name.clone(),
                    message: FastStr::new("collection has no value type"),
                    path: Box::new(PathDetail {
                        path: original_path.clone(),
                        position: token.get_begin_pos(),
                    }),
                })?;

        let mut ids = Vec::new();
        let mut is_wildcard = false;
        let mut empty = true;

        while it.has_next() {
            let idx_token = it.next();
            if idx_token.data == TokenData::IndexR {
                if empty {
                    return Err(FieldMaskError::EmptyCollection {
                        collection_type: FastStr::new("index collection"),
                        path: Box::new(PathDetail {
                            path: original_path.clone(),
                            position: idx_token.get_begin_pos(),
                        }),
                    });
                }
                break;
            }
            empty = false;

            if matches!(idx_token.data, TokenData::Elem) {
                continue;
            }

            if matches!(idx_token.data, TokenData::Any) {
                is_wildcard = true;
                continue;
            }

            if let TokenData::LitInt(id) = idx_token.data {
                ids.push(id);
            } else {
                return Err(FieldMaskError::InvalidToken {
                    token_type: FastStr::new(format!("{:?}", idx_token.data)),
                    expected: FastStr::new("integer index or '*'"),
                    path: Box::new(PathDetail {
                        path: original_path.clone(),
                        position: idx_token.get_begin_pos(),
                    }),
                });
            }
        }

        if is_wildcard {
            let sub_mask = self.mut_wildcard(FieldMaskData::new(element_desc));
            return sub_mask.add_path(it, element_desc, original_path);
        }

        if !it.has_next() {
            for id in ids {
                self.set_int_key_mask(id, FieldMaskData::Scalar);
            }
        } else {
            let mut sub_mask = FieldMask {
                is_black: self.is_black,
                data: FieldMaskData::new(element_desc),
            };
            sub_mask.add_path(it, element_desc, original_path)?;
            for id in ids {
                self.set_int_key_mask(id, sub_mask.data.clone());
            }
        }
        Ok(())
    }

    fn add_map_path(
        &mut self,
        it: &mut PathIterator,
        token: &PathToken,
        cur_desc: &TypeDescriptor,
        original_path: &FastStr,
    ) -> Result<(), FieldMaskError> {
        let is_str_map = matches!(self.data, FieldMaskData::StrMap { .. });
        let is_int_map = matches!(self.data, FieldMaskData::IntMap { .. });

        if !is_str_map && !is_int_map {
            return Err(FieldMaskError::TypeMismatch {
                detail: Box::new(TypeMismatchDetail {
                    expected: "IntMap or StrMap".into(),
                    actual: FastStr::new(self.data.type_name()),
                    context: "FieldMask type check for map".into(),
                }),
                path: Box::new(PathDetail {
                    path: original_path.clone(),
                    position: token.get_begin_pos(),
                }),
            });
        }

        let element_desc =
            cur_desc
                .value_type
                .as_ref()
                .ok_or_else(|| FieldMaskError::DescriptorError {
                    type_name: cur_desc.name.clone(),
                    message: FastStr::new("map has no value type"),
                    path: Box::new(PathDetail {
                        path: original_path.clone(),
                        position: token.get_begin_pos(),
                    }),
                })?;

        let mut int_keys = Vec::new();
        let mut str_keys = Vec::new();
        let mut is_wildcard = false;
        let mut empty = true;

        while it.has_next() {
            let key_token = it.next();
            if key_token.data == TokenData::MapR {
                if empty {
                    return Err(FieldMaskError::EmptyCollection {
                        collection_type: FastStr::new("key collection"),
                        path: Box::new(PathDetail {
                            path: original_path.clone(),
                            position: key_token.get_begin_pos(),
                        }),
                    });
                }
                break;
            }
            empty = false;

            if matches!(key_token.data, TokenData::Elem) {
                continue;
            }

            if matches!(key_token.data, TokenData::Any) {
                is_wildcard = true;
                continue;
            }

            match &key_token.data {
                TokenData::LitInt(id) => {
                    if !is_int_map {
                        return Err(FieldMaskError::TypeMismatch {
                            detail: Box::new(TypeMismatchDetail {
                                expected: "string key".into(),
                                actual: "integer key".into(),
                                context: "map key type check".into(),
                            }),
                            path: Box::new(PathDetail {
                                path: original_path.clone(),
                                position: key_token.get_begin_pos(),
                            }),
                        });
                    }
                    int_keys.push(*id);
                }
                TokenData::Str(key) => {
                    if !is_str_map {
                        return Err(FieldMaskError::TypeMismatch {
                            detail: Box::new(TypeMismatchDetail {
                                expected: "integer key".into(),
                                actual: "string key".into(),
                                context: "map key type check".into(),
                            }),
                            path: Box::new(PathDetail {
                                path: original_path.clone(),
                                position: key_token.get_begin_pos(),
                            }),
                        });
                    }
                    str_keys.push(key.clone());
                }
                _ => {
                    return Err(FieldMaskError::InvalidToken {
                        token_type: FastStr::new(format!("{:?}", key_token.data)),
                        expected: FastStr::new("integer, string or '*' as key"),
                        path: Box::new(PathDetail {
                            path: original_path.clone(),
                            position: key_token.get_begin_pos(),
                        }),
                    });
                }
            }
        }

        if is_wildcard {
            let sub_mask = self.mut_wildcard(FieldMaskData::new(element_desc));
            return sub_mask.add_path(it, element_desc, original_path);
        }

        if !it.has_next() {
            if is_int_map {
                for id in int_keys {
                    self.set_int_key_mask(id, FieldMaskData::Scalar);
                }
            } else {
                for key in str_keys {
                    self.set_str_key_mask(FastStr::new(key), FieldMaskData::Scalar);
                }
            }
        } else {
            let mut sub_mask = FieldMask {
                is_black: self.is_black,
                data: FieldMaskData::new(element_desc),
            };
            sub_mask.add_path(it, element_desc, original_path)?;

            if is_int_map {
                for id in int_keys {
                    self.set_int_key_mask(id, sub_mask.data.clone());
                }
            } else {
                for key in str_keys {
                    self.set_str_key_mask(FastStr::new(key), sub_mask.data.clone());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use chumsky::prelude::*;

    use super::*;

    #[test]
    fn test_field_mask_type_display() {
        let struct_data = FieldMaskData::Struct {
            children: AHashMap::new(),
            is_all: false,
        };
        let list_data = FieldMaskData::List {
            children: AHashMap::new(),
            wildcard: None,
            is_all: false,
        };

        assert_eq!(FieldMaskData::Scalar.type_name(), "Scalar");
        assert_eq!(list_data.type_name(), "List");
        assert_eq!(struct_data.type_name(), "Struct");
    }

    #[test]
    fn test_field_mask_default() {
        let fm = FieldMask::default();
        assert!(!fm.exist());
        assert_eq!(fm.typ(), "Invalid");
        assert!(!fm.is_black());
    }

    #[test]
    fn test_for_each_child() {
        let mut fm = FieldMask::default();

        let mut children = AHashMap::new();
        let mut child1 = FieldMask::default();
        child1.data = FieldMaskData::Scalar;
        let mut child2 = FieldMask::default();
        child2.data = FieldMaskData::List {
            children: AHashMap::new(),
            wildcard: None,
            is_all: false,
        };
        children.insert(1, Box::new(child1));
        children.insert(2, Box::new(child2));
        fm.data = FieldMaskData::Struct {
            children,
            is_all: false,
        };
        let mut count = 0;
        fm.for_each_child(|_, id, mask| {
            match id {
                1 => {
                    assert_eq!(mask.typ(), "Scalar");
                    assert!(mask.exist());
                }
                2 => {
                    assert_eq!(mask.typ(), "List");
                    assert!(mask.exist());
                }
                _ => {
                    assert!(false);
                }
            }
            count += 1;
            true
        });
        assert_eq!(count, 2);
        assert_eq!(fm.field(1).0.unwrap().typ(), "Scalar");
        assert_eq!(fm.field(2).0.unwrap().typ(), "List");
        assert!(fm.field(3).0.is_none());
    }

    #[test]
    fn test_field_mask_error_display() {
        let err = FieldMaskError::PathError {
            path: FastStr::new("$.invalid"),
            source: Box::new(PathError::SyntaxError { position: 5 }),
        };
        assert!(err.to_string().contains("path '$.invalid', parse error"));

        let err = FieldMaskError::TypeMismatch {
            detail: Box::new(TypeMismatchDetail {
                expected: "Struct".into(),
                actual: "List".into(),
                context: "field access".into(),
            }),
            path: Box::new(PathDetail {
                path: FastStr::new("$.test"),
                position: 0,
            }),
        };
        assert!(err.to_string().contains("type mismatch"));
    }

    #[test]
    fn test_field_mask_builder() {
        let desc = TypeDescriptor {
            name: "UserStruct".into(),
            ..Default::default()
        };

        let paths = vec![FastStr::new("$.test")];
        let builder = FieldMaskBuilder::new(&desc, &paths);

        assert_eq!(builder.paths.len(), 1);
        assert_eq!(builder.desc.name, "UserStruct");
    }

    #[test]
    fn test_thiserror_integration() {
        let path_error = PathError::InvalidCharacter {
            position: 5,
            character: '@',
        };
        assert!(path_error.to_string().contains("invalid character"));

        let fieldmask_error = FieldMaskError::PathError {
            path: FastStr::new("$.invalid"),
            source: Box::new(path_error.clone()),
        };

        assert!(
            fieldmask_error
                .to_string()
                .contains("path '$.invalid', parse error")
        );

        use std::error::Error;
        assert!(fieldmask_error.source().is_some());
        let source = fieldmask_error.source().unwrap();
        assert_eq!(source.to_string(), path_error.to_string());
    }

    #[test]
    fn test_get_path() {
        let content = std::fs::read_to_string("../examples/idl/fieldmask.thrift").unwrap();
        let mut ast = pilota_thrift_parser::descriptor::File::parse()
            .parse(&content)
            .unwrap();
        ast.path = Arc::from(
            PathBuf::from("../examples/idl/fieldmask.thrift")
                .canonicalize()
                .unwrap(),
        );
        let desc: pilota_thrift_reflect::thrift_reflection::FileDescriptor = (&ast).into();
        let key = FastStr::new(ast.path.to_string_lossy());
        pilota_thrift_reflect::service::Register::register(key, desc.clone());

        let content = std::fs::read_to_string("../examples/idl/base.thrift").unwrap();
        let mut ast = pilota_thrift_parser::descriptor::File::parse()
            .parse(&content)
            .unwrap();
        ast.path = Arc::from(
            PathBuf::from("../examples/idl/base.thrift")
                .canonicalize()
                .unwrap(),
        );
        let base_desc: pilota_thrift_reflect::thrift_reflection::FileDescriptor = (&ast).into();
        let key = FastStr::new(ast.path.to_string_lossy());
        pilota_thrift_reflect::service::Register::register(key, base_desc.clone());

        println!("{:?}", desc);

        let paths = &[
            "$.f1",
            "$.f9[1, 3]",
            "$.f11.b",
            "$.f12[0][*]",
            "$.f14{*}",
            "$.f15{ \"key1\",\"key3\"}",
            "$.f16{\"key1\"}[1].a",
            "$.f17[*]{\"key1\"}",
            "$.base.Addr",
            "$.base.EnumMap{1, 2}",
        ];
        let fm = FieldMaskBuilder::new(
            &desc
                .find_struct_by_name("Request")
                .unwrap()
                .type_descriptor(),
            paths,
        )
        .build()
        .unwrap();
        let req_desc = desc.find_struct_by_name("Request").unwrap();

        let (sub_fm, exist) = fm.get_path(&req_desc.type_descriptor(), "$.f1").unwrap();
        assert!(exist);
        assert!(sub_fm.unwrap().all());

        let (sub_fm, exist) = fm.get_path(&req_desc.type_descriptor(), "$.f9[1]").unwrap();
        assert!(exist);
        assert!(sub_fm.unwrap().all());

        let (sub_fm, exist) = fm.get_path(&req_desc.type_descriptor(), "$.f11.b").unwrap();
        assert!(exist);
        assert!(sub_fm.unwrap().all());

        let (sub_fm, exist) = fm
            .get_path(&req_desc.type_descriptor(), "$.f12[0][1]")
            .unwrap();
        assert!(exist);
        assert!(sub_fm.unwrap().all());

        let (sub_fm, exist) = fm
            .get_path(&req_desc.type_descriptor(), "$.f14{0, 1}")
            .unwrap();
        assert!(exist);
        assert!(sub_fm.unwrap().all());

        let (sub_fm, exist) = fm
            .get_path(&req_desc.type_descriptor(), "$.f15{ \"key1\",\"key2\"}")
            .unwrap();
        assert!(!exist);
        assert!(sub_fm.is_none());

        let (sub_fm, exist) = fm
            .get_path(&req_desc.type_descriptor(), "$.f16{\"key1\"}[1]")
            .unwrap();
        assert!(exist);
        assert!(!sub_fm.unwrap().all());

        let (sub_fm, exist) = fm
            .get_path(&req_desc.type_descriptor(), "$.f17[*]{\"key1\"}")
            .unwrap();
        assert!(exist);
        assert!(sub_fm.unwrap().all());

        let (sub_fm, exist) = fm
            .get_path(&req_desc.type_descriptor(), "$.base.LogID")
            .unwrap();
        assert!(!exist);
        assert!(sub_fm.is_none());

        let (sub_fm, exist) = fm
            .get_path(&req_desc.type_descriptor(), "$.base.EnumMap{1}")
            .unwrap();
        assert!(exist);
        assert!(sub_fm.unwrap().all());
    }
}
