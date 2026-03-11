// Assertions are hard stops for unexpected states

fn main() {
    let x = 5;
    let y = 2;
    assert!(x < y, "x should be less than y, but {} is not less than {}", x, y);
    assert_eq!(x + 1, y, "simple equality check");
}

