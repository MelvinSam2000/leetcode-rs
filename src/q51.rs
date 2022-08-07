/*
    51 - N Queens
    Time: O(n!)
    Space: O(n^2)
*/
use std::collections::HashSet;

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut solutions = vec![];
    let available = HashSet::from_iter(0..n);
    let mut board = vec![vec![false; n]; n];
    solve(&mut solutions, &mut board, available, 0);
    solutions
}

fn solve(
    solutions: &mut Vec<Vec<String>>,
    board: &mut Vec<Vec<bool>>,
    available: HashSet<usize>,
    row: usize,
) {
    if available.is_empty() {
        let sol = create_solution(board);
        solutions.push(sol);
        return;
    }

    for &col in available.iter() {
        if check_valid(board, (row, col)) {
            let mut available = available.clone();
            available.remove(&col);
            board[row][col] = true;
            solve(solutions, board, available, row + 1);
            board[row][col] = false;
        }
    }
}

fn check_valid(board: &Vec<Vec<bool>>, pos: (usize, usize)) -> bool {
    let n = board.len() as isize;
    let (row, col) = pos;
    // diag north west
    let (mut i, mut j) = (row as isize - 1, col as isize - 1);
    while i >= 0 && j >= 0 {
        if board[i as usize][j as usize] {
            return false;
        }
        i -= 1;
        j -= 1;
    }
    // diag north east
    let (mut i, mut j) = (row as isize - 1, col as isize + 1);
    while i >= 0 && j < n {
        if board[i as usize][j as usize] {
            return false;
        }
        i -= 1;
        j += 1;
    }
    // diag south west
    let (mut i, mut j) = (row as isize + 1, col as isize - 1);
    while i < n && j >= 0 {
        if board[i as usize][j as usize] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    // diag south east
    let (mut i, mut j) = (row as isize + 1, col as isize + 1);
    while i < n && j < n {
        if board[i as usize][j as usize] {
            return false;
        }
        i += 1;
        j += 1;
    }
    true
}

fn create_solution(sol: &Vec<Vec<bool>>) -> Vec<String> {
    sol.iter()
        .map(|row| {
            row.iter()
                .map(|&tile| if tile { 'Q' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
