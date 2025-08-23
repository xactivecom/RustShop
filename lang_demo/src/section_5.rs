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

pub fn run_lesson() {
    println!("\nSection 4:");

    let fish_cod = Fish::Cod;
    println!("{}", fish_cod.what_fish_am_i());
}
