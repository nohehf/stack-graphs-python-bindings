use std::fmt::Display;

use pyo3::prelude::*;

use stack_graphs::storage::{SQLiteReader, SQLiteWriter};
use tree_sitter_stack_graphs::cli::util::{SourcePosition, SourceSpan};
use tree_sitter_stack_graphs::loader::Loader;

use crate::stack_graphs_wrapper::{index_all, new_loader, query_definition};

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

#[pyclass]
pub struct Querier {
    db_reader: SQLiteReader,
    db_path: String,
}

#[pymethods]
impl Querier {
    #[new]
    pub fn new(db_path: String) -> Self {
        println!("Opening database: {}", db_path);
        Querier {
            db_reader: SQLiteReader::open(db_path.clone()).unwrap(),
            db_path: db_path,
        }
    }

    pub fn definitions(&mut self, reference: Position) -> PyResult<Vec<Position>> {
        let result = query_definition(reference.into(), &mut self.db_reader)?;

        let positions: Vec<Position> = result
            .into_iter()
            .map(|r| r.targets)
            .flatten()
            .map(|t| t.into())
            .collect();

        Ok(positions)
    }

    fn __repr__(&self) -> String {
        format!("Querier(db_path=\"{}\")", self.db_path)
    }
}

#[pyclass]
pub struct Indexer {
    db_writer: SQLiteWriter,
    db_path: String,
    loader: Loader,
}

#[pymethods]
impl Indexer {
    #[new]
    pub fn new(db_path: String, languages: Vec<Language>) -> Self {
        Indexer {
            db_writer: SQLiteWriter::open(db_path.clone()).unwrap(),
            db_path: db_path,
            loader: new_loader(languages),
        }
    }

    pub fn index_all(&mut self, paths: Vec<String>) -> PyResult<()> {
        let paths: Vec<std::path::PathBuf> =
            paths.iter().map(|p| std::path::PathBuf::from(p)).collect();

        match index_all(paths, &mut self.loader, &mut self.db_writer) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    // @TODO: Add a method to retrieve the status of the files (indexed, failed, etc.)
    // pub fn get_files_status(&self) -> PyResult<Vec<String>> {
    //     let files = self.loader.files_status();
    //     Ok(files)
    // }

    fn __repr__(&self) -> String {
        format!("Indexer(db_path=\"{}\")", self.db_path)
    }
}

#[pymethods]
impl Position {
    #[new]
    fn new(path: String, line: usize, column: usize) -> Self {
        Position { path, line, column }
    }

    fn __eq__(&self, other: &Position) -> bool {
        self.path == other.path && self.line == other.line && self.column == other.column
    }

    fn __repr__(&self) -> String {
        format!(
            "Position(path=\"{}\", line={}, column={})",
            self.path, self.line, self.column
        )
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
