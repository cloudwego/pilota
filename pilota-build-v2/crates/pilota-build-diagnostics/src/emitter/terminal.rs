//! Terminal emitter for diagnostics.

use crate::{
    diagnostic::{Diagnostic, Level, SpanLabel, SpanStyle},
    emitter::Emitter,
    snippet::{Annotation, AnnotationType, Snippet},
};
use pilota_build_common::{SourceMap, Span};
use std::io::{self, Write};
use std::sync::Arc;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Configuration for terminal output.
#[derive(Clone, Debug)]
pub struct Config {
    pub color_choice: ColorChoice,
    pub show_code_snippets: bool,
    pub max_line_length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            color_choice: ColorChoice::Auto,
            show_code_snippets: true,
            max_line_length: 80,
        }
    }
}

/// Terminal emitter for diagnostics.
pub struct TerminalEmitter {
    dst: StandardStream,
    source_map: Arc<SourceMap>,
    config: Config,
}

impl TerminalEmitter {
    pub fn new(source_map: Arc<SourceMap>, config: Config) -> Self {
        let dst = StandardStream::stderr(config.color_choice);
        TerminalEmitter {
            dst,
            source_map,
            config,
        }
    }

    fn emit_header(&mut self, diagnostic: &Diagnostic) -> io::Result<()> {
        // Set color based on level
        let color = match diagnostic.level {
            Level::Error => Color::Red,
            Level::Warning => Color::Yellow,
            Level::Info => Color::Blue,
            Level::Note => Color::Green,
            Level::Help => Color::Cyan,
        };

        self.dst.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
        write!(self.dst, "{}", diagnostic.level.to_str())?;

        // Add error code if present
        if let Some(code) = &diagnostic.code {
            write!(self.dst, "[{}]", code)?;
        }

        self.dst.set_color(ColorSpec::new().set_bold(true))?;
        writeln!(self.dst, ": {}", diagnostic.message)?;
        self.dst.reset()?;

        Ok(())
    }

    fn emit_span(&mut self, span_label: &SpanLabel) -> io::Result<()> {
        if !self.config.show_code_snippets {
            return Ok(());
        }

        // Get source snippet
        let snippet = match self.build_snippet(&span_label.span, &span_label.label) {
            Some(snippet) => snippet,
            None => return Ok(()),
        };

        self.emit_snippet(&snippet)
    }

    fn emit_snippet(&mut self, snippet: &Snippet) -> io::Result<()> {
        // Emit file location
        self.dst.set_color(ColorSpec::new().set_bold(true))?;
        write!(self.dst, "  --> ")?;
        self.dst.reset()?;
        writeln!(self.dst, "{}", snippet.file_name)?;

        // Calculate line number width
        let max_line_num = snippet.lines.iter().map(|l| l.line_number).max().unwrap_or(1);
        let line_num_width = max_line_num.to_string().len();

        // Emit lines
        for line in &snippet.lines {
            // Line number
            self.dst.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))?;
            write!(self.dst, "{:>width$} | ", line.line_number, width = line_num_width)?;
            self.dst.reset()?;

            // Source line
            writeln!(self.dst, "{}", line.text)?;

