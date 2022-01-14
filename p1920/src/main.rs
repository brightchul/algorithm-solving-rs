use std::io::{self, Write};

fn input_number() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if let Ok(result) = buf.trim().parse::<i32>() {
        return result;
    }
    panic!("invalid numberic string");
}

fn input_number_vector() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_vec: Vec<i32> = buf
        .trim()
        .to_string()
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    num_vec
}

fn main() {
    let mut sout = io::BufWriter::new(io::stdout());

    let n = input_number();
    let mut a_vec = input_number_vector();
    a_vec.sort();

    let _m = input_number();
    let target_vec = input_number_vector();

    for target in target_vec {
        writeln!(sout, "{}", binary_search_solution(target, &a_vec, 0, n - 1)).unwrap();
    }
}

fn binary_search_solution(target: i32, a_vec: &Vec<i32>, mut from: i32, mut to: i32) -> i32 {
    if from > to {
        return 0;
    }

    let mid = (from + to) / 2;
    if a_vec[mid as usize] == target {
        return 1;
    }

    if target < a_vec[mid as usize] {
        to = mid - 1;
    } else {
        from = mid + 1;
    }

    binary_search_solution(target, a_vec, from, to)
}

#[test]
fn binary_search_solution_test() {
    let vec = vec![1, 2, 3, 4, 5];
    let last_idx = (vec.len() - 1) as i32;

    assert_eq!(binary_search_solution(1, &vec, 0, last_idx), 1);
    assert_eq!(binary_search_solution(3, &vec, 0, last_idx), 1);
    assert_eq!(binary_search_solution(7, &vec, 0, last_idx), 0);
    assert_eq!(binary_search_solution(9, &vec, 0, last_idx), 0);
    assert_eq!(binary_search_solution(5, &vec, 0, last_idx), 1);
}
