//! # p1052
//!
//! 1052번 물병
//!
//! 지민이는 N개의 물병을 가지고 있다. 각 물병에는 물을 무한대로 부을 수 있다. 처음에 모든 물병에는 물이 1리터씩 들어있다. 지민이는 이 물병을 또 다른 장소로 옮기려고 한다. 지민이는 한 번에 K개의 물병을 옮길 수 있다. 하지만, 지민이는 물을 낭비하기는 싫고, 이동을 한 번보다 많이 하기는 싫다. 따라서, 지민이는 물병의 물을 적절히 재분배해서, K개를 넘지 않는 비어있지 않은 물병을 만들려고 한다.
//! 물은 다음과 같이 재분배 한다.
//!
//! 더 자세한 내용은 [https://www.acmicpc.net/problem/1052](https://www.acmicpc.net/problem/1052)를 참고하세요.

/// solution2를 개선한 함수입니다.
///
/// 처음에 2진수로 변환, 배열로 생성한 다음에 그 배열을 순회합니다.
pub fn solution1(n: i32, k: i32) -> i32 {
    let binary_text = format!("{:b}", n);
    let mut result: Vec<u32> = binary_text
        .chars()
        .map(|v| v.to_digit(10).unwrap())
        .rev()
        .collect();

    result.push(0);

    let mut result_value = 0;
    let mut two = 1;
    let mut count = 0;

    for idx in 0..result.len() {
        if result[idx] == 1 {
            count += 1;
        }
    }
    for idx in 0..result.len() - 1 {
        if k >= count {
            break;
        }
        if result[idx] == 1 {
            result_value += two;
        }
        if result[idx] > 0 {
            result[idx] = 0;
            result[idx + 1] += 1;
            count -= 1;

            if result[idx + 1] == 1 {
                count += 1;
            }
        }
        if k >= count {
            break;
        }
        two *= 2;
    }
    result_value
}

/// 1차 시도한 함수입니다.
///
/// 매회마다 1씩을 더하고 다시 2진수로 변환해서 1을 카운트 했습니다.
pub fn solution2(mut n: i32, k: i32) -> i32 {
    let mut result = 0;
    while count_one(n) > k {
        n += 1;
        result += 1;
    }
    return result;
}

/// 주어진 수를 2진수로 변환하면서 1의 개수를 카운트해서 그 값을 반환합니다.
pub fn count_one(mut num: i32) -> i32 {
    let mut count = 0;
    while num > 0 {
        if num % 2 > 0 {
            count += 1;
        }
        num /= 2;
    }
    count
}
