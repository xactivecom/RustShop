///////////////////////////////
// Course Section 4
///////////////////////////////

struct User {
    active: bool,
    name: String,
    count: u32,
}

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Lifetime in function
fn life_fn<'a>(x: &'a str) -> &'a str {
    x
}

// Lifetime in struct
struct LifeString<'a> {
    name: &'a str,
}

pub fn run_lesson() {
    println!("\nSection 4:");

    let user1 = User{active: true, name: String::from("Tyler"), count: 0};
    println!("active:{}, name:{}, count:{}", user1.active, user1.name, user1.count);

    let obj = Square{width: 2, height:4};
    println!("area:{}", obj.area());

    // Lifetime needed to ensure x DOES NOT outlives returned y, otherwise 
    // x would be a dangling reference
    let x = String::from("hello");
    let y = life_fn(&x);
    println!("x:{}, y:{}", x, y);

    // Lifetime needed to ensure that s DOES NOT outlive returned p
    // s would be a dangling reference
    let s = String::from("Life String");
    let p = LifeString{name: s.as_str()};
    println!("{}", p.name);
}
