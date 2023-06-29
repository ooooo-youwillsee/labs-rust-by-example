
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct MinMax(i64, i64);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(1, 2);
    println!("{}", minmax);

    let point = Point2D { x: 1.0, y: 2.0 };
    println!("{}", point)
}
