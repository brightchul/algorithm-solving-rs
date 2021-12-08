use util::{input_number, input_number_vec, input_string};

// 10
// 9
// 0 1 2 3 4 5 6 7 8
// 정답: 2
// 오답: 90

// 11
// 4
// 0 1 7 9
// 정답 4

// 10
// 3
// 0 1 2
// 정답2

// 21
// 1
// 2
// 정답4

// 64
// 2
// 2 6
// 정답7

// 99
// 0
// 정답1

// 101
// 0
// 정답1

// 0
// 0
// 정답1

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
    let target_number = target_string.parse::<i32>().unwrap();
    let error_len = input_number::<i32>();
    let mut error_num_list = vec![];

    if error_len > 0 {
        error_num_list = input_number_vec::<i32>();
    }

    println!("{}", solution(target_string, target_number, error_num_list));
}

fn solution(target_string: String, target_number: i32, error_num_list: Vec<i32>) -> String {
    if target_number == 100 {
        return String::from("0");
    }

    let target_len = target_string.len() as i32;
    let mut num_list: Vec<i32> = vec![];
    for i in 0..10 {
        if !error_num_list.contains(&i) {
            num_list.push(i);
        }
    }

    let mut result_list: Vec<i32> = vec![];

    for target in &num_list {
        run(*target, target_len, 1, &num_list, &mut result_list);
    }

    let mut result = 100;
    for one in result_list {
        let one_count = (target_number - one).abs() + one.to_string().len() as i32;
        let result_count = (result - target_number).abs() + result.to_string().len() as i32;
        if result_count > one_count {
            result = one;
        }
    }

    let temp1 = (target_number - result).abs() + result.to_string().len() as i32;
    let temp2 = (target_number - 100).abs();

    temp1.min(temp2).to_string()
}

fn run(
    target: i32,
    target_len: i32,
    current_len: i32,
    num_list: &Vec<i32>,
    result_list: &mut Vec<i32>,
) {
    if target_len < current_len {
        result_list.push(target);
        return;
    }

    if target_len - 1 <= current_len {
        result_list.push(target);
    }

    for num in num_list {
        let current_num = target * 10 + num;
        run(
            current_num,
            target_len,
            current_len + 1,
            num_list,
            result_list,
        );
    }
}
