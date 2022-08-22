use std::sync::Arc;

use super::File;

pub trait Visitor: Sized {
    fn visit_item(&mut self, item: Arc<super::Item>) {
        walk_item(self, item)
    }

    fn visit_file(&mut self, file: Arc<File>) {
        walk_file(self, file)
    }
}

pub fn walk_file<V: Visitor>(v: &mut V, file: Arc<File>) {
    file.items
        .iter()
        .cloned()
        .for_each(|item| v.visit_item(item))
}

pub fn walk_item<V: Visitor>(v: &mut V, item: Arc<super::Item>) {
    if let super::ItemKind::Mod(m) = &item.kind {
        m.items.iter().for_each(|item| v.visit_item(item.clone()));
    }
}
