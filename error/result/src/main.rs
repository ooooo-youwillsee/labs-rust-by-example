use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    match first.parse::<i32>() {
        Ok(first) => {
            match second.parse::<i32>() {
                Ok(second) => {
                    Ok(first * second)
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e),
    }
}

fn multiply2(first: &str, second: &str) -> Result<i32, ParseIntError> {
    first.parse::<i32>().and_then(|first| {
        second.parse::<i32>().map(|second| first * second)
    })
}

// Define a generic alias for a `Result` with the error type `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply3(first: &str, second: &str) -> AliasedResult<i32> {
    let first = match first.parse::<i32>() {
        Ok(first) => { first }
        Err(e) => { return Err(e); }
    };
    let second = match second.parse::<i32>() {
        Ok(second) => second,
        Err(e) => { return Err(e); }
    };
    Ok(first * second)
}

fn multiply4(first: &str, second: &str) -> AliasedResult<i32> {
    let first = first.parse::<i32>()?;
    let second = second.parse::<i32>()?;
    Ok(first * second)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let result = multiply("1", "2");
    print(result);

    let result2 = multiply2("1", "2");
    print(result2);

    let result3 = multiply3("1", "2");
    print(result3);

    let result4 = multiply4("1", "2");
    print(result4);
}
