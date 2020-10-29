// Pointers pint to a reference in memory

pub fn run() {

    // With non-primitives, if you asign another veriable to a piece of data, the first
    // variable will no longer hold that value. A reference is needed to point to the resource. 
    
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}

