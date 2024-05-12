use crate::classes::Language;
use pyo3::exceptions::PyException;
use pyo3::PyErr;
use stack_graphs::storage::{SQLiteReader, SQLiteWriter};
use std::path::PathBuf;
use tree_sitter_stack_graphs::cli::query::{Querier, QueryResult};
use tree_sitter_stack_graphs::cli::util::SourcePosition;
use tree_sitter_stack_graphs::cli::{index::Indexer, util::reporter::ConsoleReporter};
use tree_sitter_stack_graphs::loader::LanguageConfiguration;
use tree_sitter_stack_graphs::{loader::Loader, NoCancellation};

// TODO(@nohehf): Better error handling
#[derive(Debug, Clone)]
pub struct StackGraphsError {
    message: String,
}

impl std::convert::From<StackGraphsError> for PyErr {
    fn from(err: StackGraphsError) -> PyErr {
        PyException::new_err(err.message)
    }
}

fn get_langauge_configuration(lang: Language) -> LanguageConfiguration {
    match lang {
        Language::Python => {
            tree_sitter_stack_graphs_python::language_configuration(&NoCancellation)
        }
        Language::JavaScript => {
            tree_sitter_stack_graphs_javascript::language_configuration(&NoCancellation)
        }
        Language::TypeScript => {
            tree_sitter_stack_graphs_typescript::language_configuration(&NoCancellation)
        }
        Language::Java => tree_sitter_stack_graphs_java::language_configuration(&NoCancellation),
    }
}

pub fn index(
    paths: Vec<PathBuf>,
    db_path: &str,
    language: Language,
) -> Result<(), StackGraphsError> {
    let configurations = vec![get_langauge_configuration(language)];

    let mut loader = match Loader::from_language_configurations(configurations, None) {
        Ok(ldr) => ldr,
        Err(e) => {
            return Err(StackGraphsError {
                message: format!("Failed to create loader: {}", e),
            });
        }
    };

    let mut db_write = match SQLiteWriter::open(&db_path) {
        Ok(db) => db,
        Err(e) => {
            return Err(StackGraphsError {
                message: format!("Failed to open database: {}", e),
            });
        }
    };

    let reporter = ConsoleReporter::none();

    let mut indexer = Indexer::new(&mut db_write, &mut loader, &reporter);

    // For now, force reindexing
    indexer.force = true;

    let paths = canonicalize_paths(paths);

    // https://github.com/github/stack-graphs/blob/7db914c01b35ce024f6767e02dd1ad97022a6bc1/tree-sitter-stack-graphs/src/cli/index.rs#L107
    let continue_from_none: Option<PathBuf> = None;

    match indexer.index_all(paths, continue_from_none, &NoCancellation) {
        Ok(_) => Ok(()),
        Err(e) => Err(StackGraphsError {
            message: format!("Failed to index: {}", e),
        }),
    }
}

pub fn query_definition(
    reference: SourcePosition,
    db_reader: &mut SQLiteReader,
) -> Result<Vec<QueryResult>, StackGraphsError> {
    let reporter = ConsoleReporter::none();

    let mut querier = Querier::new(db_reader, &reporter);

    // print_source_position(&reference);

    match querier.definitions(reference, &NoCancellation) {
        Ok(results) => Ok(results),
        Err(e) => Err(StackGraphsError {
            message: format!("Failed to query definitions: {}", e),
        }),
    }
}

// https://github.com/github/stack-graphs/blob/7db914c01b35ce024f6767e02dd1ad97022a6bc1/tree-sitter-stack-graphs/src/cli/index.rs#L118
fn canonicalize_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths
        .into_iter()
        .map(|p| p.canonicalize())
        .collect::<std::result::Result<Vec<_>, _>>()
        .unwrap()
}
