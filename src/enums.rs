// Enums are types that have a few definite values

enum Movement {
    // Variants
    Up, Down, Left, Right
}

fn move_avatar(m: Movement) {
    // Perform action
    match m {
        Movement::Up => println!("Moving Up!"),
        Movement::Down => println!("Moving Down!"),
        Movement::Left => println!("Moving Left!"),
        Movement::Right => println!("Moving Right!")
    }
}

pub fn run() {
    let avatar1 = Movement::Down;
    let avatar2 = Movement::Left;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Up;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
