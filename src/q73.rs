/*
    73 - Set Matrix Zeroes
    Time: O(n*m)
    Space: O(1)
*/
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    let n = matrix[0].len();

    // mark rows and columns to clear
    let mut zero_row_clear = false;
    let mut zero_col_clear = false;

    for i in 0..m {
        if matrix[i][0] == 0 {
            zero_col_clear = true;
        }
    }
    for j in 0..n {
        if matrix[0][j] == 0 {
            zero_row_clear = true;
        }
    }

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }
    // begin clearing
    for i in 1..m {
        if matrix[i][0] == 0 {
            for j in 1..n {
                matrix[i][j] = 0;
            }
        }
    }
    for j in 1..n {
        if matrix[0][j] == 0 {
            for i in 1..m {
                matrix[i][j] = 0;
            }
        }
    }
    if zero_col_clear {
        for i in 0..m {
            matrix[i][0] = 0;
        }
    }
    if zero_row_clear {
        for j in 0..n {
            matrix[0][j] = 0;
        }
    }
}
