// Basically control structures

pub fn run() {
    let age: u8 = 22;
    let checked: bool = false;
    let knows_person: bool = true;

    // if else
    if age >= 21 && checked || knows_person {
        println!("What can I serve you sir?");
    } else if age < 21 && checked {
        println!("Sorry, this is not for kids!");
    } else {
        println!("I need to see your ID!");
    }

    let is_of_age = if age < 21 { false } else { true };
        println!("{}", is_of_age);
    }
