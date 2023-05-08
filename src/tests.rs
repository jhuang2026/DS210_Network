#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    use crate::exploring::bfs_distance;
    
    #[test]
    fn test_create_undirected() {
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let graph = Graph::create_undirected(3, &edges);
        assert_eq!(graph.n, 3);
        assert_eq!(graph.outedges[0], vec![1, 2]);
        assert_eq!(graph.outedges[1], vec![0, 2]);
        assert_eq!(graph.outedges[2], vec![0, 1]);
    }
    
    #[test]
    fn test_bfs_distance() {
        let edges = vec![(0, 1), (1, 2), (2, 0)];
        let graph = Graph::create_undirected(3, &edges);
        let avg_distance = bfs_distance(&graph);
        assert_eq!(avg_distance, 1.0);
    }
}