// Tuples group together values of different types
// Max 12 Elements

pub fn run() {
    let person: (&str, &str, i8) = ("Cihan", "Berlin", 36);
    println!("{} is from {} and is {}.", person.0, person.1, person.2);
    
}
