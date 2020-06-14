/*
Structs - Used to create custom data type
*/

// Traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuole Struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Contructthe Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get Full Name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set Last Name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    //  Name to tuple

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };
    // c.green = 200;
    // println!("Color: {} {} {}", c.red, c.green, c.blue)

    // let mut c = Color(255, 0, 0);
    // c.0 = 200;
    // println!("Color: {} {} {}", c.0, c.1, c.2)

    let mut p = Person::new("Chiamaka", "Nnakwu");
    println!("Person {} ", p.full_name());
    p.set_last_name("Okpala");
    println!("Person {} ", p.full_name());
    println!("Person Tuple {:?} ", p.to_tuple());
}