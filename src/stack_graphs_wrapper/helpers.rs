use stack_graphs::stats::FrequencyDistribution;
use stack_graphs::stitching::Stats as StitchingStats;
use std::fmt::Display;
use std::hash::Hash;
use tree_sitter_stack_graphs::cli::index::IndexingStats;

fn print_quartiles_header(title: &str) {
    println!(
        "| {:^29} | {:^9} | {:^9} | {:^9} | {:^9} | {:^9} | {:^9} |",
        title, "min", "p25", "p50", "p75", "max", "count",
    );
    println!(
        "|-------------------------------|-----------|-----------|-----------|-----------|-----------|-----------|"
    );
}

fn print_quartiles_row<X: Display + Eq + Hash + Ord>(title: &str, hist: FrequencyDistribution<X>) {
    let qs = hist.quantiles(4);
    if qs.is_empty() {
        println!(
            "| {:>29} | {:>9} | {:>9} | {:>9} | {:>9} | {:>9} | {:>9} |",
            title, "-", "-", "-", "-", "-", 0
        );
    } else {
        println!(
            "| {:>29} | {:>9} | {:>9} | {:>9} | {:>9} | {:>9} | {:>9} |",
            title,
            qs[0],
            qs[1],
            qs[2],
            qs[3],
            qs[4],
            hist.count(),
        );
    }
}

pub(crate) fn print_indexing_stats(stats: IndexingStats) {
    print_quartiles_header("graph stats");
    print_quartiles_row("total graph nodes", stats.total_graph_nodes);
    print_quartiles_row("total graph edges", stats.total_graph_edges);
    print_quartiles_row("node out degrees", stats.node_out_degrees);
    print_value_row("root out degree", stats.root_out_degree);
    println!();
    print_stitching_stats(stats.stitching_stats);
}

fn print_value_row<X: Display>(title: &str, value: X) {
    println!(
        "| {:>29} | {:>9} | {:>9} | {:>9} | {:>9} | {:>9} | {:>9} |",
        title, "-", "-", "-", "-", "-", value
    );
}

pub(crate) fn print_stitching_stats(stats: StitchingStats) {
    print_quartiles_header("stitching stats");
    print_quartiles_row("initial paths", stats.initial_paths);
    print_quartiles_row("queued paths per phase", stats.queued_paths_per_phase);
    print_quartiles_row("processed paths per phase", stats.processed_paths_per_phase);
    print_quartiles_row("accepted path length", stats.accepted_path_length);
    print_quartiles_row("terminal path length", stats.terminal_path_lengh);
    print_quartiles_row("node path candidates", stats.candidates_per_node_path);
    print_quartiles_row("node path extensions", stats.extensions_per_node_path);
    print_quartiles_row("root path candidates", stats.candidates_per_root_path);
    print_quartiles_row("root path extensions", stats.extensions_per_root_path);
    print_quartiles_row("node visits", stats.node_visits.frequencies());
    print_value_row("root visits", stats.root_visits);
    print_quartiles_row(
        "similar path counts",
        stats.similar_paths_stats.similar_path_count,
    );
    print_quartiles_row(
        "similar path bucket sizes",
        stats.similar_paths_stats.similar_path_bucket_size,
    );
}
