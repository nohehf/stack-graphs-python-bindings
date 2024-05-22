use pyo3::prelude::*;

mod classes;
mod stack_graphs_wrapper;

use classes::{FileEntry, FileStatus, Indexer, Language, Position, Querier};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
/// Indexes the given paths into stack graphs, and stores the results in the given database.
#[pyfunction]
fn index(paths: Vec<String>, db_path: String, language: Language) -> PyResult<()> {
    // TODO(@nohehf): Add a verbose mode to toggle the logs
    // println!("Indexing paths: {:?}", paths);
    // println!("Database path: {:?}", db_path);

    let paths: Vec<std::path::PathBuf> =
        paths.iter().map(|p| std::path::PathBuf::from(p)).collect();

    Ok(stack_graphs_wrapper::index_legacy(
        paths,
        &db_path,
        &language.into(),
    )?)
}

/// A Python module implemented in Rust.
#[pymodule]
fn stack_graphs_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(index, m)?)?;
    m.add_class::<Position>()?;
    m.add_class::<Language>()?;
    m.add_class::<FileStatus>()?;
    m.add_class::<FileEntry>()?;
    m.add_class::<Querier>()?;
    m.add_class::<Indexer>()?;
    Ok(())
}
