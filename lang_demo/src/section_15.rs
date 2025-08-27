///////////////////////////////
// Course Section 15
///////////////////////////////

// Macro times 5
macro_rules! x5 {
    ( $e:expr ) => { 5 * $e };
}

// Macro to create vector of strings
macro_rules! vec_str {
    (
        $($e:expr), * 
    ) => { 
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $e));
            )*
            v
        } 
    }
}

pub fn run_lesson() {
    println!("\nSection 15:");

    // See online book "Little Book of Rust Macros"
    let res = x5!(5);
    println!("{}", res);
    assert_eq!(res, 25);

    let res = vec_str!["a", "b", "c"];
    println!("{:?}", res);
}
