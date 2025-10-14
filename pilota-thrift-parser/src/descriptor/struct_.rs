use std::ops::{Deref, DerefMut};

use faststr::FastStr;

use super::{Annotations, Field, Ident};

#[derive(Debug)]
pub struct Struct {
    pub struct_like: StructLike,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}

macro_rules! struct_like {
    ($name: ty) => {
        impl Deref for $name {
            type Target = StructLike;
            fn deref(&self) -> &StructLike {
                &self.struct_like
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.struct_like
            }
        }
    };
}

struct_like!(Struct);

#[derive(Debug)]
pub struct Union {
    pub struct_like: StructLike,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}

struct_like!(Union);

#[derive(Debug)]
pub struct Exception {
    pub struct_like: StructLike,
    pub leading_comments: FastStr,
    pub trailing_comments: FastStr,
}

struct_like!(Exception);

#[derive(Debug)]
pub struct StructLike {
    pub name: Ident,
    pub fields: Vec<Field>,
    pub annotations: Annotations,
    pub comments: FastStr,
}
