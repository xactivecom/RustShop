///////////////////////////////
// Course Section 27a
///////////////////////////////

use std::collections::{ VecDeque, HashSet };

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u8);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u8, u8);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    // Ctor
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

impl From<u8> for Vertex {
    fn from(item: u8) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph.edges.iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into()).collect()
    }
}

impl From<(u8, u8)> for Edge {
    fn from(item: (u8, u8)) -> Self {
        Edge(item.0, item.1)
    }
}


// Depth-first search algorithm
pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u8>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u8> = Vec::new();
    let mut queue = VecDeque::new();

    // Start at root node
    visited.insert(root);
    queue.push_back(root);

    // Pop nodes off the queue until objective is found
    while let Some(cur_vertex) = queue.pop_front() {
        history.push(cur_vertex.value());

        // Found objective
        if cur_vertex == objective {
            return Some(history)
        }

        // Visit all neighbors of current vertex (in reverse order)
        for neighbor in cur_vertex.neighbors(graph).into_iter().rev() {
            if visited.insert(neighbor) {
                queue.push_front(neighbor);
            }
        }
    }

    // Objective not found
    None
}

// Breadth-first search algorithm
pub fn breadth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u8>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u8> = Vec::new();
    let mut queue = VecDeque::new();

    // Start at root node
    visited.insert(root);
    queue.push_back(root);

    // Pop nodes off the queue until objective is found
    while let Some(cur_vertex) = queue.pop_front() {
        history.push(cur_vertex.value());

        // Found objective
        if cur_vertex == objective {
            return Some(history)
        }

        // Visit all unvisited neighbors of current vertex depth
        for neighbor in cur_vertex.neighbors(graph) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);

                // Go up one depth level
                queue.push_back(neighbor);
            }
        }
    }

    // Objective not found
    None
}


pub fn run_lesson() {
    println!("\nSection 27a:");

    // Populate graph
    let vertices = vec![1, 2, 3, 4, 5, 6, 7];
    let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
    let root = 1;
    let objective = 7;

    let graph = Graph::new(
        vertices.into_iter().map(|v| v.into()).collect(), 
        edges.into_iter().map(|e| e.into()).collect(), 
    );

    // Test DFS search
    let dfs_order = vec![1, 2, 4, 5, 3, 6, 7];
    assert_eq!(depth_first_search(&graph, root.into(), objective.into()), Some(dfs_order));

    // Test BFS search
    let bfs_order = vec![1, 2, 3, 4, 5, 6, 7];
    assert_eq!(breadth_first_search(&graph, root.into(), objective.into()), Some(bfs_order));
}
