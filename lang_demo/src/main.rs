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

    // Empty vector
    let mut v2 = Vec::new();
    v2.push("second");
    v2.push("first");
    v2.reverse();
    println!("{:?}", v2);
    assert_eq!(v2, ["first", "second"]);

    // Empty vector
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

fn test_gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn main() {
    test_array();
    test_vector();
    test_slice();
    test_string();

    let c = test_gcd(20, 5);
    println!("{:?}", c);
    assert_eq!(c, 5);
}