enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place {x: i32, y: i32},
}

impl DispenserItem {
    fn display(&self) -> String {
        match self {
            Empty => {
                "No Item".to_string()
            },
            Ammo(bullets) => {
                format!("Bullets: {}", bullets)
            },
            Things(thing, count) => {
                format!("{}: {}", thing, count)
            },
            Place{ x, y } => {
                format!("x:{} y:{}", x, y)
            },
        }
    }
}

use DispenserItem::*;

fn main() {
    let item1 = Empty;
    let item2 = Ammo(97);
    let item3 = Things("ball".to_string(), 7);
    let item4 = Place { x: 12, y: 50 };

    println!("{}", item1.display());
    println!("{}", item2.display());
    println!("{}", item3.display());
    println!("{}", item4.display());
}
