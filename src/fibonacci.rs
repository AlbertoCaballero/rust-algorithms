use std::thread;
use colored::Colorize;

extern crate colored;

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn colorize(color: &Color, text: String) -> String {
    match color {
        Color::Red => format!("{}", text).red().to_string(),
        Color::Green => format!("{}", text).green().to_string(),
        Color::Blue => format!("{}", text).blue().to_string(),
        Color::Yellow => format!("{}", text).yellow().to_string(),
    }
}

// unefficient fibonacci calculation
pub fn fibonacci(n: u64) -> u64 {
    if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) }
}

fn iterate_to(t: u64, color_thread: Color) -> () {
    for x in 0..=t {
        println!("{}: {:?}", colorize(&color_thread, x.to_string()), fibonacci(x));
    }
}

pub fn run() {
    // fibonacci_to(100, Color::Red);
    const ITERATIONS: u64 = 45;

    let red_thread = thread::spawn(|| {
        iterate_to(ITERATIONS, Color::Red)
    });
    let green_thread = thread::spawn(|| {
        iterate_to(ITERATIONS, Color::Green)
    });
    let blue_thread = thread::spawn(|| {
        iterate_to(ITERATIONS, Color::Blue)
    });
    let yellow_thread = thread::spawn(|| {
        iterate_to(ITERATIONS, Color::Yellow)
    });

    red_thread.join().unwrap();
    green_thread.join().unwrap();
    blue_thread.join().unwrap();
    yellow_thread.join().unwrap();
}
