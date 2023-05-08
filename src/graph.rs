pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex, Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
    pub inedges: AdjacencyLists,
}

impl Graph {
    pub fn add_undirected_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
            self.inedges[*v].push(*u); // add incoming edge to v
            self.outedges[*v].push(*u); // add edge in opposite direction
            self.inedges[*u].push(*v); // add incoming edge to u
        }
    }

    // This method sorts the adjacency lists in increasing order
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
        for l in self.inedges.iter_mut() {
            l.sort();
        }
    }

    // This method creates an undirected graph from the given edges and sorts the adjacency lists
    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
            inedges: vec![vec![]; n],
        };
        g.add_undirected_edges(edges);
        g.sort_graph_lists();
        g
    }

    // This prints out our adjacency list
    // pub fn print(&self) {
    //     println!("Number of vertices: {}", self.n);
    //     println!("Adjacency lists:");
    //     for (i, adjlist) in self.outedges.iter().enumerate() {
    //         print!("{}: ", i);
    //         for &v in adjlist {
    //             print!("{} ", v);
    //         }
    //         println!();
    //     }
    // }
}
