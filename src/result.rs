use std::fs::File;

fn main () {
    let result = File::open("foo");
    match result {
        Ok(file) => {
            println!("{:?}", file)
        },
        Err(error) => {
            println!("{}", error)
        }
    }
}
