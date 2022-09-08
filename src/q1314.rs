/*
    1314 - Matrix Block Sum
    Time: O(m*n)
    Space: O(1)
*/
pub fn matrix_block_sum(
    mut mat: Vec<Vec<i32>>,
    k: i32,
) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let k = k as usize;
    let mut dp = mat.clone();

    for i in 0..m {
        for j in 1..n {
            dp[i][j] += dp[i][j - 1];
        }
    }
    for i in 1..m {
        for j in 0..n {
            dp[i][j] += dp[i - 1][j];
        }
    }
    for i in 0..m {
        for j in 0..n {
            let x1 = if j > k { j - k } else { 0 };
            let x2 = (n - 1).min(j + k);
            let y1 = if i > k { i - k } else { 0 };
            let y2 = (m - 1).min(i + k);

            let dx =
                if x1 > 0 { dp[y2][x1 - 1] } else { 0 };
            let dy =
                if y1 > 0 { dp[y1 - 1][x2] } else { 0 };
            let dz = if x1 > 0 && y1 > 0 {
                dp[y1 - 1][x1 - 1]
            } else {
                0
            };

            mat[i][j] = dp[y2][x2] - dx - dy + dz
        }
    }
    mat
}
