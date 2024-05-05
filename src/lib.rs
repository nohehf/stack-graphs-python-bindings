use pyo3::prelude::*;

mod stack_graphs_wrapper;

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

/// A Python module implemented in Rust.
#[pymodule]
fn stack_graphs_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(index, m)?)?;
    Ok(())
}
