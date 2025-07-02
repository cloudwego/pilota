# 错误处理与诊断系统设计

## 1. 设计目标

### 1.1 核心需求

1. **精确定位**：准确指出错误位置，包括文件、行号、列号
2. **友好提示**：提供清晰的错误描述和修复建议
3. **错误恢复**：遇到错误后能继续编译，发现更多问题
4. **上下文感知**：根据上下文提供相关信息
5. **可扩展性**：易于添加新的错误类型和诊断规则

### 1.2 设计原则

- **用户优先**：错误信息对用户友好，而非编译器内部表示
- **渐进式报告**：先报告最重要的错误，避免错误雪崩
- **结构化数据**：错误信息结构化，便于工具集成
- **国际化支持**：支持多语言错误信息

## 2. 错误类型体系

### 2.1 错误分类

```rust
/// 错误等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    /// 内部编译器错误
    Ice,
    /// 致命错误（停止编译）
    Fatal,
    /// 错误
    Error,
    /// 警告
    Warning,
    /// 提示
    Note,
    /// 建议
    Suggestion,
    /// 帮助信息
    Help,
}

/// 错误类别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// 语法错误
    Syntax,
    /// 类型错误
    Type,
    /// 名称解析错误
    Resolution,
    /// 语义错误
    Semantic,
    /// 协议规范错误
    Protocol,
    /// 代码生成错误
    Codegen,
    /// 插件错误
    Plugin,
    /// IO 错误
    Io,
}

/// 诊断信息
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// 错误级别
    pub level: DiagnosticLevel,
    /// 错误类别
    pub category: ErrorCategory,
    /// 错误代码
    pub code: ErrorCode,
    /// 主要信息
    pub message: String,
    /// 位置信息
    pub spans: Vec<SpanLabel>,
    /// 子诊断信息
    pub children: Vec<SubDiagnostic>,
    /// 修复建议
    pub suggestions: Vec<CodeSuggestion>,
    /// 相关链接
    pub links: Vec<String>,
}

/// 子诊断信息
#[derive(Debug, Clone)]
pub struct SubDiagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub span: Option<Span>,
}
```

### 2.2 错误代码系统

```rust
/// 错误代码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCode {
    /// 错误类别前缀
    pub prefix: &'static str,
    /// 错误编号
    pub number: u32,
}

impl ErrorCode {
    pub const E0001: Self = Self { prefix: "E", number: 1 };
    pub const E0002: Self = Self { prefix: "E", number: 2 };
    // ... 更多错误代码
    
    /// 获取错误文档链接
    pub fn doc_url(&self) -> String {
        format!("https://pilota.dev/errors/{}{:04}", self.prefix, self.number)
    }
}

/// 错误代码注册表
pub struct ErrorRegistry {
    codes: FxHashMap<ErrorCode, ErrorInfo>,
}

pub struct ErrorInfo {
    pub code: ErrorCode,
    pub name: &'static str,
    pub description: &'static str,
    pub example: Option<&'static str>,
    pub explanation: &'static str,
}

impl ErrorRegistry {
    /// 注册错误代码
    pub fn register(&mut self, info: ErrorInfo) {
        self.codes.insert(info.code, info);
    }
    
    /// 获取错误信息
    pub fn get(&self, code: ErrorCode) -> Option<&ErrorInfo> {
        self.codes.get(&code)
    }
}
```

## 3. 位置追踪系统

### 3.1 源码位置

