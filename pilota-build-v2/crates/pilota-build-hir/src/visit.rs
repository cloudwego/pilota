//! Visitor pattern for traversing HIR.

use crate::*;

/// Trait for visiting HIR nodes.
pub trait Visitor: Sized {
    /// Visit a crate.
    fn visit_crate(&mut self, krate: &HirCrate) {
        walk_crate(self, krate);
    }

    /// Visit an item.
    fn visit_item(&mut self, item: &Item) {
        walk_item(self, item);
    }

    /// Visit a service.
    fn visit_service(&mut self, service: &Service) {
        walk_service(self, service);
    }

    /// Visit a message.
    fn visit_message(&mut self, message: &Message) {
        walk_message(self, message);
    }

    /// Visit an enum.
    fn visit_enum(&mut self, enum_: &Enum) {
        walk_enum(self, enum_);
    }

    /// Visit a constant.
    fn visit_const(&mut self, const_: &Const) {
        walk_const(self, const_);
    }

    /// Visit a type alias.
    fn visit_type_alias(&mut self, type_alias: &TypeAlias) {
        walk_type_alias(self, type_alias);
    }

    /// Visit a module.
    fn visit_module(&mut self, module: &Module) {
        walk_module(self, module);
    }

    /// Visit a use statement.
    fn visit_use(&mut self, use_: &Use) {
        walk_use(self, use_);
    }

    /// Visit a type.
    fn visit_type(&mut self, ty: &Type) {
        walk_type(self, ty);
    }

    /// Visit a path.
    fn visit_path(&mut self, path: &Path) {
        walk_path(self, path);
    }

    /// Visit an expression.
    fn visit_expr(&mut self, expr: &Expr) {
        walk_expr(self, expr);
    }

    /// Visit a field.
    fn visit_field(&mut self, field: &Field) {
        walk_field(self, field);
    }

    /// Visit a method.
    fn visit_method(&mut self, method: &Method) {
        walk_method(self, method);
    }

    /// Visit an attribute.
    fn visit_attribute(&mut self, attr: &Attribute) {
        walk_attribute(self, attr);
    }
}

/// Walk a crate.
pub fn walk_crate<V: Visitor>(visitor: &mut V, krate: &HirCrate) {
    for item in &krate.items {
        visitor.visit_item(item);
    }
}

/// Walk an item.
pub fn walk_item<V: Visitor>(visitor: &mut V, item: &Item) {
    for attr in &item.attrs {
        visitor.visit_attribute(attr);
    }

    match &item.kind {
        ItemKind::Service(service) => visitor.visit_service(service),
        ItemKind::Message(message) => visitor.visit_message(message),
        ItemKind::Enum(enum_) => visitor.visit_enum(enum_),
        ItemKind::Const(const_) => visitor.visit_const(const_),
        ItemKind::TypeAlias(type_alias) => visitor.visit_type_alias(type_alias),
        ItemKind::Module(module) => visitor.visit_module(module),
        ItemKind::Use(use_) => visitor.visit_use(use_),
    }
}

/// Walk a service.
pub fn walk_service<V: Visitor>(visitor: &mut V, service: &Service) {
    if let Some(extends) = &service.extends {
        visitor.visit_path(extends);
    }

    for method in &service.methods {
        visitor.visit_method(method);
    }
}

/// Walk a message.
pub fn walk_message<V: Visitor>(visitor: &mut V, message: &Message) {
    for field in &message.fields {
        visitor.visit_field(field);
    }
}

/// Walk an enum.
pub fn walk_enum<V: Visitor>(visitor: &mut V, enum_: &Enum) {
    for variant in &enum_.variants {
        for attr in &variant.attrs {
            visitor.visit_attribute(attr);
        }
    }
}

/// Walk a constant.
pub fn walk_const<V: Visitor>(visitor: &mut V, const_: &Const) {
    visitor.visit_type(&const_.ty);
    visitor.visit_expr(&const_.value);
}

/// Walk a type alias.
pub fn walk_type_alias<V: Visitor>(visitor: &mut V, type_alias: &TypeAlias) {
    visitor.visit_type(&type_alias.ty);
}

/// Walk a module.
pub fn walk_module<V: Visitor>(visitor: &mut V, module: &Module) {
    for item in &module.items {
        visitor.visit_item(item);
    }
}

/// Walk a use statement.
pub fn walk_use<V: Visitor>(visitor: &mut V, use_: &Use) {
    visitor.visit_path(&use_.path);
}

/// Walk a type.
pub fn walk_type<V: Visitor>(visitor: &mut V, ty: &Type) {
    match ty {
        Type::Path(path) => visitor.visit_path(path),
        Type::Vec(inner) => visitor.visit_type(inner),
        Type::Set(inner) => visitor.visit_type(inner),
        Type::Map { key, value } => {
            visitor.visit_type(key);
            visitor.visit_type(value);
        }
        Type::Optional(inner) => visitor.visit_type(inner),
        Type::Reference { ty, .. } => visitor.visit_type(ty),
        Type::Primitive(_) => {}
    }
}

/// Walk a path.
pub fn walk_path<V: Visitor>(visitor: &mut V, path: &Path) {
    for segment in &path.segments {
        if let Some(args) = &segment.args {
            for arg in &args.args {
                visitor.visit_type(arg);
            }
        }
    }
}

/// Walk an expression.
pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: &Expr) {
    match expr {
        Expr::Literal(_) => {}
        Expr::Path(path) => visitor.visit_path(path),
        Expr::List(exprs) => {
            for expr in exprs {
                visitor.visit_expr(expr);
            }
        }
        Expr::Map(pairs) => {
            for (k, v) in pairs {
                visitor.visit_expr(k);
                visitor.visit_expr(v);
            }
        }
        Expr::Struct { path, fields } => {
            visitor.visit_path(path);
            for field in fields {
                visitor.visit_expr(&field.value);
            }
        }
        Expr::Unary { expr, .. } => visitor.visit_expr(expr),
        Expr::Binary { left, right, .. } => {
            visitor.visit_expr(left);
            visitor.visit_expr(right);
        }
    }
}

/// Walk a field.
pub fn walk_field<V: Visitor>(visitor: &mut V, field: &Field) {
    for attr in &field.attrs {
        visitor.visit_attribute(attr);
    }

    visitor.visit_type(&field.ty);

    if let Some(default) = &field.default {
        visitor.visit_expr(default);
    }
}

/// Walk a method.
pub fn walk_method<V: Visitor>(visitor: &mut V, method: &Method) {
    for attr in &method.attrs {
        visitor.visit_attribute(attr);
    }

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

/// Walk an attribute.
pub fn walk_attribute<V: Visitor>(visitor: &mut V, attr: &Attribute) {
    visitor.visit_path(&attr.name);

    if let Some(args) = &attr.args {
        match args {
            AttrArgs::Eq(_, expr) => visitor.visit_expr(expr),
            AttrArgs::Paren(exprs) => {
                for expr in exprs {
                    visitor.visit_expr(expr);
                }
            }
        }
    }
}