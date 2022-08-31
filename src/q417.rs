/*
    417 - Pacific Alantic Waterflow
    Time: O(m*n)
    Space: O(m*n)
*/
pub fn pacific_atlantic(h: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = h.len();
    let n = h[0].len();

    let mut pacific = vec![vec![false; n]; m];
    let mut atlantic = vec![vec![false; n]; m];

    for i in 0..m {
        helper((i, 0), (m, n), &mut pacific, &h);
        helper((i, n - 1), (m, n), &mut atlantic, &h);
    }

    for j in 0..n {
        helper((0, j), (m, n), &mut pacific, &h);
        helper((m - 1, j), (m, n), &mut atlantic, &h);
    }

    let mut res = vec![];
    for i in 0..m {
        for j in 0..n {
            if pacific[i][j] && atlantic[i][j] {
                res.push(vec![i as i32, j as i32]);
            }
        }
    }
    res
}

fn helper(pos: (usize, usize), dim: (usize, usize), ocean: &mut Vec<Vec<bool>>, h: &Vec<Vec<i32>>) {
    let (i, j) = pos;
    let (m, n) = dim;

    ocean[i][j] = true;
    if i != 0 && !ocean[i - 1][j] && h[i][j] <= h[i - 1][j] {
        helper((i - 1, j), (m, n), ocean, h);
    }
    if i != m - 1 && !ocean[i + 1][j] && h[i][j] <= h[i + 1][j] {
        helper((i + 1, j), (m, n), ocean, h);
    }
    if j != 0 && !ocean[i][j - 1] && h[i][j] <= h[i][j - 1] {
        helper((i, j - 1), (m, n), ocean, h);
    }
    if j != n - 1 && !ocean[i][j + 1] && h[i][j] <= h[i][j + 1] {
        helper((i, j + 1), (m, n), ocean, h);
    }
}
