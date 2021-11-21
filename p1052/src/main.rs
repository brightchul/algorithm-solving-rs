mod lib;
use lib::{solution1, solution2};
use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let n = values[0];
    let k = values[1];
    println!("{}", solution1(n, k));
    println!("{}", solution2(n, k));
}
