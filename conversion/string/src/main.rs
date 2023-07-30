use std::fmt;
use std::fmt::{Display, Formatter, Write};

struct Circle {
    radius: i32,
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}


fn main() {
    let c = Circle { radius: 2 };
    println!("{}", c);


    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
