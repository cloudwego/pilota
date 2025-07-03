//! Thrift lexer implementation.

use logos::Logos;

/// Thrift token types.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum ThriftToken {
    // Keywords
    #[token("namespace")]
    Namespace,
    #[token("include")]
    Include,
    #[token("struct")]
    Struct,
    #[token("exception")]
    Exception,
    #[token("service")]
    Service,
    #[token("enum")]
    Enum,
    #[token("const")]
    Const,
    #[token("typedef")]
    Typedef,
    #[token("required")]
    Required,
    #[token("optional")]
    Optional,
    #[token("void")]
    Void,
    #[token("bool")]
    Bool,
    #[token("byte")]
    Byte,
    #[token("i8")]
    I8,
    #[token("i16")]
    I16,
    #[token("i32")]
    I32,
    #[token("i64")]
    I64,
    #[token("double")]
    Double,
    #[token("string")]
    String,
    #[token("binary")]
    Binary,
    #[token("list")]
    List,
    #[token("set")]
    Set,
    #[token("map")]
    Map,
    #[token("oneway")]
    Oneway,
    #[token("extends")]
    Extends,
    #[token("throws")]
    Throws,

    // Identifiers
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Literals
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntegerLiteral(Option<i64>),
    
    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    FloatLiteral(Option<f64>),
    
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        Some(s[1..s.len()-1].to_string())
    })]
    StringLiteral(Option<String>),

    // Punctuation
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("<")]
    LeftAngle,
    #[token(">")]
    RightAngle,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("=")]
    Equals,
    #[token(".")]
    Dot,

    // Comments and whitespace
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}

impl std::fmt::Display for ThriftToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThriftToken::Namespace => write!(f, "namespace"),
            ThriftToken::Struct => write!(f, "struct"),
            ThriftToken::Service => write!(f, "service"),
            ThriftToken::Identifier(s) => write!(f, "identifier '{}'", s),
            ThriftToken::IntegerLiteral(Some(n)) => write!(f, "integer {}", n),
            ThriftToken::StringLiteral(Some(s)) => write!(f, "string \"{}\"", s),
            ThriftToken::LeftBrace => write!(f, "{{"),
            ThriftToken::RightBrace => write!(f, "}}"),
            ThriftToken::Semicolon => write!(f, ";"),
            _ => write!(f, "{:?}", self),
        }
    }
}