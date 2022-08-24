/*
    64 - Minimum Path Sum (Optimized)
    Time: O(n*m)
    Space: O(n)
*/
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = [vec![0; n], vec![0; n]];

    for i in 0..m {
        for j in 0..n {
            dp[0][j] = grid[i][j]
                + match (i, j) {
                    (0, 0) => 0,
                    (_, 0) => dp[1][j],
                    (0, _) => dp[0][j - 1],
                    _ => dp[1][j].min(dp[0][j - 1]),
                };
        }
        dp.swap(0, 1);
    }
    dp[1][n - 1]
}

/*
    64 - Minimum Path Sum
    Time: O(n*m)
    Space: O(n*m)
    Note: Can be optimized to O(m)
*/
pub fn min_path_sum_v2(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            dp[i][j] = grid[i][j]
                + match (i, j) {
                    (0, 0) => 0,
                    (_, 0) => dp[i - 1][j],
                    (0, _) => dp[i][j - 1],
                    _ => dp[i - 1][j].min(dp[i][j - 1]),
                };
        }
    }
    dp[m - 1][n - 1]
}
