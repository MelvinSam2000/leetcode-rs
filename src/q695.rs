/*
    695 - Max Area of Island
    Time: O(n*m)
    Space: O(n*m)
*/
pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::max;
    let m = grid.len();
    let n = grid[0].len();
    let mut visited = vec![vec![false; n]; m];
    let mut max_count = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 && !visited[i][j] {
                let mut count = 0;
                helper(
                    &grid,
                    &mut visited,
                    (i, j),
                    &mut count,
                    m,
                    n,
                );
                max_count = max(max_count, count);
            }
        }
    }
    max_count
}

fn helper(
    grid: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    pos: (usize, usize),
    count: &mut i32,
    m: usize,
    n: usize,
) {
    if grid[pos.0][pos.1] != 1 || visited[pos.0][pos.1] {
        return;
    }
    visited[pos.0][pos.1] = true;
    *count += 1;
    if pos.0 > 0 {
        helper(
            grid,
            visited,
            (pos.0 - 1, pos.1),
            count,
            m,
            n,
        );
    }
    if pos.1 > 0 {
        helper(
            grid,
            visited,
            (pos.0, pos.1 - 1),
            count,
            m,
            n,
        );
    }
    if pos.0 < m - 1 {
        helper(
            grid,
            visited,
            (pos.0 + 1, pos.1),
            count,
            m,
            n,
        );
    }
    if pos.1 < n - 1 {
        helper(
            grid,
            visited,
            (pos.0, pos.1 + 1),
            count,
            m,
            n,
        );
    }
}
