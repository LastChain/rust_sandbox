// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut num: Vec<i32> = vec![0, 1, 2, 3, 4];

    // Re-assign value
    num[2] = 20;

    // Add on to vector
    num.push(4);
    num.push(33);

    // Pop off last value
    num.pop();

    // Using Debug trait to print the whole vector
    println!("{:?}", num);

    // Get vector length
    println!("Vector Length is \"{}\".", num.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes in total.", mem::size_of_val(&num));

    // Get Slice
    let slice: &[i32] = &num; // whole slice
                              // let slice: &[i32] = &num[0..3]; // parted

    // for vector using the debug-trait
    println!("Slice: {:?}.", slice);

    // Get single Value
    println!("Single Values: {}, {}, {}", num[0], num[1], num[2]);

    // Loop through vector values
    for x in num.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    // similar to javascript map (multiply x2 each)
    for x in num.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", num)
} // fn run
