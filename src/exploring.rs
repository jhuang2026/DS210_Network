use rand::Rng;
use crate::graph::{Vertex, Graph};
use std::collections::{VecDeque};

// Define a function to simulate random walks on a graph and return the top 5 most frequent nodes
pub fn simulate_walks(graph: &Graph, num_walks: usize, walk_length: usize) -> Vec<(Vertex, f64)> {
    let mut rng = rand::thread_rng();
    let mut occurrences = vec![0; graph.n];

    // Loop over each node in the graph and simulate a random walk starting at that node
    for start_node in 0..graph.n {
        for _ in 0..num_walks {
            let mut curr_node = start_node;
            for _ in 0..walk_length {
                let neighbors = &graph.outedges[curr_node];

                // Choose a random node otherwise if the node has no neighbors
                if neighbors.is_empty() {
                    curr_node = rng.gen_range(0..graph.n);
                } else {
                    curr_node = neighbors[rng.gen_range(0..neighbors.len())];
                }
            }
            occurrences[curr_node] += 1;
        }
    }

    let total_walks = graph.n * num_walks;
    let mut ratios = (0..graph.n)
        .map(|i| (i, occurrences[i] as f64 / total_walks as f64))
        .collect::<Vec<_>>();
    ratios.sort_by(|(_, r1), (_, r2)| r2.partial_cmp(r1).unwrap());

    // Return the top 5 nodes and their frequency as a tuple vector
    ratios.iter().take(5).map(|(i, r)| (*i, *r)).collect()
}

// Define a function to compute the average shortest path length of a graph using BFS
pub fn bfs_distance(graph: &Graph) -> f64 {
    let mut total_distance = 0.0;
    let mut count = 0;

    // Loop over each node in the graph and perform BFS
    for start_node in 0..graph.n {
        let mut visited = vec![false; graph.n];
        let mut distance = vec![0; graph.n];
        let mut queue = VecDeque::new();
        visited[start_node] = true;
        queue.push_back(start_node);

        while let Some(curr_node) = queue.pop_front() {
            for &neighbor in &graph.outedges[curr_node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    distance[neighbor] = distance[curr_node] + 1;
                    queue.push_back(neighbor);
                }
            }
        }

        for d in distance {
            if d > 0 {
                total_distance += d as f64;
                count += 1;
            }
        }
    }
    total_distance / count as f64
}