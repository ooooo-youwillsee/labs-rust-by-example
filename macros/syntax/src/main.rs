macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

macro_rules! test {
    ($left: expr, and $right: expr) => {
        println!("{:?}, and {:?} is {:?}", stringify!($left), stringify!($right), $left && $right)
    };

    ($left: expr, or $right: expr) => {
         println!("{:?}, or {:?} is {:?}", stringify!($left), stringify!($right), $left || $right)
    };
}

macro_rules! find_min {
    ($val: expr) => {$val};
    ($val: expr, $($val2: expr),+) => {
        std::cmp::min($val, find_min!($($val2),+))
    }

}

fn main() {
    foo();
    bar();

    print_result!(1 + 2);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    test!(1+2 == 3, and 1+1 == 2);

    test!(1 + 2 == 3, or 1 == 2);

    let val = find_min!(1, 2, 3);
    println!("find_min: {}", val)
}
