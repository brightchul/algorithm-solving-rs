use std::{collections::HashMap, io};

fn input_number_vector() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim()
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

fn input_number() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<i32>().unwrap()
}

fn main() {
    let inputs = input_number_vector();
    let (dishes_count, sushi_number, eat_count, coupon) =
        (inputs[0], inputs[1], inputs[2], inputs[3]);

    let mut sushi_vec: Vec<i32> = vec![];
    for _ in 0..dishes_count {
        sushi_vec.push(input_number());
    }

    let mut result = 0;
    let mut count = 0;
    let mut visited = vec![0; (sushi_number + 1) as usize];

    for idx in 0..eat_count {
        let sushi = sushi_vec[idx as usize];
        if visited[sushi as usize] == 0 {
            count += 1;
        }
        visited[sushi as usize] += 1;
    }

    let (mut left, mut right) = (0, eat_count - 1);
    while left < dishes_count {
        let left_sushi = sushi_vec[left as usize] as usize;
        visited[left_sushi] -= 1;
        if visited[left_sushi] == 0 {
            count -= 1;
        }
        left += 1;

        right += 1;
        if right == dishes_count {
            right = 0;
        }

        let right_sushi = sushi_vec[right as usize] as usize;
        if visited[right_sushi] == 0 {
            count += 1;
        }
        visited[right_sushi] += 1;

        if visited[coupon as usize] == 0 {
            result = result.max(count + 1);
        } else {
            result = result.max(count);
        }
    }

    println!("{}", result);
}

fn _main1() {
    let inputs = input_number_vector();
    let (dishes_count, _sushi_number, eat_count, coupon) =
        (inputs[0], inputs[1], inputs[2], inputs[3]);

    let mut sushi_vec: Vec<i32> = vec![];
    for _ in 0..dishes_count {
        sushi_vec.push(input_number());
    }

    let mut result = 0;
    let mut eat_map = HashMap::<i32, i32>::new();

    for idx in 0..eat_count {
        let sushi = sushi_vec[idx as usize];
        if let Some(v) = eat_map.get_mut(&sushi) {
            *v += 1;
        } else if sushi != coupon {
            eat_map.insert(sushi, 1);
        }
    }
    result = result.max(eat_map.len());

    let (mut left, mut right) = (0, eat_count - 1);
    while left < dishes_count {
        if let Some(v) = eat_map.get_mut(&sushi_vec[left as usize]) {
            *v -= 1;
            if *v == 0 {
                eat_map.remove(&sushi_vec[left as usize]);
            }
        }
        left += 1;

        right += 1;
        if right == dishes_count {
            right = 0;
        }

        if let Some(v) = eat_map.get_mut(&sushi_vec[right as usize]) {
            *v += 1;
        } else if sushi_vec[right as usize] != coupon {
            eat_map.insert(sushi_vec[right as usize], 1);
        }

        result = result.max(eat_map.len());
    }

    println!("{}", result + 1);
}
