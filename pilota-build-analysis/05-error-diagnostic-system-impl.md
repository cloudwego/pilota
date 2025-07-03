# 错误诊断系统实现指南

## 1. 项目结构

```
pilota-build-diagnostics/
├── src/
│   ├── lib.rs              # 主入口
│   ├── diagnostic.rs       # 诊断类型定义
│   ├── error_code.rs       # 错误代码系统
│   ├── source_map.rs       # 源码映射
│   ├── emitter/           # 错误输出
│   │   ├── mod.rs
│   │   ├── terminal.rs    # 终端输出
│   │   ├── json.rs       # JSON 输出
│   │   └── html.rs       # HTML 报告
│   ├── recovery.rs        # 错误恢复
│   └── quick_fix.rs      # 快速修复
├── tests/
│   ├── ui/               # UI 测试
│   └── integration.rs    # 集成测试
└── Cargo.toml
```

## 2. 核心实现

### 2.1 错误代码定义

```rust
// src/error_code.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// 错误代码定义宏
macro_rules! define_error_codes {
    (
        $($code:ident = $num:literal: $name:literal => $desc:literal,)*
    ) => {
        $(
            pub const $code: ErrorCode = ErrorCode {
                prefix: "E",
                number: $num,
            };
        )*
        
        pub static ERROR_REGISTRY: Lazy<ErrorRegistry> = Lazy::new(|| {
            let mut registry = ErrorRegistry::new();
            $(
                registry.register(ErrorInfo {
                    code: $code,
                    name: $name,
                    description: $desc,
                    example: None,
                    explanation: include_str!(concat!("../error_docs/", stringify!($code), ".md")),
                });
            )*
            registry
        });
    };
}

// 定义所有错误代码
define_error_codes! {
    E0001 = 1: "duplicate_field_id" => "Duplicate field ID in message",
    E0002 = 2: "undefined_type" => "Type not found",
    E0003 = 3: "circular_dependency" => "Circular type dependency detected",
    E0004 = 4: "invalid_field_id" => "Field ID must be positive",
    E0005 = 5: "reserved_field_id" => "Field ID is reserved",
    // ... 更多错误代码
}

/// 错误严重性助手
impl ErrorCode {
    pub fn severity(&self) -> Severity {
        match self.number {
            1..=999 => Severity::Error,      // 语法错误
            1000..=1999 => Severity::Error,  // 类型错误
            2000..=2999 => Severity::Warning, // 兼容性警告
            3000..=3999 => Severity::Info,   // 优化建议
            _ => Severity::Error,
        }
    }
    
    pub fn category(&self) -> ErrorCategory {
        match self.number {
            1..=999 => ErrorCategory::Syntax,
            1000..=1999 => ErrorCategory::Type,
            2000..=2999 => ErrorCategory::Semantic,
            3000..=3999 => ErrorCategory::Protocol,
            _ => ErrorCategory::Other,
        }
    }
}
```

### 2.2 源码映射实现

```rust
// src/source_map.rs
use std::sync::Arc;
use parking_lot::RwLock;

pub struct SourceMap {
    files: Arc<RwLock<FxHashMap<FileId, Arc<SourceFile>>>>,
    interner: StringInterner,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            files: Arc::new(RwLock::new(FxHashMap::default())),
            interner: StringInterner::new(),
        }
    }
    
    pub fn load_file(&self, path: &Path) -> io::Result<FileId> {
        let content = fs::read_to_string(path)?;
        let file_id = FileId::new();
        
        let source_file = SourceFile::new(
            path.to_path_buf(),
            content,
            self.interner.clone(),
        );
        
        self.files.write().insert(file_id, Arc::new(source_file));
        Ok(file_id)
    }
    
    pub fn lookup_char_pos(&self, pos: BytePos) -> Option<Loc> {
        let files = self.files.read();
        
        for (_, file) in files.iter() {
            if file.contains(pos) {
                return Some(file.lookup_pos(pos));
            }
        }
        
        None
    }
    
    pub fn span_to_snippet(&self, span: Span) -> Option<String> {
        let files = self.files.read();
        let file = files.get(&span.file_id)?;
        
        let start = span.lo.0 as usize;
        let end = span.hi.0 as usize;
        
        Some(file.content[start..end].to_string())
    }
}

/// 高效的行位置缓存
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
    /// 每行的起始字节位置
    line_starts: Vec<BytePos>,
    /// 多字节字符位置缓存
    multibyte_chars: Vec<(BytePos, usize)>,
}

impl SourceFile {
    fn new(path: PathBuf, content: String, _interner: StringInterner) -> Self {
        let line_starts = Self::compute_line_starts(&content);
        let multibyte_chars = Self::find_multibyte_chars(&content);
        
        Self {
            path,
            content,
            line_starts,
            multibyte_chars,
        }
    }
    
    fn compute_line_starts(content: &str) -> Vec<BytePos> {
        std::iter::once(BytePos(0))
            .chain(content.match_indices('\n')
                .map(|(i, _)| BytePos(i as u32 + 1)))
            .collect()
    }
    
    fn lookup_pos(&self, pos: BytePos) -> Loc {
        let line = self.lookup_line(pos);
        let line_start = self.line_starts[line];
        let col = self.compute_col(line_start, pos);
        
        Loc {
            file: self.path.clone(),
            line: line + 1, // 1-based
            col: col + 1,   // 1-based
        }
    }
}
```

