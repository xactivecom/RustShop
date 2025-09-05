///////////////////////////////
// Course Section 27a
///////////////////////////////

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Traversal state
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// Compatator for traversal state
impl Ord for State {
    // Compare the costs of two states, if same then compare their positions
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Cost-weighted edge
struct Edge {
    node: usize,
    cost: usize,
}

// Compute the cost of traversing from start to objective
fn shortest_path(graph: &Vec<Vec<Edge>>, root: usize, objective: usize) -> Option<usize> {
    // Compute an initial worst case cost
    let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();

    // Define visited
    let mut visited = BinaryHeap::new();

    // Start at root node
    dist[root] = 0;
    visited.push(State { cost: 0, position: root });

    // Pop off the node from the heap to evaluate lowest cost
    while let Some(State { cost, position }) = visited.pop() {

        // Found objective
        if position == objective {
            return Some(cost)
        }

        // Continue looking for lower cost
        if cost > dist[position] {
            continue
        }

        // Continue looking for lower cost
        for edge in &graph[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            if next.cost < dist[next.position] {
                visited.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    // Objective not found
    None
}


pub fn run_lesson() {
    println!("\nSection 27b:");

    // Populate undirected graph
    let graph = vec![
        vec![ // node 0
            Edge { node: 1, cost: 6 },
            Edge { node: 2, cost: 4 },
            Edge { node: 3, cost: 1 },
        ],
        vec![ // node 1
            Edge { node: 0, cost: 6 },
            Edge { node: 2, cost: 3 },
        ],
        vec![ // node 2
            Edge { node: 0, cost: 4 },
            Edge { node: 1, cost: 3 },
            Edge { node: 3, cost: 1 },
        ],
        vec![ // node 3
            Edge { node: 0, cost: 1 },
            Edge { node: 2, cost: 1 },
        ],
    ];
    let root = 0;
    let objective = 1;

    // Test search algorithm
    assert_eq!(shortest_path(&graph, root, objective), Some(5));
}
