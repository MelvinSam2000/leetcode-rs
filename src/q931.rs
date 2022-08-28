/*
    931 - Minimum Falling Path Sum (Optimized)
    Time: O(m*n)
    Space: O(n)
*/
pub fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
    let m = a.len();
    let n = a[0].len();
    let mut dp = [vec![0; n], vec![0; n]];
    dp[1] = a[m - 1].clone();
    for i in (0..m - 1).rev() {
        for j in 0..n {
            let mut x = dp[1][j];
            if j != 0 {
                x = x.min(dp[1][j - 1]);
            }
            if j != n - 1 {
                x = x.min(dp[1][j + 1]);
            }
            dp[0][j] = a[i][j] + x;
        }
        dp.swap(0, 1);
    }
    dp[1].iter().min().cloned().unwrap()
}

/*
    931 - Minimum Falling Path Sum
    Time: O(m*n)
    Space: O(m*n) (Can be optimized to O(n))
*/
pub fn min_falling_path_sum_v2(a: Vec<Vec<i32>>) -> i32 {
    let m = a.len();
    let n = a[0].len();
    let mut dp = vec![vec![0; n]; m];
    dp[m - 1] = a[m - 1].clone();
    for i in (0..m - 1).rev() {
        for j in 0..n {
            let mut x = dp[i + 1][j];
            if j != 0 {
                x = x.min(dp[i + 1][j - 1]);
            }
            if j != n - 1 {
                x = x.min(dp[i + 1][j + 1]);
            }
            dp[i][j] = a[i][j] + x;
        }
    }
    dp[0].iter().min().cloned().unwrap()
}
