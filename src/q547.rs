/*
    547 - Number of Provinces
    Time: O(n^3)
    Space: O(n)
    Note: Time is VERY unlikely to be O(n^3),
    since it assumes union-find parent array becomes one linked list.
    Thus its average O(n^2).
    Adding a rank array to help balance the parents array can help mitigate this.
*/
pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut count = n as i32;
    let mut parent = (0..n).collect::<Vec<_>>();
    for i in 0..n {
        for j in 0..i {
            if is_connected[i][j] == 1 {
                let pi = find(i, &parent);
                let pj = find(j, &parent);
                if pi != pj {
                    parent[pi] = pj;
                    count -= 1;
                }
            }
        }
    }
    count
}

fn find(mut index: usize, parent: &[usize]) -> usize {
    while index != parent[index] {
        index = parent[index];
    }
    index
}

#[test]
fn t1() {
    let tcases = [(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]], 2)];

    for (grid, count) in tcases {
        assert_eq!(find_circle_num(grid), count);
    }
}
