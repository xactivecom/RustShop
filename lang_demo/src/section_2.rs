///////////////////////////////
// Course Section 2
///////////////////////////////
#[allow(dead_code)]

fn test_array() {
    // Basic array
    let a1 = [1, 2, 3];
    println!("{}", a1[0]);
    assert_eq!(a1[0], 1);

    // Mutable array
    let mut a2: [i32; 3] = [4, 5, 6];
    a2[1] = 8;
    println!("{}", a2[1]);
    assert_eq!(a2[1], 8);
}

fn test_vector() {
    // Basic vector
    let mut v1 = vec!(7, 8, 9);
    v1.push(10);
    v1.pop();
    println!("{:?}", v1);
    assert_eq!(v1, [7, 8, 9]);

    // Empty vectors
    let mut v2 = Vec::new();
    v2.push("second");
    v2.push("first");
    v2.reverse();
    println!("{:?}", v2);
    assert_eq!(v2, ["first", "second"]);

    let mut v2_1 = vec![];
    v2_1.push(1);
    assert_eq!(v2_1[0], 1);

    // Fixed vector
    let mut v3 = Vec::<i32>::with_capacity(5);
    v3.push(10);
    println!("{:?}", v3);
    assert_eq!(v3, [10]);

    // Iterator initialized vector
    let v4: Vec<i32> = (0..5).collect();
    println!("{:?}", v4);
    assert_eq!(v4, [0, 1, 2, 3, 4]);
}

fn test_slice() {
    // Fat pointer
    let v: Vec<i32> = (0..5).collect();
    let p_v = &v[2..4];
    println!("{:?}", p_v);
    assert_eq!(p_v, [2, 3]);
}

fn test_string() {
    let name = String::from("Tyler");
    let the = " the ".to_string();
    let p_the = &the;
    let verb = String::from("Creator");
    let title = [name, p_the.to_string(), verb].concat();
    println!("{:?}", title);
    assert_eq!(title, "Tyler the Creator");
}

// Calculate the greatest common denominator
fn test_gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a; // swap
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn test_control() {
    // Named counter
    let mut inc = 0;
    'counter: loop {
        let mut dec = 5;
        loop {
            if dec == 4 {
                break;
            }
            if inc == 2 {
                break 'counter;
            }
            dec -= 1;
        }
        inc += 1;
    }
    println!("{}", inc);
    assert_eq!(inc, 2);

    // Loop over range
    for iter in (0..10).rev() {
        println!("{}", iter);
    }
}

fn test_assign_2() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("{}", ans);

    let mut vec = vec![2, 4, 6, 8, 10];
    println!("{:?}", vec);
    vec.pop();
    vec.push(12);
    println!("{:?}", vec);

    let res = concat_string(String::from("Hello"));
    println!("{}", res);

    control_flow(1);
    control_flow(10);
    control_flow(20);
    control_flow(30);
    control_flow(50);
    control_flow(60);
}

fn concat_string(s: String) -> String {
    s + " World"
}

fn control_flow(n: i32) {
    if n == 1 {
        println!("The value is one");
    } else if n > 50 {
        println!("The value is greater than 50");
    } else if n < 25 {
        println!("The value is less than 25");
    } else {
        println!("The value is greater than 25 but less than 50");
    }
}

pub fn run_lesson() {
    println!("\nSection 2:");

    test_array();
    test_vector();
    test_slice();
    test_string();

    let c = test_gcd(20, 5);
    println!("{:?}", c);
    assert_eq!(c, 5);

    test_control();
    test_assign_2();
}
