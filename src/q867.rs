/*
    867 - Transpose Matrix
    Time: O(n*m)
    Space: O(n*m)
*/
pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut out = vec![vec![0; m]; n];
    for i in 0..m {
        for j in 0..n {
            out[j][i] = matrix[i][j];
        }
    }
    out
}
