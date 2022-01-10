use std::{cmp, io};

// 5
// 7
// 3 8
// 8 1 0
// 2 7 4 4
// 4 5 2 6 5

fn input_number() -> usize {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if let Ok(result) = buf.trim().parse::<usize>() {
        return result;
    }
    panic!("invalid numberic string");
}

fn input_number_vector() -> Vec<usize> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_vec: Vec<usize> = buf
        .trim()
        .to_string()
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    num_vec
}

fn main() {
    let n = input_number();

    let mut vecs = Vec::<Vec<usize>>::new();

    for _ in 0..n {
        vecs.push(input_number_vector());
    }

    for row in 1..n {
        for (col, &value) in vecs[row].clone().iter().enumerate() {
            let mut upper_value = 0;
            if col > 0 {
                upper_value = cmp::max(upper_value, vecs[row - 1][col - 1]);
            }

            if col < row {
                upper_value = cmp::max(upper_value, vecs[row - 1][col]);
            }

            vecs[row][col] = value + upper_value;
        }
    }

    println!("{}", vecs[n - 1].iter().max().unwrap());
}
