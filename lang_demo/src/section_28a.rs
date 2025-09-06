///////////////////////////////
// Course Section 28a
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

    // Add graph edges
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

// Kruska's MST algorithm
fn mst_kruska(graph: &Graph) -> Vec<(Node, Node)> {
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

    // Populate weighted graph
    let graph = vec![
        vec![ // Node 0
            Edge { dest: 1, weight: 3 },
            Edge { dest: 3, weight: 6 },
            Edge { dest: 5, weight: 1 },
        ],
        vec![ // Node 1
            Edge { dest: 3, weight: 5 },
            Edge { dest: 5, weight: 4 },
            Edge { dest: 2, weight: 1 },
        ],
        vec![ // Node 2
            Edge { dest: 3, weight: 2 },
            Edge { dest: 4, weight: 3 },
        ],
        vec![ // Node 3
            Edge { dest: 4, weight: 7 },
        ],
        vec![ // Node 4
            Edge { dest: 5, weight: 2 },
        ],
        vec![ // Node 5
        ]
    ];

    // Test Kruska's MST
    let mst_order= vec![(0, 5), (1, 2), (2, 3), (4, 5), (0, 1)];
    assert_eq!(mst_kruska(&graph), mst_order);
}