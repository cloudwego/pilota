//! Source file management and mapping.

use super::{BytePos, CharPos, Span};
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A file ID.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct FileId(u32);

impl FileId {
    pub const DUMMY: FileId = FileId(u32::MAX);

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

/// A source file.
#[derive(Debug)]
pub struct SourceFile {
    /// The name of the file.
    pub name: PathBuf,
    /// The complete source code.
    pub src: String,
    /// The start position of this file.
    pub start_pos: BytePos,
    /// The end position of this file.
    pub end_pos: BytePos,
    /// Locations of line beginnings.
    pub lines: Vec<BytePos>,
}

impl SourceFile {
    fn new(name: PathBuf, src: String, start_pos: BytePos) -> Self {
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

    /// Find the line containing the given position.
    pub fn lookup_line(&self, pos: BytePos) -> Option<usize> {
        if pos < self.start_pos || pos > self.end_pos {
            return None;
        }

        match self.lines.binary_search(&pos) {
            Ok(line) => Some(line),
            Err(line) => Some(line.saturating_sub(1)),
        }
    }

    /// Get the line and column for a position.
    pub fn lookup_line_col(&self, pos: BytePos) -> Option<(usize, usize)> {
        let line = self.lookup_line(pos)?;
        let line_start = self.lines[line];
        let col = (pos.0 - line_start.0) as usize;
        Some((line, col))
    }

    /// Get a snippet of the source code.
    pub fn snippet(&self, span: Span) -> Option<&str> {
        if span.lo < self.start_pos || span.hi > self.end_pos {
            return None;
        }

        let lo = (span.lo.0 - self.start_pos.0) as usize;
        let hi = (span.hi.0 - self.start_pos.0) as usize;
        Some(&self.src[lo..hi])
    }

    /// Check if this file contains the given position.
    pub fn contains(&self, pos: BytePos) -> bool {
        self.start_pos <= pos && pos <= self.end_pos
    }
}

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

/// The source map containing all source files.
pub struct SourceMap {
    files: RwLock<SourceMapFiles>,
}

#[derive(Default)]
struct SourceMapFiles {
    files: Vec<Arc<SourceFile>>,
    file_id_to_index: FxHashMap<FileId, usize>,
    next_file_id: u32,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap {
            files: RwLock::new(SourceMapFiles::default()),
        }
    }

    /// Load a file into the source map.
    pub fn load_file(&self, path: &Path) -> std::io::Result<FileId> {
        let src = std::fs::read_to_string(path)?;
        Ok(self.new_source_file(path.to_path_buf(), src))
    }

    /// Create a new source file.
    pub fn new_source_file(&self, name: PathBuf, src: String) -> FileId {
        let mut files = self.files.write();
        
        let start_pos = if let Some(last) = files.files.last() {
            BytePos(last.end_pos.0 + 1)
        } else {
            BytePos(0)
        };

        let file = Arc::new(SourceFile::new(name, src, start_pos));
        let file_id = FileId(files.next_file_id);
        files.next_file_id += 1;

        let index = files.files.len();
        files.files.push(file);
        files.file_id_to_index.insert(file_id, index);

        file_id
    }

    /// Get a source file by ID.
    pub fn get_file(&self, file_id: FileId) -> Option<Arc<SourceFile>> {
        let files = self.files.read();
        let index = *files.file_id_to_index.get(&file_id)?;
        files.files.get(index).cloned()
    }

    /// Look up the file containing the given position.
    pub fn lookup_file(&self, pos: BytePos) -> Option<Arc<SourceFile>> {
        let files = self.files.read();
        
        // Binary search for the file containing pos
        let idx = files.files.binary_search_by(|file| {
            if pos < file.start_pos {
                std::cmp::Ordering::Greater
            } else if pos > file.end_pos {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        }).ok()?;

        files.files.get(idx).cloned()
    }

    /// Get a snippet of source code.
    pub fn span_to_snippet(&self, span: Span) -> Option<String> {
        let file = self.get_file(span.file_id)?;
        file.snippet(span).map(|s| s.to_string())
    }

    /// Get the location (file, line, column) for a position.
    pub fn lookup_char_pos(&self, pos: BytePos) -> Option<Location> {
        let file = self.lookup_file(pos)?;
        let (line, col) = file.lookup_line_col(pos)?;
        
        Some(Location {
            file: file.name.clone(),
            line: line + 1,  // Convert to 1-based
            col: col + 1,    // Convert to 1-based
        })
    }
}

impl Default for SourceMap {
    fn default() -> Self {
        Self::new()
    }
}

/// A location in a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.col)
    }
}