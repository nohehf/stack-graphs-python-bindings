use std::fmt::Display;

use pyo3::prelude::*;

use tree_sitter_stack_graphs::cli::util::{SourcePosition, SourceSpan};

#[pyclass]
#[derive(Clone)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Java,
}

#[pyclass]
#[derive(Clone)]
pub struct Position {
    #[pyo3(get, set)]
    path: String,
    #[pyo3(get, set)]
    line: usize,
    #[pyo3(get, set)]
    column: usize,
}

#[pymethods]
impl Position {
    #[new]
    fn new(path: String, line: usize, column: usize) -> Self {
        Position { path, line, column }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.path, self.line, self.column)
    }
}

impl From<Position> for SourcePosition {
    fn from(pos: Position) -> Self {
        SourcePosition {
            path: pos.path.into(),
            line: pos.line,
            column: pos.column,
        }
    }
}

impl Into<Position> for SourceSpan {
    fn into(self) -> Position {
        Position {
            // TODO(@nohehf): Unwrap is unsafe
            path: self.path.to_str().unwrap().to_string(),
            line: self.span.start.line,
            // TODO(@nohehf): Handle both utf8 and utf16
            column: self.span.start.column.utf8_offset,
        }
    }
}
