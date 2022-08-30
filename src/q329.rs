/*
    329 - Longest Increasing Path in Matrix
    Time: O(m*n)
    Space: O(m*n)
    Note: First DP problem that cant be done bottom up...
*/
pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut dp = vec![vec![None; n]; m];
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            dfs((i as i32, j as i32), -1, &matrix, &mut dp, (m, n));
            if let Some(val) = dp[i][j] {
                res = res.max(val);
            }
        }
    }
    res
}

fn dfs(
    pos: (i32, i32),
    prev: i32,
    matrix: &Vec<Vec<i32>>,
    dp: &mut Vec<Vec<Option<i32>>>,
    dim: (usize, usize),
) -> i32 {
    let (i, j) = pos;
    let (m, n) = dim;
    if i < 0 || i == m as i32 || j < 0 || j == n as i32 {
        return 0;
    }
    let (i, j) = (i as usize, j as usize);
    if matrix[i][j] <= prev {
        return 0;
    }
    if let Some(val) = dp[i][j] {
        return val;
    }
    let prev = matrix[i][j];
    let (i, j) = (i as i32, j as i32);
    let mut res = 1;
    res = res.max(1 + dfs((i + 1, j), prev, matrix, dp, (m, n)));
    res = res.max(1 + dfs((i - 1, j), prev, matrix, dp, (m, n)));
    res = res.max(1 + dfs((i, j + 1), prev, matrix, dp, (m, n)));
    res = res.max(1 + dfs((i, j - 1), prev, matrix, dp, (m, n)));
    let (i, j) = (i as usize, j as usize);
    dp[i][j] = Some(res);
    res
}
