use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

use faststr::FastStr;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Default)]
pub struct Adjust {
    boxed: bool,
    attrs: Vec<FastStr>,
    pub(crate) nested_items: Vec<FastStr>,
    named_items: FxHashSet<FastStr>,
    codegen_data: FxHashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Debug for Adjust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Adjust")
            .field("boxed", &self.boxed)
            .field("attrs", &self.attrs)
            .field("impls", &self.nested_items)
            .field("named_items", &self.named_items)
            .field("codegen_data_types", &self.codegen_data.keys())
            .finish()
    }
}

impl Adjust {
    /// Share typed backend settings with attached generators on this
    /// definition. This data is local to one compilation and is never
    /// emitted as source.
    pub fn set_codegen_data<T: Any + Send + Sync>(&mut self, value: T) {
        self.codegen_data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn codegen_data<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.codegen_data
            .get(&TypeId::of::<T>())
            .and_then(|value| value.downcast_ref())
    }

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
    pub fn add_nested_item(&mut self, r#impl: FastStr) {
        self.nested_items.push(r#impl)
    }

    /// Reserves a generated item's name on this definition before adding its
    /// source. Plugins using the same name receive a diagnostic instead of
    /// duplicate output.
    pub fn add_named_nested_item(&mut self, name: FastStr, item: FastStr) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.named_items.insert(name.clone()),
            "duplicate generated item `{name}`"
        );
        self.nested_items.push(item);
        Ok(())
    }
}
