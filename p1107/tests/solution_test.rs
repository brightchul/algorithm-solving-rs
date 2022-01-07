use p1107::{solution1, solution2};

type TestCase = (String, Vec<i32>, i32);

#[test]
fn solution1_test() {
    let test_case_list: Vec<TestCase> = vec![
        (String::from("10"), vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 2),
        (String::from("11"), vec![0, 1, 7, 9], 4),
        (String::from("10"), vec![0, 1, 2], 2),
        (String::from("21"), vec![2], 4),
        (String::from("64"), vec![2, 6], 7),
        (String::from("99"), vec![], 1),
        (String::from("101"), vec![], 1),
        (String::from("0"), vec![], 1),
        (String::from("99999"), vec![8, 9], 7),
        (String::from("64"), vec![2, 6], 7),
        (String::from("21"), vec![2], 4),
        (String::from("97"), vec![6, 7, 8], 3),
        (String::from("88"), vec![8, 9], 12),
        (String::from("10"), vec![0], 2),
        (String::from("0"), vec![0], 1),
    ];

    for (target_string, error_num_list, answer) in test_case_list {
        assert_eq!(solution1(target_string, error_num_list), answer);
    }
}

#[test]
fn solution2_test() {
    let test_case_list: Vec<TestCase> = vec![
        (String::from("10"), vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 2),
        (String::from("11"), vec![0, 1, 7, 9], 4),
        (String::from("10"), vec![0, 1, 2], 2),
        (String::from("21"), vec![2], 4),
        (String::from("64"), vec![2, 6], 7),
        (String::from("99"), vec![], 1),
        (String::from("101"), vec![], 1),
        (String::from("0"), vec![], 1),
        (String::from("99999"), vec![8, 9], 7),
        (String::from("64"), vec![2, 6], 7),
        (String::from("21"), vec![2], 4),
        (String::from("97"), vec![6, 7, 8], 3),
        (String::from("88"), vec![8, 9], 12),
        (String::from("10"), vec![0], 2),
        (String::from("0"), vec![0], 2),
    ];

    for (target_string, error_num_list, answer) in test_case_list {
        println!("{:?}", &error_num_list);
        assert_eq!(solution2(target_string, error_num_list), answer,);
    }
}
