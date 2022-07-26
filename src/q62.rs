/*
    62 - Unique Paths
    Time: O(n*m)
    Space: O(n*m)
*/
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = 1;
    for i in 0..m {
        for j in 0..n {
            if i == j && j == 0 {
                continue;
            }
            let x1 = if i != 0 {
                dp[i - 1][j]
            } else {
                0
            };
            let x2 = if j != 0 {
                dp[i][j - 1]
            } else {
                0
            };
            dp[i][j] = x1 + x2;
        }
    }
    dp[m - 1][n - 1]
}