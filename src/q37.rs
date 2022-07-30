/*
    37 - Sudoku Solver
    Time: O(9^n)
    Space: O(1)
    Note: Technically its O(1) since n = 9 always
*/
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    solve(board);
}

fn solve(board: &mut Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                for k in 1..=9 {
                    let k = char::from_digit(k, 10).unwrap();
                    if is_safe_to_assign(board, i, j, k) {
                        board[i][j] = k;
                        if solve(board) {
                            return true;
                        }
                        board[i][j] = '.';
                    }
                }
                return false;
            }
        }
    }
    true
}

fn is_safe_to_assign(board: &mut [Vec<char>], i: usize, j: usize, k: char) -> bool {
    for x in 0..9 {
        if board[x][j] == k {
            return false;
        }
        if board[i][x] == k {
            return false;
        }
        let box_i = i / 3;
        let box_j = j / 3;
        let little_i = x / 3;
        let little_j = x % 3;
        if board[3 * box_i + little_i][3 * box_j + little_j] == k {
            return false;
        }
    }
    true
}

#[test]
fn t1() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    solve_sudoku(&mut board);
    dbg!(&board);
}
