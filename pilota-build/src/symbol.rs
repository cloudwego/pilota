use std::{
    fmt::Display,
    ops::Deref,
    sync::{Arc, OnceLock},
};

use faststr::FastStr;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use phf::phf_set;

crate::newtype_index! {
    pub struct FileId { .. }
}

crate::newtype_index! {
    pub struct DefId { .. }
}

pub static SPECIAL_NAMINGS: OnceLock<Vec<FastStr>> = OnceLock::new();

static KEYWORDS_SET: phf::Set<&'static str> = phf_set![
    "as", "use", "break", "const", "continue", "crate", "else", "if", "enum", "extern", "false",
    "fn", "for", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "Self", "self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "where", "while", "abstract", "alignof", "become", "box", "do", "final", "macro", "offsetof",
    "override", "priv", "proc", "pure", "sizeof", "typeof", "unsized", "virtual", "yield", "dyn",
    "async", "await", "try", "gen"
];

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
        if self.is_path_segment_keyword() {
            return write!(f, "{}_", &**self);
        }
        if KEYWORDS_SET.contains(self) {
            write!(f, "r#{}", &**self)
        } else {
            write!(f, "{}", &**self)
        }
    }
}

impl Symbol {
    // https://github.com/rust-lang/rust/blob/master/compiler/rustc_span/src/symbol.rs#L2395-L2398
    fn is_path_segment_keyword(&self) -> bool {
        ["super", "self", "Self", "crate"].contains(&&**self)
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

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ModPath(Arc<[FastStr]>);

impl Deref for ModPath {
    type Target = [FastStr];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for ModPath
where
    T: Into<Arc<[FastStr]>>,
{
    fn from(t: T) -> Self {
        ModPath(t.into())
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
        if let Some(index) = self.find(|c: char| c != '_') {
            let s = self[index..].to_upper_camel_case();
            return format!("{}{}", &self[0..index], s).into();
        }
        self.to_string().into()
    }

    fn snake_ident(&self) -> FastStr {
        if let Some(index) = self.find(|c: char| c != '_') {
            let s = &self[index..];
            let s = if is_common_initialism(s) {
                to_snake_case(s)
            } else {
                s.to_snake_case()
            };
            return format!("{}{}", &self[0..index], s).into();
        }
        self.to_string().into()
    }

    fn shouty_snake_case(&self) -> FastStr {
        if let Some(index) = self.find(|c: char| c != '_') {
            let s = &self[index..];
            let s = if is_common_initialism(s) {
                to_snake_case(s).to_uppercase()
            } else {
                s.to_shouty_snake_case()
            };
            return format!("{}{}", &self[0..index], s).into();
        }
        self.to_string().into()
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

// Taken from rustc.
fn to_snake_case(mut str: &str) -> String {
    let mut words = vec![];
    // Preserve leading underscores
    str = str.trim_start_matches(|c: char| {
        if c == '_' {
            words.push(String::new());
            true
        } else {
            false
        }
    });
    for s in str.split('_') {
        let mut last_upper = false;
        let mut buf = String::new();
        if s.is_empty() {
            continue;
        }
        for ch in s.chars() {
            if !buf.is_empty() && buf != "'" && ch.is_uppercase() && !last_upper {
                words.push(buf);
                buf = String::new();
            }
            last_upper = ch.is_uppercase();
            buf.extend(ch.to_lowercase());
        }
        words.push(buf);
    }
    words.join("_")
}

fn is_common_initialism(s: &str) -> bool {
    for name in SPECIAL_NAMINGS.get().unwrap_or(&Default::default()).iter() {
        if s.contains(name.as_str()) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use heck::ToSnakeCase;

    use crate::symbol::to_snake_case;

    #[test]
    fn snake_case() {
        assert_eq!("IDs".to_snake_case(), "i_ds");
        // positive
        assert_eq!(to_snake_case("IDs"), "ids");

        assert_eq!("UIDSecure".to_snake_case(), "uid_secure");
        // negative
        assert_eq!(to_snake_case("UIDSecure"), "uidsecure");
    }
}
