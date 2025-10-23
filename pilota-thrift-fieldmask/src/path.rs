use std::{fmt, str};

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use pilota::FastStr;
use thiserror::Error;

#[derive(Clone, Error)]
pub enum PathError {
    #[error("syntax error: {message}")]
    SyntaxError { summary: FastStr, message: FastStr },
    #[error("unexpected EOF")]
    UnexpectedEof,
    #[error("path cannot be empty")]
    EmptyPath,
}

impl std::fmt::Debug for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathError::SyntaxError { summary, .. } => {
                write!(f, "{}", summary)
            }
            PathError::UnexpectedEof => write!(f, "unexpected EOF"),
            PathError::EmptyPath => write!(f, "path cannot be empty"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
    // $
    Root,
    // .
    Field,
    // [
    IndexL,
    // ]
    IndexR,
    // {
    MapL,
    // }
    MapR,
    // ,
    Elem,
    // *
    Any,
    // identifier
    LitStr(FastStr),
    // integer
    LitInt(i32),
    // string
    Str(FastStr),
    // EOF
    EOF,
}

#[derive(Debug, Clone)]
pub struct PathToken {
    pub data: TokenData,
    pub loc: [usize; 2], // [start, end]
}

impl PathToken {
    pub fn new(data: TokenData, start: usize, end: usize) -> Self {
        Self {
            data,
            loc: [start, end],
        }
    }

    pub fn new_eof(pos: usize) -> Self {
        Self {
            data: TokenData::EOF,
            loc: [pos, pos],
        }
    }

    pub fn get_begin_pos(&self) -> usize {
        self.loc[0]
    }
}

impl fmt::Display for PathToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            TokenData::EOF => write!(f, "EOF at {}", self.loc[0]),
            TokenData::Any => write!(f, "* at {}", self.loc[0]),
            TokenData::Elem => write!(f, ", at {}", self.loc[0]),
            TokenData::Field => write!(f, ". at {}", self.loc[0]),
            TokenData::Root => write!(f, "$ at {}", self.loc[0]),
            TokenData::IndexL => write!(f, "[ at {}", self.loc[0]),
            TokenData::IndexR => write!(f, "] at {}", self.loc[0]),
            TokenData::MapL => write!(f, "{{ at {}", self.loc[0]),
            TokenData::MapR => write!(f, "}} at {}", self.loc[0]),
            TokenData::LitStr(s) => write!(f, "Lit({}) at {}-{}", s, self.loc[0], self.loc[1]),
            TokenData::LitInt(i) => write!(f, "Lit({}) at {}-{}", i, self.loc[0], self.loc[1]),
            TokenData::Str(s) => write!(f, "Str(\"{}\") at {}-{}", s, self.loc[0], self.loc[1]),
        }
    }
}

pub struct PathParser;

