/*
    119 - Pascals Triangle II
    Time: O(n^2)
    Space: O(n)
*/
pub fn get_row(row_index: i32) -> Vec<i32> {
    use std::mem::swap;
    let n = row_index as usize;

    if n == 1 {
        return vec![1, 1];
    }

    let mut vprev = vec![1; n + 1];
    let mut vnext = vec![1; n + 1];
    for i in 2..=n {
        for j in 1..i {
            vnext[j] = vprev[j] + vprev[j - 1];
        }
        swap(&mut vprev, &mut vnext);
    }
    vprev
}
