use std::{collections::VecDeque, io};

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).unwrap();
  
  let  n: i32 = s.trim().parse().unwrap();

  if n < 10 {
    println!("{}", n);
    return;
  }

  let mut queue: VecDeque<i64> = VecDeque::new();
  for idx in 1..10 {
    queue.push_back(idx);
  }

  let mut count = 9;
  
  while queue.len() > 0 {
    let queue_front_value = queue.pop_front().unwrap();
    let digit_limit = queue_front_value % 10;

    for one in 0..digit_limit {
      let generated_value = queue_front_value * 10 + one;
      queue.push_back(generated_value);
      count +=1;

      if n == count {
        println!("{}", generated_value);
        return;
      }
    }
  }

  println!("-1");
  return;
}
