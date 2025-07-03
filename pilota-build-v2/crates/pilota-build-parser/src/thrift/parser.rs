//! Thrift parser implementation.

use super::lexer::ThriftToken;
use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    lexer::{Lexer, Token},
    parser::{ParseContext, Parser as ParserTrait},
};
use pilota_build_common::{BytePos, FileId, Span, Symbol};
use pilota_build_hir::{
    ast::*, HirCrate, HirId, HirNode, Item, ItemKind, LocalId, lower::LoweringContext,
};
use rustc_hash::FxHashSet;

/// Thrift parser.
pub struct ThriftParser {
    lowering_ctx: LoweringContext,
}

impl ThriftParser {
    pub fn new() -> Self {
        ThriftParser {
            lowering_ctx: LoweringContext::new(),
        }
    }

    fn parse_document(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext) -> Vec<Item> {
        let mut items = Vec::new();

        while let Some(token) = lexer.next_token() {
            match &token.kind {
                ThriftToken::Namespace => {
                    // Skip namespace declarations for now
                    self.skip_namespace(lexer);
                }
                ThriftToken::Include => {
                    // Skip includes for now
                    self.skip_include(lexer);
                }
                ThriftToken::Struct => {
                    if let Some(item) = self.parse_struct(lexer, ctx, false) {
                        items.push(item);
                    }
                }
                ThriftToken::Exception => {
                    if let Some(item) = self.parse_struct(lexer, ctx, true) {
                        items.push(item);
                    }
                }
                ThriftToken::Service => {
                    if let Some(item) = self.parse_service(lexer, ctx) {
                        items.push(item);
                    }
                }
                ThriftToken::Enum => {
                    if let Some(item) = self.parse_enum(lexer, ctx) {
                        items.push(item);
                    }
                }
                ThriftToken::Const => {
                    if let Some(item) = self.parse_const(lexer, ctx) {
                        items.push(item);
                    }
                }
                ThriftToken::Typedef => {
                    if let Some(item) = self.parse_typedef(lexer, ctx) {
                        items.push(item);
                    }
                }
                _ => {
                    ctx.error(ParseError::unexpected_token(
                        "top-level declaration",
                        &token.kind.to_string(),
                        token.span,
                    ));
                }
            }
        }

        items
    }

    fn skip_namespace(&mut self, lexer: &mut Lexer<ThriftToken>) {
        // namespace <lang> <name>
        lexer.next_token(); // lang
        lexer.next_token(); // name
    }

    fn skip_include(&mut self, lexer: &mut Lexer<ThriftToken>) {
        // include "<path>"
        lexer.next_token(); // path
    }

    fn parse_struct(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext, is_exception: bool) -> Option<Item> {
        let start_span = lexer.position();

        // struct/exception <name> {
        let name_token = lexer.next_token()?;
        let name = match name_token.kind {
            ThriftToken::Identifier(s) => Symbol::intern(&s),
            _ => {
                ctx.error(ParseError::unexpected_token(
                    "identifier",
                    &name_token.kind.to_string(),
                    name_token.span,
                ));
                return None;
            }
        };

        self.expect_token(lexer, ctx, ThriftToken::LeftBrace)?;

        let mut fields = Vec::new();
        let mut field_ids = FxHashSet::default();

        while let Some(token) = lexer.peek() {
            if matches!(token, ThriftToken::RightBrace) {
                break;
            }

            if let Some(field) = self.parse_field(lexer, ctx, &mut field_ids) {
                fields.push(field);
            }
        }

        self.expect_token(lexer, ctx, ThriftToken::RightBrace)?;

        let end_span = lexer.position();
        let span = Span::new(start_span, end_span, ctx.file_id);

        let message = Message {
            name,
            fields,
            is_exception,
        };

        Some(self.create_item(ItemKind::Message(message), span))
    }

