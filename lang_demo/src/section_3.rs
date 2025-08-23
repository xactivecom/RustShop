///////////////////////////////
// Course Section 3
///////////////////////////////

///////////////////////////////
// Course Section 3
///////////////////////////////
fn test_ownership() {
    // These lines demonstrate how ownership of a heap-based variable gets moved 
    // to x, and that it goes out of scope upon assignment.
    // Note: copy trait is not implemented for heap-based variables
    let heap_v1 = vec!("Tyler");
    let x = heap_v1; // ownership gets moved to x
    println!("{:?}", x);

    // Cloning (deep copy) DOES NOT move ownership of a heap-based variable.
    let heap_v2 = vec!("Hello");
    let y = heap_v2.clone(); // ownership remains intact
    println!("{:?} {:?}", heap_v2, y);

    // Copy trait is pre-implemented for stack-based variables, so assignment
    // DOES NOT move ownership upon assignment.
    let stack_v1 = 10;
    let x = stack_v1;
    println!("{} {}", stack_v1, x);

    // Allocate heap-based variable, and move ownership to the function.
    // Once the function returns the variable is out of scope.
    let heap_s = String::from("First");
    string_own(heap_s); // ownership gets moved to function
    //println!("{:?}", heap_s);
}

fn string_own(s: String) -> String {
    let own_s = s;
    println!("{}", own_s);
    own_s
}

fn test_reference() {
    // Allocate heap-based variable, and use a shared reference to show
    // that it DOES NOT move ownership upon assignment.
    let heap1_s = String::from("Hello");
    let ref1_s = &heap1_s;
    let ref2_s = &heap1_s;
    println!("{} {} {}", heap1_s, ref1_s, ref2_s);

    // Allocate heap-based variable, and pass mutable reference to the function
    // which changes the value being referenced.
    let mut heap2_s = String::from("Hello");
    string_change(&mut heap2_s);
    println!("{}", heap2_s);
}

fn string_change(s: &mut String) {
    s.push_str(" World");
}

fn test_assign_3() {
    let mut vec = vec![1, 3, 5, 7];
    let res = assign3_1(&vec);
    println!("assign3_1 {}", res);
    vec.push(15);
    println!("vec {:?}", vec);

    let mut n: i8 = 10;
    add_two(&mut n);
    println!("{}", n);
}

fn assign3_1(vec: &Vec<i32>) -> bool {
    if vec[0] == 1 {
        true
    } else {
        false
    }
}

fn add_two(n: &mut i8) {
    *n += 2;
}

pub fn run_lesson() {
    println!("\nSection 3:");

    test_ownership();
    test_reference();
    test_assign_3();
}
