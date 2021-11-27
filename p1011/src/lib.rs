pub fn solution(n: i64) -> i64 {
    if n <= 3 {
        return n;
    }
    let almost_sqrt_dis = (n as f64).sqrt().floor() as i64;
    let rest = n - almost_sqrt_dis.pow(2);

    if rest == 0 {
        return almost_sqrt_dis * 2 - 1;
    }
    if rest <= almost_sqrt_dis {
        return almost_sqrt_dis * 2;
    }
    return almost_sqrt_dis * 2 + 1;
}

#[test]
fn p1011_test() {
    assert_eq!(solution(0), 0);
    assert_eq!(solution(1), 1);
    assert_eq!(solution(2), 2);
    assert_eq!(solution(3 - 0), 3);
    assert_eq!(solution(5 - 1), 3);
    assert_eq!(solution(50 - 45), 4);
}
