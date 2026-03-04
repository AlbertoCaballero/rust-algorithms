fn main() {
    let mut s1 = String::from("Hey");

    do_something_borrowed(&s1);
    println!("{}", s1);

    s1 = do_something_moving(s1);
    println!("{}", s1);

    do_something_referenced(&mut s1);
    println!("{}", s1);
}

// param is mutable variable from s1, now s is owner untile returned
fn do_something_moving(mut s: String) -> String {
    println!("Moved {}", s);
    s = String::from("moved");
    s
}

// s is only borrowing s1
fn do_something_borrowed(s: &String) {
    println!("Borrowed {}", s)
}

// s is borrowing a mutable reference of s1
fn do_something_referenced(s: &mut String) {
    *s = String::from("referenced");
    println!("Referenced {}", s)
}