```rust
/// 源码范围
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// 起始位置
    pub start: Position,
    /// 结束位置
    pub end: Position,
    /// 文件 ID
    pub file_id: FileId,
}

/// 位置信息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// 字节偏移
    pub offset: usize,
    /// 行号（1-based）
    pub line: u32,
    /// 列号（1-based）
    pub column: u32,
}

/// 带标签的位置
#[derive(Debug, Clone)]
pub struct SpanLabel {
    pub span: Span,
    pub label: String,
    pub style: SpanStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanStyle {
    /// 主要错误位置
    Primary,
    /// 次要位置
    Secondary,
    /// 帮助信息位置
    Help,
}

/// 源码管理器
pub struct SourceMap {
    files: FxHashMap<FileId, SourceFile>,
}

pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
    /// 行起始位置索引
    line_starts: Vec<usize>,
}

impl SourceMap {
    /// 获取源码片段
    pub fn span_snippet(&self, span: Span) -> Result<String, SourceMapError> {
        let file = self.files.get(&span.file_id)
            .ok_or(SourceMapError::FileNotFound)?;
        
        let start = span.start.offset;
        let end = span.end.offset;
        
        Ok(file.content[start..end].to_string())
    }
    
    /// 获取行内容
    pub fn line_content(&self, file_id: FileId, line: u32) -> Option<&str> {
        let file = self.files.get(&file_id)?;
        let line_idx = (line - 1) as usize;
        
        if line_idx >= file.line_starts.len() {
            return None;
        }
        
        let start = file.line_starts[line_idx];
        let end = file.line_starts.get(line_idx + 1)
            .copied()
            .unwrap_or(file.content.len());
        
        Some(&file.content[start..end])
    }
}
```

### 3.2 宏展开追踪

```rust
/// 宏展开信息
#[derive(Debug, Clone)]
pub struct ExpansionInfo {
    /// 调用位置
    pub call_site: Span,
    /// 定义位置
    pub def_site: Option<Span>,
    /// 宏名称
    pub macro_name: Symbol,
    /// 展开深度
    pub depth: u32,
}

/// 展开上下文
pub struct ExpansionContext {
    /// 展开栈
    expansions: Vec<ExpansionInfo>,
    /// 最大展开深度
    max_depth: u32,
}

impl ExpansionContext {
    /// 进入宏展开
    pub fn enter_expansion(&mut self, info: ExpansionInfo) -> Result<(), ExpansionError> {
        if self.expansions.len() >= self.max_depth as usize {
            return Err(ExpansionError::DepthLimitExceeded);
        }
        self.expansions.push(info);
        Ok(())
    }
    
    /// 退出宏展开
    pub fn exit_expansion(&mut self) {
        self.expansions.pop();
    }
    
    /// 获取完整的展开栈
    pub fn expansion_stack(&self) -> &[ExpansionInfo] {
        &self.expansions
    }
}
```

## 4. 诊断构建器

### 4.1 流式 API

```rust
/// 诊断构建器
pub struct DiagnosticBuilder<'a> {
    handler: &'a DiagnosticHandler,
    diagnostic: Diagnostic,
}

impl<'a> DiagnosticBuilder<'a> {
    /// 设置主要信息
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.diagnostic.message = message.into();
        self
    }
    
    /// 添加错误代码
    pub fn with_code(mut self, code: ErrorCode) -> Self {
        self.diagnostic.code = code;
        self
    }
    
    /// 添加主要位置
    pub fn span(mut self, span: Span) -> Self {
        self.diagnostic.spans.push(SpanLabel {
            span,
            label: String::new(),
            style: SpanStyle::Primary,
        });
        self
    }
    
    /// 添加带标签的位置
    pub fn span_label(mut self, span: Span, label: impl Into<String>) -> Self {
        self.diagnostic.spans.push(SpanLabel {
            span,
            label: label.into(),
            style: SpanStyle::Primary,
        });
        self
    }
    
    /// 添加次要位置
    pub fn span_note(mut self, span: Span, note: impl Into<String>) -> Self {
        self.diagnostic.spans.push(SpanLabel {
            span,
            label: note.into(),
            style: SpanStyle::Secondary,
        });
        self
    }
    
    /// 添加帮助信息
    pub fn help(mut self, message: impl Into<String>) -> Self {
        self.diagnostic.children.push(SubDiagnostic {
            level: DiagnosticLevel::Help,
            message: message.into(),
            span: None,
        });
        self
    }
    
    /// 添加注释
    pub fn note(mut self, message: impl Into<String>) -> Self {
        self.diagnostic.children.push(SubDiagnostic {
            level: DiagnosticLevel::Note,
            message: message.into(),
            span: None,
        });
        self
    }
    
    /// 添加修复建议
    pub fn suggest(
        mut self,
        message: impl Into<String>,
        suggestion: CodeSuggestion,
    ) -> Self {
        self.diagnostic.suggestions.push(suggestion);
        self
    }
    
    /// 发出诊断信息
    pub fn emit(self) {
        self.handler.emit_diagnostic(self.diagnostic);
    }
}
```

