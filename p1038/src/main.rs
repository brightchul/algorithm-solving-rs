use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).unwrap();
  let n: i32 = s.trim().parse().unwrap();

  if n > 1022 {
    println!("-1");
    return;
  }


  let mut list = Vec::<Vec<i32>>::new();

  // 10 ~ 9x
  let ten0: Vec<i32> = (0..10).map(|v| if v == 0 {0} else {1}).collect();
  list.push(ten0);

  let ten1: Vec<i32> = (0..10).map(|v| v).collect();
  list.push(ten1);
  for one in 2..13 {

    let one = one as usize;
    let mut temp: Vec<i32> = vec![0];
    
    for idx in 1..10 {
      let idx = idx as usize;
      let value = temp[idx-1] + list[one-1][idx-1]; 
      temp.push(value);
    }
    list.push(temp);
  }
  
}