### 2.3 诊断构建器实现

```rust
// src/diagnostic.rs
pub struct DiagnosticBuilder<'a> {
    handler: &'a DiagnosticHandler,
    diagnostic: Diagnostic,
    committed: bool,
}

impl<'a> DiagnosticBuilder<'a> {
    pub(crate) fn new(handler: &'a DiagnosticHandler, level: Level, message: String) -> Self {
        Self {
            handler,
            diagnostic: Diagnostic {
                level,
                message,
                code: None,
                spans: Vec::new(),
                children: Vec::new(),
                suggestions: Vec::new(),
            },
            committed: false,
        }
    }
    
    pub fn span_label(mut self, span: Span, label: impl Into<String>) -> Self {
        self.diagnostic.spans.push(SpanLabel {
            span,
            label: label.into(),
            style: SpanStyle::Primary,
        });
        self
    }
    
    pub fn multipart_suggestion(
        mut self,
        msg: impl Into<String>,
        suggestion: Vec<(Span, String)>,
        applicability: Applicability,
    ) -> Self {
        self.diagnostic.suggestions.push(CodeSuggestion {
            message: msg.into(),
            substitutions: suggestion.into_iter()
                .map(|(span, code)| Substitution { span, code })
                .collect(),
            applicability,
        });
        self
    }
    
    pub fn emit(mut self) {
        self.committed = true;
        self.handler.emit_diagnostic(self.diagnostic);
    }
}

impl<'a> Drop for DiagnosticBuilder<'a> {
    fn drop(&mut self) {
        if !self.committed {
            // 自动提交未显式 emit 的诊断
            self.handler.emit_diagnostic(self.diagnostic.clone());
        }
    }
}
```

### 2.4 终端输出实现

