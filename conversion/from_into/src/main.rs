#![allow(dead_code)]

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}


fn main() {
    let num = Number::from(30);
    println!("My Number is {:?}", num);

    let value: Number = num.into();
    println!("value is {:?}", value)
}