            // Annotations
            if !line.annotations.is_empty() {
                write!(self.dst, "{:>width$} | ", "", width = line_num_width)?;
                
                let mut col = 0;
                for annotation in &line.annotations {
                    // Pad to start column
                    while col < annotation.start_col {
                        write!(self.dst, " ")?;
                        col += 1;
                    }

                    // Draw underline
                    let color = match annotation.annotation_type {
                        AnnotationType::Error => Color::Red,
                        AnnotationType::Warning => Color::Yellow,
                        AnnotationType::Info => Color::Blue,
                        AnnotationType::Note => Color::Green,
                        AnnotationType::Help => Color::Cyan,
                    };

                    self.dst.set_color(ColorSpec::new().set_fg(Some(color)))?;
                    for _ in annotation.start_col..annotation.end_col {
                        write!(self.dst, "^")?;
                        col += 1;
                    }
                    self.dst.reset()?;
                }

                // Add label if present
                if let Some(annotation) = line.annotations.first() {
                    if !annotation.label.is_empty() {
                        let color = match annotation.annotation_type {
                            AnnotationType::Error => Color::Red,
                            AnnotationType::Warning => Color::Yellow,
                            AnnotationType::Info => Color::Blue,
                            AnnotationType::Note => Color::Green,
                            AnnotationType::Help => Color::Cyan,
                        };
                        self.dst.set_color(ColorSpec::new().set_fg(Some(color)))?;
                        write!(self.dst, " {}", annotation.label)?;
                        self.dst.reset()?;
                    }
                }
                writeln!(self.dst)?;
            }
        }

        Ok(())
    }

    fn build_snippet(&self, span: &Span, label: &str) -> Option<Snippet> {
        let file = self.source_map.get_file(span.file_id)?;
        let loc_start = self.source_map.lookup_char_pos(span.lo)?;
        let loc_end = self.source_map.lookup_char_pos(span.hi)?;

        let mut snippet = Snippet {
            file_name: loc_start.file.to_string_lossy().to_string(),
            lines: Vec::new(),
        };

        // Get lines to display
        let start_line = loc_start.line.saturating_sub(1);
        let end_line = loc_end.line;

        for line_num in start_line..=end_line {
            if line_num == 0 || line_num > file.lines.len() {
                continue;
            }

            let line_start = file.lines[line_num - 1];
            let line_end = if line_num < file.lines.len() {
                file.lines[line_num]
            } else {
                file.end_pos
            };

            let line_span = Span::new(line_start, line_end, span.file_id);
            let line_text = file.snippet(line_span)?.trim_end_matches('\n');

            let mut line = crate::snippet::Line {
                line_number: line_num,
                text: line_text.to_string(),
                annotations: Vec::new(),
            };

            // Add annotation if this line contains part of the span
            if line_num >= loc_start.line && line_num <= loc_end.line {
                let start_col = if line_num == loc_start.line {
                    loc_start.col - 1
                } else {
                    0
                };
                let end_col = if line_num == loc_end.line {
                    loc_end.col - 1
                } else {
                    line_text.len()
                };

                line.annotations.push(Annotation {
                    start_col,
                    end_col,
                    label: if line_num == loc_start.line {
                        label.to_string()
                    } else {
                        String::new()
                    },
                    annotation_type: AnnotationType::Error,
                });
            }

            snippet.lines.push(line);
        }

        Some(snippet)
    }
}

impl Emitter for TerminalEmitter {
    fn emit_diagnostic(&mut self, diagnostic: &Diagnostic) {
        let _ = self.emit_header(diagnostic);

        // Emit main spans
        for span_label in &diagnostic.spans {
            let _ = self.emit_span(span_label);
        }

        // Emit children (notes, helps)
        for child in &diagnostic.children {
            let child_diag = Diagnostic {
                level: child.level,
                message: child.message.clone(),
                code: None,
                spans: child.spans.clone(),
                children: Vec::new(),
                suggestions: Vec::new(),
            };
            let _ = self.emit_header(&child_diag);
        }

        // Emit suggestions
        for suggestion in &diagnostic.suggestions {
            self.dst.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true)).ok();
            write!(self.dst, "help: ").ok();
            self.dst.reset().ok();
            writeln!(self.dst, "{}", suggestion.message).ok();

            // Show the suggested code
            for substitution in &suggestion.substitutions {
                if let Some(snippet) = self.source_map.span_to_snippet(substitution.span) {
                    self.dst.set_color(ColorSpec::new().set_fg(Some(Color::Green))).ok();
                    writeln!(self.dst, "   - {}", snippet).ok();
                    writeln!(self.dst, "   + {}", substitution.code).ok();
                    self.dst.reset().ok();
                }
            }
        }

        // Add a blank line after each diagnostic
        writeln!(self.dst).ok();
    }
}