```rust
// src/emitter/terminal.rs
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct TerminalEmitter {
    dst: StandardStream,
    source_map: Arc<SourceMap>,
    config: TerminalConfig,
}

impl TerminalEmitter {
    pub fn new(source_map: Arc<SourceMap>, config: TerminalConfig) -> Self {
        let dst = StandardStream::stderr(config.color_choice);
        Self { dst, source_map, config }
    }
    
    fn emit_diagnostic(&mut self, diag: &Diagnostic) -> io::Result<()> {
        // 输出诊断头
        self.emit_header(diag)?;
        
        // 输出代码片段
        for span_label in &diag.spans {
            self.emit_span(span_label)?;
        }
        
        // 输出子诊断
        for child in &diag.children {
            self.emit_sub_diagnostic(child)?;
        }
        
        // 输出建议
        for suggestion in &diag.suggestions {
            self.emit_suggestion(suggestion)?;
        }
        
        Ok(())
    }
    
    fn emit_header(&mut self, diag: &Diagnostic) -> io::Result<()> {
        // 设置颜色
        let color = match diag.level {
            Level::Error => Color::Red,
            Level::Warning => Color::Yellow,
            Level::Info => Color::Blue,
            Level::Note => Color::Green,
            Level::Help => Color::Cyan,
        };
        
        self.dst.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
        write!(self.dst, "{}: ", diag.level)?;
        
        self.dst.reset()?;
        self.dst.set_color(ColorSpec::new().set_bold(true))?;
        writeln!(self.dst, "{}", diag.message)?;
        self.dst.reset()?;
        
        // 输出错误代码
        if let Some(code) = &diag.code {
            write!(self.dst, "  ")?;
            self.dst.set_color(ColorSpec::new().set_bold(true))?;
            writeln!(self.dst, "[{}{}]", code.prefix, code.number)?;
            self.dst.reset()?;
        }
        
        Ok(())
    }
    
    fn emit_span(&mut self, span_label: &SpanLabel) -> io::Result<()> {
        let loc = self.source_map.lookup_char_pos(span_label.span.lo)?;
        
        // 输出文件位置
        self.dst.set_color(ColorSpec::new().set_bold(true))?;
        write!(self.dst, "--> ")?;
        self.dst.reset()?;
        writeln!(self.dst, "{}:{}:{}", loc.file.display(), loc.line, loc.col)?;
        
        // 获取代码上下文
        let context = self.get_span_context(&span_label.span)?;
        
        // 输出行号边栏
        let line_num_width = context.lines.last()
            .map(|l| l.line_num.to_string().len())
            .unwrap_or(1);
        
        for (i, line) in context.lines.iter().enumerate() {
            // 行号
            write!(self.dst, "{:>width$} | ", line.line_num, width = line_num_width)?;
            
            // 代码行
            writeln!(self.dst, "{}", line.text)?;
            
            // 错误标记
            if i == context.primary_line {
                write!(self.dst, "{:>width$} | ", "", width = line_num_width)?;
                
                let start_col = context.start_col;
                let end_col = context.end_col;
                
                // 输出指示箭头
                for i in 0..line.text.len() {
                    if i >= start_col && i < end_col {
                        self.dst.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                        write!(self.dst, "^")?;
                        self.dst.reset()?;
                    } else {
                        write!(self.dst, " ")?;
                    }
                }
                
                // 输出标签
                if !span_label.label.is_empty() {
                    self.dst.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                    writeln!(self.dst, " {}", span_label.label)?;
                    self.dst.reset()?;
                } else {
                    writeln!(self.dst)?;
                }
            }
        }
        
        Ok(())
    }
}
```

### 2.5 快速修复实现

```rust
// src/quick_fix.rs
pub trait QuickFix: Send + Sync {
    fn id(&self) -> &'static str;
    fn applicable(&self, diag: &Diagnostic) -> bool;
    fn generate(&self, diag: &Diagnostic, ctx: &FixContext) -> Vec<CodeSuggestion>;
}

/// 添加缺失字段的快速修复
pub struct AddMissingFieldFix;

impl QuickFix for AddMissingFieldFix {
    fn id(&self) -> &'static str {
        "add_missing_field"
    }
    
    fn applicable(&self, diag: &Diagnostic) -> bool {
        diag.code == Some(E0101) // 缺失必需字段
    }
    
    fn generate(&self, diag: &Diagnostic, ctx: &FixContext) -> Vec<CodeSuggestion> {
        let field_name = extract_field_name(diag);
        let field_type = infer_field_type(ctx, &field_name);
        
        vec![CodeSuggestion {
            message: format!("Add missing field `{}`", field_name),
            substitutions: vec![Substitution {
                span: find_insertion_point(ctx),
                code: format!("    {}: {},\n", field_name, field_type),
            }],
            applicability: Applicability::MaybeIncorrect,
        }]
    }
}

/// 修复管理器
pub struct FixRegistry {
    fixes: Vec<Box<dyn QuickFix>>,
}

impl FixRegistry {
    pub fn new() -> Self {
        let mut registry = Self { fixes: Vec::new() };
        
        // 注册内置修复
        registry.register(Box::new(AddMissingFieldFix));
        registry.register(Box::new(RemoveUnusedImportFix));
        registry.register(Box::new(RenameTypoFix));
        
        registry
    }
    
    pub fn suggest_fixes(&self, diag: &Diagnostic, ctx: &FixContext) -> Vec<CodeSuggestion> {
        self.fixes.iter()
            .filter(|fix| fix.applicable(diag))
            .flat_map(|fix| fix.generate(diag, ctx))
            .collect()
    }
}
```

## 3. 集成示例

### 3.1 在编译器中使用