    fn parse_field(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext, field_ids: &mut FxHashSet<i32>) -> Option<Field> {
        let start_span = lexer.position();

        // [<field_id>:] [<requiredness>] <type> <name> [= <default>] [;|,]
        
        // Try to parse field ID
        let field_id = if let Some(Token { kind: ThriftToken::IntegerLiteral(Some(id)), .. }) = lexer.peek() {
            let id = *id as i32;
            lexer.next_token();
            
            if !field_ids.insert(id) {
                ctx.error(ParseError::new(
                    ParseErrorKind::DuplicateFieldId(id),
                    Span::new(start_span, lexer.position(), ctx.file_id),
                ));
            }
            
            self.expect_token(lexer, ctx, ThriftToken::Colon)?;
            Some(id)
        } else {
            None
        };

        // Parse requiredness
        let required = if let Some(token) = lexer.peek() {
            match token {
                ThriftToken::Required => {
                    lexer.next_token();
                    FieldRequired::Required
                }
                ThriftToken::Optional => {
                    lexer.next_token();
                    FieldRequired::Optional
                }
                _ => FieldRequired::Default,
            }
        } else {
            FieldRequired::Default
        };

        // Parse type
        let ty = self.parse_type(lexer, ctx)?;

        // Parse name
        let name_token = lexer.next_token()?;
        let name = match name_token.kind {
            ThriftToken::Identifier(s) => Symbol::intern(&s),
            _ => {
                ctx.error(ParseError::unexpected_token(
                    "field name",
                    &name_token.kind.to_string(),
                    name_token.span,
                ));
                return None;
            }
        };

        // Parse default value (optional)
        let default = if let Some(Token { kind: ThriftToken::Equals, .. }) = lexer.peek() {
            lexer.next_token();
            self.parse_expr(lexer, ctx)
        } else {
            None
        };

        // Skip optional separator
        if let Some(token) = lexer.peek() {
            if matches!(token, ThriftToken::Semicolon | ThriftToken::Comma) {
                lexer.next_token();
            }
        }

        let end_span = lexer.position();
        let span = Span::new(start_span, end_span, ctx.file_id);

        Some(Field {
            id: field_id,
            name,
            ty,
            required,
            default,
            attrs: Vec::new(),
            span,
        })
    }

    fn parse_type(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext) -> Option<Type> {
        let token = lexer.next_token()?;

        match &token.kind {
            ThriftToken::Bool => Some(Type::Primitive(PrimitiveType::Bool)),
            ThriftToken::Byte => Some(Type::Primitive(PrimitiveType::Byte)),
            ThriftToken::I8 => Some(Type::Primitive(PrimitiveType::I8)),
            ThriftToken::I16 => Some(Type::Primitive(PrimitiveType::I16)),
            ThriftToken::I32 => Some(Type::Primitive(PrimitiveType::I32)),
            ThriftToken::I64 => Some(Type::Primitive(PrimitiveType::I64)),
            ThriftToken::Double => Some(Type::Primitive(PrimitiveType::F64)),
            ThriftToken::String => Some(Type::Primitive(PrimitiveType::String)),
            ThriftToken::Binary => Some(Type::Primitive(PrimitiveType::Bytes)),
            ThriftToken::Void => Some(Type::Primitive(PrimitiveType::Void)),
            
            ThriftToken::List => {
                self.expect_token(lexer, ctx, ThriftToken::LeftAngle)?;
                let inner = Box::new(self.parse_type(lexer, ctx)?);
                self.expect_token(lexer, ctx, ThriftToken::RightAngle)?;
                Some(Type::Vec(inner))
            }
            
            ThriftToken::Set => {
                self.expect_token(lexer, ctx, ThriftToken::LeftAngle)?;
                let inner = Box::new(self.parse_type(lexer, ctx)?);
                self.expect_token(lexer, ctx, ThriftToken::RightAngle)?;
                Some(Type::Set(inner))
            }
            
            ThriftToken::Map => {
                self.expect_token(lexer, ctx, ThriftToken::LeftAngle)?;
                let key = Box::new(self.parse_type(lexer, ctx)?);
                self.expect_token(lexer, ctx, ThriftToken::Comma)?;
                let value = Box::new(self.parse_type(lexer, ctx)?);
                self.expect_token(lexer, ctx, ThriftToken::RightAngle)?;
                Some(Type::Map { key, value })
            }
            
            ThriftToken::Identifier(name) => {
                let path = Path::from_ident(Symbol::intern(name), token.span);
                Some(Type::Path(path))
            }
            
            _ => {
                ctx.error(ParseError::unexpected_token(
                    "type",
                    &token.kind.to_string(),
                    token.span,
                ));
                None
            }
        }
    }

    fn parse_expr(&mut self, lexer: &mut Lexer<ThriftToken>, _ctx: &mut ParseContext) -> Option<Expr> {
        let token = lexer.next_token()?;

        match token.kind {
            ThriftToken::IntegerLiteral(Some(n)) => Some(Expr::Literal(Literal::Int(n))),
            ThriftToken::FloatLiteral(Some(f)) => Some(Expr::Literal(Literal::Float(f))),
            ThriftToken::StringLiteral(Some(s)) => Some(Expr::Literal(Literal::String(Symbol::intern(&s)))),
            _ => None, // TODO: Support more expressions
        }
    }

