use std::{collections::VecDeque, io};

fn main() {
    let mut buf_str = String::new();
    io::stdin().read_line(&mut buf_str).unwrap();

    let len: usize = buf_str.trim().parse().unwrap();

    buf_str.clear();
    io::stdin().read_line(&mut buf_str).unwrap();
    let mut list: Vec<i32> = buf_str
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    buf_str.clear();
    io::stdin().read_line(&mut buf_str).unwrap();

    let remove_node: usize = buf_str.trim().parse().unwrap();

    if remove_node == 0 {
        println!("{}", 0);
        return;
    }

    list[remove_node] = -100;

    let mut result = 0;

    let mut queue = VecDeque::new();
    queue.push_back(0);

    while !queue.is_empty() {
        let node_num = queue.pop_front().unwrap() as i32;
        if list[node_num as usize] == -100 {
            continue;
        }

        let left = (node_num * 2 + 1) as usize;
        let right = (node_num * 2 + 2) as usize;

        let mut is_leaf = true;
        if left < len {
            if node_num == list[left] {
                is_leaf = false;
                queue.push_back(left as i32);
            }
        }
        if right < len {
            if node_num == list[right] {
                is_leaf = false;
                queue.push_back(right as i32);
            }
        }
        if is_leaf {
            result += 1;
        }
    }
    println!("{}", result);
}
