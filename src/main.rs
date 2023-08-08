mod utils;

use utils::{
    get_i64,
};


fn fibonacci(n: i64) -> i64 {
    if n <= 1 {1i64} else {fibonacci(n - 1i64) + fibonacci(n - 2i64)}
}

fn main() {
    let fibs: Vec<i64> = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    for (i, f) in fibs.iter().enumerate() {
        assert!(*f == fibonacci(i as i64));
    }

    let n = get_i64("Enter fibonacci number to calculate: ");
    let val = fibonacci(n);
    println!("Fibonacci number {n} = {val}");
}
