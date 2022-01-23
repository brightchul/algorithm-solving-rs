use std::io;

fn input_number() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if let Ok(result) = buf.trim().parse::<i32>() {
        return result;
    }
    panic!("invalid numberic string");
}

fn main() {
    let n = input_number();
    let mut vec: Vec<i32> = vec![];
    for _ in 0..n {
        vec.push(input_number());
    }

    let (mut left, mut right) = (0, 0);
    let mut dis = vec[0];
    let total: i32 = vec.iter().sum();

    let mut result = 0;

    while right < n - 1 {
        result = result.max(dis.min(total - dis));

        if dis < total - dis {
            if right + 1 == n - 1 {
                break;
            }
            right += 1;
            dis += vec[right as usize];
            continue;
        }
        if dis > total - dis {
            dis -= vec[left as usize];
            left += 1;
            continue;
        }
        if dis == total - dis {
            break;
        }
    }

    println!("{}", result);
}
