//! Visitor pattern for HIR traversal.

use crate::ast::*;
use crate::Item;

/// Trait for visiting HIR nodes.
pub trait Visitor<'hir>: Sized {
    /// Visit an item.
    fn visit_item(&mut self, item: &'hir Item) {
        walk_item(self, item);
    }

    /// Visit a message.
    fn visit_message(&mut self, message: &'hir Message) {
        walk_message(self, message);
    }

    /// Visit a service.
    fn visit_service(&mut self, service: &'hir Service) {
        walk_service(self, service);
    }

    /// Visit an enum.
    fn visit_enum(&mut self, enum_: &'hir Enum) {
        walk_enum(self, enum_);
    }

    /// Visit a type alias.
    fn visit_type_alias(&mut self, alias: &'hir TypeAlias) {
        walk_type_alias(self, alias);
    }

    /// Visit a constant.
    fn visit_const(&mut self, const_: &'hir Const) {
        walk_const(self, const_);
    }

    /// Visit a module.
    fn visit_module(&mut self, module: &'hir Module) {
        walk_module(self, module);
    }

    /// Visit a field.
    fn visit_field(&mut self, field: &'hir Field) {
        walk_field(self, field);
    }

    /// Visit a method.
    fn visit_method(&mut self, method: &'hir Method) {
        walk_method(self, method);
    }

    /// Visit a type.
    fn visit_type(&mut self, ty: &'hir Type) {
        walk_type(self, ty);
    }

    /// Visit an expression.
    fn visit_expr(&mut self, expr: &'hir Expr) {
        walk_expr(self, expr);
    }

    /// Visit a path.
    fn visit_path(&mut self, path: &'hir Path) {
        walk_path(self, path);
    }
}

/// Walk an item.
pub fn walk_item<'hir, V: Visitor<'hir>>(visitor: &mut V, item: &'hir Item) {
    match &item.kind {
        ItemKind::Message(message) => visitor.visit_message(message),
        ItemKind::Service(service) => visitor.visit_service(service),
        ItemKind::Enum(enum_) => visitor.visit_enum(enum_),
        ItemKind::TypeAlias(alias) => visitor.visit_type_alias(alias),
        ItemKind::Const(const_) => visitor.visit_const(const_),
        ItemKind::Module(module) => visitor.visit_module(module),
    }
}

/// Walk a message.
pub fn walk_message<'hir, V: Visitor<'hir>>(visitor: &mut V, message: &'hir Message) {
    for field in &message.fields {
        visitor.visit_field(field);
    }
}

/// Walk a service.
pub fn walk_service<'hir, V: Visitor<'hir>>(visitor: &mut V, service: &'hir Service) {
    if let Some(extends) = &service.extends {
        visitor.visit_path(extends);
    }
    for method in &service.methods {
        visitor.visit_method(method);
    }
}

/// Walk an enum.
pub fn walk_enum<'hir, V: Visitor<'hir>>(_visitor: &mut V, _enum: &'hir Enum) {
    // Enum variants don't need deep traversal
}

/// Walk a type alias.
pub fn walk_type_alias<'hir, V: Visitor<'hir>>(visitor: &mut V, alias: &'hir TypeAlias) {
    visitor.visit_type(&alias.ty);
}

/// Walk a constant.
pub fn walk_const<'hir, V: Visitor<'hir>>(visitor: &mut V, const_: &'hir Const) {
    visitor.visit_type(&const_.ty);
    visitor.visit_expr(&const_.value);
}

/// Walk a module.
pub fn walk_module<'hir, V: Visitor<'hir>>(visitor: &mut V, module: &'hir Module) {
    for item in &module.items {
        visitor.visit_item(item);
    }
}

/// Walk a field.
pub fn walk_field<'hir, V: Visitor<'hir>>(visitor: &mut V, field: &'hir Field) {
    visitor.visit_type(&field.ty);
    if let Some(default) = &field.default {
        visitor.visit_expr(default);
    }
}

/// Walk a method.
pub fn walk_method<'hir, V: Visitor<'hir>>(visitor: &mut V, method: &'hir Method) {
    for param in &method.params {
        visitor.visit_field(param);
    }
    if let Some(result) = &method.result {
        visitor.visit_type(result);
    }
    for exception in &method.exceptions {
        visitor.visit_field(exception);
    }
}

/// Walk a type.
pub fn walk_type<'hir, V: Visitor<'hir>>(visitor: &mut V, ty: &'hir Type) {
    match ty {
        Type::Path(path) => visitor.visit_path(path),
        Type::Vec(inner) => visitor.visit_type(inner),
        Type::Set(inner) => visitor.visit_type(inner),
        Type::Map { key, value } => {
            visitor.visit_type(key);
            visitor.visit_type(value);
        }
        Type::Optional(inner) => visitor.visit_type(inner),
        Type::Primitive(_) => {}
    }
}

/// Walk an expression.
pub fn walk_expr<'hir, V: Visitor<'hir>>(visitor: &mut V, expr: &'hir Expr) {
    match expr {
        Expr::Path(path) => visitor.visit_path(path),
        Expr::List(exprs) => {
            for expr in exprs {
                visitor.visit_expr(expr);
            }
        }
        Expr::Map(pairs) => {
            for (key, value) in pairs {
                visitor.visit_expr(key);
                visitor.visit_expr(value);
            }
        }
        Expr::Literal(_) => {}
    }
}

/// Walk a path.
pub fn walk_path<'hir, V: Visitor<'hir>>(_visitor: &mut V, _path: &'hir Path) {
    // Path segments don't need deep traversal for now
}