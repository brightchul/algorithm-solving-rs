use std::{collections::HashSet, io};

fn input_number() -> i64 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<i64>().unwrap()
}

fn input_number_vector() -> (i64, i64) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut split = buf.trim().split_whitespace();

    (
        split.next().unwrap().parse::<i64>().unwrap(),
        split.next().unwrap().parse::<i64>().unwrap(),
    )
}

fn main() {
    let n = input_number();
    let mut tree = Vec::<HashSet<i64>>::new();
    for _ in 0..(n + 1) {
        tree.push(HashSet::<i64>::new())
    }

    let (one, two) = input_number_vector();
    let start = one;
    tree[one as usize].insert(two);
    tree[two as usize].insert(one);

    for _ in 2..n {
        let (one, two) = input_number_vector();
        tree[one as usize].insert(two);
        tree[two as usize].insert(one);
    }

    let mut visited: [bool; 300001] = [false; 300001];
    let mut result = [0, 0];

    recur(0, start, &tree, &mut visited, &mut result);
    let [d, g] = result;

    if d < g * 3 {
        println!("G");
    } else if d == g * 3 {
        println!("DUDUDUNGA");
    } else {
        println!("D");
    }
}

fn recur(
    parent: i64,
    target: i64,
    tree: &Vec<HashSet<i64>>,
    visited: &mut [bool],
    result: &mut [i64],
) {
    visited[target as usize] = true;

    if parent > 0 {
        result[0] +=
            (tree[parent as usize].len() as i64 - 1) * (tree[target as usize].len() as i64 - 1);
    }

    if tree[target as usize].len() >= 3 {
        let len = tree[target as usize].len() as i64;
        result[1 as usize] += len * (len - 1) * (len - 2) / 6;
    }

    for &one in &tree[target as usize] {
        if visited[one as usize] {
            continue;
        }
        recur(target, one, &tree, visited, result);
    }
}
