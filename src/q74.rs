/*
    74 - Search a 2D Matrix
    Time: O(log(m*n))
    Space: O(1)
*/
pub fn search_matrix(
    matrix: Vec<Vec<i32>>,
    target: i32,
) -> bool {
    use std::cmp::Ordering;
    let m = matrix.len();
    let n = matrix[0].len();

    fn search_row(
        matrix: &Vec<Vec<i32>>,
        target: i32,
        dim: (usize, usize),
    ) -> Option<usize> {
        let (m, n) = dim;
        let (mut l, mut r) = (0, m as i32 - 1);
        while l <= r {
            let m = (l + r) / 2;
            match (
                matrix[m as usize][0].cmp(&target),
                matrix[m as usize][n - 1].cmp(&target),
            ) {
                (Ordering::Greater, _) => r = m - 1,
                (_, Ordering::Less) => l = m + 1,
                _ => return Some(m as usize),
            }
        }
        None
    }

    fn search_tile(
        matrix: &Vec<Vec<i32>>,
        target: i32,
        cols: usize,
        row: usize,
    ) -> bool {
        let (mut l, mut r) = (0, cols as i32 - 1);
        while l <= r {
            let m = (l + r) / 2;
            match matrix[row][m as usize].cmp(&target) {
                Ordering::Greater => r = m - 1,
                Ordering::Less => l = m + 1,
                Ordering::Equal => return true,
            }
        }
        false
    }

    let row = search_row(&matrix, target, (m, n));
    if let Some(row) = row {
        search_tile(&matrix, target, n, row)
    } else {
        false
    }
}
