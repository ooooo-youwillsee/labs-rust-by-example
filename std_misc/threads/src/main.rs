use std::thread;

fn main() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];
    for (i, s) in data.split_whitespace().enumerate() {
        println!("{} segment {}", i, s);

        children.push(thread::spawn(move || -> u32 {
            let result = s.chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();

            println!("processed segment {}, result={}", i, result);

            result
        }));
    }

    let sum = children.into_iter().map(|x| x.join().unwrap()).sum::<u32>();
    println!("sum {}", sum);
}
