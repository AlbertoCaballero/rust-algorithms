// Vectors are resizeable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // You can do everything from arrays and
    
    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);

    numbers.pop();
    println!("{:?}", numbers);

    // Loop through vector values
    for x in numbers.iter() {
        println!("N: {}", x);
    }

    // Loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    } println!("{:?}", numbers);
}
