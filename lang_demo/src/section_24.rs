///////////////////////////////
// Course Section 24
///////////////////////////////


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

    // Insert a new value into the BST
    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            // Simple set if root node
            self.value = Some(value)
        } else {
            // Insert into proper order
            match &self.value {
                None => (),
                Some(key) => {
                    // Smaller value to the left, otherwise right
                    let target_node = if value < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };

                    match target_node {
                        Some(ref mut node) => {
                            node.insert(value)
                        },
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        },
                    }
                },
            }
        }
    }

    // Return an iterator which iterates in order
    // of smallest to greatest
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }

}

// Iterator for binary search tree
struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,    
}

impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    pub fn new() {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

}



pub fn run_lesson() {
    println!("\nSection 24:");

    // Create binary search for integers
    let mut int_bst = BinarySearchTree::new();
    int_bst.insert(1);
    int_bst.insert(3);
    int_bst.insert(5);
}