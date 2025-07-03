//! Terminal emitter for diagnostics.

use crate::{
    diagnostic::{Diagnostic, Level, SpanLabel},
    emitter::Emitter,
};
use pilota_build_common::{SourceMap, Span};
use std::io::{self, Write};
use std::sync::Arc;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Terminal emitter that outputs diagnostics to stderr with colors.
pub struct TerminalEmitter {
    writer: StandardStream,
    source_map: Arc<SourceMap>,
}

impl TerminalEmitter {
    /// Create a new terminal emitter.
    pub fn new(source_map: Arc<SourceMap>, color_choice: ColorChoice) -> Self {
        TerminalEmitter {
            writer: StandardStream::stderr(color_choice),
            source_map,
        }
    }

    /// Emit a diagnostic header.
    fn emit_header(&mut self, diagnostic: &Diagnostic) -> io::Result<()> {
        let (level_str, color) = match diagnostic.level {
            Level::Error => ("error", Color::Red),
            Level::Warning => ("warning", Color::Yellow),
            Level::Info => ("info", Color::Blue),
            Level::Note => ("note", Color::Green),
            Level::Help => ("help", Color::Cyan),
        };

        self.writer.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
        write!(self.writer, "{}", level_str)?;
        self.writer.reset()?;

        if let Some(code) = &diagnostic.code {
            self.writer.set_color(ColorSpec::new().set_bold(true))?;
            write!(self.writer, "[{}]", code)?;
            self.writer.reset()?;
        }

        writeln!(self.writer, ": {}", diagnostic.message)?;

        Ok(())
    }

    /// Emit a source snippet with annotations.
    fn emit_snippet(&mut self, span: &Span, label: &SpanLabel) -> io::Result<()> {
        let location = match self.source_map.lookup_span(*span) {
            Some(loc) => loc,
            None => return Ok(()),
        };

        let snippet = match self.source_map.snippet(*span) {
            Some(s) => s,
            None => return Ok(()),
        };

        // Print file location
        self.writer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))?;
        write!(self.writer, "--> ")?;
        self.writer.reset()?;
        writeln!(
            self.writer,
            "{}:{}:{}",
            location.file.name,
            location.start_line,
            location.start_column
        )?;

        // Print source lines with line numbers
        let lines: Vec<&str> = snippet.lines().collect();
        let line_num_width = location.end_line.to_string().len();

        for (i, line) in lines.iter().enumerate() {
            let line_num = location.start_line + i;
            
            // Line number
            self.writer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))?;
            write!(self.writer, "{:>width$} | ", line_num, width = line_num_width)?;
            self.writer.reset()?;

            // Source line
            writeln!(self.writer, "{}", line)?;

            // Underline for the first line
            if i == 0 {
                self.writer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))?;
                write!(self.writer, "{:>width$} | ", "", width = line_num_width)?;
                self.writer.reset()?;

                // Calculate underline position
                let start_col = if location.start_line == location.end_line {
                    location.start_column - 1
                } else {
                    0
                };
                let end_col = if location.start_line == location.end_line {
                    location.end_column - 1
                } else {
                    line.len()
                };

                // Print spaces before underline
                for _ in 0..start_col {
                    write!(self.writer, " ")?;
                }

                // Print underline
                let color = match label.style {
                    crate::diagnostic::SpanStyle::Primary => Color::Red,
                    crate::diagnostic::SpanStyle::Secondary => Color::Yellow,
                };
                self.writer.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
                for _ in start_col..end_col {
                    write!(self.writer, "^")?;
                }

                // Print label message
                if !label.label.is_empty() {
                    write!(self.writer, " {}", label.label)?;
                }
                self.writer.reset()?;
                writeln!(self.writer)?;
            }
        }

        Ok(())
    }

    /// Emit child diagnostics.
    fn emit_children(&mut self, diagnostic: &Diagnostic) -> io::Result<()> {
        for child in &diagnostic.children {
            writeln!(self.writer)?;
            
            // Print child header
            let (level_str, color) = match child.level {
                Level::Error => ("error", Color::Red),
                Level::Warning => ("warning", Color::Yellow),
                Level::Info => ("info", Color::Blue),
                Level::Note => ("note", Color::Green),
                Level::Help => ("help", Color::Cyan),
            };

            self.writer.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
            write!(self.writer, "{}", level_str)?;
            self.writer.reset()?;
            writeln!(self.writer, ": {}", child.message)?;
            
            for span_label in &child.spans {
                self.emit_snippet(&span_label.span, span_label)?;
            }
        }

        Ok(())
    }
}

impl Emitter for TerminalEmitter {
    fn emit_diagnostic(&mut self, diagnostic: &Diagnostic) {
        let _ = self.emit_header(diagnostic);

        // Emit primary spans
        for span_label in &diagnostic.spans {
            let _ = self.emit_snippet(&span_label.span, span_label);
        }

        // Emit children
        let _ = self.emit_children(diagnostic);

        // Emit suggestions
        if !diagnostic.suggestions.is_empty() {
            let _ = writeln!(self.writer);
            let _ = self.writer.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true));
            let _ = writeln!(self.writer, "help: {}", diagnostic.suggestions[0].message);
            let _ = self.writer.reset();

            for substitution in &diagnostic.suggestions[0].substitutions {
                if let Some(snippet) = self.source_map.snippet(substitution.span) {
                    let _ = self.writer.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
                    let _ = writeln!(self.writer, "   {}", substitution.code);
                    let _ = self.writer.reset();
                }
            }
        }

        let _ = writeln!(self.writer);
    }
}