use std::cmp;
use std::io::{self, Write};
// 10 12 3 9
// 10 12 라고 한다면 최소공배수만큼 경우의 수가 나타난다.
// 그래서 가장 최악이라고 한다면 40000 40000 인 16억 정도일 것이다. 따라서 부르트 포스는 비효율적이다.
// 패턴을 보자면 1,1 | 1, 11 | 1, 9 | 1, 7 | 1, 5 | 1, 3 | 1, 1 (반복) 이다.
// 3,9의 경우 1, 7에서부터 시작한다고 볼수 있다. 1,7부터 3번째 경우이다.
// 1,1 => 1,7까지 가는데는 30번의 순회가 필요하고, 1,7 => 3,9 가는데는 3이 필요하므로  33이다.

fn input_number() -> usize {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if let Ok(result) = buf.trim().parse::<usize>() {
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

    let len = input_number();

    for _ in 0..len {
        let vec: Vec<i32> = input_number_vector();
        writeln!(sout, "{}", solution(vec[0], vec[1], vec[2], vec[3])).unwrap();
    }
}

fn solution(m: i32, n: i32, x: i32, y: i32) -> i32 {
    if x == y {
        return x;
    }

    if m == n && x != y {
        return -1;
    }

    let flag = m == cmp::min(m, n);
    let amount = i32::abs(m - n);

    let mut start_x = 1;
    let mut start_y = 1;

    if flag {
        start_y = if x < y { 1 + y - x } else { n + 1 + y - x };
    } else {
        start_x = if x > y { 1 + x - y } else { m + 1 + x - y };
    }

    let mut start_m = 1;
    let mut start_n = 1;
    let mut count = 0;

    loop {
        if flag {
            start_n -= amount;
            if start_n <= 0 {
                start_n += n;
            }

            count += m;
        } else {
            start_m -= amount;
            if start_m <= 0 {
                start_m += m;
            }

            count += n;
        }

        if start_m == start_x && start_n == start_y {
            if flag {
                count += x
            } else {
                count += y
            };
            return count;
        }

        if start_m == 1 && start_n == 1 {
            return -1;
        }
    }
}

#[test]
fn solution_example_test() {
    assert_eq!(solution(10, 12, 3, 9), 33);
    assert_eq!(solution(10, 12, 5, 5), 5);
    assert_eq!(solution(10, 12, 7, 2), -1);
    assert_eq!(solution(13, 11, 5, 6), 83);
    assert_eq!(solution(11, 11, 5, 5), 5);
    assert_eq!(solution(11, 11, 5, 6), -1);
    assert_eq!(solution(13, 11, 9, 2), 35);
    assert_eq!(solution(13, 11, 1, 1), 1);
    assert_eq!(solution(40000, 39999, 39999, 39998), 1599959999);
    assert_eq!(solution(40000, 39999, 40000, 39999), 1599960000);
    assert_eq!(solution(40000, 40000, 40000, 39999), -1);
    assert_eq!(solution(40000, 39998, 40000, 39997), -1);
    assert_eq!(solution(39999, 2, 39998, 2), 39998);
    assert_eq!(solution(3, 40000, 3, 39999), 39999);
    assert_eq!(solution(40000, 3, 40000, 3), 120000);
    assert_eq!(solution(8, 2, 4, 2), 4);
    assert_eq!(solution(10, 12, 2, 12), 12);
    assert_eq!(solution(12, 10, 12, 10), 60);
    assert_eq!(solution(12, 10, 1, 1), 1);
    assert_eq!(solution(12, 6, 12, 6), 12);
    assert_eq!(solution(10, 1, 5, 1), 5);
    assert_eq!(solution(1, 10, 1, 5), 5);
    assert_eq!(solution(1, 1, 1, 1), 1);
}
