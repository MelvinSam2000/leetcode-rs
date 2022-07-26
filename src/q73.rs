/*
    73 - Set Matrix Zeroes
    Time: O(n*m)
    Space: O(n + m)
    Note: Apparently theres a O(1) space solution (todo)
*/
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut marked_rows = vec![false; m];
    let mut marked_cols = vec![false; n];
    // mark rows and columns to clear
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                marked_rows[i] = true;
                marked_cols[j] = true;
            }
        }
    }
    // begin clearing
    for i in 0..m {
        if marked_rows[i] {
            for j in 0..n {
                matrix[i][j] = 0;
            }
        }
    }
    for j in 0..n {
        if marked_cols[j] {
            for i in 0..m {
                matrix[i][j] = 0;
            }
        }
    }
}