    fn parse_service(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext) -> Option<Item> {
        let start_span = lexer.position();

        // service <name> [extends <parent>] {
        let name_token = lexer.next_token()?;
        let name = match name_token.kind {
            ThriftToken::Identifier(s) => Symbol::intern(&s),
            _ => {
                ctx.error(ParseError::unexpected_token(
                    "service name",
                    &name_token.kind.to_string(),
                    name_token.span,
                ));
                return None;
            }
        };

        let extends = if let Some(Token { kind: ThriftToken::Extends, .. }) = lexer.peek() {
            lexer.next_token();
            let parent_token = lexer.next_token()?;
            match parent_token.kind {
                ThriftToken::Identifier(s) => Some(Path::from_ident(Symbol::intern(&s), parent_token.span)),
                _ => None,
            }
        } else {
            None
        };

        self.expect_token(lexer, ctx, ThriftToken::LeftBrace)?;

        let mut methods = Vec::new();
        while let Some(token) = lexer.peek() {
            if matches!(token, ThriftToken::RightBrace) {
                break;
            }

            if let Some(method) = self.parse_method(lexer, ctx) {
                methods.push(method);
            }
        }

        self.expect_token(lexer, ctx, ThriftToken::RightBrace)?;

        let end_span = lexer.position();
        let span = Span::new(start_span, end_span, ctx.file_id);

        let service = Service {
            name,
            extends,
            methods,
        };

        Some(self.create_item(ItemKind::Service(service), span))
    }

    fn parse_method(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext) -> Option<Method> {
        let start_span = lexer.position();

        // [oneway] <return_type> <name>(<params>) [throws (<exceptions>)] [;|,]
        let oneway = if let Some(Token { kind: ThriftToken::Oneway, .. }) = lexer.peek() {
            lexer.next_token();
            true
        } else {
            false
        };

        let result = if let Some(Token { kind: ThriftToken::Void, .. }) = lexer.peek() {
            lexer.next_token();
            None
        } else {
            Some(self.parse_type(lexer, ctx)?)
        };

        let name_token = lexer.next_token()?;
        let name = match name_token.kind {
            ThriftToken::Identifier(s) => Symbol::intern(&s),
            _ => {
                ctx.error(ParseError::unexpected_token(
                    "method name",
                    &name_token.kind.to_string(),
                    name_token.span,
                ));
                return None;
            }
        };

        self.expect_token(lexer, ctx, ThriftToken::LeftParen)?;

        let mut params = Vec::new();
        let mut param_ids = FxHashSet::default();

        while let Some(token) = lexer.peek() {
            if matches!(token, ThriftToken::RightParen) {
                break;
            }

            if let Some(param) = self.parse_field(lexer, ctx, &mut param_ids) {
                params.push(param);
            }

            if let Some(Token { kind: ThriftToken::Comma, .. }) = lexer.peek() {
                lexer.next_token();
            }
        }

        self.expect_token(lexer, ctx, ThriftToken::RightParen)?;

        let exceptions = if let Some(Token { kind: ThriftToken::Throws, .. }) = lexer.peek() {
            lexer.next_token();
            self.expect_token(lexer, ctx, ThriftToken::LeftParen)?;

            let mut exceptions = Vec::new();
            let mut exception_ids = FxHashSet::default();

            while let Some(token) = lexer.peek() {
                if matches!(token, ThriftToken::RightParen) {
                    break;
                }

                if let Some(exception) = self.parse_field(lexer, ctx, &mut exception_ids) {
                    exceptions.push(exception);
                }

                if let Some(Token { kind: ThriftToken::Comma, .. }) = lexer.peek() {
                    lexer.next_token();
                }
            }

            self.expect_token(lexer, ctx, ThriftToken::RightParen)?;
            exceptions
        } else {
            Vec::new()
        };

        // Skip optional separator
        if let Some(token) = lexer.peek() {
            if matches!(token, ThriftToken::Semicolon | ThriftToken::Comma) {
                lexer.next_token();
            }
        }

        let end_span = lexer.position();
        let span = Span::new(start_span, end_span, ctx.file_id);

        Some(Method {
            name,
            oneway,
            params,
            result,
            exceptions,
            attrs: Vec::new(),
            span,
        })
    }

