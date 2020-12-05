mod prime;
mod fibonacci;

fn main() {
    // Prime number
    let n :i32 = 11;
    println!("{}", prime::is_prime(n));

    // Fibonacci
    println!("\n\nFIBONACCI");
    for i in 0..20 {
        println!("{:?}", fibonacci::fibonacci(i));
    }
}

//mod print;
//mod vars;
//mod types;
//mod strings;
//mod tuples;
//mod arrays;
//mod vectors;
//mod conditionals;
//mod loops;
//mod functions;
//mod pointers_ref;
//mod structs;
//mod enums;
//mod cli;

    // Prints
//    println!("\n\nPRINTS");
//    print::run();

//    // Variables
//    println!("\n\nVARIABLES");
//    vars::run();

    // Data Types
//    println!("\n\nTYPES");
//    types::run();

    // Strings
//    println!("\n\nSTRINGS");
//    strings::run();
    
    // Tuples
//    println!("\n\nTUPLES");
//    tuples::run();
    
    // Arrays
//    println!("\n\nARRAYS");
//    arrays::run();

    // Vectors 
//    println!("\n\nVECTORS");
//    vectors::run();
    
    // Conditionals 
//    println!("\n\nCONDITIONALS");
//    conditionals::run();

    // Loops
//    println!("\n\nLOOPS");
//    loops::run();
    
    // Functions
//    println!("\n\nFUNCTIONS");
//    functions::run();
    
    // Pointers
//    println!("\n\nPOINTERS");
//    pointers_ref::run();
    
    // Structs
//    println!("\n\nSTRUCTS");
//    structs::run();
    
    // Enums
//    println!("\n\nENUMS");
//    enums::run();

    // CLI
//    println!("\n\nCLI");
//    cli::run();
