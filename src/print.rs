pub fn run() {
    // Console printing
    println!("Hello from print.rs");

    // Print formating
    println!("{} is from {}", "Alberto", "Mexico");

    // Positional arguments
    println!("{0} is a {1} from {2}, that's {0}", "Alberto", "Developer", "Mexico");

    // Named parameters
    println!("{name} is from {country}", name="Alberto", country="Mexico");

    // Palcehoder traits
    println!("{} in Binary: {:b} Hex: {:x} Octal: {:o}", 14,14,14,14);

    // Placeholder for debug trait (and tuple)
    println!("{:?}", (12, true, "Hello"));
}
