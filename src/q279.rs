/*
    279 - Perfect Squares
    Time: O(n^(3/2))
    Space: O(n)
*/
pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = (1..=n as i32 + 1).collect::<Vec<_>>();

    for i in 0..=(n as f64).sqrt() as usize {
        dp[i.pow(2)] = 1;
    }

    for i in 1..=n {
        for j in 0..=(i as f64).sqrt() as usize {
            let j = j.pow(2);
            dp[i] = dp[i].min(dp[j] + dp[i - j]);
        }
    }
    dp[n]
}
