/*
    1014 - Best Sightseeing pair
    Time: O(n)
    Space: O(1)
*/
pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut dp = a[n - 1] - n as i32 + 1;
    let mut res = 0;
    for i in (0..n - 1).rev() {
        dp = dp.max(a[i + 1] - i as i32 - 1);
        res = res.max(a[i] + i as i32 + dp);
    }
    res
}
