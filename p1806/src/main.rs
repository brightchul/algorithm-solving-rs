use std::io;

fn input_number_vector() -> Vec<i64> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_vec: Vec<i64> = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    num_vec
}

fn input_number_tuple() -> (i64, i64) {
    let num_vec = input_number_vector();
    (num_vec[0], num_vec[1])
}

fn main() {
    let (n, s) = input_number_tuple();
    let num_vec = input_number_vector();

    let mut left = 0;
    let mut right = 0;
    let mut total = num_vec[0];
    let mut result = i64::MAX;

    loop {
        if total >= s {
            result = result.min(right - left + 1);
        }

        if total <= s || left == right {
            if right + 1 == n {
                break;
            }
            right += 1;
            total += num_vec[right as usize]
        } else {
            total -= num_vec[left as usize];
            left += 1;
        }
    }

    if result == i64::MAX {
        result = 0;
    }

    println!("{}", result);
}
