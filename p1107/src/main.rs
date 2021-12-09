use p1107::solution1;
use util::{input_number, input_number_vec, input_string};

// 99999
// 2
// 8 9
// 정답 7

// 64
// 2
// 2 6
// ans 7

// 21
// 1
// 2
// ans 4

// 97
// 3
// 6 7 8
// ans 3

// 88
// 2
// 8 9
// ans 12

// 10
// 1
// 0
// ans 2

fn main() {
    let target_string = input_string();
    let error_len = input_number::<i32>();
    let mut error_num_list = vec![];

    if error_len > 0 {
        error_num_list = input_number_vec::<i32>();
    }

    println!("{}", solution1(target_string, error_num_list));
}
