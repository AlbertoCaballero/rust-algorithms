pub fn is_prime(n: i32) -> bool {
    let lim = (n as f32).sqrt() as i32;
    for i in 2..=lim {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
