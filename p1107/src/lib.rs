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
