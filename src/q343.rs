/*
    343 - Integer Break
    Time: O(n^2)
    Space: O(n)
*/
pub fn integer_break(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![1; n + 1];
    for i in 2..=n {
        for j in 1..=i / 2 {
            dp[i] = dp[i]
                .max(j.max(dp[j]) * (i - j).max(dp[i - j]));
        }
    }
    dp[n] as i32
}
