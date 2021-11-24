use std::io;

use p1038::solution;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).unwrap();

  let  n: i32 = s.trim().parse().unwrap();
  println!("{}", solution(n));
}
