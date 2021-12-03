use p1068::solution;

#[test]
fn solution_example_test() {
    assert_eq!(solution(vec![-1], 0), 0);
    assert_eq!(solution(vec![-1, 0, 0, 1, 1], 2), 2);
    assert_eq!(solution(vec![-1, 0, 0, 1, 1], 1), 1);
    assert_eq!(solution(vec![-1, 0, 0, 1, 1], 0), 0);
    assert_eq!(solution(vec![-1, 0, 0, 2, 2, 4, 4, 6, 6], 4), 2);
}
