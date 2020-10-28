// Arrays are fixed length and same type

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);
    println!("At [2] = {}", numbers[2]);

    // Reasign values
    numbers[2] = 22;
    println!("{:?}", numbers);

    // Get length
    println!("Array length {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slices
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);
}
