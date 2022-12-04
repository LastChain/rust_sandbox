// Structs - Used to create custom data types

// Uppercase-Convention using Structs
// Traditional Struct
struct ColorStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorTupleStruct(u8, u8, u8);

// --
struct Person {
    first_name: String,
    last_name: String,
}

// Implement function
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name of "Person"
    // self is similar to how "this" works
    // referencing the struct of Person
    // again, returning a value without semicolon
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set first name
    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c1 = ColorStruct {
        red: 255,
        green: 0,
        blue: 0,
    };

    c1.red = 200;

    println!("Color Struct: R({}) G({}) B({})", c1.red, c1.green, c1.blue);

    let mut c2 = ColorTupleStruct(255, 0, 0);

    c2.0 = 80;
    c2.1 = 80;
    c2.2 = 80;

    println!("Color Tuple: R({}) G({}) B({})", c2.0, c2.1, c2.2);

    // --- hint value borrowed here after move

    let mut p = Person::new("Mary", "Williams");
    println!("(A) Full name of Person: {}", p.full_name());
    println!("(A) Person: {}, {}", p.last_name, p.first_name);
    p.set_first_name("Monica");
    println!("(A) Person Tuple: {:?}", p.to_tuple()); // using the "debug-trait" (tuple)

    p = Person::new("Mary", "Doe");
    p.set_last_name("Williams");

    println!("(B) Full name of Person: {}", p.full_name());
    println!("(B) Person: {}, {}", p.last_name, p.first_name);
    println!("(B) Person Tuple: {:?}", p.to_tuple());
}
