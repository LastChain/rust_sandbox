// Variables hold primitive data or references to data
// Variables are immutable by default (meaning you cannot re-assign them afterwards)
// Rust is a block-scoped language (e.g. Variables inside functions are available inside that function ONLY)

pub fn run() {
    let name = "Cihan";
    // let age = 36;
    let mut age = 36;
    println!("My name is {} and I am {}", name, age);
    // happy Bday!
    age = 37;
    println!("My name is {} and I am {}", name, age);

    // Define constant 
    // Constants have to be explicitly defined (and UPPERCASE)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Cihan", 36);
    println!("{} is {}", my_name, my_age);
}
