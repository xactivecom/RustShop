///////////////////////////////
// Course Section 6
///////////////////////////////
use std::ops::Add;

// Generic struct
struct Point<T, U> {
    x: T,
    y: T,
    name: U,
}

trait Overview {
    fn overview(&self) -> String {
        format!("Generic description")
    }
}

struct Course {
    headline: String,
    author: String,
}

struct AdvancedCourse {
    headline: String,
    author: String,
}

// Generic overview format
impl Overview for Course {}

// Override overview format
impl Overview for AdvancedCourse {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

// Generic trait parameter
fn call_overview<T: Overview>(item: &T) {
    println!("Overview: {}", item.overview())
}

// Utility traits
impl Drop for Course {
    fn drop(&mut self) {
        println!("drop {}", self.author)
    }
}

struct Coord<T> {
    x: T,
    y: T,
}

// Implement trait which restricts types that can be added to itself, 
// thereby yielding another T value.
impl<T> Add for Coord<T>
    where
    T: Add<Output = T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Coord {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

pub fn run_lesson() {
    println!("\nSection 6:");

    // Generic struct example
    let pt_1 = Point{x: -17, y: 9, name: String::from("origin")};
    println!("x:{}, y:{}, name:{}", pt_1.x, pt_1.y, pt_1.name);

    // Override trait
    let course_1 = Course{ author: String::from("Tyler"), headline: String::from("Learn from the original") };
    let course_2 = AdvancedCourse{ author: String::from("Sasha"), headline: String::from("Listen to the presentation") };
    println!("{}", course_1.overview());
    println!("{}", course_2.overview());

    // Trait parameter
    call_overview(&course_1);
    call_overview(&course_2);

    // Utility trait implicitly calls drop(course_1) when it goes out of scope

    // Test Add trait
    let coord_1 = Coord{ x: 10.4, y: 2.2 };
    let coord_2 = Coord{ x: 1.6, y: -1.2 };
    let sum = coord_1 + coord_2;
    println!("x:{}, y:{}", sum.x, sum.y);
}
