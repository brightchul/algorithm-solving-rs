use std::io;
// 2 1 1000
// 1 1000000000
// 2 50
// 1

fn input_number_vector() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num_vec: Vec<i32> = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    num_vec
}

fn input_tuple3() -> (i32, i32, i32) {
    let vec = input_number_vector();
    (vec[0], vec[1], vec[2])
}

fn main() {
    let (_m, n, l) = input_tuple3();
    let mut hunters = input_number_vector();
    hunters.sort();

    let mut count = 0;
    for _ in 0..n {
        let target = input_number_vector();
        if bs(&target, &hunters, l) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn bs(target: &Vec<i32>, hunters: &Vec<i32>, l: i32) -> bool {
    if target[1] > l {
        return false;
    }

    let (mut left, mut right) = (0, (hunters.len() - 1) as i32);
    while left <= right {
        let mid = (left + right) / 2;
        let hunter = hunters[mid as usize];

        if (hunter - target[0]).abs() + target[1] <= l {
            return true;
        }

        if target[0] < hunter {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return false;
}
