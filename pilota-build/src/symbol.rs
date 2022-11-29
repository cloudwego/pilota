use std::{fmt::Display, ops::Deref};

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
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
pub struct Symbol(pub smol_str::SmolStr);

impl std::borrow::Borrow<str> for Symbol {
    fn borrow(&self) -> &str {
        &**self
    }
}

impl Deref for Symbol {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Symbol
where
    T: Into<smol_str::SmolStr>,
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
    T: Into<smol_str::SmolStr>,
{
    fn from(t: T) -> Self {
        Ident {
            sym: Symbol(t.into()),
        }
    }
}

pub trait IdentName {
    fn struct_ident(&self) -> smol_str::SmolStr {
        self.upper_camel_ident()
    }

    fn enum_ident(&self) -> smol_str::SmolStr {
        self.upper_camel_ident()
    }

    fn mod_ident(&self) -> smol_str::SmolStr {
        self.snake_ident()
    }

    fn variant_ident(&self) -> smol_str::SmolStr {
        self.upper_camel_ident()
    }
    fn fn_ident(&self) -> smol_str::SmolStr {
        self.snake_ident()
    }
    fn field_ident(&self) -> smol_str::SmolStr {
        self.snake_ident()
    }
    fn const_ident(&self) -> smol_str::SmolStr {
        self.shouty_snake_case()
    }

    fn trait_ident(&self) -> smol_str::SmolStr {
        self.upper_camel_ident()
    }

    fn newtype_ident(&self) -> smol_str::SmolStr {
        self.upper_camel_ident()
    }

    fn upper_camel_ident(&self) -> smol_str::SmolStr;
    fn snake_ident(&self) -> smol_str::SmolStr;
    fn shouty_snake_case(&self) -> smol_str::SmolStr;

    fn as_syn_ident(&self) -> syn::Ident;
}

fn str2ident(s: &str) -> syn::Ident {
    if s == "Self" {
        return format_ident!("Self_");
    }
    if KEYWORDS_SET.contains(s) {
        format_ident!("r#{}", s)
    } else {
        format_ident!("{}", s)
    }
}

impl IdentName for &str {
    fn upper_camel_ident(&self) -> smol_str::SmolStr {
        let s = self.to_upper_camel_case();
        s.into()
    }

    fn snake_ident(&self) -> smol_str::SmolStr {
        self.to_snake_case().into()
    }

    fn shouty_snake_case(&self) -> smol_str::SmolStr {
        self.to_shouty_snake_case().into()
    }

    fn as_syn_ident(&self) -> syn::Ident {
        str2ident(self)
    }
}

impl IdentName for smol_str::SmolStr {
    fn upper_camel_ident(&self) -> smol_str::SmolStr {
        (&**self).upper_camel_ident()
    }

    fn snake_ident(&self) -> smol_str::SmolStr {
        (&**self).snake_ident()
    }

    fn shouty_snake_case(&self) -> smol_str::SmolStr {
        (&**self).shouty_snake_case()
    }

    fn as_syn_ident(&self) -> syn::Ident {
        str2ident(self)
    }
}
