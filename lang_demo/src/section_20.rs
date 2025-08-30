///////////////////////////////
// Course Section 20
///////////////////////////////

// Compute factorial recursively
fn fact(num: i32) -> i32 {
    if num < 1 {
        return 1
    }
    num * fact(num - 1)
}

// Compute last Fibonacci sequence number
fn fib(num: i32) -> i32 {
    if num <= 1 {
        return num
    }
    let n_1 = fib(num - 1);
    let n_2 = fib(num - 2);
    n_1 + n_2
}

// Determine if sequence is a padindrome.
// This determines if left-to-right is the same as right-to-left
fn is_palindrome(seq: &Vec<i32>, start: usize, end: usize) -> bool {
    // Quick result
    if start >= end {
        return true;
    }

    // Start with initial check, then close the gap
    if seq[start] == seq[end] {
        return is_palindrome(seq, start + 1, end - 1)
    }
    false
}

// Tower of Hanoi rules:
// - there are three poles Left, Center and Right
// - move all disks from Left pole to Right pole in the same order
// - a larger disk cannot sit on a smaller disk
// - use pole Center pole for temporary swapping
fn play_hanoi(num_disks: u8, from: char, to: char, via: char) {
    // Check if all disks already moved
    if num_disks == 0 {
        return;
    }

    // Move n-1 disks from 'from' to 'via' via 'to'
    play_hanoi(num_disks - 1, from, via, to);

    // Move remaining largest disk from 'from' to 'to'
    println!("move {} disks from {from} to {to}", num_disks);

    // Move n-1 disks from 'via' to 'to' via 'from'
    play_hanoi(num_disks - 1, via, to, from);
}

pub fn run_lesson() {
    println!("\nSection 20:");

    // Factorial example
    let num = 5;
    let fact_num = fact(num);
    println!("num:{}, fact:{}", num, fact_num);
    assert_eq!(fact_num, 120);

    // Fibonacci example
    let num = 15;
    let fib_num = fib(num);
    println!("num:{}, fib:{}", num, fib_num);
    assert_eq!(fib_num, 610);

    // Palindrome examples
    let seq_1 = vec![1, 2, 3, 4];
    let test_1 = is_palindrome(&seq_1, 0, seq_1.len() - 1);
    println!("is_pal:{test_1}");

    let seq_2 = vec![1, 2, 3, 4, 3, 2, 1];
    let test_2 = is_palindrome(&seq_2, 0, seq_2.len() - 1);
    println!("is_pal:{test_2}");

    // Tower of Hanoi example
    let num_disks = 4;
    play_hanoi(num_disks, 'L', 'R', 'C');
}
