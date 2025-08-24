///////////////////////////////
// Course Section 4
///////////////////////////////

// Tuple-like struct
struct Point(i32, i32);

// Named-field struct
struct User {
    active: bool,
    name: String,
    count: u32,
}

// Named-field struct with implementation method
struct Square {
    width: u32,
    height: u32,
}

impl Square {
    // Method for area of square
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Note reference to mutable self for modification
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    // Note reference to mutable self for modification
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

// Unit-like may not contain data, but just a trait
struct Marker;
impl Marker {
    // Marker method
    fn show(&self) -> String {
        "hello".to_string()
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

// Assignment 4
struct Car {
    mpg: u32,
    color: String,
    top_speed: u32,
}
impl Car {
    fn set_mpg(&mut self, mpg: u32) {
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String) {
        self.color = color;
    }
    fn set_top_speed(&mut self, top_speed: u32) {
        self.top_speed = top_speed;
    }
}

pub fn run_lesson() {
    println!("\nSection 4:");

    // Tuple-like struct
    let coord = Point(3, -2);
    println!("x:{}, y:{}", coord.0, coord.1);

    // Field-named struct
    let user1 = User{ active: true, name: String::from("Tyler"), count: 0 };
    println!("active:{}, name:{}, count:{}", user1.active, user1.name, user1.count);

    let mut obj = Square{ width: 2, height:4 };
    println!("area:{}", obj.area());
    obj.set_height(3);
    obj.set_width(4);
    println!("area:{}", obj.area());

    // Unit struct
    println!("{}", Marker.show());

    // Lifetime needed to ensure x DOES NOT outlives returned y, 
    // otherwise x would be a dangling reference
    let x = String::from("hello");
    let y = life_fn(&x);
    println!("x:{}, y:{}", x, y);

    // Lifetime needed to ensure that s DOES NOT outlive returned p,
    // otherwise s would be a dangling reference
    let s = String::from("Life String");
    let p = LifeString{name: s.as_str()};
    println!("{}", p.name);

    // Static lifetime are stored in the programs binary
    let always_str: &'static str = "text is forever";
    println!("always {}", always_str);

    // Assignment 4
    let mut car = Car{mpg: 350, color: String::from("white"), top_speed: 160};
    println!("mpg:{}, color:{}, top_speed:{}", car.mpg, car.color, car.top_speed);
    car.set_mpg(370);
    car.set_color(String::from("blue"));
    car.set_top_speed(180);
    println!("mpg:{}, color:{}, top_speed:{}", car.mpg, car.color, car.top_speed);
}
