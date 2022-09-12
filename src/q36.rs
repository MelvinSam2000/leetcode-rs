/*
    36 - Valid Sudoku
    Time: O(1)
    Space: O(1)
    Note: Complexity is constant because the input dim is capped at 9
    If dim was n, Time: O(n^2), Space: O(n)
*/
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;

    // check row
    for i in 0..9 {
        let mut visited = HashSet::new();
        for j in 0..9 {
            if board[i][j] == '.' {
                continue;
            }
            if visited.contains(&board[i][j]) {
                return false;
            } else {
                visited.insert(board[i][j]);
            }
        }
    }

    // check col
    for j in 0..9 {
        let mut visited = HashSet::new();
        for i in 0..9 {
            if board[i][j] == '.' {
                continue;
            }
            if visited.contains(&board[i][j]) {
                return false;
            } else {
                visited.insert(board[i][j]);
            }
        }
    }

    // check regions
    for i in 0..9 {
        let mut visited = HashSet::new();
        for j in 0..9 {
            let row = 3 * (i / 3) + j / 3;
            let col = 3 * (i % 3) + j % 3;

            if board[row][col] == '.' {
                continue;
            }
            if visited.contains(&board[row][col]) {
                return false;
            } else {
                visited.insert(board[row][col]);
            }
        }
    }

    true
}
