/*
    120 - Triangle
    Time: O(n^2)
    Space: O(n)
*/
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let n = triangle.len();
    let mut dp = [triangle[n - 1].clone(), vec![0; n]];
    for i in (0..n - 1).rev() {
        for j in 0..=i {
            dp[1][j] = triangle[i][j] + dp[0][j].min(dp[0][j + 1]);
        }
        dp.swap(0, 1);
    }
    dp[0][0]
}
