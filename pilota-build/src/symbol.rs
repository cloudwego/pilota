use std::{fmt::Display, ops::Deref};

use faststr::FastStr;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use phf::phf_set;

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

#[derive(Hash, PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct Symbol(pub FastStr);

impl std::borrow::Borrow<str> for Symbol {
    fn borrow(&self) -> &str {
        self
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
    T: Into<FastStr>,
{
    fn from(t: T) -> Self {
        Symbol(t.into())
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if &**self == "Self" {
            return write!(f, "Self_");
        }
        if KEYWORDS_SET.contains(self) {
            write!(f, "r#{}", &**self)
        } else {
            write!(f, "{}", &**self)
        }
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

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.sym, f)
    }
}

impl<T> From<T> for Ident
where
    T: Into<FastStr>,
{
    fn from(t: T) -> Self {
        Ident {
            sym: Symbol(t.into()),
        }
    }
}

pub trait IdentName {
    fn struct_ident(&self) -> FastStr {
        self.upper_camel_ident()
    }

    fn enum_ident(&self) -> FastStr {
        self.upper_camel_ident()
    }

    fn mod_ident(&self) -> FastStr {
        self.snake_ident()
    }

    fn variant_ident(&self) -> FastStr {
        self.upper_camel_ident()
    }
    fn fn_ident(&self) -> FastStr {
        self.snake_ident()
    }
    fn field_ident(&self) -> FastStr {
        self.snake_ident()
    }
    fn const_ident(&self) -> FastStr {
        self.shouty_snake_case()
    }

    fn trait_ident(&self) -> FastStr {
        self.upper_camel_ident()
    }

    fn newtype_ident(&self) -> FastStr {
        self.upper_camel_ident()
    }

    fn upper_camel_ident(&self) -> FastStr;
    fn snake_ident(&self) -> FastStr;
    fn shouty_snake_case(&self) -> FastStr;
}

impl IdentName for &str {
    fn upper_camel_ident(&self) -> FastStr {
        let s = self.to_upper_camel_case();
        s.into()
    }

    fn snake_ident(&self) -> FastStr {
        self.to_snake_case().into()
    }

    fn shouty_snake_case(&self) -> FastStr {
        self.to_shouty_snake_case().into()
    }
}

impl IdentName for FastStr {
    fn upper_camel_ident(&self) -> FastStr {
        (&**self).upper_camel_ident()
    }

    fn snake_ident(&self) -> FastStr {
        (&**self).snake_ident()
    }

    fn shouty_snake_case(&self) -> FastStr {
        (&**self).shouty_snake_case()
    }
}
