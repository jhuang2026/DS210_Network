use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

// Import our module files
mod tests;
mod graph;
mod print_graph;
mod exploring;

fn main() {
    // Open our input file "facebook_combined.txt"
    let file = File::open("facebook_combined.txt").expect("failed to open input file");
    let reader = BufReader::new(file);

    // Read the number of vertices from the first line of the input file
    let mut lines_iter = reader.lines();
    let firstline = lines_iter.next().unwrap().unwrap();
    let n = firstline.trim().parse::<usize>().unwrap();

    // Read the edges from the remaining lines of the input file
    let mut edges = Vec::new();
    for line in lines_iter {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let u = split.next().unwrap().parse::<graph::Vertex>().unwrap();
        let v = split.next().unwrap().parse::<graph::Vertex>().unwrap();
        edges.push((u, v));
    }

    // Create an undirected graph from the edges
    let graph = graph::Graph::create_undirected(n, &edges);

    // Print the graph to a dot file
    let dot_file = "graph.dot";
    print_graph::print_graph(&graph, 1, 500);

    // Generate a PNG image of the graph using the 'dot' command
    let png_file = "graph.png";
    let output = Command::new("dot")
        .args(&["-Tpng", "-o", png_file, dot_file])
        .output()
        .expect("failed to execute dot command");

    // Check if the 'dot' command executed successfully and print the result
    if !output.status.success() {
        eprintln!("Failed to generate PNG image from DOT file:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Generated PNG image: {}", png_file);
    }

    // Simulate random walks on the graph and print the top vertices
    let top_vertices = exploring::simulate_walks(&graph, 1000, 6);
    println!("Top 5 vertices:");
    for (vertex, ratio) in top_vertices {
        println!("{}: {}", vertex, ratio);
    }

    // Calculate the average distance between pairs of vertices using BFS
    let avg_distance = exploring::bfs_distance(&graph);
    println!("Average distance between pairs of vertices: {}", avg_distance); 
}
