fn f1() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }

    // ERROR
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}

fn f2() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding)
    }

    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}


fn main() {
    f1();
    f2();
}
