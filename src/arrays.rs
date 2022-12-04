// Arrays - Fixed list where elements are the same data types

// before L:21 was commented
use std::mem;

pub fn run() {
  // let num: [i32;5] = [0,1,2,3,4];
  let mut num: [i32;5] = [0,1,2,3,4];

  // Re-assign value
  num[2] = 20;

  // Using Debug trait to print the whole array
  println!("{:?}", num);


  // Get array length
  println!("Array Length is \"{}\".", num.len());

  // Arrays are stack allocated
  // println!("Array occupies {} bytes in total", std::mem::size_of_val(&num));
  println!("Array occupies {} bytes in total.", mem::size_of_val(&num));

  // Get Slice
  // let slice: &[i32] = &num; // whole slice
  let slice: &[i32] = &num[0..3]; // parted
  // for array using the debug-trait
  println!("Slice: {:?}.", slice);  

  // Get single Value
  println!("Single Values: {}, {}, {}", num[0], num[1], num[2]);
}