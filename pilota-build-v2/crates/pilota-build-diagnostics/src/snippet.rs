//! Code snippet representation for diagnostics.

/// A code snippet to display.
#[derive(Clone, Debug)]
pub struct Snippet {
    pub file_name: String,
    pub lines: Vec<Line>,
}

/// A line in a snippet.
#[derive(Clone, Debug)]
pub struct Line {
    pub line_number: usize,
    pub text: String,
    pub annotations: Vec<Annotation>,
}

/// An annotation on a line.
#[derive(Clone, Debug)]
pub struct Annotation {
    pub start_col: usize,
    pub end_col: usize,
    pub label: String,
    pub annotation_type: AnnotationType,
}

/// The type of annotation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AnnotationType {
    Error,
    Warning,
    Info,
    Note,
    Help,
}