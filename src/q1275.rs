/*
    1275 - Find winner in tic tac toe game
    Time: O(1)
    Space: O(1)
*/
pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    let win_a = "A".to_owned();
    let win_b = "B".to_owned();
    let draw = "Draw".to_owned();
    let pending = "Pending".to_owned();

    let mut grid = [[None; 3]; 3];
    let mut turn_x = true;
    for mov in moves.iter() {
        let [i, j] = [mov[0] as usize, mov[1] as usize];
        grid[i][j] = Some(turn_x);
        turn_x = !turn_x;
    }

    // check row
    'a: for i in 0..3 {
        if let Some(tile) = grid[i][0] {
            for j in 1..3 {
                if grid[i][j] != Some(tile) {
                    continue 'a;
                }
            }
            return if tile { win_a } else { win_b };
        }
    }

    // check col
    'a: for j in 0..3 {
        if let Some(tile) = grid[0][j] {
            for i in 1..3 {
                if grid[i][j] != Some(tile) {
                    continue 'a;
                }
            }
            return if tile { win_a } else { win_b };
        }
    }

    // check diags
    if grid[0][0] == grid[1][1] && grid[0][0] == grid[2][2]
    {
        if let Some(winner_a) = grid[0][0] {
            return if winner_a { win_a } else { win_b };
        }
    }

    // check diags
    if grid[0][2] == grid[1][1] && grid[0][2] == grid[2][0]
    {
        if let Some(winner_a) = grid[1][1] {
            return if winner_a { win_a } else { win_b };
        }
    }

    if moves.len() == 9 {
        draw
    } else {
        pending
    }
}
