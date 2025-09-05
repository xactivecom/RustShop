///////////////////////////////
// Course Section 28
///////////////////////////////

use disjoint_sets::UnionFind;

// Rules for a minimum spanning tree:
// 1: vertices must be connected
// 2: no cycles in vertices
// 3: number of edges = number of vetices - 1
// 4: sum of edge weights should be minimum

type Node = usize;
type Weight = usize;

struct Edge {
    dest: Node,
    weight: Weight,
}

type Graph = Vec<Vec<Edge>>;

fn edges_by_weight(graph: &Graph) -> Vec<(Node, Node, Weight)> {
    let mut edges = vec![];

    for (src, dest) in graph.iter().enumerate() {
        for edge in dest {
            edges.push(( src, edge.dest, edge.weight ));
        }
    }

    // Sort edges by weight
    edges.sort_by_key(|&(_, _, weight)| weight);

    // Return edges
    edges
}

fn mst(graph: &Graph) -> Vec<(Node, Node)> {
    let mut result = vec![];

    // Check if the graph contains cycles
    let mut union_find = UnionFind::new(graph.len());

    for (src, dest, _) in edges_by_weight(graph) {
        // Check if the cycle is already in the tree
        if !union_find.equiv(src, dest) {
            union_find.union(src, dest);
            result.push((src, dest));
        }
    }

    // Return result
    result
}

pub fn run_lesson() {
    println!("\nSection 28:");

    let mut graph = ;
    mst(&graph);
}