```rust
// 在 pilota-build 中集成
pub struct CompilerSession {
    handler: DiagnosticHandler,
    source_map: Arc<SourceMap>,
}

impl CompilerSession {
    pub fn new() -> Self {
        let source_map = Arc::new(SourceMap::new());
        let emitter = Box::new(TerminalEmitter::new(
            source_map.clone(),
            TerminalConfig::default(),
        ));
        
        let handler = DiagnosticHandler::with_emitter(emitter);
        
        Self { handler, source_map }
    }
    
    pub fn compile(&mut self, input: &Path) -> Result<(), ErrorReported> {
        // 加载文件
        let file_id = self.source_map.load_file(input)
            .map_err(|e| {
                self.handler.error(&format!("Failed to read file: {}", e))
                    .emit();
                ErrorReported
            })?;
        
        // 解析
        let ast = match parse_file(file_id, &self.source_map) {
            Ok(ast) => ast,
            Err(errors) => {
                for error in errors {
                    self.emit_parse_error(error);
                }
                return Err(ErrorReported);
            }
        };
        
        // 类型检查
        match typecheck(&ast, &self.handler) {
            Ok(mir) => {
                // 继续编译...
            }
            Err(ErrorReported) => {
                return Err(ErrorReported);
            }
        }
        
        Ok(())
    }
    
    fn emit_parse_error(&self, error: ParseError) {
        self.handler.struct_span_err(error.span, &error.message)
            .code(E0001)
            .span_label(error.span, "expected identifier here")
            .help("valid identifiers start with a letter or underscore")
            .emit();
    }
}
```

### 3.2 错误恢复示例

```rust
// 解析器中的错误恢复
impl Parser {
    fn parse_field(&mut self) -> Result<Field, ParseError> {
        // 期望字段 ID
        let id = match self.parse_field_id() {
            Ok(id) => id,
            Err(e) => {
                // 报告错误
                self.handler.struct_span_err(self.span, "expected field ID")
                    .span_label(self.span, "field ID missing")
                    .help("field IDs must be positive integers")
                    .emit();
                
                // 尝试恢复：跳过到下一个有效 token
                self.recover_to_field_start();
                
                // 使用占位 ID
                -1
            }
        };
        
        // 继续解析...
    }
    
    fn recover_to_field_start(&mut self) {
        loop {
            match self.current_token {
                Token::Int(_) | Token::Ident(_) | Token::RBrace => break,
                Token::Eof => break,
                _ => self.advance(),
            }
        }
    }
}
```

## 4. 测试

### 4.1 UI 测试

```rust
// tests/ui/errors/duplicate_field_id.rs
struct Message {
    1: i32 field1,
    1: i32 field2, //~ ERROR duplicate field ID
    //~^ HELP use a different ID
}

// tests/ui/errors/duplicate_field_id.stderr
error[E0001]: duplicate field ID in message
 --> tests/ui/errors/duplicate_field_id.rs:3:5
  |
2 |     1: i32 field1,
  |     - first use of ID `1`
3 |     1: i32 field2,
  |     ^ duplicate field ID
  |
  = help: use a different ID for this field
```

### 4.2 集成测试

```rust
#[test]
fn test_error_recovery() {
    let input = r#"
        struct Test {
            1: i32 field1
            // 缺少逗号
            2: i32 field2
        }
    "#;
    
    let session = CompilerSession::new();
    let result = session.compile_string(input);
    
    // 应该有一个错误但仍能继续
    assert_eq!(session.handler.err_count(), 1);
    assert!(result.is_ok());
}
```

## 5. 性能优化

### 5.1 错误去重

```rust
impl DiagnosticHandler {
    fn should_emit(&self, diag: &Diagnostic) -> bool {
        let hash = self.hash_diagnostic(diag);
        self.emitted.insert(hash)
    }
    
    fn hash_diagnostic(&self, diag: &Diagnostic) -> u64 {
        let mut hasher = DefaultHasher::new();
        diag.code.hash(&mut hasher);
        diag.message.hash(&mut hasher);
        
        for span in &diag.spans {
            span.span.hash(&mut hasher);
        }
        
        hasher.finish()
    }
}
```

### 5.2 批量处理

```rust
pub struct BatchedDiagnostics {
    diagnostics: Vec<Diagnostic>,
    handler: DiagnosticHandler,
}

impl BatchedDiagnostics {
    pub fn add(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }
    
    pub fn emit_all(self) {
        // 按文件和行号排序
        let mut diags = self.diagnostics;
        diags.sort_by_key(|d| {
            d.spans.first()
                .map(|s| (s.span.file_id, s.span.lo))
                .unwrap_or_default()
        });
        
        // 批量输出
        for diag in diags {
            self.handler.emit_diagnostic(diag);
        }
    }
}
```