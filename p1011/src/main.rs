use std::io::{self, Write};

use p1011::solution;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let count: i32 = s.trim().parse().unwrap();

    let mut sout = io::BufWriter::new(io::stdout());

    for _ in 0..count {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let arr: Vec<i64> = s.split_whitespace().map(|v| v.parse().unwrap()).collect();
        writeln!(sout, "{}", solution(arr[1] - arr[0])).unwrap();
    }
}
