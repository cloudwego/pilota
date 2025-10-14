mod annotation;
mod constant;
mod enum_;
mod field;
mod function;
mod identifier;
mod include;
mod literal;
mod namespace;
mod service;
mod struct_;
mod ty;
mod typedef;

use std::{hash::Hash, path::PathBuf, sync::Arc};

pub use annotation::{Annotation, Annotations};
use bytes::Bytes;
pub use constant::{ConstValue, Constant, DoubleConstant, IntConstant};
pub use enum_::{Enum, EnumValue};
use faststr::FastStr;
pub use field::{Attribute, Field};
pub use function::Function;
pub use identifier::Ident;
pub use include::{CppInclude, Include};
pub use literal::Literal;
pub use namespace::{Namespace, Scope};
pub use service::Service;
pub use struct_::{Exception, Struct, StructLike, Union};
pub use ty::{CppType, Ty, Type};
pub use typedef::Typedef;

pub struct Components {}

#[derive(Debug, Clone)]
pub struct Path {
    pub segments: Arc<[Ident]>,
}

impl<Item> FromIterator<Item> for Path
where
    Item: Into<Ident>,
{
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        Path {
            segments: iter.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug)]
pub enum Item {
    Include(Include),
    CppInclude(CppInclude),
    Namespace(Namespace),
    Typedef(Typedef),
    Constant(Constant),
    Enum(Enum),
    Struct(Struct),
    Union(Union),
    Exception(Exception),
    Service(Service),
}

macro_rules! item_from {
    ($t: tt) => {
        impl From<$t> for Item {
            fn from(i: $t) -> Self {
                Item::$t(i)
            }
        }
    };
}

item_from!(Typedef);
item_from!(Constant);
item_from!(Enum);
item_from!(Struct);
item_from!(Union);
item_from!(Exception);
item_from!(Service);

#[derive(Default, Debug)]
pub struct File {
    pub path: Arc<PathBuf>,
    pub uuid: FastStr,
    pub package: Option<Path>,
    pub items: Vec<Item>,
    pub descriptor: Bytes,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for File {}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

impl Hash for File {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
