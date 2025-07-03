//! Error code definitions and registry.

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An error code.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ErrorCode {
    pub prefix: &'static str,
    pub number: u16,
}

impl ErrorCode {
    pub const fn new(number: u16) -> Self {
        ErrorCode {
            prefix: "E",
            number,
        }
    }
}

impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.number)
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let number = u16::deserialize(deserializer)?;
        Ok(ErrorCode::new(number))
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:04}", self.prefix, self.number)
    }
}

/// Information about an error code.
pub struct ErrorInfo {
    pub code: ErrorCode,
    pub name: &'static str,
    pub description: &'static str,
    pub explanation: &'static str,
}

/// Registry of all error codes.
pub struct ErrorRegistry {
    errors: FxHashMap<ErrorCode, ErrorInfo>,
}

impl ErrorRegistry {
    fn new() -> Self {
        ErrorRegistry {
            errors: FxHashMap::default(),
        }
    }

    fn register(&mut self, info: ErrorInfo) {
        self.errors.insert(info.code, info);
    }

    pub fn get(&self, code: ErrorCode) -> Option<&ErrorInfo> {
        self.errors.get(&code)
    }
}

// Define all error codes
macro_rules! define_error_codes {
    (
        $(
            $code:ident = $num:literal: $name:literal => $desc:literal;
        )*
    ) => {
        $(
            pub const $code: ErrorCode = ErrorCode::new($num);
        )*

        pub static ERROR_CODES: Lazy<ErrorRegistry> = Lazy::new(|| {
            let mut registry = ErrorRegistry::new();
            $(
                registry.register(ErrorInfo {
                    code: $code,
                    name: $name,
                    description: $desc,
                    explanation: concat!(
                        "Error code ", stringify!($code), " (", stringify!($num), "): ", $desc, "\n\n",
                        "This error occurs when ", $desc, ".",
                    ),
                });
            )*
            registry
        });
    };
}

define_error_codes! {
    // Syntax errors (E0001-E0999)
    E0001 = 1: "duplicate_field_id" => "duplicate field ID in message";
    E0002 = 2: "invalid_field_id" => "field ID must be positive";
    E0003 = 3: "reserved_field_id" => "field ID is reserved";
    E0004 = 4: "missing_field_type" => "field type is missing";
    E0005 = 5: "invalid_syntax" => "invalid syntax";
    E0006 = 6: "unexpected_token" => "unexpected token";
    E0007 = 7: "unterminated_string" => "unterminated string literal";
    E0008 = 8: "invalid_number" => "invalid number literal";
    E0009 = 9: "missing_semicolon" => "missing semicolon";
    E0010 = 10: "unclosed_brace" => "unclosed brace";

    // Type errors (E1000-E1999)
    E1001 = 1001: "undefined_type" => "type not found";
    E1002 = 1002: "circular_dependency" => "circular type dependency detected";
    E1003 = 1003: "type_mismatch" => "type mismatch";
    E1004 = 1004: "invalid_type_argument" => "invalid type argument";
    E1005 = 1005: "missing_type_argument" => "missing type argument";
    E1006 = 1006: "too_many_type_arguments" => "too many type arguments";
    E1007 = 1007: "recursive_type" => "recursive type without indirection";
    E1008 = 1008: "conflicting_definitions" => "conflicting definitions";
    E1009 = 1009: "invalid_generic_param" => "invalid generic parameter";
    E1010 = 1010: "unresolved_import" => "unresolved import";

    // Semantic errors (E2000-E2999)
    E2001 = 2001: "duplicate_definition" => "duplicate definition";
    E2002 = 2002: "invalid_attribute" => "invalid attribute";
    E2003 = 2003: "missing_required_field" => "missing required field";
    E2004 = 2004: "unknown_annotation" => "unknown annotation";
    E2005 = 2005: "invalid_default_value" => "invalid default value";
    E2006 = 2006: "incompatible_versions" => "incompatible protocol versions";
    E2007 = 2007: "deprecated_feature" => "use of deprecated feature";
    E2008 = 2008: "invalid_namespace" => "invalid namespace";
    E2009 = 2009: "missing_include" => "missing include file";
    E2010 = 2010: "cyclic_include" => "cyclic include detected";

    // Protocol-specific errors (E3000-E3999)
    E3001 = 3001: "thrift_specific" => "Thrift-specific error";
    E3002 = 3002: "protobuf_specific" => "Protobuf-specific error";
    E3003 = 3003: "incompatible_protocol" => "incompatible protocol features";
    E3004 = 3004: "unsupported_feature" => "unsupported protocol feature";
    E3005 = 3005: "invalid_service_definition" => "invalid service definition";
    E3006 = 3006: "invalid_method_definition" => "invalid method definition";
    E3007 = 3007: "oneway_with_return" => "oneway method cannot have return type";
    E3008 = 3008: "invalid_exception" => "invalid exception definition";
    E3009 = 3009: "reserved_name" => "use of reserved name";
    E3010 = 3010: "invalid_constant" => "invalid constant definition";
}

/// Get the severity of an error code.
pub fn severity_of(code: ErrorCode) -> crate::Severity {
    match code.number {
        1..=999 => crate::Severity::Error,      // Syntax errors
        1000..=1999 => crate::Severity::Error,  // Type errors
        2000..=2999 => crate::Severity::Error,  // Semantic errors
        3000..=3999 => crate::Severity::Warning, // Protocol warnings
        _ => crate::Severity::Error,
    }
}