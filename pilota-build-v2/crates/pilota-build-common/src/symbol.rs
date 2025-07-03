//! String interning for symbols.

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An interned string.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Symbol(u32);

impl Symbol {
    /// Intern a string.
    pub fn intern(string: &str) -> Self {
        INTERNER.intern(string)
    }

    /// Get the string value of this symbol.
    pub fn as_str(self) -> &'static str {
        INTERNER.get(self)
    }

    /// Get the symbol index.
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symbol({})", self.as_str())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

/// Global string interner.
static INTERNER: Lazy<Interner> = Lazy::new(Interner::new);

/// String interner implementation.
struct Interner {
    strings: RwLock<Vec<&'static str>>,
    indices: RwLock<FxHashMap<&'static str, u32>>,
}

impl Interner {
    fn new() -> Self {
        let interner = Self {
            strings: RwLock::new(Vec::new()),
            indices: RwLock::new(FxHashMap::default()),
        };

        // Pre-intern common symbols
        interner.intern("self");
        interner.intern("Self");
        interner.intern("super");
        interner.intern("crate");
        interner.intern("std");
        interner.intern("core");
        interner.intern("alloc");
        interner.intern("String");
        interner.intern("Vec");
        interner.intern("Option");
        interner.intern("Result");
        interner.intern("Box");
        interner.intern("Rc");
        interner.intern("Arc");
        interner.intern("i8");
        interner.intern("i16");
        interner.intern("i32");
        interner.intern("i64");
        interner.intern("u8");
        interner.intern("u16");
        interner.intern("u32");
        interner.intern("u64");
        interner.intern("f32");
        interner.intern("f64");
        interner.intern("bool");
        interner.intern("char");
        interner.intern("str");
        interner.intern("main");
        interner.intern("new");
        interner.intern("default");
        interner.intern("clone");
        interner.intern("drop");
        interner.intern("fn");
        interner.intern("struct");
        interner.intern("enum");
        interner.intern("trait");
        interner.intern("impl");
        interner.intern("type");
        interner.intern("const");
        interner.intern("static");
        interner.intern("let");
        interner.intern("mut");
        interner.intern("ref");
        interner.intern("move");
        interner.intern("if");
        interner.intern("else");
        interner.intern("match");
        interner.intern("loop");
        interner.intern("while");
        interner.intern("for");
        interner.intern("in");
        interner.intern("return");
        interner.intern("break");
        interner.intern("continue");
        interner.intern("pub");
        interner.intern("mod");
        interner.intern("use");
        interner.intern("as");
        interner.intern("where");
        interner.intern("async");
        interner.intern("await");
        interner.intern("dyn");
        interner.intern("abstract");
        interner.intern("become");
        interner.intern("do");
        interner.intern("final");
        interner.intern("macro");
        interner.intern("override");
        interner.intern("priv");
        interner.intern("typeof");
        interner.intern("unsized");
        interner.intern("virtual");
        interner.intern("yield");
        interner.intern("try");

        interner
    }

    fn intern(&self, string: &str) -> Symbol {
        {
            let indices = self.indices.read();
            if let Some(&idx) = indices.get(string) {
                return Symbol(idx);
            }
        }

        let string: &'static str = Box::leak(string.to_string().into_boxed_str());
        
        let mut strings = self.strings.write();
        let mut indices = self.indices.write();
        
        let idx = strings.len() as u32;
        strings.push(string);
        indices.insert(string, idx);
        
        Symbol(idx)
    }

    fn get(&self, symbol: Symbol) -> &'static str {
        self.strings.read()[symbol.0 as usize]
    }
}

// Implement Serialize/Deserialize by converting to/from string
impl Serialize for Symbol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Symbol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Symbol::intern(&s))
    }
}

/// Common pre-interned symbols.
pub mod sym {
    use super::Symbol;

    macro_rules! define_symbols {
        ($($name:ident => $str:expr,)*) => {
            $(
                pub fn $name() -> Symbol {
                    Symbol::intern($str)
                }
            )*
        };
    }

    define_symbols! {
        empty => "",
        self_ => "self",
        Self_ => "Self",
        super_ => "super",
        crate_ => "crate",
        std => "std",
        core => "core",
        alloc => "alloc",
        
        // Common types
        bool => "bool",
        i8 => "i8",
        i16 => "i16",
        i32 => "i32",
        i64 => "i64",
        u8 => "u8",
        u16 => "u16",
        u32 => "u32",
        u64 => "u64",
        f32 => "f32",
        f64 => "f64",
        string => "string",
        bytes => "bytes",
        void => "void",
        
        // Collections
        vec => "vec",
        set => "set",
        map => "map",
        list => "list",
        
        // Attributes
        deprecated => "deprecated",
        required => "required",
        optional => "optional",
    }
}