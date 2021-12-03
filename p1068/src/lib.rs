//! # p1068
//!
//!  1068 트리
//!
//! 트리에서 리프 노드란, 자식의 개수가 0인 노드를 말한다. 트리가 주어졌을 때, 노드 하나를 지울 것이다.
//!
//! 그 때, 남은 트리에서 리프 노드의 개수를 구하는 프로그램을 작성하시오.
//!
//! 노드를 지우면 그 노드와 노드의 모든 자손이 트리에서 제거된다.
//!
//! <br/>
//!
//! 더 자세한 내용은 [https://www.acmicpc.net/problem/1068](https://www.acmicpc.net/problem/1068) 참고하세요
use std::{cmp, collections::BTreeMap, io};

/// 숫자를 io 입력값에서 파싱하는 함수
pub fn get_number_io() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let result = buf.trim().parse().unwrap();
    result
}

/// 배열 값을 io 입력값에서 파싱하는 함수
pub fn get_vec_i32_io() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

/// 1068 문제 해결 함수
///
/// 트리를 이용하면 되지만, BTreeMap와 DFS를 이용해서 풀었다.
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