### 4.2 代码建议

```rust
/// 代码修复建议
#[derive(Debug, Clone)]
pub struct CodeSuggestion {
    /// 建议描述
    pub message: String,
    /// 替换操作
    pub substitutions: Vec<Substitution>,
    /// 应用策略
    pub applicability: Applicability,
}

/// 代码替换
#[derive(Debug, Clone)]
pub struct Substitution {
    /// 替换位置
    pub span: Span,
    /// 替换内容
    pub replacement: String,
}

/// 建议的适用性
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applicability {
    /// 可以自动应用
    MachineApplicable,
    /// 可能正确
    MaybeIncorrect,
    /// 有语义变化
    HasPlaceholders,
    /// 仅供参考
    Unspecified,
}

/// 快速修复
pub trait QuickFix {
    /// 是否适用
    fn is_applicable(&self, diagnostic: &Diagnostic) -> bool;
    
    /// 生成修复建议
    fn suggest(&self, diagnostic: &Diagnostic) -> Vec<CodeSuggestion>;
}

/// 快速修复注册表
pub struct QuickFixRegistry {
    fixes: Vec<Box<dyn QuickFix>>,
}

impl QuickFixRegistry {
    /// 注册快速修复
    pub fn register(&mut self, fix: Box<dyn QuickFix>) {
        self.fixes.push(fix);
    }
    
    /// 获取适用的修复
    pub fn get_fixes(&self, diagnostic: &Diagnostic) -> Vec<CodeSuggestion> {
        self.fixes.iter()
            .filter(|fix| fix.is_applicable(diagnostic))
            .flat_map(|fix| fix.suggest(diagnostic))
            .collect()
    }
}
```

## 5. 错误处理器

### 5.1 诊断处理器

```rust
/// 诊断处理器
pub struct DiagnosticHandler {
    /// 错误计数
    error_count: AtomicUsize,
    /// 警告计数
    warning_count: AtomicUsize,
    /// 发射器
    emitter: Box<dyn Emitter>,
    /// 延迟的诊断信息
    delayed: Mutex<Vec<Diagnostic>>,
    /// 配置
    config: DiagnosticConfig,
}

/// 诊断配置
pub struct DiagnosticConfig {
    /// 最大错误数
    pub max_errors: Option<usize>,
    /// 是否显示警告
    pub show_warnings: bool,
    /// 将警告视为错误
    pub warnings_as_errors: bool,
    /// 错误格式
    pub format: OutputFormat,
    /// 颜色输出
    pub color: ColorChoice,
}

impl DiagnosticHandler {
    /// 创建错误构建器
    pub fn error(&self, message: impl Into<String>) -> DiagnosticBuilder {
        self.build(DiagnosticLevel::Error, message)
    }
    
    /// 创建警告构建器
    pub fn warning(&self, message: impl Into<String>) -> DiagnosticBuilder {
        self.build(DiagnosticLevel::Warning, message)
    }
    
    /// 发出诊断信息
    pub fn emit_diagnostic(&self, mut diagnostic: Diagnostic) {
        // 应用配置
        if self.config.warnings_as_errors && diagnostic.level == DiagnosticLevel::Warning {
            diagnostic.level = DiagnosticLevel::Error;
        }
        
        // 更新计数
        match diagnostic.level {
            DiagnosticLevel::Error | DiagnosticLevel::Fatal => {
                self.error_count.fetch_add(1, Ordering::Relaxed);
            }
            DiagnosticLevel::Warning => {
                self.warning_count.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }
        
        // 检查错误限制
        if let Some(max) = self.config.max_errors {
            if self.error_count.load(Ordering::Relaxed) > max {
                return;
            }
        }
        
        // 发送到发射器
        self.emitter.emit(&diagnostic);
    }
    
    /// 延迟诊断
    pub fn delay_diagnostic(&self, diagnostic: Diagnostic) {
        self.delayed.lock().unwrap().push(diagnostic);
    }
    
    /// 刷新延迟的诊断
    pub fn flush_delayed(&self) {
        let diagnostics = self.delayed.lock().unwrap().drain(..).collect::<Vec<_>>();
        for diagnostic in diagnostics {
            self.emit_diagnostic(diagnostic);
        }
    }
}
```

