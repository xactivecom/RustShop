///////////////////////////////
// Course Section 22
///////////////////////////////

// A generic linked-list connector
// Use Option because a list node can point to None
type Link<T> = Option<Box<Node<T>>>;

// A linked-list structure starts with a head link
pub struct List<T> {
    head: Link<T>,
}

// A linked-list consists of a data element and link to next
struct Node<T> {
    elem: T,
    next: Link<T>,
}

// Linked-list implementation
impl<T> List<T> {
    // Ctor
    pub fn new() -> Self {
        List { head: None }
    }

    // Push a new node with specified data element in the list
    pub fn push(&mut self, elem: T) {
        // Wrap new node with a Box pointer
        // Take the value from the current head which leaves None in its place
        let new_node = Box::new(
            Node {
                elem: elem,
                next: self.head.take(),
            }
        );

        // Make new node the head
        self.head = Some(new_node);
    }

    // Pop the node from the head of the list
    pub fn pop(&mut self) -> Option<T> {
        // Take the value from the current head and return it
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

pub fn run_lesson() {
    println!("\nSection 22:");

    // Linked list
    let mut int_list = List::new();
    int_list.push(1);
    int_list.push(2);
    int_list.push(3);

    // Test pop
    assert_eq!(int_list.pop(), Some(3));
    assert_eq!(int_list.pop(), Some(2));

    let mut str_list = List::new();
    str_list.push(String::from("red"));
    str_list.push(String::from("green"));
    str_list.push(String::from("blue"));

    // Test pop
    assert_eq!(str_list.pop(), Some(String::from("blue")));
    assert_eq!(str_list.pop(), Some(String::from("green")));
}
