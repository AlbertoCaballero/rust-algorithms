pub fn run() {
    greeting("Hey", "Jane");

    println!("Functions: {}", add(5, 5));

    // Closures
    let n3: i32 = 5;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Function: {}", add_nums(10, 15));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 { n1 + n2 } 
