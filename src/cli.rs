// Used to get Arguments passed in with cargo run
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Cihan";
    let status = "100%";

    // passing in additional Args whilst using cargo run XYZ will appear
    // println!("Args: {:?}", args);
    // println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}.", status);
    } else {
        println!("That is not a valid command");
    }
}
