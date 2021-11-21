//! # p1992
//!
//! 1992번 쿼드트리
//!
//! 흑백 영상을 압축하여 표현하는 데이터 구조로 쿼드 트리(Quad Tree)라는 방법이 있다. 흰 점을 나타내는 0과 검은 점을 나타내는 1로만 이루어진 영상(2차원 배열)에서 같은 숫자의 점들이 한 곳에 많이 몰려있으면, 쿼드 트리에서는 이를 압축하여 간단히 표현할 수 있다.
//! 주어진 영상이 모두 0으로만 되어 있으면 압축 결과는 "0"이 되고, 모두 1로만 되어 있으면 압축 결과는 "1"이 된다. 만약 0과 1이 섞여 있으면 전체를 한 번에 나타내지를 못하고, 왼쪽 위, 오른쪽 위, 왼쪽 아래, 오른쪽 아래, 이렇게 4개의 영상으로 나누어 압축하게 되며, 이 4개의 영역을 압축한 결과를 차례대로 괄호 안에 묶어서 표현한다
//!
//! 더 자세한 내용은 [https://www.acmicpc.net/problem/1992](https://www.acmicpc.net/problem/1992)를 참고하세요.

struct Pos {
    row: usize,
    col: usize,
}
const DIR_LIST: &'static [Pos] = &[
    Pos { row: 0, col: 0 },
    Pos { row: 0, col: 1 },
    Pos { row: 1, col: 0 },
    Pos { row: 1, col: 1 },
];

/// 문제 풀이 코드입니다.
///
/// 트리를 이용하지 않고 벡터를 이용해서 각 지수에 대한 캐시를 만듭니다.
/// 캐시가 생성한 이후 각각을 순회하면서 0, 1에 대해서는 문자열에 저장, 그외의 경우인 2에 대해서는
/// 다시 재귀로 순회하면서 0과 1을 찾아서 반환 문자열에 더해서 그 결과를 반환합니다.
pub fn solution(n: usize, list: Vec<Vec<u32>>) -> String {
    let mut cache: Vec<Vec<Vec<u32>>> = Vec::<Vec<Vec<u32>>>::new();
    cache.push(list);

    let mut idx = 2;
    while idx <= n {
        let size = n / idx;
        let mut cache_list: Vec<Vec<u32>> = Vec::<Vec<u32>>::with_capacity(size);
        for _ in 0..size {
            cache_list.push(vec![2; size]);
        }

        for i in 0..size {
            let list = cache.last().unwrap();
            let row: usize = 2 * i;

            for j in 0..size {
                let col: usize = 2 * j;
                let mut is_all_zero = true;
                let mut is_all_one = true;

                for dir in DIR_LIST {
                    let row_dir = row + dir.row;
                    let col_dir = col + dir.col;
                    let value = list[row_dir][col_dir];
                    match value {
                        0 => is_all_one = false,
                        1 => is_all_zero = false,
                        _ => {
                            is_all_zero = false;
                            is_all_one = false;
                        }
                    }
                }

                if is_all_zero {
                    cache_list[i][j] = 0;
                } else if is_all_one {
                    cache_list[i][j] = 1;
                }
            }
        }
        cache.push(cache_list);
        idx *= 2;
    }
    cache.reverse();
    return start(&cache, 0, 0, 0);
}

// 최상위 시작 함수입니다.
fn start(cache: &Vec<Vec<Vec<u32>>>, row: usize, col: usize, level: usize) -> String {
    let mut result_text = String::from("");
    if cache[0][0][0] == 2 {
        recur(cache, row * 2, col * 2, level + 1, &mut result_text);
    } else {
        result_text.push_str(&cache[0][0][0].to_string());
    }
    result_text
}

fn recur(
    cache: &Vec<Vec<Vec<u32>>>,
    row: usize,
    col: usize,
    level: usize,
    result_text: &mut String,
) {
    result_text.push_str("(");
    let list = &cache[level];

    for dir in DIR_LIST {
        let row_dir = row + dir.row;
        let col_dir = col + dir.col;

        if list[row_dir][col_dir] == 2 {
            recur(cache, row_dir * 2, col_dir * 2, level + 1, result_text);
        } else {
            result_text.push_str(&list[row_dir][col_dir].to_string());
        }
    }
    result_text.push_str(")")
}
