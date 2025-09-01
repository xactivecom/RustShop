///////////////////////////////
// Course Section 23
///////////////////////////////

use std::collections::LinkedList;

// The queue structure is first-in-first-out (FIFO)
// Backed by stantard linked-list
pub struct Queue<T> {
    list: LinkedList<T>,
}

impl<T> Queue<T> {
    // Ctor
    pub fn new() -> Queue<T> {
        Queue {
            list: LinkedList::new() 
        }
    }

    // Push a new node with specified data element into the queue.
    pub fn enqueue(&mut self, elem: T) {
        self.list.push_back(elem);
    }

    // Pop the node from the first element that was added to the queue.
    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    // Peek at the node at the front of the queue and return a reference to it.
    pub fn peek(& self) -> Option<&T> {
        self.list.front()
    }

    // Get the length of the queue.
    pub fn length(& self) -> usize {
        self.list.len()
    }

    // Determine if the queue is empty.
    pub fn is_empty(& self) -> bool {
        self.list.is_empty()
    }
}


pub fn run_lesson() {
    println!("\nSection 23:");

    // Queue of integers
    let mut int_queue: Queue<i32> = Queue::new();
    int_queue.enqueue(1);
    int_queue.enqueue(2);
    int_queue.enqueue(3);

    // Test queue size
    assert_eq!(int_queue.length(), 3);
    assert_eq!(int_queue.is_empty(), false);

    // Test dequeue
    assert_eq!(int_queue.dequeue(), Some(1));
    assert_eq!(int_queue.dequeue(), Some(2));
    assert_eq!(int_queue.dequeue(), Some(3));

    // Test queue size
    assert_eq!(int_queue.length(), 0);
    assert_eq!(int_queue.is_empty(), true);

    // Repopulate queue
    int_queue.enqueue(1);
    int_queue.enqueue(2);
    int_queue.enqueue(3);

    // Test peek
    assert_eq!(int_queue.peek(), Some(&1));
    assert_eq!(int_queue.peek(), Some(&1));
    assert_eq!(int_queue.length(), 3);
    assert_eq!(int_queue.is_empty(), false);

}
