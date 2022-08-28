/*
    96 - Number of unique BSTs
    Time: O(n)
    Space: O(n)
    Note: Nth catalan number problem.
    Can be optimized to use O(1) with the
    direct formula, but that solution is unlikely to be in an interview.
    Full formula may not work due to floating point errors as well.
*/
pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = (0..i).map(|j| dp[j] * dp[i - j - 1]).sum();
    }
    dp[n]
}

/*
    96 - Number of unique BSTs (Optimized, but not 100% accurate)
    Time: O(n)
    Space: O(1)
    Note: Computes nth catalan number using formula.
    Doesnt always work due to floating point errors.
*/
pub fn num_trees_v2(n: i32) -> i32 {
    (2..=n).fold(1f64, |prod, k| {
        prod as f64 * (n as f64 + k as f64) / k as f64
    }) as i32
}
