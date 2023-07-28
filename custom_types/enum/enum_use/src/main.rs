#![allow(dead_code)]


use crate::Status::{Poor, Rich};
use crate::Work::{Civilian, Soldier};

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    let status = Poor;

    let work = Civilian;

    match status {
        Rich => println!("rich"),
        Poor => println!("poor"),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
