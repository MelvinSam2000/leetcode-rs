/*
    657 - Robot return to origin
    Time: O(n)
    Space: O(1)
*/
pub fn judge_circle(moves: String) -> bool {
    moves.as_bytes().iter().fold((0, 0), |mut pos, step| {
        let new_pos = match step {
            b'U' => (0, 1),
            b'D' => (0, -1),
            b'L' => (1, 0),
            b'R' => (-1, 0),
            _ => unreachable!(),
        };
        pos.0 += new_pos.0;
        pos.1 += new_pos.1;
        pos
    }) == (0, 0)
}
