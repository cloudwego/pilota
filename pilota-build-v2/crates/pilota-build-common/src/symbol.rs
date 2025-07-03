//! String interning for efficient symbol handling.

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An interned string symbol.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Symbol(u32);

impl Symbol {
    /// Create a new symbol from a string.
    pub fn intern(s: &str) -> Self {
        INTERNER.intern(s)
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
        write!(f, "Symbol({:?})", self.as_str())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

/// Global string interner.
static INTERNER: Lazy<SymbolInterner> = Lazy::new(SymbolInterner::new);

/// String interner for symbols.
pub struct SymbolInterner {
    strings: RwLock<Vec<&'static str>>,
    indices: RwLock<FxHashMap<&'static str, u32>>,
}

impl SymbolInterner {
    fn new() -> Self {
        let mut interner = Self {
            strings: RwLock::new(Vec::new()),
            indices: RwLock::new(FxHashMap::default()),
        };
        
        // Pre-intern common strings
        interner.intern("");
        interner.intern("self");
        interner.intern("Self");
        interner.intern("super");
        interner.intern("crate");
        interner.intern("std");
        interner.intern("core");
        interner.intern("alloc");
        
        interner
    }

    /// Intern a string and return its symbol.
    pub fn intern(&self, s: &str) -> Symbol {
        // Fast path: check if already interned
        if let Some(&idx) = self.indices.read().get(s) {
            return Symbol(idx);
        }

        // Slow path: intern the string
        let mut strings = self.strings.write();
        let mut indices = self.indices.write();

        // Double-check after acquiring write lock
        if let Some(&idx) = indices.get(s) {
            return Symbol(idx);
        }

        let s = Box::leak(s.to_string().into_boxed_str());
        let idx = strings.len() as u32;
        strings.push(s);
        indices.insert(s, idx);
        Symbol(idx)
    }

    /// Get the string value of a symbol.
    pub fn get(&self, sym: Symbol) -> &'static str {
        self.strings.read()[sym.0 as usize]
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