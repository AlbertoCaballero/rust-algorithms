fn main() {
    let mut s1 = String::from("Hey");

    do_something_borrowed(&s1);
    println!("{}", s1);

    s1 = do_something_moving(s1);
    println!("{}", s1);
}

fn do_something_moving(mut s: String) -> String {
    println!("Moved {}", s);
    s = String::from("moved");
    s
}

fn do_something_borrowed(s: &String) {
    println!("Borrowed {}", s)
}
