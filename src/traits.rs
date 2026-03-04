struct Puppy {
    name: String,
    age: i8,
}

impl Grumpy for Puppy {
    fn angry_bark(&self) {
        println!("{} grrwuf", self.name);
        println!("{}", self.age);
    }
}

//////////////////////////////////
struct Dog {
    name: String,
    age: i8,
}

impl Dog {
    fn bark(&self) {
        println!("{} Wuf!", self.name);
        println!("{}", self.age);
    }
}

impl Grumpy for Dog {
    fn angry_bark(&self) {
        println!("{} Grrwuf!", self.name);
        println!("{}", self.age);
    }
}

//////////////////////////////////
struct Oldog {
    name: String,
    age: i8,
}

impl Grumpy for Oldog {
    fn angry_bark(&self) {
        println!("{} GRRWUF", self.name);
        println!("{}", self.age);
    }
}

//////////////////////////////////
trait Grumpy {
    fn angry_bark(&self);
}

fn main() {
    let waldo = Puppy {
        name: String::from("Waldo"),
        age: 2,
    };
    let rufus = Dog {
        name: String::from("Rufus"),
        age: 8,
    };
    let buster = Oldog {
        name: String::from("Buster"),
        age: 12,
    };

    // waldo.bark();
    waldo.angry_bark();

    rufus.bark();
    rufus.angry_bark();

    // buster.bark();
    buster.angry_bark();

    angry(waldo);
    angry(rufus);
    angry(buster);
}

fn angry<T: Grumpy>(dog: T) {
    println!("ANGRY!");
    dog.angry_bark();
}
