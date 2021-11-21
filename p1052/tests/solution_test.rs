use p1052::{count_one, solution1, solution2};

#[test]
fn solution1_example_test() {
    assert_eq!(solution1(3, 1), 1);
    assert_eq!(solution1(13, 2), 3);
    assert_eq!(solution1(1000000, 5), 15808);
}
#[test]
fn solution2_example_test() {
    assert_eq!(solution2(3, 1), 1);
    assert_eq!(solution2(13, 2), 3);
    assert_eq!(solution2(1000000, 5), 15808);
}

#[test]
fn count_one_test() {
    assert_eq!(count_one(1), 1);
    assert_eq!(count_one(2), 1);
    assert_eq!(count_one(3), 2);
    assert_eq!(count_one(4), 1);
    assert_eq!(count_one(5), 2);
    assert_eq!(count_one(7), 3);
}
