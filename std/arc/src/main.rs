use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let apple = Arc::new("the same apple");

    for i in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{}", apple)
        }).join().unwrap()
    }

    thread::sleep(Duration::from_secs(10));
    println!("finished");
}
