use std::{
    collections::{HashSet, VecDeque},
    io,
};

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

struct ParentChild {
    parent: Option<i64>,
    child: i64,
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

    let mut queue = VecDeque::<ParentChild>::new();
    queue.push_back(ParentChild {
        parent: None,
        child: start,
    });

    while queue.len() > 0 {
        let one = queue.pop_front().unwrap();
        let child = one.child;

        visited[child as usize] = true;

        if let Some(parent) = one.parent {
            result[0] +=
                (tree[parent as usize].len() as i64 - 1) * (tree[child as usize].len() as i64 - 1);
        }

        let len = tree[child as usize].len() as i64;
        if len >= 3 {
            result[1 as usize] += len * (len - 1) * (len - 2) / 6;
        }

        for &child_num in &tree[child as usize] {
            if visited[child_num as usize] {
                continue;
            }
            queue.push_back(ParentChild {
                parent: Some(child),
                child: child_num,
            })
        }
    }

    let [d, g] = result;

    if d < g * 3 {
        println!("G");
    } else if d == g * 3 {
        println!("DUDUDUNGA");
    } else {
        println!("D");
    }
}
