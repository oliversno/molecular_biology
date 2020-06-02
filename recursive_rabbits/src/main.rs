use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u64>().unwrap();
    let k = args[2].parse::<u64>().unwrap();
    println!("After {} months there are {} pairs", n, iterative_rabbits(n, k));
}

fn iterative_rabbits(n: u64, k: u64) -> u64{
    let mut fn_1 = 1;
    let mut fn_2 = 1;
    let mut f_n = 1; // placeolder incase n <= 2, because we start at 1 pair
    for _ in 2..n {
        f_n = fn_1 + fn_2 * k;
        fn_2 = fn_1;
        fn_1 = f_n;
    }
    f_n
}