//! Protobuf lexer implementation.

use logos::Logos;

/// Protobuf token types.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum ProtobufToken {
    // Keywords
    #[token("syntax")]
    Syntax,
    #[token("package")]
    Package,
    #[token("import")]
    Import,
    #[token("message")]
    Message,
    #[token("service")]
    Service,
    #[token("rpc")]
    Rpc,
    #[token("enum")]
    Enum,
    #[token("oneof")]
    Oneof,
    #[token("repeated")]
    Repeated,
    #[token("optional")]
    Optional,
    #[token("required")]
    Required,
    #[token("returns")]
    Returns,
    #[token("stream")]
    Stream,

    // Types
    #[token("int32")]
    Int32,
    #[token("int64")]
    Int64,
    #[token("uint32")]
    UInt32,
    #[token("uint64")]
    UInt64,
    #[token("sint32")]
    SInt32,
    #[token("sint64")]
    SInt64,
    #[token("fixed32")]
    Fixed32,
    #[token("fixed64")]
    Fixed64,
    #[token("sfixed32")]
    SFixed32,
    #[token("sfixed64")]
    SFixed64,
    #[token("float")]
    Float,
    #[token("double")]
    Double,
    #[token("bool")]
    Bool,
    #[token("string")]
    String,
    #[token("bytes")]
    Bytes,

    // Identifiers
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Literals
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntegerLiteral(Option<i64>),
    
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