// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {

    // Immutable
    let name = "Alberto";

    // Declared as mutable
    let mut age = 21;
    age = age + 1;
    
    println!("My name is {} and I am {}", name, age);


    // Define constants
    const ID: i32 = 1998;
    println!("ID: {}", ID);

    // Multiple var asignment
    let (my_name, my_age) = ("Alberto", 22);
    println!("N: {} A: {}", my_name, my_age);
}
