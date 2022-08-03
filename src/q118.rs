/*
    118 - Pascals Triangle
    Time: O(n^2)
    Space: O(n^2)
*/
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    use std::mem::swap;

    let n = num_rows as usize;

    if n == 1 {
        return vec![vec![1]];
    }

    let mut vprev = vec![1; n];
    let mut vnext = vec![1; n];
    let mut out = vec![vec![1], vec![1, 1]];
    for i in 2..n {
        for j in 1..i {
            vnext[j] = vprev[j] + vprev[j - 1];
        }
        out.push(vnext[..=i].to_vec());
        swap(&mut vprev, &mut vnext);
    }
    out
}
