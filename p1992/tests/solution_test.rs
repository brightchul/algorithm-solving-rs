use p1992::solution;

#[test]
fn solution_size8_test() {
    let test_list: Vec<Vec<u32>> = vec![
        vec![1, 1, 1, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0, 0, 1, 1],
        vec![1, 1, 1, 1, 0, 0, 1, 1],
    ];
    assert_eq!(
        solution(8, test_list),
        String::from("((110(0101))(0010)1(0001))")
    );
}
