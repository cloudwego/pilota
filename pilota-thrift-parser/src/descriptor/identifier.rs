use std::{
    ops::{Add, Deref},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct Ident(pub Arc<str>);

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        &&*self.0 == other
    }
}

impl Add<&str> for Ident {
    type Output = String;

    fn add(self, rhs: &str) -> Self::Output {
        let mut s = String::with_capacity(self.0.len() + rhs.len());
        s.push_str(&self.0);

        s.push_str(rhs);
        s
    }
}

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T: Into<Arc<str>>> From<T> for Ident {
    fn from(s: T) -> Self {
        Ident(s.into())
    }
}

impl Deref for Ident {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
