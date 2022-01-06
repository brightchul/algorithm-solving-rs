use std::{
    cmp::{max, min},
    io,
};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let two_num: Vec<i32> = buf
        .trim()
        .to_string()
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    let (mut big, mut small) = (max(two_num[0], two_num[1]), min(two_num[0], two_num[1]));

    while big % small > 0 {
        let temp = big % small;
        big = small;
        small = temp;
    }

    let gcf = small;
    let lcm = two_num[0] * two_num[1] / gcf;
    println!("{}\n{}", gcf, lcm);
}