### 5.2 错误发射器

```rust
/// 错误发射器 trait
pub trait Emitter: Send + Sync {
    fn emit(&self, diagnostic: &Diagnostic);
}

/// 终端发射器
pub struct TerminalEmitter {
    source_map: Arc<SourceMap>,
    config: TerminalConfig,
}

pub struct TerminalConfig {
    pub width: usize,
    pub color: bool,
    pub show_line_numbers: bool,
    pub context_lines: usize,
}

impl Emitter for TerminalEmitter {
    fn emit(&self, diagnostic: &Diagnostic) {
        let mut output = String::new();
        
        // 格式化诊断头
        self.format_header(&mut output, diagnostic);
        
        // 格式化代码片段
        for span_label in &diagnostic.spans {
            self.format_snippet(&mut output, span_label);
        }
        
        // 格式化子诊断
        for child in &diagnostic.children {
            self.format_sub_diagnostic(&mut output, child);
        }
        
        // 格式化建议
        for suggestion in &diagnostic.suggestions {
            self.format_suggestion(&mut output, suggestion);
        }
        
        // 输出到终端
        eprintln!("{}", output);
    }
}

/// JSON 发射器（用于 IDE 集成）
pub struct JsonEmitter {
    pretty: bool,
}

impl Emitter for JsonEmitter {
    fn emit(&self, diagnostic: &Diagnostic) {
        let json = if self.pretty {
            serde_json::to_string_pretty(diagnostic).unwrap()
        } else {
            serde_json::to_string(diagnostic).unwrap()
        };
        println!("{}", json);
    }
}
```

## 6. 错误恢复机制

### 6.1 解析错误恢复

```rust
/// 错误恢复策略
pub enum RecoveryStrategy {
    /// 跳过到特定 token
    SkipUntil(TokenSet),
    /// 插入缺失的 token
    InsertToken(Token),
    /// 删除当前 token
    DeleteCurrent,
    /// 替换 token
    Replace(Token),
}

/// 解析器错误恢复
pub struct ParserRecovery {
    /// 同步点
    sync_tokens: TokenSet,
    /// 恢复策略
    strategies: Vec<RecoveryStrategy>,
}

impl ParserRecovery {
    /// 尝试恢复
    pub fn try_recover(
        &self,
        parser: &mut Parser,
        error: ParseError,
    ) -> Result<(), ParseError> {
        // 记录错误
        parser.handler.error(&error.message)
            .span(error.span)
            .emit();
        
        // 尝试各种恢复策略
        for strategy in &self.strategies {
            if self.apply_strategy(parser, strategy) {
                return Ok(());
            }
        }
        
        // 同步到安全点
        self.sync_to_safe_point(parser);
        Ok(())
    }
    
    /// 同步到安全点
    fn sync_to_safe_point(&self, parser: &mut Parser) {
        while !parser.at_eof() && !self.sync_tokens.contains(parser.current()) {
            parser.advance();
        }
    }
}
```

### 6.2 类型错误恢复

```rust
/// 类型错误恢复
pub struct TypeErrorRecovery {
    /// 类型推导上下文
    infcx: InferCtxt,
}

impl TypeErrorRecovery {
    /// 恢复类型错误
    pub fn recover_type_error(
        &mut self,
        expected: Type,
        found: Type,
        span: Span,
    ) -> Type {
        // 报告错误
        self.infcx.handler.error("type mismatch")
            .span_label(span, format!("expected `{}`, found `{}`", expected, found))
            .note(format!("expected type: {}", self.infcx.display_type(&expected)))
            .note(format!("found type: {}", self.infcx.display_type(&found)))
            .emit();
        
        // 尝试类型强制转换
        if let Some(coerced) = self.try_coerce(&found, &expected) {
            return coerced;
        }
        
        // 返回期望类型以继续检查
        expected
    }
    
    /// 尝试类型强制转换
    fn try_coerce(&self, from: &Type, to: &Type) -> Option<Type> {
        // 实现各种强制转换规则
        match (from, to) {
            // &T -> &U if T: Deref<Target=U>
            (Type::Ref(_, t1, _), Type::Ref(_, t2, _)) => {
                if self.infcx.can_deref(t1, t2) {
                    return Some(to.clone());
                }
            }
            // ... 其他强制转换
            _ => {}
        }
        None
    }
}
```

