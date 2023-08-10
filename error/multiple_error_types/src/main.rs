use std::{error, fmt};
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|x| {
        x.parse::<i32>().map(|x| x * 2)
    })
}

fn double_first2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| n * 2)
    });
    opt.map_or(Ok(None), |r| r.map(Some))
}

type CResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first3(vec: Vec<&str>) -> CResult<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|err| err.into())
                .map(|i| i * 2)
        })
}

fn print(result: CResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn double_first4(vec: Vec<&str>) -> CResult<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));

    println!("The first doubled is {:?}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", double_first(strings));


    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first3(numbers));
    print(double_first3(empty));
    print(double_first3(strings));
}
