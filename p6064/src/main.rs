use std::{cmp, io};

// 10 12 3 9
// 10 12 라고 한다면 최소공배수만큼 경우의 수가 나타난다.
// 그래서 가장 최악이라고 한다면 40000 40000 인 16억 정도일 것이다. 따라서 부르트 포스는 비효율적이다.
// 패턴을 보자면 1,1 | 1, 11 | 1, 9 | 1, 7 | 1, 5 | 1, 3 | 1, 1 (반복) 이다.
// 3,9의 경우 1, 7에서부터 시작한다고 볼수 있다. 1,7부터 3번째 경우이다.
// 1,1 => 1,7까지 가는데는 30번의 순회가 필요하고, 1,7 => 3,9 가는데는 3이 필요하므로  33이다.

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let len = buf.trim().parse::<usize>().unwrap();

    for _ in 0..len {
        buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let num_vec: Vec<i32> = buf
            .trim()
            .to_string()
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        let m = num_vec[0];
        let n = num_vec[1];
        let x = num_vec[2];
        let y = num_vec[3];

        if x == y {
            println!("{}", x);
            continue;
        }

        if m == n {
            if x == y {
                println!("{}", x);
            } else {
                println!("-1");
            }
            continue;
        }

        // true : m, false : n
        let flag = m == cmp::min(m, n);
        let amount = i32::abs(m - n);

        let mut start_x = 1;
        let mut start_y = 1;

        if flag {
            // m
            if x < y {
                start_y = 1 + y - x
            } else {
                start_y = n + 1 + y - x
            };
        } else {
            // n
            if x > y {
                start_x = 1 + x - y
            } else {
                start_x = m + 1 + x - y
            };
        }

        let mut start_m = 1;
        let mut start_n = 1;
        let mut count = 0;

        if start_m == start_x && start_n == start_y {
            if flag {
                // m
                count += m;
            } else {
                // n
                count += n;
            }
            println!("{}", count);
            continue;
        }

        loop {
            if flag {
                // m
                start_n = if start_n <= amount {
                    n + start_n - amount
                } else {
                    start_n - amount
                };
                count += m;
            } else {
                // n
                start_m = if start_m <= amount {
                    m + start_m - amount
                } else {
                    start_m - amount
                };
                count += n;
            }

            if start_m == start_x && start_n == start_y {
                if flag {
                    // m
                    count += x
                } else {
                    // n
                    count += y
                };
                println!("{}", count);
                break;
            }

            if start_m == 1 && start_n == 1 {
                println!("-1");
                break;
            }
        }
    }
}
