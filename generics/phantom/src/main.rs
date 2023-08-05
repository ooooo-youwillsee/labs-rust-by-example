use std::marker::PhantomData;
use std::ops::Add;

#[derive(Clone, Copy)]
enum Inch {}

#[derive(Clone, Copy)]
enum Mm {}

#[derive(Clone, Copy)]
struct Length<U> (f64, PhantomData<U>);

impl<U> Add for Length<U> {
    type Output = Length<U>;

    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}


fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);

    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works.
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

}
