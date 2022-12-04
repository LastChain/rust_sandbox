// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // For non-primitives: Assigning another variable -> first variable cannot hold the value.
    // You'll need to use a reference (&) to point to given resource.
    // Vector
    let vec1 = vec![3, 2, 1];
    let vec2 = &vec1; // & - else error

    println!("Values: {:?}.", (arr1, arr2));
    println!("Values: {:?}.", (&vec1, vec2));
}
