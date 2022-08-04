/*
    289 - Game of Life
    Time: O(n*m)
    Space: O(n*m)
    Note: Can be optimized for O(1) space saving neighbours count on board.
*/
pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    use std::mem::swap;
    let m = board.len();
    let n = board[0].len();
    let mut aux = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            match (board[i][j], count((i, j), board, (m, n))) {
                //(1, 0..=1 | 4..=8) => {
                //    aux[i][j] = 0;
                //}
                (1, 2..=3) => {
                    aux[i][j] = 1;
                }
                (0, 3) => {
                    aux[i][j] = 1;
                }
                _ => {
                    //aux[i][j] = board[i][j];
                }
            }
        }
    }
    swap(board, &mut aux);
}

fn count(pos: (usize, usize), board: &Vec<Vec<i32>>, dim: (usize, usize)) -> usize {
    let (m, n) = dim;
    let (i, j) = pos;
    let i = i as i32;
    let j = j as i32;
    let neighbours = [
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ];
    let mut count = 0;
    for (i, j) in neighbours {
        if (0..m as i32).contains(&i)
            && (0..n as i32).contains(&j)
            && board[i as usize][j as usize] == 1
        {
            count += 1;
        }
    }
    count
}

#[test]
fn t1() {
    let tcases = [(
        vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]],
        vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]],
    )];
    for (mut before, after) in tcases {
        game_of_life(&mut before);
        assert_eq!(before, after);
    }
}
