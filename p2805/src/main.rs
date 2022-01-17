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
    let num_vec: Vec<i64> = input_number_vector();
    (num_vec[0], num_vec[1])
}

fn main() {
    let (_n, m) = input_number_tuple();
    let trees = input_number_vector();

    let mut left = 0;
    let mut right = *(trees.iter().max().unwrap());

    let mut result = -1;
    while left <= right {
        let mid = (left + right) / 2;
        let cutting_amount: i64 = trees.iter().map(|&tree| (tree - mid).max(0)).sum();

        if cutting_amount < m {
            right = mid - 1;
        } else {
            left = mid + 1;
            result = mid;
        }
    }

    println!("{}", result);
}