impl PathParser {
    fn blank<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
        one_of(" \t\r\n").repeated().ignored()
    }

    fn parse_root<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        just("$").map(|_| TokenData::Root)
    }

    fn parse_field<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        just(".").map(|_| TokenData::Field)
    }

    fn parse_index_left<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        just("[")
            .then_ignore(Self::blank())
            .map(|_| TokenData::IndexL)
    }

    fn parse_index_right<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        Self::blank()
            .ignore_then(just("]"))
            .map(|_| TokenData::IndexR)
    }
    fn parse_map_left<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        just("{")
            .then_ignore(Self::blank())
            .map(|_| TokenData::MapL)
    }

    fn parse_map_right<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        Self::blank()
            .ignore_then(just("}"))
            .map(|_| TokenData::MapR)
    }

    fn parse_elem<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        Self::blank()
            .ignore_then(just(","))
            .then_ignore(Self::blank())
            .map(|_| TokenData::Elem)
    }

    fn parse_any<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        just("*").map(|_| TokenData::Any)
    }

    fn parse_quoted_string<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>>
    {
        let normal_char = none_of("\"\\").map(|c: char| c.to_string());

        let escape_seq = just('\\')
            .then(one_of("\"ntr\\"))
            .map(|(_, esc)| match esc {
                '"' => "\"".to_string(),
                'n' => "\n".to_string(),
                't' => "\t".to_string(),
                'r' => "\r".to_string(),
                '\\' => "\\".to_string(),
                _ => esc.to_string(),
            });

        let content = normal_char
            .or(escape_seq)
            .repeated()
            .collect::<Vec<String>>()
            .map(|frags: Vec<String>| frags.concat());

        content
            .delimited_by(just('"'), just('"'))
            .map(|s: String| TokenData::Str(FastStr::new(s)))
    }

    fn parse_integer<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        text::digits(10)
            .collect::<String>()
            .map(|s| TokenData::LitInt(s.parse::<i32>().unwrap()))
    }

    fn parse_identifier<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        any()
            .filter(|c: &char| c.is_alphanumeric() || *c == '_' || *c == '-')
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(|s: String| TokenData::LitStr(FastStr::new(s)))
    }

    fn parse_literal<'a>() -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        choice((Self::parse_integer(), Self::parse_identifier()))
    }

    pub fn parse_single_token<'a>()
    -> impl Parser<'a, &'a str, TokenData, extra::Err<Rich<'a, char>>> {
        choice((
            Self::parse_root(),
            Self::parse_field(),
            Self::parse_index_left(),
            Self::parse_index_right(),
            Self::parse_map_left(),
            Self::parse_map_right(),
            Self::parse_elem(),
            Self::parse_any(),
            Self::parse_quoted_string(),
            Self::parse_literal(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct PathIterator {
    tokens: Vec<PathToken>,
    position: usize,
}

impl PathIterator {
    pub fn new<S: AsRef<str>>(src: S) -> Result<Self, PathError> {
        if src.as_ref().is_empty() {
            return Err(PathError::EmptyPath);
        }

        let (tokens, errs) = PathParser::parse_single_token()
            .map_with(|token, e| {
                let span = e.span();
                PathToken::new(token, span.start, span.end)
            })
            .repeated()
            .collect::<Vec<PathToken>>()
            .parse(src.as_ref())
            .into_output_errors();
        if !errs.is_empty() {
            let mut report_strings = Vec::with_capacity(errs.len() + 1);

            let title = format!("{} errors found: ", errs.len());
            report_strings.push(title);
            report_strings.push(String::new());

            for (i, e) in errs.iter().enumerate() {
                if errs.len() > 1 {
                    let error_header = format!("Error {}:", i + 1);
                    report_strings.push(error_header.clone());
                }

                let mut buffer = Vec::new();
                Report::build(ReportKind::Error, e.span().into_range())
                    .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                    .with_message(e.to_string())
                    .with_label(
                        Label::new(e.span().into_range())
                            .with_message(e.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .write(Source::from(src.as_ref()), &mut buffer)
                    .unwrap();
                report_strings.push(String::from_utf8_lossy(&buffer).to_string());

                if i < errs.len() - 1 {
                    report_strings.push(String::new());
                }
            }

            let report = report_strings.join("\n");
            let summary = create_error_summary(&errs);

            return Err(PathError::SyntaxError {
                summary: summary.into(),
                message: report.into(),
            });
        }

        Ok(Self {
            tokens: tokens.unwrap(),
            position: 0,
        })
    }

    pub fn has_next(&self) -> bool {
        self.position < self.tokens.len()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> PathToken {
        if self.position >= self.tokens.len() {
            return PathToken::new_eof(self.tokens.last().map(|t| t.loc[1]).unwrap_or(0));
        }

        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }
}

fn create_error_summary(errs: &[chumsky::error::Rich<char>]) -> String {
    if errs.is_empty() {
        return String::new();
    }

    let mut summary = String::new();

    if errs.len() == 1 {
        let err = &errs[0];
        let col = err.span().start;
        summary.push_str(&format!(" at position {} - {}", col, err.reason()));
    } else {
        summary.push_str(&format!(" ({} errors found):", errs.len()));
        for (i, err) in errs.iter().enumerate() {
            let col = err.span().start;
            summary.push_str(&format!(
                "\n  {}. Position {} - {}",
                i + 1,
                col,
                err.reason()
            ));
        }
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_iterator_basic() {
        let mut iter = PathIterator::new("$.field[0]").unwrap();

        let token = iter.next();
        assert_eq!(token.data, TokenData::Root);

        let token = iter.next();
        assert_eq!(token.data, TokenData::Field);

        let token = iter.next();
        assert_eq!(token.data, TokenData::LitStr("field".into()));

        let token = iter.next();
        assert_eq!(token.data, TokenData::IndexL);

        let token = iter.next();
        assert_eq!(token.data, TokenData::LitInt(0));

        let token = iter.next();
        assert_eq!(token.data, TokenData::IndexR);
    }

    #[test]
    fn test_path_iterator_string() {
        let mut iter = PathIterator::new("\"hello world\"").unwrap();

        let token = iter.next();
        assert_eq!(token.data, TokenData::Str("hello world".into()));
    }

    #[test]
    fn test_path_iterator_map() {
        let mut iter = PathIterator::new("{\"key\"}").unwrap();

        let token = iter.next();
        assert_eq!(token.data, TokenData::MapL);

        let token = iter.next();
        assert_eq!(token.data, TokenData::Str("key".into()));

        let token = iter.next();
        assert_eq!(token.data, TokenData::MapR);
    }

    #[test]
    fn test_escaped_string() {
        let mut iter = PathIterator::new(r#""hello\nworld""#).unwrap();

        let token = iter.next();
        assert_eq!(token.data, TokenData::Str("hello\nworld".into()));
    }

    #[test]
    fn test_error_handling() {
        let result = PathIterator::new("$@invalid");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PathError::SyntaxError { .. }));

        let result = PathIterator::new("\"unclosed");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PathError::SyntaxError { .. }));
    }

    #[test]
    fn test_path_validation() {
        assert!(PathIterator::new("$.field[0].name").is_ok());
        assert!(PathIterator::new("$[*]").is_ok());
        assert!(PathIterator::new(r#"${"key"}"#).is_ok());

        assert!(PathIterator::new("").is_err());
        assert!(PathIterator::new("$@invalid").is_err());
        assert!(PathIterator::new("\"unclosed").is_err());
    }

    #[test]
    fn test_complex_path() {
        let mut iter = PathIterator::new("$.users[0].name").unwrap();

        let tokens: Vec<_> = std::iter::from_fn(|| {
            let token = iter.next();
            if matches!(token.data, TokenData::EOF) {
                None
            } else {
                Some(token.data)
            }
        })
        .collect();

        assert_eq!(
            tokens,
            vec![
                TokenData::Root,
                TokenData::Field,
                TokenData::LitStr("users".into()),
                TokenData::IndexL,
                TokenData::LitInt(0),
                TokenData::IndexR,
                TokenData::Field,
                TokenData::LitStr("name".into()),
            ]
        );
    }

    #[test]
    fn test_complex_map_access() {
        let mut iter = PathIterator::new(r#"$.data{"user name"}.profile{"avatar url"}"#).unwrap();

        let tokens: Vec<_> = std::iter::from_fn(|| {
            let token = iter.next();
            if matches!(token.data, TokenData::EOF) {
                None
            } else {
                let str_val = match &token.data {
                    TokenData::LitStr(s) | TokenData::Str(s) => Some(s.to_string()),
                    _ => None,
                };
                Some((token.data, str_val))
            }
        })
        .collect();

        let expected = vec![
            (TokenData::Root, None),
            (TokenData::Field, None),
            (TokenData::LitStr("data".into()), Some("data".to_string())),
            (TokenData::MapL, None),
            (
                TokenData::Str("user name".into()),
                Some("user name".to_string()),
            ),
            (TokenData::MapR, None),
            (TokenData::Field, None),
            (
                TokenData::LitStr("profile".into()),
                Some("profile".to_string()),
            ),
            (TokenData::MapL, None),
            (
                TokenData::Str("avatar url".into()),
                Some("avatar url".to_string()),
            ),
            (TokenData::MapR, None),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_path_error_display() {
        let error = PathError::SyntaxError {
            summary: "syntax error".into(),
            message: "at position 5".into(),
        };
        assert!(error.to_string().contains("syntax error"));
        assert!(error.to_string().contains("at position 5"));
    }
}
