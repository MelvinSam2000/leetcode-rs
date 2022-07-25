/*
    200 - Number of Islands
    Time: O(n*m)
    Space: O(n*m)
*/
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut visited = vec![vec![false; n]; m];
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            if !visited[i][j] && grid[i][j] == '1' {
                count += 1;
                mark_island(&grid, &mut visited, (i, j), m, n);
            }
        }
    }
    count
}

fn mark_island(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    pos: (usize, usize),
    m: usize,
    n: usize,
) {
    if grid[pos.0][pos.1] != '1' {
        return;
    }
    visited[pos.0][pos.1] = true;
    if pos.0 != m - 1 && !visited[pos.0 + 1][pos.1] {
        mark_island(grid, visited, (pos.0 + 1, pos.1), m, n);
    }
    if pos.1 != n - 1 && !visited[pos.0][pos.1 + 1] {
        mark_island(grid, visited, (pos.0, pos.1 + 1), m, n);
    }
    if pos.0 != 0 && !visited[pos.0 - 1][pos.1] {
        mark_island(grid, visited, (pos.0 - 1, pos.1), m, n);
    }
    if pos.1 != 0 && !visited[pos.0][pos.1 - 1] {
        mark_island(grid, visited, (pos.0, pos.1 - 1), m, n);
    }
}