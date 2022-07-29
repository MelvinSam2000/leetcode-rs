/*
    54 - Spiral Matrix
    Time: O(n*m)
    Space: O(n*m)
*/
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut out = vec![];

    let mut wall_right = n - 1;
    let mut wall_down = m - 1;
    let mut wall_left = 0;
    let mut wall_up = 0;

    let mut dir = Dir::Right;
    let (mut i, mut j) = (0, 0);
    while out.len() < m * n {
        out.push(matrix[i][j]);
        match dir {
            Dir::Right => {
                if j == wall_right {
                    wall_up += 1;
                    dir = Dir::Down;
                    i += 1;
                } else {
                    j += 1;
                }
            }
            Dir::Down => {
                if i == wall_down {
                    wall_right -= 1;
                    dir = Dir::Left;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
            Dir::Left => {
                if j == wall_left {
                    wall_down -= 1;
                    dir = Dir::Up;
                    i -= 1;
                } else {
                    j -= 1;
                }
            }
            Dir::Up => {
                if i == wall_up {
                    wall_left += 1;
                    dir = Dir::Right;
                    j += 1;
                } else {
                    i -= 1;
                }
            }
        }
    }
    out
}

enum Dir {
    Right,
    Down,
    Left,
    Up,
}

#[test]
fn t1() {
    let tcases = [(
        vec![
            vec![1, 2, 3, 4],
            vec![12, 13, 14, 5],
            vec![11, 16, 15, 6],
            vec![10, 9, 8, 7],
        ],
        (1..=16).collect::<Vec<i32>>(),
    )];
    for (param, res) in tcases {
        assert_eq!(spiral_order(param), res);
    }
}
