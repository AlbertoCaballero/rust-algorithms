// Primitive str: Immutable fixed-length string somewhere in memory
// String: Growable, heap-allocated data structure

pub fn run() {
    // Primitive
    let hey = "Hey!";

    // Structure
    let mut hello = String::from("Hello!");
    
    // Push character
    hello.push('?');
    // Push string
    hello.push_str(" World!");

    // Length
    println!("hello.len() = {}", hello.len());
    // Capacity in bytes
    println!("hello.capacity() = {}", hello.capacity());
    // Is empty
    println!("hello.is_empty() = {}", hello.is_empty());
    // Contains
    println!("hello.contains(Hello) = {}", hello.contains("Hello"));
    // Replace
    println!("hello.replace() = {}", hello.replace("World", "Alberto"));

    // Loop through string by white space
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    
    println!("{:?}", (hey, hello));

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

}
