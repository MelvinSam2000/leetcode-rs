/*
    1014 - Best Sightseeing pair
    Time: O(n)
    Space: O(1)
*/
pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut l = values[n - 1] - (n as i32 - 1);
    let mut res = 0;
    for i in (0..=n - 2).rev() {
        let r = values[i] + i as i32;
        l = l.max(values[i + 1] - (i as i32 + 1));
        res = res.max(l + r);
    }
    res
}
