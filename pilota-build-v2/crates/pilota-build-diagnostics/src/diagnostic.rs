//! Core diagnostic types.

use crate::error_code::ErrorCode;
use pilota_build_common::Span;
use serde::{Deserialize, Serialize};

/// The severity of a diagnostic.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

impl Severity {
    pub fn to_str(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Note => "note",
            Severity::Help => "help",
        }
    }
}

/// The level of a diagnostic (for backwards compatibility).
pub type Level = Severity;

/// A diagnostic message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: Level,
    pub message: String,
    pub code: Option<ErrorCode>,
    pub spans: Vec<SpanLabel>,
    pub children: Vec<SubDiagnostic>,
    pub suggestions: Vec<CodeSuggestion>,
}

impl Diagnostic {
    /// Create a new error diagnostic.
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(Level::Error, message)
    }

    /// Create a new warning diagnostic.
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(Level::Warning, message)
    }

    /// Create a new info diagnostic.
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(Level::Info, message)
    }

    /// Create a new diagnostic with the given level.
    pub fn new(level: Level, message: impl Into<String>) -> Self {
        Diagnostic {
            level,
            message: message.into(),
            code: None,
            spans: Vec::new(),
            children: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Set the error code.
    pub fn code(mut self, code: ErrorCode) -> Self {
        self.code = Some(code);
        self
    }

    /// Add a span with a label.
    pub fn span_label(mut self, span: Span, label: impl Into<String>) -> Self {
        self.spans.push(SpanLabel {
            span,
            label: label.into(),
            style: SpanStyle::Primary,
        });
        self
    }

    /// Add a note.
    pub fn note(mut self, message: impl Into<String>) -> Self {
        self.children.push(SubDiagnostic {
            level: Level::Note,
            message: message.into(),
            spans: Vec::new(),
        });
        self
    }

    /// Add a help message.
    pub fn help(mut self, message: impl Into<String>) -> Self {
        self.children.push(SubDiagnostic {
            level: Level::Help,
            message: message.into(),
            spans: Vec::new(),
        });
        self
    }

    /// Add a code suggestion.
    pub fn suggestion(
        mut self,
        message: impl Into<String>,
        span: Span,
        replacement: impl Into<String>,
        applicability: Applicability,
    ) -> Self {
        self.suggestions.push(CodeSuggestion {
            message: message.into(),
            substitutions: vec![Substitution {
                span,
                code: replacement.into(),
            }],
            applicability,
        });
        self
    }

    /// Check if this is an error.
    pub fn is_error(&self) -> bool {
        self.level == Level::Error
    }
}

/// A span with a label.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpanLabel {
    pub span: Span,
    pub label: String,
    pub style: SpanStyle,
}

/// The style of a span.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum SpanStyle {
    Primary,
    Secondary,
}

/// A sub-diagnostic message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubDiagnostic {
    pub level: Level,
    pub message: String,
    pub spans: Vec<SpanLabel>,
}

/// A code suggestion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeSuggestion {
    pub message: String,
    pub substitutions: Vec<Substitution>,
    pub applicability: Applicability,
}

/// A substitution in a code suggestion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Substitution {
    pub span: Span,
    pub code: String,
}

/// The applicability of a suggestion.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Applicability {
    /// The suggestion is definitely what the user intended.
    MachineApplicable,
    /// The suggestion may be what the user intended.
    MaybeIncorrect,
    /// The suggestion contains placeholders.
    HasPlaceholders,
    /// The suggestion is probably not what the user intended.
    Unspecified,
}

/// A diagnostic builder for ergonomic construction.
pub struct DiagnosticBuilder<'a> {
    handler: &'a crate::DiagnosticHandler,
    diagnostic: Diagnostic,
    committed: bool,
}

impl<'a> DiagnosticBuilder<'a> {
    pub(crate) fn new(
        handler: &'a crate::DiagnosticHandler,
        level: Level,
        message: String,
    ) -> Self {
        DiagnosticBuilder {
            handler,
            diagnostic: Diagnostic::new(level, message),
            committed: false,
        }
    }

    /// Set the error code.
    pub fn code(&mut self, code: ErrorCode) -> &mut Self {
        self.diagnostic.code = Some(code);
        self
    }

    /// Add a span with a label.
    pub fn span_label(&mut self, span: Span, label: impl Into<String>) -> &mut Self {
        self.diagnostic.spans.push(SpanLabel {
            span,
            label: label.into(),
            style: SpanStyle::Primary,
        });
        self
    }

    /// Add a note.
    pub fn note(&mut self, message: impl Into<String>) -> &mut Self {
        self.diagnostic.children.push(SubDiagnostic {
            level: Level::Note,
            message: message.into(),
            spans: Vec::new(),
        });
        self
    }

    /// Add a help message.
    pub fn help(&mut self, message: impl Into<String>) -> &mut Self {
        self.diagnostic.children.push(SubDiagnostic {
            level: Level::Help,
            message: message.into(),
            spans: Vec::new(),
        });
        self
    }

    /// Add a code suggestion.
    pub fn suggestion(
        &mut self,
        message: impl Into<String>,
        span: Span,
        replacement: impl Into<String>,
        applicability: Applicability,
    ) -> &mut Self {
        self.diagnostic.suggestions.push(CodeSuggestion {
            message: message.into(),
            substitutions: vec![Substitution {
                span,
                code: replacement.into(),
            }],
            applicability,
        });
        self
    }

    /// Emit the diagnostic.
    pub fn emit(&mut self) {
        self.committed = true;
        self.handler.emit_diagnostic(self.diagnostic.clone());
    }
}

impl<'a> Drop for DiagnosticBuilder<'a> {
    fn drop(&mut self) {
        if !self.committed {
            self.handler.emit_diagnostic(self.diagnostic.clone());
        }
    }
}