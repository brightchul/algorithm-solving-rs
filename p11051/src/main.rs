use std::{cmp, io};

fn input_number_tuple() -> (i64, i64) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_vec: Vec<i64> = buf
        .trim()
        .to_string()
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    (num_vec[0], num_vec[1])
}

fn make_prime_vec() -> Vec<i64> {
    let mut num_arr = [true; 1001];
    let mut prime_vec: Vec<i64> = vec![];

    num_arr[0] = false;
    num_arr[1] = false;

    for i in 2..33 {
        let mut j = i * i;
        while j < 1001 {
            num_arr[j] = false;
            j += i;
        }
    }

    for (idx, &is_prime) in num_arr.iter().enumerate() {
        if is_prime {
            prime_vec.push(idx as i64);
        }
    }
    prime_vec
}
fn main() {
    let (n, k) = input_number_tuple();
    let prime_vec = make_prime_vec();
    // nCk

    let mut n_arr: [i64; 1000] = [0; 1000];
    let mut k_arr: [i64; 1000] = [0; 1000];

    // 각 값들에 대한 소인수 분해를 해야 함
    // n-k부터 n까지
    for mut one in (n - k + 1)..(n + 1) {
        let mut iter = prime_vec.iter();

        while one > 1 {
            let &prime = iter.next().unwrap();
            while one % prime == 0 {
                one = one / prime;
                n_arr[prime as usize] = n_arr[prime as usize] + 1;
            }
        }
    }

    // 1~k 까지
    // for mut one in 1..(k + 1) {}
    for mut one in 1..(k + 1) {
        let mut iter = prime_vec.iter();

        while one > 1 {
            let &prime = iter.next().unwrap();
            while one % prime == 0 {
                one = one / prime;
                k_arr[prime as usize] = k_arr[prime as usize] + 1;
            }
        }
    }

    for one in 2..1000 {
        let amount = cmp::min(n_arr[one], k_arr[one]);
        n_arr[one] -= amount;
        k_arr[one] -= amount;
    }

    let mut result_n = 1;
    for (idx, &value) in n_arr.iter().enumerate() {
        if idx == 1 && value == 0 {
            continue;
        }
        result_n = (result_n * (idx as i64).pow(value as u32)) % 10007;
    }

    let mut result_k = 1;
    for (idx, &value) in k_arr.iter().enumerate() {
        if idx == 1 && value == 0 {
            continue;
        }
        result_k *= (idx as i64).pow(value as u32);
    }

    println!("{}", result_n / result_k % 10_007);
}
