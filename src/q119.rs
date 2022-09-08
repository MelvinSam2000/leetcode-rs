/*
    119 - Pascals Triangle II
    Time: O(n^2)
    Space: O(n)
*/
pub fn get_row(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut dp: [Vec<i32>; 2] =
        [vec![1; n + 1], vec![1; n + 1]];
    for i in 2..=n {
        for j in 1..i {
            dp[0][j] = dp[1][j] + dp[1][j - 1];
        }
        dp.swap(0, 1);
    }
    let [.., out] = dp;
    out
}
