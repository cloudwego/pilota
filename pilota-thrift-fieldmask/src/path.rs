use std::{fmt, str};

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{map, map_res},
    sequence::{delimited, preceded, terminated},
};
use pilota::FastStr;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum PathError {
    #[error("syntax error at position {position}: expected '{expected}', but found '{found}'")]
    SyntaxError {
        position: usize,
        expected: FastStr,
        found: FastStr,
    },
    #[error("invalid character '{character}' at position {position}")]
    InvalidCharacter { position: usize, character: char },
    #[error("unterminated string at position {start_position}")]
    UnterminatedString { start_position: usize },
    #[error("invalid escape sequence '{sequence}' at position {position}")]
    InvalidEscape { position: usize, sequence: FastStr },
    #[error("invalid number '{value}' at position {position}")]
    InvalidNumber { position: usize, value: FastStr },
    #[error("unexpected EOF at position {position}, expected '{expected}'")]
    UnexpectedEof { position: usize, expected: FastStr },
    #[error("path cannot be empty")]
    EmptyPath,
    #[error("parse error at position {position}: {message}")]
    ParseError { position: usize, message: FastStr },
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
    fn parse_root(input: &str) -> IResult<&str, TokenData> {
        map(tag("$"), |_| TokenData::Root)(input)
    }

    fn parse_field(input: &str) -> IResult<&str, TokenData> {
        map(tag("."), |_| TokenData::Field)(input)
    }

    fn parse_index_left(input: &str) -> IResult<&str, TokenData> {
        map(terminated(tag("["), multispace0), |_| TokenData::IndexL)(input)
    }

    fn parse_index_right(input: &str) -> IResult<&str, TokenData> {
        map(preceded(multispace0, tag("]")), |_| TokenData::IndexR)(input)
    }

    fn parse_map_left(input: &str) -> IResult<&str, TokenData> {
        map(terminated(tag("{"), multispace0), |_| TokenData::MapL)(input)
    }

    fn parse_map_right(input: &str) -> IResult<&str, TokenData> {
        map(preceded(multispace0, tag("}")), |_| TokenData::MapR)(input)
    }

    fn parse_elem(input: &str) -> IResult<&str, TokenData> {
        map(delimited(multispace0, tag(","), multispace0), |_| {
            TokenData::Elem
        })(input)
    }

    fn parse_any(input: &str) -> IResult<&str, TokenData> {
        map(tag("*"), |_| TokenData::Any)(input)
    }

    fn parse_quoted_string(input: &str) -> IResult<&str, TokenData> {
        let (input, content) = delimited(
            char('"'),
            escaped(
                take_while1(|c: char| c != '"' && c != '\\'),
                '\\',
                one_of("\"ntr\\"),
            ),
            char('"'),
        )(input)?;

        let unescaped = content
            .replace(r#"\""#, "\"")
            .replace(r"\n", "\n")
            .replace(r"\t", "\t")
            .replace(r"\r", "\r")
            .replace(r"\\", "\\");

        Ok((input, TokenData::Str(unescaped.into())))
    }

    fn parse_integer(input: &str) -> IResult<&str, TokenData> {
        map_res(digit1, |s: &str| s.parse::<i32>().map(TokenData::LitInt))(input)
    }

    fn parse_identifier(input: &str) -> IResult<&str, TokenData> {
        let (input, ident) =
            take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '-')(input)?;
        Ok((input, TokenData::LitStr(FastStr::new(ident))))
    }

    fn parse_literal(input: &str) -> IResult<&str, TokenData> {
        alt((Self::parse_integer, Self::parse_identifier))(input)
    }

    pub fn parse_single_token(input: &str) -> IResult<&str, TokenData> {
        alt((
            Self::parse_root,
            Self::parse_field,
            Self::parse_index_left,
            Self::parse_index_right,
            Self::parse_map_left,
            Self::parse_map_right,
            Self::parse_elem,
            Self::parse_any,
            Self::parse_quoted_string,
            Self::parse_literal,
        ))(input)
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

        let mut tokens = Vec::new();
        let mut remaining = src.as_ref();
        let mut position = 0;

        while !remaining.is_empty() {
            let start_pos = position;

            match PathParser::parse_single_token(remaining) {
                Ok((rest, token)) => {
                    let consumed = remaining.len() - rest.len();
                    position += consumed;
                    remaining = rest;

                    tokens.push(PathToken::new(token, start_pos, position));
                }
                Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                    return Err(Self::create_parse_error(&e, src.as_ref(), start_pos));
                }
                Err(nom::Err::Incomplete(_)) => {
                    return Err(PathError::UnexpectedEof {
                        position: start_pos,
                        expected: "完整的token".into(),
                    });
                }
            }
        }

        Ok(Self {
            tokens,
            position: 0,
        })
    }

    pub fn has_next(&self) -> bool {
        self.position < self.tokens.len()
    }

    pub fn next(&mut self) -> PathToken {
        if self.position >= self.tokens.len() {
            return PathToken::new_eof(self.tokens.last().map(|t| t.loc[1]).unwrap_or(0));
        }

        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }

    fn create_parse_error(
        _nom_error: &nom::error::Error<&str>,
        original: &str,
        position: usize,
    ) -> PathError {
        let remaining = &original[position..];
        let remaining_chars: Vec<char> = remaining.chars().take(3).collect();
        let context = remaining_chars.iter().collect::<String>();

        if remaining.starts_with('"') && !remaining[1..].contains('"') {
            PathError::UnterminatedString {
                start_position: position,
            }
        } else if let Some(first_char) = remaining_chars.first() {
            if !first_char.is_ascii_alphanumeric()
                && !matches!(
                    *first_char,
                    '$' | '.' | '[' | ']' | '{' | '}' | ',' | '*' | '"'
                )
            {
                PathError::InvalidCharacter {
                    position,
                    character: *first_char,
                }
            } else {
                PathError::SyntaxError {
                    position,
                    expected: FastStr::new("有效的路径token"),
                    found: FastStr::new(context),
                }
            }
        } else {
            PathError::ParseError {
                position,
                message: FastStr::new("无法解析token"),
            }
        }
    }
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
        // 测试无效字符
        let result = PathIterator::new("$@invalid");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PathError::InvalidCharacter { character: '@', .. }
        ));

        // 测试未闭合的字符串
        let result = PathIterator::new("\"unclosed");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PathError::UnterminatedString { .. }
        ));
    }

    #[test]
    fn test_path_validation() {
        // 有效路径
        assert!(PathIterator::new("$.field[0].name").is_ok());
        assert!(PathIterator::new("$[*]").is_ok());
        assert!(PathIterator::new(r#"${"key"}"#).is_ok());

        // 无效路径
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
        // 测试复杂的映射访问语法
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
            position: 5,
            expected: FastStr::new("数字"),
            found: FastStr::new("abc"),
        };
        assert!(error.to_string().contains("syntax error")); // 英文错误消息
        assert!(error.to_string().contains("at position 5")); // 修正：实际消息是 "at position 5"

        let error = PathError::InvalidCharacter {
            position: 3,
            character: '@',
        };
        assert!(error.to_string().contains("invalid character")); // 英文错误消息
        assert!(error.to_string().contains("'@'"));
    }
}
