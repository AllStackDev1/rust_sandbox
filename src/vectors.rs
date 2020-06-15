/*
Vectors - Resizable arrays
*/

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Reassign value
    numbers[0] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off the last value
    numbers.pop();

    println!("{:?}", numbers);
    // Get signle value
    println!("Signle value: {}", numbers[0]);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes ", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutated vector values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
