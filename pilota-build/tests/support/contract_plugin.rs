//! Test plugins attach service aliases and one field trait impl per message.
use std::sync::Arc;

use pilota_build::{db::RirDatabase, plugin::walk_item, rir::Item};

#[derive(Clone)]
pub struct ContractPlugin;

impl pilota_build::Plugin for ContractPlugin {
    fn on_item(&mut self, cx: &pilota_build::Context, id: pilota_build::DefId, item: Arc<Item>) {
        if let Item::Service(_) = &*item {
            let module = format!("{}_contract", cx.rust_name(id).to_ascii_lowercase());
            let path = |target| {
                let value = cx.related_item_path(id, target);
                if value.starts_with("::") || value.starts_with("crate::") {
                    value
                } else {
                    format!("super::{}", value.strip_prefix("self::").unwrap_or(&value)).into()
                }
            };
            let mut aliases = String::new();
            for method in cx.service_methods(id).iter() {
                let name = method.name.raw_str();
                for (index, arg) in method.args.iter().enumerate() {
                    let ty = cx.codegen_item_ty(arg.ty.kind.clone()).qualified_path(path);
                    aliases.push_str(&format!("pub type {name}Arg{index} = {ty};"));
                }
                let ty = cx
                    .codegen_item_ty(method.ret.kind.clone())
                    .qualified_path(path);
                aliases.push_str(&format!("pub type {name}Return = {ty};"));
                if let Some(exception) = &method.exceptions {
                    aliases.push_str(&format!(
                        "pub type {name}Exception = {};",
                        path(exception.did)
                    ));
                    let variants = cx.item(exception.did).unwrap();
                    let Item::Enum(variants) = &*variants else {
                        panic!("expected exception enum")
                    };
                    for variant in &variants.variants {
                        let ty = cx
                            .codegen_item_ty(variant.fields[0].kind.clone())
                            .qualified_path(path);
                        aliases.push_str(&format!(
                            "pub type {name}{} = {ty};",
                            cx.rust_name(variant.did)
                        ));
                    }
                }
            }
            cx.with_adjust_mut(id, |adjust| {
                adjust.add_nested_item(format!("pub mod {module} {{ {aliases} }}").into())
            });
        }
        walk_item(self, cx, id, item);
    }
}

// Independent field plugin: it owns the one impl on each original type.
// The service plugin never generates or duplicates this impl.
#[derive(Clone)]
pub struct NumberPlugin;

impl pilota_build::Plugin for NumberPlugin {
    fn on_item(&mut self, cx: &pilota_build::Context, id: pilota_build::DefId, item: Arc<Item>) {
        if let Item::Message(message) = &*item {
            if let Some(field) = message.fields.iter().find(|f| f.name.raw_str() == "number") {
                let name = cx.rust_name(id);
                let field = cx.rust_name(field.did);
                cx.with_adjust_mut(id, |adjust| adjust.add_nested_item(format!(
                    "impl ::std::convert::AsRef<i32> for {name} {{ fn as_ref(&self) -> &i32 {{ &self.{field} }} }}"
                ).into()));
            }
        }
        walk_item(self, cx, id, item);
    }
}
