mod fibonacci;
mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;

fn main() {
    // Prints
    println!("\n\nPRINTS");
    print::run();

    // Variables
    println!("\n\nVARIABLES");
    vars::run();

    // Data Types
    println!("\n\nTYPES");
    types::run();

    // Strings
    println!("\n\nSTRINGS");
    strings::run();
    
    // Tuples
    println!("\n\nTUPLES");
    tuples::run();
    
    // Arrays
    println!("\n\nARRAYS");
    arrays::run();

    // Vectors 
    println!("\n\nVECTORS");
    vectors::run();
    
    // Conditionals 
    println!("\n\nCONDITIONALS");
    conditionals::run();

    // Fibonacci
    println!("\n\nFIBONACCI");
    for i in 0..40 {
        println!("{:?}", fibonacci::fibonacci(i));
    }
}
