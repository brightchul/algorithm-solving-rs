use std::{cmp, collections::BTreeMap, io};

pub fn get_number_io() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let result = buf.trim().parse().unwrap();
    result
}

pub fn get_vec_i32_io() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

pub fn solution(list: Vec<i32>, remove_node: usize) -> i32 {
    let root_idx = list.iter().position(|x| *x == -1).unwrap();
    if root_idx == remove_node {
        return 0;
    }

    let mut tree_map: BTreeMap<i32, Vec<usize>> = BTreeMap::new();

    for (idx, value) in list.iter().enumerate() {
        if *value == -1 {
            continue;
        }

        if !tree_map.contains_key(value) {
            tree_map.insert(*value, Vec::new());
        }

        tree_map.get_mut(value).unwrap().push(idx);
    }

    tree_map
        .get_mut(&list[remove_node])
        .unwrap()
        .retain(|x| *x != remove_node);

    dfs(root_idx as i32, &tree_map)
}

pub fn dfs(value: i32, tree_map: &BTreeMap<i32, Vec<usize>>) -> i32 {
    let mut result = 0;
    if !tree_map.contains_key(&value) {
        return 1;
    }
    for idx in tree_map.get(&value).unwrap().iter() {
        result += dfs(*idx as i32, tree_map);
    }

    return cmp::max(result, 1);
}
