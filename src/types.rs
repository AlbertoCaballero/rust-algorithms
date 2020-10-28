/*
 * Primitive types in Rust
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (Number of bits in memory)
 * Floats: f32, f64
 * Boolean: bool
 * Characters: char
 * Tuples
 * Arrays
 * */

// Rust is statically typed, so you must know all the types in compilation time.
// However the compiler can infer the types from usage and value.

pub fn run() {
    // i32
    let x = 1;

    // f64
    let y = 2.5;

    // Explicit
    let z: i64 = 454545454;


    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);
    println!("Max u128: {}", std::u128::MAX);

    // Boolean
    let is_active: bool = false;

    // Boolean from expression
    let is_greater = 10 > 5;

    // Character
    let a1 = 'ƒ';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}
