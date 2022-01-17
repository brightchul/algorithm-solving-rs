use std::io;
// 4 3
// 11
// 13
// 16
// 18

// 5 3
// 1
// 2
// 8
// 4
// 9

fn input_number_tuple2() -> (i32, i32) {
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
    buf.trim().parse::<i32>().unwrap()
}

fn main() {
    let (n, c) = input_number_tuple2();
    let mut houses = Vec::<i32>::new();
    for _ in 0..n {
        houses.push(input_number());
    }
    houses.sort();

    println!("{}", bs(c, &houses));
}

fn bs(c: i32, houses: &Vec<i32>) -> i32 {
    let mut left = 1;
    let mut right = houses[houses.len() - 1];

    let mut result = 0;
    while left <= right {
        let mut count = 1;
        let mut pre_house = houses[0];
        let mid = (left + right) / 2;

        for idx in 1..houses.len() {
            if houses[idx] - pre_house + 1 > mid {
                pre_house = houses[idx];
                count += 1;
            }
        }

        if count < c {
            right = mid - 1;
        } else {
            result = mid;
            left = mid + 1;
        }
    }
    result
}
