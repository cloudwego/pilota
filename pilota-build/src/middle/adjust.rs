#[derive(Default)]
pub struct Adjust {
    boxed: bool,
    attrs: Vec<syn::Attribute>,
    lifetimes: Vec<syn::Lifetime>,
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
    pub fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }

    #[inline]
    pub fn add_attrs(&mut self, attrs: &[syn::Attribute]) {
        self.attrs.extend_from_slice(attrs)
    }

    #[inline]
    pub fn add_lifetime(&mut self, lifetime: syn::Lifetime) {
        self.lifetimes.push(lifetime)
    }
}
