#![allow(dead_code)]

use crate::Number::{One, Two};

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    println!("zero is {}", One as i32);
    println!("one is {}", Two as i32);

    println!("rose are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32)
}
