///////////////////////////////
// Course Section 14
///////////////////////////////

use std::thread;
use std::sync::{mpsc, Arc, Mutex};

fn single_producer() {
    let treasure_msg = String::from("treasure");

    // A channel is a message passing mechanism for threads
    // Note: mpsc stands for "multiple producer, single consumer"
    // Note: ownership of message is transferred to sender within closure
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        // Unwrap because it returns a Result object
        sender.send(treasure_msg).unwrap();
    });

    // Unwrap Result object
    // Note: ownership of message is transferred to receiver
    let msg = receiver.recv().unwrap();
    println!("received:{}", msg);
}

fn multiple_producers() {
    // Create sender, receiver
    let (sender_1, receiver) = mpsc::channel();
    let sender_2 = sender_1.clone();

    // Send multiple messages from sender 1
    thread::spawn(move || {
        let msgs = vec![
            String::from("intro 1"),
            String::from("intro 2"),
            String::from("intro 3"),
        ];
        for m in msgs {
            // Unwrap because it returns a Result object
            sender_1.send(m).unwrap();
        }
    });

    // Send multiple messages from sender 2
    thread::spawn(move || {
        let msgs = vec![
            String::from("part 1"),
            String::from("part 2"),
            String::from("part 3"),
        ];
        for m in msgs {
            // Unwrap because it returns a Result object
            sender_2.send(m).unwrap();
        }
    });

    // Receive all messages
    for rec in receiver {
        println!("{}", rec);
    }
}

fn sync_channel() {
    // A sync channel uses a fixed queue size which blocks the sender when the
    // queue becomes full, until the receiver can drain the queue.
    let (sender, receiver) = mpsc::sync_channel::<String>(5);

    thread::spawn(move || {
        for i in 0..10 {
            let msg = String::from(format!("msg {i}"));
            // Unwrap because it returns a Result object
            sender.send(msg).unwrap();
        }
    });

    // Receive all messages
    for rec in receiver {
        println!("{}", rec);
    }
}

fn shared_state() {
    // Initialize atomic counter
    let counter = Arc::new(Mutex::new(0));

    // Create threads and increment count
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait for threads
    for handle in handles {
        handle.join().unwrap();
    }
    println!("count:{}", counter.lock().unwrap());
}

pub fn run_lesson() {
    println!("\nSection 14:");

    let treasure = String::from("treasure");

    // The move keyword allows the closure to take ownership of the values
    // it uses, such as variable treasure
    let handle = thread::spawn(move || {
        println!("thread {:?}", treasure);
    });

    // Wait for thread
    println!("main waiting for thread");
    handle.join().unwrap();
    println!("main done waiting for thread");

    // Create multiple threads
    let v_n = vec![1, 2, 3];
    let mut handles = Vec::new();
    for v in v_n {
        handles.push(thread::spawn(move || println!("thread:{}", v)));
    }

    // Wait for threads. Note: thread order may not be sequential.
    println!("main waiting for all threads");
    for handle in handles {
        handle.join().unwrap();
    }
    println!("main done waiting for all threads");

    // Multithread examples
    single_producer();
    multiple_producers();

    // Thread communication
    sync_channel();

    // Concurrency
    shared_state();

}