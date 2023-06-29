use std::mem;

fn analyze_slice(s: &[i32]) {
    println!("First element of the slice: {}", s[0]);
    println!("The slice has {} elements", s.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    println!("Number of elements in array: {}", xs.len());

    let ys = [0; 500];

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            None => {}
            Some(v) => { println!("{}", v) }
        }
    }
}
