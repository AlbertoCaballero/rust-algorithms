// The macro acts like a code template, letting you generate boilerplate functions programmatically.

struct Bar;
impl Bar {
    fn foo(&self) {
        println!("FOO!");
    }
}

macro_rules! bar {
    ($i:ident, $s:expr, $m:ident) => {
        fn $i() { $s.$m(); }
    }
}

fn main() {
    // This expands to
    // fn bar_fn() { Bar{}.foo(); }
    bar!(bar_fn, Bar{}, foo);
    bar_fn();
}

