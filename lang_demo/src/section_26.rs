///////////////////////////////
// Course Section 26
///////////////////////////////

use std::collections::HashMap;

// Custom graph error
#[derive(Debug)]
pub struct NodeNotInGraph;

// A directed and weighted graph.
// Implemented with an adjacency matrix that consists of a hash map of identifier string key
// with a vector of edges containing identifier key and integer weight of the edge.
pub struct DirectedGraph {
    adj_matrix: HashMap<String, Vec<(String, i32)>>,
}

// An undirected and weighted graph.
// Implemented with an adjacency matrix that consists of a hash map of identifier string key
// with a vector of edges containing identifier key and integer weight of the edge.
pub struct UndirectedGraph {
    adj_matrix: HashMap<String, Vec<(String, i32)>>,
}

// Graph trait
pub trait Graph {
    // Ctor
    fn new() -> Self;

    // Property: adjacency matrix
    fn adj_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    // Add a new node to the graph.
    // Return true if node added, otherwise false when already exists
    fn add_node(&mut self, node: &str) -> bool {
        // Check if node identifier already exists
        match self.adj_matrix().get(node) {
            None => {
                // Add a new empty vector for the new node
                self.adj_matrix().insert((*node).to_string(), Vec::new());
                true
            },
            _ => false,
        }
    }

    // Add an edge to the graph between the two specified nodes and weight
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // Add the two nodes between the edge, ignore if node already exists
        self.add_node(edge.0);
        self.add_node(edge.1);

        // Connect node 0 to node 1 by adding an edge to the vector (directed)
        self.adj_matrix().entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2))
            });
    }

    // Retrieve all the neighbors for the specified node
    fn neighbors(&mut self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
        // Retrieve node by identifier
        match self.adj_matrix().get(node) {
            None => Err(NodeNotInGraph),
            Some(n) => Ok(n),
        }
    }
}

// Implementation directed graph
impl Graph for DirectedGraph {
    // Ctor
    fn new() -> DirectedGraph {
        DirectedGraph {
            adj_matrix: HashMap::new(),
        }
    }

    // Property: adjacency matrix
    fn adj_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adj_matrix
    }
}

// Implementation undirected graph
impl Graph for UndirectedGraph {
    // Ctor
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adj_matrix: HashMap::new(),
        }
    }

    // Property: adjacency matrix
    fn adj_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adj_matrix
    }

    // The undirected graph needs an edge to go both directions beteen nodes
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // Add the two nodes between the edge, ignore if node already exists
        self.add_node(edge.0);
        self.add_node(edge.1);

        // Connect node 0 to node 1 by adding an edge to the vector (directed)
        self.adj_matrix().entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2))
            });

        // Connect node 1 to node 0 by adding an edge to the vector (directed)
        self.adj_matrix().entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2))
            });
    }
}


pub fn run_lesson() {
    println!("\nSection 26:");

    // Populate node names
    let name_mtl = String::from("Montreal");
    let name_ott = String::from("Ottawa");
    let name_tor = String::from("Toronto");
    let name_lon = String::from("London");

    // Test directed graph
    let mut d_graph = DirectedGraph::new();
    d_graph.add_edge((&name_mtl, &name_ott, 180));
    d_graph.add_edge((&name_mtl, &name_tor, 600));
    d_graph.add_edge((&name_mtl, &name_lon, 800));
    assert_eq!(d_graph.neighbors(&name_mtl).unwrap(), 
        &vec![(name_ott, 180), (name_tor, 600), (name_lon, 800)]);


    // Populate node names
    let name_mtl = String::from("Montreal");
    let name_ott = String::from("Ottawa");
    let name_tor = String::from("Toronto");
    let name_lon = String::from("London");

    // Test undirected graph
    let mut u_graph = UndirectedGraph::new();
    u_graph.add_edge((&name_mtl, &name_ott, 180));
    u_graph.add_edge((&name_mtl, &name_tor, 600));
    u_graph.add_edge((&name_mtl, &name_lon, 800));
    assert_eq!(u_graph.neighbors(&name_mtl).unwrap(), 
        &vec![(name_ott, 180), (name_tor, 600), (name_lon, 800)]);
}
