pub fn fibonacci(n: i32) -> i32 {
    return if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) };
}
