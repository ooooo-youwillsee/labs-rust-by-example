fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn test_add() {
        let i = add(1, 2);
        assert_eq!(3, i);
    }
}