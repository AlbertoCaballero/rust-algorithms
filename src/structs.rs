// Structs are used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct TColor(u8, u8, u8);

// Structs with functions
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Constructor of person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 250,
        green: 100,
        blue: 0
    };

    c.blue = 50;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let tc = TColor(250, 80, 64);
    println!("TColor: {} {} {}", tc.0, tc.1, tc.2);

    let mut person = Person::new("John", "Doe");
    person.set_last_name("William");
    println!("Person {} {}", person.first_name, person.last_name);
    println!("{}", person.full_name());
    
    println!("{:?}", person.to_tuple());
}

