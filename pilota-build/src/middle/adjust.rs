#[derive(Default)]
pub struct Adjust {
    boxed: bool,
    attrs: Vec<syn::Attribute>,
    lifetimes: Vec<syn::Lifetime>,
}

impl Adjust {
    pub fn set_boxed(&mut self) {
        self.boxed = true
    }

    pub fn boxed(&self) -> bool {
        self.boxed
    }

    pub fn attrs(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }

    pub fn add_attrs(&mut self, attrs: &[syn::Attribute]) {
        self.attrs.extend_from_slice(attrs)
    }

    pub fn add_lifetime(&mut self, lifetime: syn::Lifetime) {
        self.lifetimes.push(lifetime)
    }
}
