use std::{
    io::{self, Write},
    str::FromStr,
};

fn make_prime_vec() -> Vec<usize> {
    let mut num_arr = [true; 10001];
    let mut prime_vec: Vec<usize> = vec![];

    num_arr[0] = false;
    num_arr[1] = false;

    for i in 2..101 {
        let mut j = i * i;
        while j < 10001 {
            num_arr[j] = false;
            j += i;
        }
    }

    for (idx, &is_prime) in num_arr.iter().enumerate() {
        if is_prime {
            prime_vec.push(idx);
        }
    }
    prime_vec
}

fn input_number<T>() -> T
where
    T: FromStr,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if let Ok(result) = buf.trim().parse::<T>() {
        return result;
    }
    panic!("invalid numberic string");
}

fn solution(target: usize, prime_vec: &Vec<usize>) -> (usize, usize) {
    let idx = prime_vec.iter().position(|v| *v >= target / 2).unwrap();
    let mut left = idx;
    let mut right = idx;

    while prime_vec[left] + prime_vec[right] != target {
        if prime_vec[left] + prime_vec[right] < target {
            right += 1;
        } else {
            left -= 1;
        }
    }

    (prime_vec[left], prime_vec[right])
}

fn main() {
    let mut sout = io::BufWriter::new(io::stdout());

    let prime_vec: Vec<usize> = make_prime_vec();
    let len = input_number::<usize>();

    for _ in 0..len {
        let target = input_number::<usize>();
        let (left_value, right_value) = solution(target, &prime_vec);

        writeln!(sout, "{} {}", left_value, right_value).unwrap();
    }
}

#[test]
fn solution_example_test() {
    let prime_vec = make_prime_vec();
    assert_eq!(solution(8, &prime_vec), (3, 5));
    assert_eq!(solution(10, &prime_vec), (5, 5));
    assert_eq!(solution(16, &prime_vec), (5, 11));
}
