// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    println!("\n---------\nInfinite Loop\n---------\n");

    // Infinite Loop
    loop {
        count += 1;
        println!("Number: {}.", count);

        if count == 20 {
            break;
        }
    }

    println!("\n---------\nWhile Loop\n---------\n");

    // While Loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz")
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("Number: {}.", count);
        }

        // Inc
        count += 1;
    }

    println!("\n---------\nFor Range\n---------\n");

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("Number: {}.", x);
        }
    }
}
