///////////////////////////////
// Course Section 5
///////////////////////////////

enum Fish {
    Cod, Salmon, Tuna
}

impl Fish {
    fn what_fish_am_i(self) -> &'static str {
        match self {
            Fish::Cod => "cod",
            Fish::Salmon => "salmon",
            Fish::Tuna => "tuna",
        }
    }
}

enum IpAddrKind {
    IPv4, IPv6
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

// Increment number if not None
fn optional_add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1)
    }
}

// Assignment 5
enum Shape {
    Triangle, Square, Pentagon, Octagon
}
impl Shape {
    fn corners(self) -> i32 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Octagon => 8,
        }
    }
}

#[allow(unused_variables)]
pub fn run_lesson() {
    println!("\nSection 5:");

    // Enum examples
    let fish_cod = Fish::Cod;
    println!("I am a {}", fish_cod.what_fish_am_i());

    let ip_1 = IpAddr{ kind: IpAddrKind::IPv4, addr: String::from("192.168.2.114") };
    let ip_2 = IpAddr{ kind: IpAddrKind::IPv6, addr: String::from("::1") };
    println!("IPs {}, {}", ip_1.addr, ip_2.addr);

    // Optional enum examples
    let som_num = Some(7);
    let some_str = Some("naming");
    let no_num: Option<i32> = None;

    // Optional function
    let x_n = None; 
    let res_x_n = optional_add_one(x_n);
    println!("x_n {:?} -> res:{:?}", x_n, res_x_n);
    assert_eq!(res_x_n, None);

    let x_1 = Some(5); 
    let res_x_1 = optional_add_one(x_1);
    println!("x_1 {:?} -> res:{:?}", x_1, res_x_1);
    assert_eq!(res_x_1, Some(6));

    // Assignment 5
    println!("triangle {}", Shape::Triangle.corners());
    println!("square {}", Shape::Square.corners());
    println!("pentagon {}", Shape::Pentagon.corners());
    println!("octagon {}", Shape::Octagon.corners());
}
