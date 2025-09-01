///////////////////////////////
// Course Section 22
///////////////////////////////

// A generic linked-list connector
// Use Option because a list node can point to None
type Link<T> = Option<Box<Node<T>>>;

// A linked-list structure starts with a head link.
pub struct List<T> {
    head: Link<T>,
}

// A linked-list consists of a data element and link to next
struct Node<T> {
    elem: T,
    next: Link<T>,
}

// Wrap the linked-list in a tuple struct, then access by index number
pub struct IntoIter<T>(List<T>);

// Linked-list iterator to get reference to next node
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// Linked-list iterator to get mutable reference to next node
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}


// Linked-list implementation
impl<T> List<T> {
    // Ctor
    pub fn new() -> Self {
        List { head: None }
    }

    // Push a new node with specified data element into the list.
    pub fn push(&mut self, elem: T) {
        // Wrap new node with a Box pointer.
        // Take the value from the current head which leaves None in its place.
        let new_node = Box::new(
            Node {
                elem: elem,
                next: self.head.take(),
            }
        );

        // Point head to the new node
        self.head = Some(new_node);
    }

    // Pop the node from the last element that was added to the list.
    // The node gets removed from the list.
    pub fn pop(&mut self) -> Option<T> {
        // Take the value from the current head and return it.
        // The head changes to the next.
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // Peek at the node from the head of the list and return a reference to it.
    // The node DOES NOT get removed from the list.
    pub fn peek(&self) -> Option<&T> {
        // Convert the &Option<T> to Option<&T>
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    // Peek at the node from the head of the list which allows its
    // value to be changed, but DOES NOT remove it from the list.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // Convert the &Option<T> to Option<&T>
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            // Dereference the box before taking the reference
            next: self.head.as_deref()
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            // Dereference the box before taking the reference
            next: self.head.as_deref_mut()
        }
    }
}

// Provide a custom iterative Drop trait to prevent the possibility that the default 
// recursive drop could blow the stack, when the linked-list goes out-of-scope.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut next_node) = link {
            link = next_node.next.take();
        }
    }
}

// Iterator trait for linked-list
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iterator trait for linked-list Iter type
impl<'a, T> Iterator for Iter<'a, T> {
    // Generic reference type with lifetime
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // Take the next value and dereference it.
        // Return a reference to the node element.
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// Iterator trait for linked-list for IterMut type
impl<'a, T> Iterator for IterMut<'a, T> {
    // Generic mutable reference type with lifetime
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // Take the next value and dereference it.
        // Return a mutable reference to the node element.
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}


pub fn run_lesson() {
    println!("\nSection 22:");

    // Linked list of integers
    let mut int_list: List<i32> = List::new();
    int_list.push(1);
    int_list.push(2);
    int_list.push(3);

    // Test pop
    assert_eq!(int_list.pop(), Some(3));
    assert_eq!(int_list.pop(), Some(2));

    // Test peek
    assert_eq!(int_list.peek(), Some(&1));
    let elem: Option<&i32> = int_list.peek();
    assert_eq!(elem, Some(&1));

    // Test peek_mutable
    int_list.peek_mut().map(|value| {
        *value = 42
    });
    assert_eq!(int_list.peek_mut(), Some(&mut 42));
    assert_ne!(int_list.peek_mut(), Some(&mut 1));

    // Test list is empty
    assert_eq!(int_list.pop(), Some(42));
    assert_eq!(int_list.pop(), None);

    // Repopulate the list
    int_list.push(1);
    int_list.push(2);
    int_list.push(3);

    // Test iterator
    let mut int_iter = int_list.iter();
    assert_eq!(int_iter.next(), Some(&3));
    assert_eq!(int_iter.next(), Some(&2));
    assert_eq!(int_iter.next(), Some(&1));
    assert_eq!(int_iter.next(), None);

    // Repopulate the list
    int_list.push(1);
    int_list.push(2);
    int_list.push(3);

    // Test mutable iterator
    let mut int_iter_mut = int_list.iter_mut();
    assert_eq!(int_iter_mut.next(), Some(&mut 3));
    assert_eq!(int_iter_mut.next(), Some(&mut 2));
    int_iter_mut.next().map(|value| {
        *value = 42;
        assert_eq!(*value, 42);
    });
    println!("{:?}", int_iter_mut.next());
    // assert_eq!(int_iter_mut.next(), None);


    // Linked list of strings
    let mut str_list: List<String> = List::new();
    str_list.push(String::from("red"));
    str_list.push(String::from("green"));
    str_list.push(String::from("blue"));

    // Test pop
    assert_eq!(str_list.pop(), Some(String::from("blue")));
    assert_eq!(str_list.pop(), Some(String::from("green")));

    // Test peek
    assert_eq!(str_list.peek(), Some(&String::from("red")));
    let elem: Option<&String> = str_list.peek();
    assert_eq!(elem, Some(&String::from("red")));

    // Test peek_mutable
    str_list.peek_mut().map(|value| {
        *value = String::from("white")
    });
    assert_eq!(str_list.peek_mut(), Some(&mut String::from("white")));
    assert_ne!(str_list.peek_mut(), Some(&mut String::from("red")));

    // Test list is empty
    assert_eq!(str_list.pop(), Some(String::from("white")));
    assert_eq!(str_list.pop(), None);

    // Repopulate the list
    str_list.push(String::from("red"));
    str_list.push(String::from("green"));
    str_list.push(String::from("blue"));

    // Test iterator
    let mut str_iter = str_list.iter();
    assert_eq!(str_iter.next(), Some(&String::from("blue")));
    assert_eq!(str_iter.next(), Some(&String::from("green")));
    assert_eq!(str_iter.next(), Some(&String::from("red")));
    assert_eq!(str_iter.next(), None);

    // Repopulate the list
    str_list.push(String::from("red"));
    str_list.push(String::from("green"));
    str_list.push(String::from("blue"));

    // Test mutable iterator
    let mut str_iter_mut = str_list.iter_mut();
    assert_eq!(str_iter_mut.next(), Some(&mut String::from("blue")));
    assert_eq!(str_iter_mut.next(), Some(&mut String::from("green")));
    str_iter_mut.next().map(|value| {
        *value = String::from("white");
        assert_eq!(*value, String::from("white"));
    });
    // assert_eq!(str_iter_mut.next(), None);

}
