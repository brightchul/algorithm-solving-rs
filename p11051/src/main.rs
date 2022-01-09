use std::io;

// 각 nCk에서 각각 의 연산에 대해서 바로 연산하지 않고 인수분해해서 그것들을 제외한 이후에
// 모듈러 연산으로 나머지를 구함

fn input_number_tuple() -> (i32, i32) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_vec: Vec<i32> = buf
        .trim()
        .to_string()
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    (num_vec[0], num_vec[1])
}

fn make_prime_vec() -> Vec<i32> {
    let mut num_arr = [true; 1001];
    let mut prime_vec: Vec<i32> = vec![];

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
            prime_vec.push(idx as i32);
        }
    }
    prime_vec
}

fn calculate_factor(from: i32, to: i32, prime_vec: &Vec<i32>, expo_arr: &mut [i32], amount: i32) {
    for mut one in from..to {
        let mut iter = prime_vec.iter();

        while one > 1 {
            let &prime = iter.next().unwrap();
            while one % prime == 0 {
                one = one / prime;
                expo_arr[prime as usize] = expo_arr[prime as usize] + amount;
            }
        }
    }
}

fn calculate_result(expo_arr: &[i32]) -> i32 {
    let mut result = 1;
    for (idx, &value) in expo_arr.iter().enumerate() {
        if idx == 1 && value == 0 {
            continue;
        }
        result = (result * (idx as i32).pow(value as u32)) % 10_007;
    }

    result % 10_007
}

fn solution(n: i32, k: i32) -> i32 {
    let prime_vec = make_prime_vec();
    let mut expo_arr: [i32; 1000] = [0; 1000];

    // 인수분해로 소수 배열에 지수 값들을 감산
    calculate_factor(n - k + 1, n + 1, &prime_vec, &mut expo_arr, 1);
    calculate_factor(1, k + 1, &prime_vec, &mut expo_arr, -1);

    calculate_result(&expo_arr)
}

fn main() {
    let (n, k) = input_number_tuple();
    println!("{}", solution(n, k));
}

#[test]
fn solution_example_test() {
    assert_eq!(solution(1000, 500), 5418);
}
