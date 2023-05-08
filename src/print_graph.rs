use std::fs::File;
use std::io::Write;
use rand::seq::IteratorRandom;
use crate::graph::{Graph};

// Define a function to print a randomly generated graph to a dot file
pub fn print_graph(graph: &Graph, number_edges: usize, number_nodes: usize) {
    let mut edges = Vec::new();
    // Initialize random number generator
    let mut rng = rand::thread_rng(); 

    // Shuffle adjacency list and take the first max_edges elements
    for i in 0..number_nodes {
        let selected_edges = graph.outedges[i].iter().cloned().choose_multiple(&mut rng, number_edges);
        for &j in &selected_edges {
            if i < j {
                edges.push((i, j));
            }
        }
    }

    // Create a new dot file and write the graph data to it
    let mut dot_file = File::create("graph.dot").expect("failed to create dot file");
    dot_file.write(b"graph {\n").expect("failed to write to dot file");
    for (u, v) in &edges {
        dot_file
            .write_fmt(format_args!("    {} -- {};\n", u, v))
            .expect("failed to write to dot file");
    }
    dot_file.write(b"}\n").expect("failed to write to dot file");
}
