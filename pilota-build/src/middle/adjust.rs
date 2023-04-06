use std::fmt::Debug;

use faststr::FastStr;

#[derive(Default)]
pub struct Adjust {
    boxed: bool,
    attrs: Vec<FastStr>,
    pub(crate) impls: Vec<FastStr>,
}

impl Debug for Adjust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Adjust")
            .field("boxed", &self.boxed)
            .field("impls", &self.impls)
            .finish()
    }
}

impl Adjust {
    #[inline]
    pub fn set_boxed(&mut self) {
        self.boxed = true
    }

    #[inline]
    pub fn boxed(&self) -> bool {
        self.boxed
    }

    #[inline]
    pub fn attrs(&self) -> &Vec<FastStr> {
        &self.attrs
    }

    #[inline]
    pub fn add_attrs(&mut self, attrs: &[FastStr]) {
        self.attrs.extend_from_slice(attrs)
    }

    #[inline]
    pub fn add_impl(&mut self, r#impl: FastStr) {
        self.impls.push(r#impl)
    }
}
