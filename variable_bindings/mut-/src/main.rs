fn main() {
    let _immutable_binding = 1;
    let mut multable_binding = 1;

    println!("Before mutation: {}", multable_binding);

    multable_binding += 1;

    println!("After mutation: {}", multable_binding);


    // ERROR
    // _immutable_binding += 1;
}
