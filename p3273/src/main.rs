use std::io;

fn input_number_vector() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_vec: Vec<i32> = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    num_vec
}

fn input_number() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = input_number();
    let mut numbers = input_number_vector();
    let x = input_number();

    numbers.sort();

    let mut count = 0;
    let (mut left, mut right) = (0, n - 1);

    while left < right {
        let total = numbers[left as usize] + numbers[right as usize];
        if total == x {
            count += 1;
            left += 1;
            right -= 1;
            continue;
        }
        if total > x {
            right -= 1;
            continue;
        }

        if total < x {
            left += 1;
            continue;
        }
    }

    println!("{}", count);
}
