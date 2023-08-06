use std::ops::Add;

struct Foo;

struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;


impl Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> Self::Output {
        FooBar
    }
}

impl Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> Self::Output {
        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
