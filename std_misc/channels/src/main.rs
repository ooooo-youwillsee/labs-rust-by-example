use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let n = 3;

    let (tx, rx) = channel();

    for i in 0..n {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap()
        });
    }

    for _i in 0..n {
        let v = rx.recv().unwrap();
        println!("{}", v);
    }
}
