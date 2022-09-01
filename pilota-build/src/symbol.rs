use std::{fmt::Display, ops::Deref, sync::Arc};

use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use phf::phf_set;
use quote::{format_ident, IdentFragment};

crate::newtype_index! {
    pub struct FileId { .. }
}

crate::newtype_index! {
    pub struct DefId { .. }
}

lazy_static::lazy_static! {
    static ref KEYWORDS_SET: phf::Set<&'static str> = phf_set![
        "as",
        "use",
        "break",
        "const",
        "continue",
        "crate",
        "else",
        "if",
        "enum",
        "extern",
        "false",
        "fn",
        "for",
        "impl",
        "in",
        "let",
        "loop",
        "match",
        "mod",
        "move",
        "mut",
        "pub",
        "ref",
        "return",
        "Self",
        "self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "type",
        "unsafe",
        "where",
        "while",
        "abstract",
        "alignof",
        "become",
        "box",
        "do",
        "final",
        "macro",
        "offsetof",
        "override",
        "priv",
        "proc",
        "pure",
        "sizeof",
        "typeof",
        "unsized",
        "virtual",
        "yield",
        "dyn",
        "async",
        "await",
        "try"
    ];
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Symbol(pub Arc<str>);

macro_rules! to_case {
    ($m: ident) => {
        pub fn $m(&self) -> Symbol {
            Symbol(Arc::from(self.0.$m()))
        }
    };
}

impl Symbol {
    to_case!(to_shouty_snake_case);
    to_case!(to_snake_case);
    to_case!(to_lower_camel_case);
    to_case!(to_upper_camel_case);
}

impl Deref for Symbol {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Symbol
where
    T: Into<Arc<str>>,
{
    fn from(t: T) -> Self {
        Symbol(t.into())
    }
}

impl IdentFragment for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if KEYWORDS_SET.contains(self) {
            write!(f, "r#{}", self)
        } else {
            write!(f, "{}", self)
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub enum EnumRepr {
    I32,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Ident {
    pub sym: Symbol,
}

impl Ident {
    pub fn new(sym: Symbol) -> Self {
        Ident { sym }
    }
}

macro_rules! ident_to_case {
    ($m: ident) => {
        pub fn $m(&self) -> Ident {
            Ident { sym: self.sym.$m() }
        }
    };
}

impl Ident {
    ident_to_case!(to_shouty_snake_case);
    ident_to_case!(to_snake_case);
    ident_to_case!(to_lower_camel_case);
    ident_to_case!(to_upper_camel_case);
}

impl Deref for Ident {
    type Target = Symbol;

    fn deref(&self) -> &Self::Target {
        &self.sym
    }
}

impl IdentFragment for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        quote::IdentFragment::fmt(&self.sym, f)
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.sym, f)
    }
}

impl<T> From<T> for Ident
where
    T: Into<Arc<str>>,
{
    fn from(t: T) -> Self {
        Ident {
            sym: Symbol(t.into()),
        }
    }
}

pub(crate) trait IdentName {
    fn struct_ident(&self) -> syn::Ident;
    fn variant_ident(&self) -> syn::Ident;
}

impl IdentName for &str {
    fn struct_ident(&self) -> syn::Ident {
        let s = self.to_upper_camel_case();
        if s == "Self" {
            format_ident!("Self_")
        } else {
            format_ident!("{}", s)
        }
    }

    fn variant_ident(&self) -> syn::Ident {
        self.struct_ident()
    }
}
