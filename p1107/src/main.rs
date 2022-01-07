use p1107::solution2;
use util::{input_number, input_number_vec, input_string};

fn main() {
    let target_string = input_string();
    let error_len = input_number::<i32>();
    let mut error_num_list = vec![];

    if error_len > 0 {
        error_num_list = input_number_vec::<i32>();
    }

    println!("{}", solution2(target_string, error_num_list));
}
