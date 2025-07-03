//! Source file management.

use super::{BytePos, Span};
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use std::sync::Arc;

/// File ID type.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FileId(pub u32);

impl FileId {
    pub const DUMMY: FileId = FileId(0);
}

/// Information about a source file.
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// The name of the file.
    pub name: String,
    /// The source code of the file.
    pub src: String,
    /// The starting byte position of this file.
    pub start_pos: BytePos,
    /// The ending byte position of this file.
    pub end_pos: BytePos,
    /// Byte positions of line beginnings.
    pub lines: Vec<BytePos>,
}

impl SourceFile {
    /// Create a new source file.
    pub fn new(name: String, src: String, start_pos: BytePos) -> Self {
        let end_pos = BytePos(start_pos.0 + src.len() as u32);
        let lines = compute_line_starts(&src, start_pos);
        
        SourceFile {
            name,
            src,
            start_pos,
            end_pos,
            lines,
        }
    }

    /// Get the line number (1-indexed) for a byte position.
    pub fn lookup_line(&self, pos: BytePos) -> Option<usize> {
        if pos < self.start_pos || pos > self.end_pos {
            return None;
        }

        match self.lines.binary_search(&pos) {
            Ok(line) => Some(line + 1),
            Err(line) => Some(line),
        }
    }

    /// Get the column number (1-indexed) for a byte position.
    pub fn lookup_column(&self, pos: BytePos) -> Option<usize> {
        let line = self.lookup_line(pos)?;
        let line_start = if line == 1 {
            self.start_pos
        } else {
            self.lines[line - 1]
        };
        
        Some((pos.0 - line_start.0) as usize + 1)
    }

    /// Get a snippet of source code for a span.
    pub fn snippet(&self, span: Span) -> Option<&str> {
        let start = (span.start.0 - self.start_pos.0) as usize;
        let end = (span.end.0 - self.start_pos.0) as usize;
        
        if start <= end && end <= self.src.len() {
            Some(&self.src[start..end])
        } else {
            None
        }
    }
}

/// Compute line start positions.
fn compute_line_starts(src: &str, start_pos: BytePos) -> Vec<BytePos> {
    let mut lines = vec![start_pos];
    let mut pos = start_pos.0;

    for (i, ch) in src.char_indices() {
        if ch == '\n' {
            pos = start_pos.0 + i as u32 + 1;
            lines.push(BytePos(pos));
        }
    }

    lines
}

/// Source map for tracking source files.
#[derive(Default)]
pub struct SourceMap {
    files: RwLock<Files>,
}

#[derive(Default)]
struct Files {
    files: Vec<Arc<SourceFile>>,
    by_name: FxHashMap<String, FileId>,
    next_file_id: u32,
}

impl SourceMap {
    /// Create a new source map.
    pub fn new() -> Self {
        SourceMap {
            files: RwLock::new(Files {
                files: vec![Arc::new(SourceFile::new(
                    "<dummy>".to_string(),
                    String::new(),
                    BytePos(0),
                ))],
                by_name: FxHashMap::default(),
                next_file_id: 1,
            }),
        }
    }

    /// Add a file to the source map.
    pub fn add_file(&self, name: impl Into<String>, src: impl Into<String>) -> super::FileId {
        let name = name.into();
        let src = src.into();
        
        let mut files = self.files.write();
        
        // Check if file already exists
        if let Some(&file_id) = files.by_name.get(&name) {
            return file_id.0;
        }
        
        let file_id = files.next_file_id;
        files.next_file_id += 1;
        
        let start_pos = if let Some(last) = files.files.last() {
            last.end_pos
        } else {
            BytePos(0)
        };
        
        let source_file = Arc::new(SourceFile::new(name.clone(), src, start_pos));
        files.files.push(source_file);
        files.by_name.insert(name, FileId(file_id));
        
        file_id
    }

    /// Get a file by ID.
    pub fn get_file(&self, file_id: super::FileId) -> Option<Arc<SourceFile>> {
        let files = self.files.read();
        files.files.get(file_id as usize).cloned()
    }

    /// Get a file by name.
    pub fn get_file_by_name(&self, name: &str) -> Option<Arc<SourceFile>> {
        let files = self.files.read();
        let file_id = files.by_name.get(name)?;
        files.files.get(file_id.0 as usize).cloned()
    }

    /// Look up file, line, and column for a span.
    pub fn lookup_span(&self, span: Span) -> Option<SpanLocation> {
        let file = self.get_file(span.file_id)?;
        let start_line = file.lookup_line(span.start)?;
        let start_column = file.lookup_column(span.start)?;
        let end_line = file.lookup_line(span.end)?;
        let end_column = file.lookup_column(span.end)?;
        
        Some(SpanLocation {
            file,
            start_line,
            start_column,
            end_line,
            end_column,
        })
    }

    /// Get a snippet for a span.
    pub fn snippet(&self, span: Span) -> Option<String> {
        let file = self.get_file(span.file_id)?;
        file.snippet(span).map(|s| s.to_string())
    }
}

/// Location information for a span.
#[derive(Debug)]
pub struct SpanLocation {
    pub file: Arc<SourceFile>,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}