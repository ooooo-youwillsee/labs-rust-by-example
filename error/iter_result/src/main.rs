fn main() {
    let strings = vec!["tofu", "93", "18"];
    let mut erros = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().map_err(|e| erros.push(e)).ok())
        .collect();
    println!("Results: {:?}", numbers);
}
