use std::collections::VecDeque;

pub fn solution(n: i32) -> i64 {
    if n < 10 {
        return n as i64;
    }

    let mut queue: VecDeque<i64> = VecDeque::new();
    for idx in 1..10 {
        queue.push_back(idx);
    }

    let mut count = 9;

    while queue.len() > 0 {
        let queue_front_value = queue.pop_front().unwrap();
        let digit_limit = queue_front_value % 10;

        for one in 0..digit_limit {
            let generated_value = queue_front_value * 10 + one;
            queue.push_back(generated_value);
            count += 1;

            if n == count {
                return generated_value;
            }
        }
    }
    -1
}
