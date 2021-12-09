pub fn solution1(target_string: String, error_num_list: Vec<i32>) -> i32 {
    let target_number = target_string.parse::<i32>().unwrap();
    if target_number == 100 {
        return 0;
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

    temp1.min(temp2)
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

pub fn solution2(target_string: String, error_num_list: Vec<i32>) -> i32 {
    let target_num = target_string.parse::<i32>().unwrap();
    if target_num == 100 {
        return 0;
    }

    let mut result = 0;
    for idx in 0..(target_num + 1) {
        let upper_num = target_num + idx;
        if check_num(upper_num, &error_num_list) {
            result = get_len(upper_num) + idx;

            break;
        }

        let lower_num = target_num - idx;

        if check_num(lower_num, &error_num_list) {
            result = get_len(lower_num) + idx;
            break;
        }
    }
    result.min((target_num - 100) as i32)
}

fn check_num(mut value: i32, err_num_list: &Vec<i32>) -> bool {
    if err_num_list.contains(&value) {
        return true;
    }
    while value > 0 {
        if !err_num_list.contains(&(value % 10)) {
            return false;
        }
        value /= 10;
    }
    return true;
}

fn get_len(mut value: i32) -> i32 {
    let mut result = 1;
    while value > 0 {
        value /= 10;
        result += 1;
    }
    result
}
