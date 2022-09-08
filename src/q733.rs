/*
    733 - Flood Fill
    Time: O(n*m)
    Space: O(n*m)
*/
pub fn flood_fill(
    mut image: Vec<Vec<i32>>,
    sr: i32,
    sc: i32,
    color: i32,
) -> Vec<Vec<i32>> {
    let m = image.len();
    let n = image[0].len();
    let mut marked = vec![vec![false; n]; m];
    helper(
        &mut image,
        (sr as usize, sc as usize),
        color,
        &mut marked,
        m,
        n,
    );
    for i in 0..m {
        for j in 0..n {
            if marked[i][j] {
                image[i][j] = color;
            }
        }
    }
    image
}

fn helper(
    image: &mut Vec<Vec<i32>>,
    pos: (usize, usize),
    color: i32,
    marked: &mut Vec<Vec<bool>>,
    m: usize,
    n: usize,
) {
    marked[pos.0][pos.1] = true;
    if pos.0 != m - 1
        && !marked[pos.0 + 1][pos.1]
        && image[pos.0][pos.1] == image[pos.0 + 1][pos.1]
    {
        helper(
            image,
            (pos.0 + 1, pos.1),
            color,
            marked,
            m,
            n,
        );
    }
    if pos.1 != n - 1
        && !marked[pos.0][pos.1 + 1]
        && image[pos.0][pos.1] == image[pos.0][pos.1 + 1]
    {
        helper(
            image,
            (pos.0, pos.1 + 1),
            color,
            marked,
            m,
            n,
        );
    }
    if pos.0 != 0
        && !marked[pos.0 - 1][pos.1]
        && image[pos.0][pos.1] == image[pos.0 - 1][pos.1]
    {
        helper(
            image,
            (pos.0 - 1, pos.1),
            color,
            marked,
            m,
            n,
        );
    }
    if pos.1 != 0
        && !marked[pos.0][pos.1 - 1]
        && image[pos.0][pos.1] == image[pos.0][pos.1 - 1]
    {
        helper(
            image,
            (pos.0, pos.1 - 1),
            color,
            marked,
            m,
            n,
        );
    }
}
