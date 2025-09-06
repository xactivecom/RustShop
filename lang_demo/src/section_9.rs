///////////////////////////////
// Course Section 9
///////////////////////////////

use std::fs::File;
// use std::io::ErrorKind;
use std::io::Error;

const TEST_FILE: &'static str = "error.txt";

fn file_open() -> Result<File, Error> {
    // The ? is how an error is propagated to caller
    let file = File::open(TEST_FILE)?;
    Ok(file)
}

pub fn run_lesson() {
    println!("\nSection 9:");

    // Explicit panic
    // panic!("crash boom!");

    // Runtime panic
    // let v = vec![1, 2, 3];
    // let vv = v[5];

    // Create or open file and handle errors
    // let file = match File::open(TEST_FILE) {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         // Create file if not found
    //         ErrorKind::NotFound => match File::create(TEST_FILE) {
    //             Ok(file_created) => file_created,
    //             Err(error) => panic!("Failed to create file {} - {:?}", TEST_FILE, error),
    //         }
    //         _ => panic!("Other error for file {}", TEST_FILE),
    //     }
    // };

    // Method unwrap() will panic upon error
    // let file = File::open(TEST_FILE).unwrap();

    // Method expect() allows custom error
    // let file = File::open(TEST_FILE).expect("Error opening file");

    // Test error propagation
    // let test = file_open();
    // test.unwrap();
}
