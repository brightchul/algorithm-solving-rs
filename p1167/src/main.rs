use std::io;

// 5
// 1 3 2 -1
// 2 4 4 -1
// 3 1 2 4 3 -1
// 4 2 4 3 3 5 6 -1
// 5 4 6 -1

// 12
// 1 2 1 6 1 10 1 -1
// 2 1 1 3 1 -1
// 3 2 1 4 1 -1
// 4 3 1 5 1 -1
// 5 4 1 -1
// 6 1 1 7 1 -1
// 7 6 1 8 1 -1
// 8 7 1 9 1 -1
// 9 8 1 -1
// 10 1 1 11 1 12 1 -1
// 11 10 1 -1
// 12 10 1 -1

fn input_number() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    return buf.trim().parse::<i32>().unwrap();
}

fn input_number_tree(tree: &mut Vec<Vec<Pair>>) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.trim().split_whitespace();
    let target = (iter.next().unwrap()).parse::<i32>().unwrap();

    while let Ok(node) = (iter.next().unwrap()).parse::<i32>() {
        if node == -1 {
            break;
        }
        let distance = iter.next().unwrap().parse::<i32>().unwrap();
        tree[target as usize].push(Pair { node, distance });
    }
}

pub struct Pair {
    pub node: i32,
    pub distance: i32,
}

fn main() {
    let count = input_number();
    let mut tree = Vec::<Vec<Pair>>::new();

    for _ in 0..(count + 1) {
        tree.push(Vec::<Pair>::new());
    }

    for _ in 0..count {
        input_number_tree(&mut tree);
    }

    let mut visited: [bool; 100001] = [false; 100001];

    let one = dfs(1, 0, &mut tree, &mut visited);

    visited.fill(false);
    let two = dfs(one.node, 0, &mut tree, &mut visited);

    println!("{}", two.distance);
}

fn dfs(target: i32, dist: i32, tree: &Vec<Vec<Pair>>, visited: &mut [bool]) -> Pair {
    if visited[target as usize] {
        return Pair {
            node: 0,
            distance: 0,
        };
    }
    visited[target as usize] = true;

    let mut result = Pair {
        node: target,
        distance: 0,
    };

    for &Pair { node, distance } in &tree[target as usize] {
        if visited[node as usize] {
            continue;
        }

        let temp = dfs(node, distance, tree, visited);
        if result.distance < temp.distance {
            result = temp;
        }
    }

    result.distance += dist;
    result
}
