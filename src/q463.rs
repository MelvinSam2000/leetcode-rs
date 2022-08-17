/*
    463 - Island Perimeter
    Time: O(n*m)
    Space: O(1)
*/
pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut perimeter = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                let up = (i == 0 || grid[i - 1][j] == 0) as i32;
                let down = (i == m - 1 || grid[i + 1][j] == 0) as i32;
                let left = (j == 0 || grid[i][j - 1] == 0) as i32;
                let right = (j == n - 1 || grid[i][j + 1] == 0) as i32;
                perimeter += up + down + left + right;
            }
        }
    }
    perimeter
}
