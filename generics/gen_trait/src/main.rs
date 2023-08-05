struct Empty;

#[derive(Clone, Copy)]
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T) {
        println!("xxx");
    }
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}
