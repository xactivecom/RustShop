///////////////////////////////
// Course Section 12
///////////////////////////////

#[derive(Debug)]
struct City {
    name: String,
    population: u64,
}

// Ascending sort cities
fn sort_cities(city: &mut Vec<City>) {
    city.sort_by_key(city_population)
}

// Helper to extract city population
fn city_population(c: &City) -> u64 {
    c.population
}

// Ascending sort cities using a closure
// Note: A closure exists on the stack (not heap) for better performance
fn sort_cities_closure(city: &mut Vec<City>) {
    city.sort_by_key(|c| c.population)
}

// Show use of closure with filter iterator
fn find_cities_by_name(cities: Vec<City>, name: String) -> Vec<City> {
    cities.into_iter().filter(|c| c.name == name).collect()
}

pub fn run_lesson() {
    println!("\nSection 12:");

    // Populate cities
    let city_1 = City{ name: String::from("Birmingham"), population: 820_891 };
    let city_2 = City{ name: String::from("Decatur"), population: 79_801 };
    let city_3 = City{ name: String::from("Mobile"), population: 1_200_431 };
    let city_4 = City{ name: String::from("Madison"), population: 48_012 };
    let city_5 = City{ name: String::from("Huntsville"), population: 549_221 };
    let mut cities: Vec<City> = vec![ city_1, city_2, city_3, city_4, city_5 ];
    println!("cities: {:?}", cities);

    // sort_cities(&mut cities);
    // println!("sorted_cities: {:?}", cities);

    sort_cities_closure(&mut cities);
    println!("sorted_cities: {:?}", cities);

    // Closure examples
    let add_ab = |a, b| a + b;
    println!("{}", add_ab(4, 5));
    assert_eq!(add_ab(4, 5), 9);

    let found_cities: Vec<City> = find_cities_by_name(cities, String::from("Huntsville"));
    println!("found_cities {:?}", found_cities);

    // Assignment 12
    let nums = vec![1, 3, 5, 7, 9];
    let nums_x_2: Vec<i32> = nums.into_iter().map(|v| v * 2).collect();
    println!("{:?}", nums_x_2);
}
