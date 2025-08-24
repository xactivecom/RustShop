///////////////////////////////
// Course Section 8
///////////////////////////////

use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn run_lesson() {
    println!("\nSection 8:");

    // Vector operations
    let mut nums = vec![];
    nums.push(2);
    nums.push(4);
    let n_pop = nums.pop();
    println!("popped {:?}", n_pop);
    let n_first = nums.first();
    println!("first {:?}", n_first);

    println!("is empty:{}, len:{}", nums.is_empty(), nums.len());
    nums.insert(0, 10);
    nums.insert(0, 12);
    nums.remove(2);
    println!("{:?}", nums);
    nums.sort();
    println!("{:?}", nums);

    // BinaryHeap implicitly performs descending sort order
    // This is useful for priority data structure
    let mut heap_1 = BinaryHeap::new();
    heap_1.push(10);
    heap_1.push(20);
    heap_1.push(30);
    heap_1.push(40);
    println!("original {:?}", heap_1);

    heap_1.pop();
    println!("popped {:?}", heap_1);
    println!("peek {:?}", heap_1.peek()); // peek only, not removed

    // HashMap data is stored in single heap
    let mut map_1 = HashMap::new();
    map_1.insert("first", 1);
    map_1.insert("first", 10); // replaces previous value
    map_1.insert("mid", 50);
    map_1.insert("last", 99);
    println!("map_1: {:?}", map_1);
    println!("mid: {:?}", map_1["mid"]);
    let first = map_1.remove_entry("first");
    println!("map_1: {:?}, removed {:?}", map_1, first);
    println!("is empty:{}, len:{}", map_1.is_empty(), map_1.len());

    // BtreeMap data is stored in an inverted tree

}
