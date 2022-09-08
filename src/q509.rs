/*
    509 - Fibonacci Number
    Time: O(n)
    Space: O(1)
*/
pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut dp = [0, 1, 0];
    for _ in 1..n {
        dp[0] = dp[1] + dp[2];
        dp[2] = dp[1];
        dp[1] = dp[0];
    }
    dp[0]
}

/*
    509 - Fibonacci Number (Ebic version)
    Time: O(1)
    Space: O(1)
*/
pub fn fib_ebic(n: i32) -> i32 {
    const K: usize = 40;
    static FIB: [i32; K] = {
        let mut fib = [0; K];
        fib[0] = 0;
        fib[1] = 1;
        let mut i = 2;
        while i < K {
            fib[i] = fib[i - 1] + fib[i - 2];
            i += 1;
        }
        fib
    };
    FIB[n as usize]
}

#[test]
fn t1() {
    let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21];
    let answer = (0..expected.len() as i32)
        .map(fib)
        .collect::<Vec<_>>();
    println!("{:?}", answer);
    assert_eq!(expected, answer);
}
