use std::mem;

fn apply<F>(f: F)
    where F: FnOnce() {
    f()
}

fn apply_to_3<F>(f: F) -> i32
    where F: Fn(i32) -> i32
{
    f(3)
}

fn test<A, B>(a: A, b: B) -> i32
    where A: Fn(i32, i32) -> i32,
          B: Fn(i32, i32) -> i32
{
    a(1, 2) + b(3, 4)
}

fn main() {
    let a = |x, y| -> i32 { x + y };
    let b = |x, y| x + y;
    let c = test(a, b);
    println!(" c is {}", c);


    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
