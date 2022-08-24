/*
    221 - Maximal Square (optimized)
    Time: O(m*n)
    Space: O(n)
*/
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut dp = [vec![0; n], vec![0; n]];
    let mut res = 0i32;

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[0][j] = if matrix[i][j] == '1' {
                1 + if i < m - 1 && j < n - 1 {
                    dp[1][j].min(dp[0][j + 1]).min(dp[1][j + 1])
                } else {
                    0
                }
            } else {
                0
            };
            res = res.max(dp[0][j]);
        }
        dp.swap(0, 1);
    }
    res.pow(2)
}

/*
    221 - Maximal Square
    Time: O(m*n)
    Space: O(m*n)
    Note: Can be optimized to O(n) space
*/
pub fn maximal_square_v2(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut dp = vec![vec![0; n]; m];
    let mut res = 0i32;

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[i][j] = if matrix[i][j] == '1' {
                1 + if i < m - 1 && j < n - 1 {
                    dp[i + 1][j].min(dp[i][j + 1]).min(dp[i + 1][j + 1])
                } else {
                    0
                }
            } else {
                0
            };
            res = res.max(dp[i][j]);
        }
    }
    res.pow(2)
}
