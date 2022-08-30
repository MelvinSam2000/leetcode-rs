/*
    714 - Buy and Sell stock with transaction fee (optimized)
    Time: O(n)
    Space: O(1)
*/
pub fn max_profit(p: Vec<i32>, fee: i32) -> i32 {
    let n = p.len();
    let mut dp = [[0, 0], [0, 0]];
    for i in (0..n).rev() {
        dp[0][0] = dp[1][0].max(p[i] - fee + dp[1][1]);
        dp[0][1] = dp[1][1].max(-p[i] + dp[1][0]);
        dp[1] = dp[0];
    }
    dp[0][1]
}

/*
    714 - Buy and Sell stock with transaction fee
    Time: O(n)
    Space: O(n)
*/
pub fn max_profit_v2(p: Vec<i32>, fee: i32) -> i32 {
    let n = p.len();
    let mut dp = [vec![0; n + 1], vec![0; n + 1]];
    for i in (0..n).rev() {
        dp[0][i] = dp[0][i + 1].max(p[i] - fee + dp[1][i + 1]);
        dp[1][i] = dp[1][i + 1].max(-p[i] + dp[0][i + 1]);
    }
    dp[1][0]
}

/*
    714 - Buy and Sell stock with transaction fee
    Time: O(n^2)
    Space: O(n^2) (Can be optimized to O(n))
    Note: This MLEs..., optimized version TLEs...
*/
pub fn max_profit_v3(p: Vec<i32>, fee: i32) -> i32 {
    let n = p.len();
    let mut dp = vec![vec![0; n + 1]; n];
    for i in 0..n {
        dp[n - 1][i] = p[n - 1] - p[i] - fee;
    }
    for i in (0..n - 1).rev() {
        for j in 0..n {
            dp[i][j] = dp[i + 1][j].max(p[i] - p[j] - fee + dp[i + 1][n]);
        }
        dp[i][n] = dp[i + 1][i].max(dp[i + 1][n])
    }
    dp[0][n]
}
