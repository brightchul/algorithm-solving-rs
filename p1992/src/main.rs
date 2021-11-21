use p1992::solution;
use std::io;

fn main() {
    let mut buf_str = String::new();
    io::stdin().read_line(&mut buf_str).unwrap();
    let n: usize = buf_str.trim().parse().unwrap();

    let mut list: Vec<Vec<u32>> = Vec::<Vec<u32>>::with_capacity(n);
    for _ in 0..n {
        buf_str.clear();
        io::stdin().read_line(&mut buf_str).unwrap();
        let row: Vec<u32> = buf_str
            .trim()
            .chars()
            .map(|v| v.to_digit(10).unwrap())
            .collect();
        list.push(row);
    }

    println!("{}", solution(n, list));
}