    fn parse_enum(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext) -> Option<Item> {
        let start_span = lexer.position();

        // enum <name> {
        let name_token = lexer.next_token()?;
        let name = match name_token.kind {
            ThriftToken::Identifier(s) => Symbol::intern(&s),
            _ => {
                ctx.error(ParseError::unexpected_token(
                    "enum name",
                    &name_token.kind.to_string(),
                    name_token.span,
                ));
                return None;
            }
        };

        self.expect_token(lexer, ctx, ThriftToken::LeftBrace)?;

        let mut variants = Vec::new();
        let mut next_value = 0;

        while let Some(token) = lexer.peek() {
            if matches!(token, ThriftToken::RightBrace) {
                break;
            }

            let variant_start = lexer.position();
            let variant_name_token = lexer.next_token()?;
            let variant_name = match variant_name_token.kind {
                ThriftToken::Identifier(s) => Symbol::intern(&s),
                _ => continue,
            };

            let value = if let Some(Token { kind: ThriftToken::Equals, .. }) = lexer.peek() {
                lexer.next_token();
                if let Some(Token { kind: ThriftToken::IntegerLiteral(Some(v)), .. }) = lexer.next_token() {
                    next_value = v as i32 + 1;
                    Some(v as i32)
                } else {
                    None
                }
            } else {
                let v = next_value;
                next_value += 1;
                Some(v)
            };

            // Skip optional separator
            if let Some(token) = lexer.peek() {
                if matches!(token, ThriftToken::Semicolon | ThriftToken::Comma) {
                    lexer.next_token();
                }
            }

            let variant_end = lexer.position();
            let variant_span = Span::new(variant_start, variant_end, ctx.file_id);

            variants.push(EnumVariant {
                name: variant_name,
                value,
                attrs: Vec::new(),
                span: variant_span,
            });
        }

        self.expect_token(lexer, ctx, ThriftToken::RightBrace)?;

        let end_span = lexer.position();
        let span = Span::new(start_span, end_span, ctx.file_id);

        let enum_ = Enum { name, variants };
        Some(self.create_item(ItemKind::Enum(enum_), span))
    }

    fn parse_const(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext) -> Option<Item> {
        let start_span = lexer.position();

        // const <type> <name> = <value>
        let ty = self.parse_type(lexer, ctx)?;

        let name_token = lexer.next_token()?;
        let name = match name_token.kind {
            ThriftToken::Identifier(s) => Symbol::intern(&s),
            _ => {
                ctx.error(ParseError::unexpected_token(
                    "const name",
                    &name_token.kind.to_string(),
                    name_token.span,
                ));
                return None;
            }
        };

        self.expect_token(lexer, ctx, ThriftToken::Equals)?;

        let value = self.parse_expr(lexer, ctx)?;

        // Skip optional semicolon
        if let Some(Token { kind: ThriftToken::Semicolon, .. }) = lexer.peek() {
            lexer.next_token();
        }

        let end_span = lexer.position();
        let span = Span::new(start_span, end_span, ctx.file_id);

        let const_ = Const { name, ty, value };
        Some(self.create_item(ItemKind::Const(const_), span))
    }

    fn parse_typedef(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext) -> Option<Item> {
        let start_span = lexer.position();

        // typedef <type> <name>
        let ty = self.parse_type(lexer, ctx)?;

        let name_token = lexer.next_token()?;
        let name = match name_token.kind {
            ThriftToken::Identifier(s) => Symbol::intern(&s),
            _ => {
                ctx.error(ParseError::unexpected_token(
                    "typedef name",
                    &name_token.kind.to_string(),
                    name_token.span,
                ));
                return None;
            }
        };

        // Skip optional semicolon
        if let Some(Token { kind: ThriftToken::Semicolon, .. }) = lexer.peek() {
            lexer.next_token();
        }

        let end_span = lexer.position();
        let span = Span::new(start_span, end_span, ctx.file_id);

        let type_alias = TypeAlias { name, ty };
        Some(self.create_item(ItemKind::TypeAlias(type_alias), span))
    }

    fn expect_token(&mut self, lexer: &mut Lexer<ThriftToken>, ctx: &mut ParseContext, expected: ThriftToken) -> Option<Token<ThriftToken>> {
        let token = lexer.next_token()?;
        if std::mem::discriminant(&token.kind) != std::mem::discriminant(&expected) {
            ctx.error(ParseError::unexpected_token(
                &expected.to_string(),
                &token.kind.to_string(),
                token.span,
            ));
            None
        } else {
            Some(token)
        }
    }

    fn create_item(&mut self, kind: ItemKind, span: Span) -> Item {
        self.lowering_ctx.reset_local_id_counter();
        let hir_id = self.lowering_ctx.next_hir_id();
        
        HirNode {
            id: hir_id,
            span,
            attrs: Vec::new(),
            kind,
        }
    }
}

impl ParserTrait for ThriftParser {
    fn parse(&self, file_id: FileId, content: &str) -> ParseResult<HirCrate> {
        let mut parser = ThriftParser::new();
        let mut lexer = Lexer::new(content, file_id);
        let mut ctx = ParseContext::new(file_id);

        let items = parser.parse_document(&mut lexer, &mut ctx);

        if ctx.has_errors() {
            let errors = ctx.take_errors();
            Err(errors.into_iter().next().unwrap())
        } else {
            Ok(HirCrate {
                items,
                span: Span::new(BytePos(0), BytePos(content.len() as u32), file_id),
            })
        }
    }
}