## 7. 批量错误处理

### 7.1 错误去重

```rust
/// 错误去重器
pub struct ErrorDeduplicator {
    seen: FxHashSet<u64>,
    hasher: DefaultHasher,
}

impl ErrorDeduplicator {
    /// 检查是否重复
    pub fn is_duplicate(&mut self, diagnostic: &Diagnostic) -> bool {
        let hash = self.hash_diagnostic(diagnostic);
        !self.seen.insert(hash)
    }
    
    /// 计算诊断哈希
    fn hash_diagnostic(&self, diagnostic: &Diagnostic) -> u64 {
        let mut hasher = self.hasher.clone();
        
        // 哈希关键信息
        diagnostic.code.hash(&mut hasher);
        diagnostic.message.hash(&mut hasher);
        
        // 哈希主要位置
        for span in &diagnostic.spans {
            if span.style == SpanStyle::Primary {
                span.span.hash(&mut hasher);
            }
        }
        
        hasher.finish()
    }
}
```

### 7.2 错误分组

```rust
/// 错误分组器
pub struct ErrorGrouper {
    groups: FxHashMap<ErrorGroup, Vec<Diagnostic>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorGroup {
    pub category: ErrorCategory,
    pub location: Option<FileId>,
    pub related_to: Option<DefId>,
}

impl ErrorGrouper {
    /// 添加诊断
    pub fn add(&mut self, diagnostic: Diagnostic) {
        let group = self.determine_group(&diagnostic);
        self.groups.entry(group).or_default().push(diagnostic);
    }
    
    /// 获取分组的诊断
    pub fn grouped_diagnostics(&self) -> Vec<(ErrorGroup, Vec<Diagnostic>)> {
        self.groups.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .sorted_by_key(|(g, _)| (g.category, g.location))
            .collect()
    }
    
    /// 确定分组
    fn determine_group(&self, diagnostic: &Diagnostic) -> ErrorGroup {
        ErrorGroup {
            category: diagnostic.category,
            location: diagnostic.spans.first().map(|s| s.span.file_id),
            related_to: self.extract_related_def(diagnostic),
        }
    }
}
```

## 8. IDE 集成

### 8.1 LSP 诊断

```rust
/// LSP 诊断适配器
pub struct LspDiagnosticAdapter {
    source_map: Arc<SourceMap>,
}

impl LspDiagnosticAdapter {
    /// 转换为 LSP 诊断
    pub fn to_lsp_diagnostic(&self, diagnostic: &Diagnostic) -> lsp_types::Diagnostic {
        lsp_types::Diagnostic {
            range: self.to_lsp_range(diagnostic.spans[0].span),
            severity: Some(self.to_lsp_severity(diagnostic.level)),
            code: Some(lsp_types::NumberOrString::String(
                format!("{}{:04}", diagnostic.code.prefix, diagnostic.code.number)
            )),
            source: Some("pilota".to_string()),
            message: diagnostic.message.clone(),
            related_information: Some(self.to_related_information(diagnostic)),
            tags: self.determine_tags(diagnostic),
        }
    }
    
    /// 转换范围
    fn to_lsp_range(&self, span: Span) -> lsp_types::Range {
        lsp_types::Range {
            start: self.to_lsp_position(span.start),
            end: self.to_lsp_position(span.end),
        }
    }
    
    /// 转换位置
    fn to_lsp_position(&self, pos: Position) -> lsp_types::Position {
        lsp_types::Position {
            line: pos.line - 1,
            character: pos.column - 1,
        }
    }
}
```

## 9. 总结

新的错误处理和诊断系统提供了：

1. **精确的错误定位**：完整的源码位置追踪
2. **友好的错误信息**：结构化的诊断信息和修复建议
3. **强大的错误恢复**：多种恢复策略，最大化发现错误
4. **灵活的输出格式**：支持终端、JSON、IDE 集成
5. **智能的错误管理**：去重、分组、批量处理

这个设计为用户提供了一流的错误体验，帮助快速定位和修复问题。