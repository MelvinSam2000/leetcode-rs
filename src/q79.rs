/*
    79 - Word Search
    Time: O(n^2*m^2), or O(n*m*4^k)
    Space: O(n*m)
    Note: m x n board, word is length k.
*/
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    let n = board[0].len();

    let mut visited = vec![vec![false; n]; m];

    for i in 0..m {
        for j in 0..n {
            if helper(&board, word.as_bytes(), &mut visited, (i, j), (m, n)) {
                return true;
            }
        }
    }

    false
}

fn helper(
    board: &Vec<Vec<char>>,
    word: &[u8],
    visited: &mut Vec<Vec<bool>>,
    pos: (usize, usize),
    dim: (usize, usize),
) -> bool {
    let (i, j) = pos;
    let (m, n) = dim;

    if visited[i][j] {
        return false;
    }

    if word.is_empty() {
        return true;
    }
    if board[i][j] as u8 != word[0] {
        return false;
    }

    if word.len() == 1 {
        return true;
    }

    visited[i][j] = true;
    if i > 0 && helper(board, &word[1..], visited, (i - 1, j), dim) {
        return true;
    }
    if j > 0 && helper(board, &word[1..], visited, (i, j - 1), dim) {
        return true;
    }
    if i < m - 1 && helper(board, &word[1..], visited, (i + 1, j), dim) {
        return true;
    }
    if j < n - 1 && helper(board, &word[1..], visited, (i, j + 1), dim) {
        return true;
    }
    visited[i][j] = false;
    false
}
