pub struct NumMatrix {
    mat: Vec<Vec<i32>>,
}

/*
    304 - Range Query 2d - Inmutable
*/
impl NumMatrix {
    // O(m*n)
    pub fn new(mut mat: Vec<Vec<i32>>) -> Self {
        let (m, n) = (mat.len(), mat[0].len());
        for i in 0..m {
            for j in 1..n {
                mat[i][j] += mat[i][j - 1];
            }
        }
        for i in 1..m {
            for j in 0..n {
                mat[i][j] += mat[i - 1][j];
            }
        }
        Self { mat }
    }

    // O(1)
    pub fn sum_region(
        &self,
        r1: i32,
        c1: i32,
        r2: i32,
        c2: i32,
    ) -> i32 {
        let (r1, c1, r2, c2) = (
            r1 as usize,
            c1 as usize,
            r2 as usize,
            c2 as usize,
        );
        let up =
            if r1 > 0 { self.mat[r1 - 1][c2] } else { 0 };
        let left =
            if c1 > 0 { self.mat[r2][c1 - 1] } else { 0 };
        let diag = if r1 > 0 && c1 > 0 {
            self.mat[r1 - 1][c1 - 1]
        } else {
            0
        };
        self.mat[r2][c2] - up - left + diag
    }
}
