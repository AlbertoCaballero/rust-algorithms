use std::env;

fn fibonacci(n: u64) -> u64 {
    if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let iters: u64 = if args.len() > 1 { args[1].parse::<u64>().expect("NaN") } else { 15 };
    for x in 0..=iters {
        println!("{}: {}", x, fibonacci(x));
    }
}
