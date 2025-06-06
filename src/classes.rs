use std::fmt::Display;
use std::sync::{Arc, Mutex};

use pyo3::prelude::*;

use stack_graphs::storage::{SQLiteReader, SQLiteWriter};
use tree_sitter_stack_graphs::cli::util::{SourcePosition, SourceSpan};
use tree_sitter_stack_graphs::loader::Loader;

use crate::stack_graphs_wrapper::{
    get_status, get_status_all, index_all, new_loader, query_definition,
};

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Java,
}

#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum FileStatus {
    Missing,
    Indexed,
    Error,
}

#[pyclass]
#[derive(Clone)]
pub struct FileEntry {
    #[pyo3(get)]
    pub path: String,
    #[pyo3(get)]
    pub tag: String,
    #[pyo3(get)]
    pub status: FileStatus,
    // As pyo3 does not support string enums, we use Option<String> here instead.
    #[pyo3(get)]
    pub error: Option<String>,
}

impl From<stack_graphs::storage::FileEntry> for FileEntry {
    fn from(entry: stack_graphs::storage::FileEntry) -> Self {
        let status = match entry.status {
            stack_graphs::storage::FileStatus::Missing => FileStatus::Missing,
            stack_graphs::storage::FileStatus::Indexed => FileStatus::Indexed,
            stack_graphs::storage::FileStatus::Error(_) => FileStatus::Error,
        };

        let error = match entry.status {
            stack_graphs::storage::FileStatus::Error(e) => Some(e),
            _ => None,
        };

        FileEntry {
            path: entry.path.to_str().unwrap().to_string(),
            tag: entry.tag,
            status,
            error,
        }
    }
}

#[pymethods]
impl FileEntry {
    fn __repr__(&self) -> String {
        match self {
            FileEntry {
                path,
                tag,
                status,
                error,
            } => {
                let error = match error {
                    Some(e) => format!("(\"{}\")", e),
                    None => "".to_string(),
                };

                format!(
                    "FileEntry(path=\"{}\", tag=\"{}\", status={}{})",
                    path,
                    tag,
                    status.__pyo3__repr__(),
                    error
                )
            }
        }
    }
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
    db_reader: Arc<Mutex<SQLiteReader>>,
    db_path: String,
}

#[pymethods]
impl Querier {
    #[new]
    pub fn new(db_path: String) -> Self {
        println!("Opening database: {}", db_path);
        Querier {
            db_reader: Arc::new(Mutex::new(
                SQLiteReader::open(db_path.clone()).unwrap().into(),
            )),
            db_path: db_path,
        }
    }

    pub fn definitions(&mut self, reference: Position) -> PyResult<Vec<Position>> {
        let result = query_definition(reference.into(), &mut *self.db_reader.lock().unwrap())?;

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
    db_writer: Arc<Mutex<SQLiteWriter>>,
    db_reader: Arc<Mutex<SQLiteReader>>,
    db_path: String,
    loader: Loader,
}

#[pymethods]
impl Indexer {
    #[new]
    pub fn new(db_path: String, languages: Vec<Language>) -> Self {
        Indexer {
            db_writer: Arc::new(Mutex::new(SQLiteWriter::open(db_path.clone()).unwrap())),
            db_reader: Arc::new(Mutex::new(SQLiteReader::open(db_path.clone()).unwrap())),
            db_path: db_path,
            loader: new_loader(languages),
        }
    }

    pub fn index_all(&mut self, paths: Vec<String>) -> PyResult<()> {
        let paths: Vec<std::path::PathBuf> =
            paths.iter().map(|p| std::path::PathBuf::from(p)).collect();

        match index_all(
            paths,
            &mut self.loader,
            &mut *self.db_writer.lock().unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn status(&mut self, paths: Vec<String>) -> PyResult<Vec<FileEntry>> {
        let paths: Vec<std::path::PathBuf> =
            paths.iter().map(|p| std::path::PathBuf::from(p)).collect();

        get_status(paths, &mut *self.db_reader.lock().unwrap())?
            .into_iter()
            .map(|e| Ok(e.into()))
            .collect()
    }

    pub fn status_all(&mut self) -> PyResult<Vec<FileEntry>> {
        get_status_all(&mut *self.db_reader.lock().unwrap())?
            .into_iter()
            .map(|e| Ok(e.into()))
            .collect()
    }

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
