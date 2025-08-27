///////////////////////////////
// Course Section 15
///////////////////////////////

// Depend on external relative create project li
use macros::debug_print;

// Macro times 5
macro_rules! x5 {
    ( $e:expr ) => { 5 * $e };
}

// Macro to create vector of strings
macro_rules! vec_str {
    (
        $($e:expr), * 
    ) => {
        // Use inner block so variable can be defined
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $e));
            )*
            v
        } 
    };
}

// Declarative macro
macro_rules! average{
    // Empty list case
    (
        $(,)*
    ) => {
        0.0
    };

    // Non-empty list case
    (
        $($val:expr), + $(,)*
    ) => {
        // Use inner block so variable can be defined
        {
            let count = 0usize $(+ { let _ = stringify!($val); 1})*;
            let sum = 0.0 $(+ $val as f64)*;
            sum / count as f64
        }
    };
}

#[debug_print]
fn test_debug_macro() {
    // The debug_print macro should output:
    // "Test debug_print procedural macro"
}

pub fn run_lesson() {
    println!("\nSection 15:");

    // See online book "Little Book of Rust Macros"
    let res = x5!(5);
    println!("{}", res);
    assert_eq!(res, 25);

    let res = vec_str!["a", "b", "c"];
    println!("{:?}", res);

    // Test declarartive macro
    println!("{}", average!());
    println!("{}", average!(1.0, 2.0, 3.0));
    println!("{}", average!(1, 2, 3, 4, 5));

    // Test debug_print procedural macro
    test_debug_macro();
}
