/*
    63 - Unique Paths II
    Time: O(m*n)
    Space: O(1)
    Note: We use the input matrix as our dp matrix to get O(1) space.
*/
pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    for i in 0..m {
        for j in 0..n {
            obstacle_grid[i][j] = if obstacle_grid[i][j] == 1 {
                -1
            } else {
                match (i, j) {
                    (0, 0) => 1,
                    (_, 0) => obstacle_grid[i - 1][j],
                    (0, _) => obstacle_grid[i][j - 1],
                    _ => match (obstacle_grid[i - 1][j], obstacle_grid[i][j - 1]) {
                        (-1, -1) => 0,
                        (x, -1) => x,
                        (-1, x) => x,
                        (x, y) => x + y,
                    },
                }
            }
        }
    }
    obstacle_grid[m - 1][n - 1].max(0)
}
