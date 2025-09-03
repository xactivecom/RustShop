///////////////////////////////
// Course Section 27
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

    queue.push_back(root);

    while let Some(cur_vertex) = queue.pop_front() {
        history.push(cur_vertex.value());
        if cur_vertex == objective {
            return Some(history);
        }

        for neighbor in cur_vertex.neighbors(graph).into_iter().rev() {
            if visited.insert(neighbor) {
                queue.push_front(neighbor)
            }
        }
    }

    // Objective not found
    None
}


pub fn run_lesson() {
    println!("\nSection 27:");

}
