use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let option = args[1].clone();

    if option == "hello" {
        println!("You asked for a greeting!");
    } else if option == "status" {
        println!("Status is {}", 100);
    } else {
        println!("That is not a valid option");
    }
}
