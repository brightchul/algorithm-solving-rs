use std::io;

fn input_number_tuple() -> (i32, i32) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_vec: Vec<i32> = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    (num_vec[0], num_vec[1])
}

fn input_number() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if let Ok(result) = buf.trim().parse::<i32>() {
        return result;
    }
    panic!("invalid numberic string");
}

fn main() {
    let (n, m) = input_number_tuple();
    let mut integers: Vec<i32> = vec![];
    for _ in 0..n {
        integers.push(input_number());
    }
    integers.sort();

    let mut left = 0;
    let mut right = 1;

    let mut result = i32::MAX;

    while left < n && right < n {
        let value = integers[right as usize] - integers[left as usize];

        if value >= m {
            result = result.min(value);
        }

        if value > m {
            left += 1;
        } else {
            right += 1;
        }

        if left == right {
            right += 1;
        }
    }

    println!("{:?}", result);
}
