use std::ops::{Deref, DerefMut};

use super::{Annotations, Field, Ident};

#[derive(Debug)]
pub struct Struct(pub StructLike);

macro_rules! struct_like {
    ($name: ty) => {
        impl Deref for $name {
            type Target = StructLike;
            fn deref(&self) -> &StructLike {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

struct_like!(Struct);

#[derive(Debug)]
pub struct Union(pub StructLike);

struct_like!(Union);

#[derive(Debug)]
pub struct Exception(pub StructLike);

struct_like!(Exception);

#[derive(Debug)]
pub struct StructLike {
    pub name: Ident,
    pub fields: Vec<Field>,
    pub annotations: Annotations,
}
