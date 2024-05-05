use core::fmt::Display;

use pyo3::prelude::*;

mod stack_graphs_wrapper;

use tree_sitter_stack_graphs::cli::util::{SourcePosition, SourceSpan};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
/// Indexes the given paths into stack graphs, and stores the results in the given database.
#[pyfunction]
fn index(paths: Vec<String>, db_path: String) -> PyResult<()> {
    println!("Indexing paths: {:?}", paths);
    println!("Database path: {:?}", db_path);

    let paths: Vec<std::path::PathBuf> =
        paths.iter().map(|p| std::path::PathBuf::from(p)).collect();

    Ok(stack_graphs_wrapper::index(paths, &db_path)?)
}

#[pyclass]
#[derive(Clone)]
struct Position {
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

/// Indexes the given paths into stack graphs, and stores the results in the given database.
#[pyfunction]
fn query_definition(reference: Position, db_path: String) -> PyResult<Vec<Position>> {
    println!("Querying reference: {:?}", reference.to_string());
    println!("Database path: {:?}", db_path);

    let result = stack_graphs_wrapper::query_definition(reference.into(), &db_path)?;

    // TODO(@nohehf): Check if we can flatten the results, see the QueryResult struct, we might be loosing some information
    let positions: Vec<Position> = result
        .into_iter()
        .map(|r| r.targets)
        .flatten()
        .map(|t| t.into())
        .collect();

    Ok(positions)
}

/// A Python module implemented in Rust.
#[pymodule]
fn stack_graphs_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(index, m)?)?;
    m.add_function(wrap_pyfunction!(query_definition, m)?)?;
    m.add_class::<Position>()?;
    Ok(())
}
