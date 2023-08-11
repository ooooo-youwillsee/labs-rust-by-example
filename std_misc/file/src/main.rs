use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut f = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            panic!("create file {:?}", e);
        }
    };

    f.write_all("xxx".as_bytes()).unwrap();

    let mut f = File::open(path).unwrap();

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    println!();

    let f = File::open(path).unwrap();
    let buf_reader = BufReader::new(f);
    for line in buf_reader.lines() {
        println!("line: {}", line.unwrap());
    }
}
