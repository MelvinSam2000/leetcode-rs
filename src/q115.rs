/*
    115 - Distinct Subsequences
    Time: O(m*n)
    Space: O(n)
*/
pub fn num_distinct(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let m = s.len();
    let n = t.len();
    let mut dp = [vec![0; n + 1], vec![0; n + 1]];
    dp[0][n] = 1;
    dp[1][n] = 1;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[0][j] = if s[i] == t[j] {
                dp[1][j] + dp[1][j + 1]
            } else {
                dp[1][j]
            };
        }
        dp.swap(0, 1);
    }
    dp[1][0]
}

/*
    115 - Distinct Subsequences
    Time: O(m*n)
    Space: O(m*n) (Can be optimized to O(n))
*/
pub fn num_distinct_v2(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let m = s.len();
    let n = t.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][n] = 1;
    }
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[i][j] = if s[i] == t[j] {
                dp[i + 1][j] + dp[i + 1][j + 1]
            } else {
                dp[i + 1][j]
            };
        }
    }
    dp[0][0]
}
