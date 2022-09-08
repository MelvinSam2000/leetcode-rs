/*
    377 - Combination Sum IV
    Time: O(m*n)
    Space: O(n)
    Note: Similar to Unbounded knapsack, coin change I
*/
pub fn combination_sum4(
    nums: Vec<i32>,
    target: i32,
) -> i32 {
    let n = target as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for x in &nums {
            let x = *x as usize;
            if i >= x {
                dp[i] += dp[i - x];
            }
        }
    }
    dp[n]
}
