/*
    509 - Fibonacci Number
    Time: O(n)
    Space: O(1)
*/
pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let (mut x, mut y, mut z) = (0, 1, 0);
    for _ in 1..n {
        z = x + y;
        x = y;
        y = z;
    }
    z
}

#[test]
fn t1() {
    let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21];
    let answer = (0..expected.len() as i32).map(fib).collect::<Vec<_>>();
    println!("{:?}", answer);
    assert_eq!(expected, answer);
}
