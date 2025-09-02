///////////////////////////////
// Course Section 24
///////////////////////////////

use std::{ cmp::Ordering, ops::Deref };

// A binary search tree (BST) has these properties:
// - the left subtree has values less than the curent node
// - the right subtree has values greater than the current node
// - the left and right subtrees must also be a binary search tree
//
// T.Ord refers to ordering
// The Box stores te pointers to other nodes on the heap
pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

// Default trait
impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    // Ctor creates the empty root node
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    // Insert a new value into the tree
    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            // Simple set if root node
            self.value = Some(value)
        } else {
            // Insert value in proper order (smaller to left, otherwise right)
            match &self.value {
                None => (),
                Some(key) => {
                    // Select target left node if a smaller value, otherwise right node
                    let target_node = if value < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };

                    // At target node, match on the value
                    match target_node {
                        Some(node) => {
                            // Recursively insert node
                            node.insert(value)
                        },
                        None => {
                            // Create new subtree at empty node and recursively insert node.
                            // Update target node to point to the new subtree.
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        }
                    }
                }
            }
        }
    }

    // Return an iterator which iterates in order of smallest to largest
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }

    // Search tree for specified value
    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => {
                // Compare tree key to value
                match key.cmp(value) {
                    Ordering::Equal => {
                        true
                    },
                    Ordering::Greater => {
                        // Recursively traverse left to smaller value
                        match &self.left {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    },
                    Ordering::Less => {
                        // Recursively traverse right to larger value
                        match &self.right {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                }
            },
            None => false,
        }
    }

    // Retrieve the smallest value
    pub fn minimum(&self) -> Option<&T> {
        // Traverse smallest values from the left
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
    }

    // Retrieve the largest value
    pub fn maximum(&self) -> Option<&T> {
        // Traverse largest values from the right
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
    }

    // Retrieve the largest value which is smaller than the specified value
    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                // Compare tree key to value
                match key.cmp(value) {
                    Ordering::Greater => {
                        // If key is too large, recursively traverse left to smaller value
                        match &self.left {
                            Some(node) => node.floor(value),
                            None => None,
                        }
                    },
                    Ordering::Less => {
                        // If key is too small recursively traverse right to larger value
                        match &self.right {
                            Some(node) => {
                                let val = node.floor(value);
                                // If value if valid, return it, otherwise return the key
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            },
                            None => Some(key),
                        }
                    },
                    Ordering::Equal => Some(key)
                }
            },
            None => None,
        }
    }

    // Retrieve the smallest value which is greater than the specified value
    pub fn ceil(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                // Compare tree key to value
                match key.cmp(value) {
                    Ordering::Less => {
                        // If key is too small, recursively traverse right to greater value
                        match &self.right {
                            Some(node) => node.ceil(value),
                            None => None,
                        }
                    },
                    Ordering::Greater => {
                        // If key is too large recursively traverse left to smaller value
                        match &self.left {
                            Some(node) => {
                                let val = node.ceil(value);
                                // If value if valid, return it, otherwise return the key
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            },
                            None => Some(key),
                        }
                    },
                    Ordering::Equal => Some(key)
                }
            },
            None => None,
        }
    }
}
// Declare binary search tree iterator
struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,    
}

// Implementation of binary earch tree iterator
impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    // Ctor references an existing tree
    pub fn new(tree: &'a BinarySearchTree<T>) -> BinarySearchTreeIter<'a, T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    // Helper to traverse all left values and push then on the stack
    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

// Implementation of binary earch tree iterator in order of smallest to largest
impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    // Iteror next method to rturn node value
    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
            None
        } else {
            // Take all values from the stack
            let node = self.stack.pop().unwrap();

            // If the node has a right value, then push that onto the stack
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();
            }
            node.value.as_ref()
        }
    }
}


pub fn run_lesson() {
    println!("\nSection 24:");

    // Create binary search for integers
    let mut int_tree: BinarySearchTree<i32> = BinarySearchTree::new();
    int_tree.insert(1);
    int_tree.insert(42);
    int_tree.insert(19);
    int_tree.insert(-5);
    int_tree.insert(25);

    // Test iterator - expect smallest to largest
    let mut int_tree_iter = BinarySearchTreeIter::new(&int_tree);
    loop {
        match int_tree_iter.next() {
            Some(value) => {
                //let value = node.unwrap();
                println!("value:{}", value);
            },
            None => break
        }
    }

    // Test search
    assert_eq!(int_tree.search(&42), true);
    assert_eq!(int_tree.search(&0), false);

    // Test minimum, maximum
    assert_eq!(int_tree.minimum(), Some(&-5));
    assert_eq!(int_tree.maximum(), Some(&42));

    // Test floor, ceil
    assert_eq!(int_tree.floor(&26).unwrap(), &25);
    assert_eq!(int_tree.floor(&0).unwrap(), &-5);
    assert_eq!(int_tree.ceil(&0).unwrap(), &1);
    assert_eq!(int_tree.ceil(&20).unwrap(), &25);

}